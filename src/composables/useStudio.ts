import {
  computed,
  inject,
  onMounted,
  onUnmounted,
  provide,
  reactive,
  shallowReadonly,
  watch,
} from "vue";
import type { InjectionKey } from "vue";
import type {
  GenerationParams,
  GenerationTask,
  ImageResult,
  Preset,
  Provider,
  ReferenceImage,
  StoredWorkspace,
  Toast,
  Workspace,
} from "../types";
import {
  createInitialHistory,
  defaultParams,
  defaultPresets,
  defaultProviders,
  initialPrompt,
  sampleImages,
} from "../data/demo";
import { chooseSample, validateGeneration } from "../services/generation";
import {
  asBackendError,
  backendAvailable,
  cancelDesktopGeneration,
  generateImages,
  usingDesktopBackend,
} from "../services/backend";
import {
  addHistory,
  deleteApiKey,
  listTasks,
  loadWorkspace,
  removeStoredProvider,
  saveWorkspace,
  setHistoryFavorite,
  storeApiKey,
  upsertTask,
} from "../services/storage";
import { t, tp } from "../i18n";

function createStudio() {
  const initialHistory = createInitialHistory();
  const state = reactive({
    page: "generate" as Workspace,
    prompt: initialPrompt,
    params: { ...defaultParams },
    providers: structuredClone(defaultProviders),
    history: initialHistory,
    presets: structuredClone(defaultPresets),
    references: [] as ReferenceImage[],
    results: initialHistory.slice(0, 4),
    selectedId: initialHistory[0].id,
    generating: false,
    progress: 0,
    taskError: "",
    failNext: false,
    toasts: [] as Toast[],
    saving: false,
    persistenceError: false,
    preferMock: false,
    ready: false,
    tasks: [] as GenerationTask[],
  });
  let progressTimer: ReturnType<typeof setInterval> | undefined;
  let persistTimer: ReturnType<typeof setTimeout> | undefined;
  let activeJob: { id: string; controller: AbortController } | null = null;
  const toastTimers = new Set<ReturnType<typeof setTimeout>>();
  let lastRequest: {
    prompt: string;
    params: GenerationParams;
    references: ReferenceImage[];
  } | null = null;
  interface QueuedTask {
    taskId: string;
    request: {
      prompt: string;
      params: GenerationParams;
      references: ReferenceImage[];
    };
  }
  const taskQueue: QueuedTask[] = [];
  const QUEUE_LIMIT = 8;
  let disposed = false;
  function persistSideEffect(operation: Promise<void>) {
    operation.catch(() => {
      if (!state.persistenceError)
        notify(t("toast.persistFail"), "error");
      state.persistenceError = true;
    });
  }
  const currentResult = computed(() =>
    state.results.find((image) => image.id === state.selectedId),
  );
  const provider = computed(
    () =>
      state.providers.find((item) => item.id === state.params.providerId) ??
      state.providers[0],
  );

  function notify(message: string, kind: Toast["kind"] = "success") {
    const id = crypto.randomUUID();
    state.toasts.push({ id, message, kind });
    const timer = setTimeout(
      () => {
        dismissToast(id);
        toastTimers.delete(timer);
      },
      kind === "error" ? 6500 : 4000,
    );
    toastTimers.add(timer);
  }
  function dismissToast(id: string) {
    state.toasts = state.toasts.filter((toast) => toast.id !== id);
  }
  function navigate(page: Workspace) {
    state.page = page;
  }
  function setPrompt(prompt: string) {
    state.prompt = prompt;
  }
  function updateParams(patch: Partial<GenerationParams>) {
    state.params = { ...state.params, ...patch };
  }
  function selectProvider(id: string) {
    const next = state.providers.find((item) => item.id === id);
    if (next) updateParams({ providerId: id, model: next.defaultModel });
  }
  function resetParams() {
    state.params = { ...defaultParams };
    notify(t("toast.reset"));
  }
  function selectImage(id: string) {
    state.selectedId = id;
  }
  function toggleFavorite(id: string) {
    const image = state.history.find((item) => item.id === id);
    if (image) {
      image.favorite = !image.favorite;
      persistSideEffect(setHistoryFavorite(id, image.favorite));
      notify(
        image.favorite ? t("toast.favAdd") : t("toast.favRemove"),
      );
    }
  }
  function openHistory(image: ImageResult) {
    state.results = state.history.filter(
      (item) => item.batchId === image.batchId,
    );
    state.selectedId = image.id;
    navigate("generate");
  }
  function reuseImage(image: ImageResult) {
    state.prompt = image.prompt;
    state.params = { ...image.params, seed: image.seed };
    state.references = image.references.map((item) => ({ ...item }));
    openHistory(image);
    notify(t("toast.restored"));
  }
  function applyPreset(preset: Preset) {
    state.prompt = preset.prompt;
    state.params = { ...preset.params };
    state.references = [];
    state.taskError = "";
    navigate("generate");
    notify(t("toast.presetApplied", { name: preset.name }));
  }
  function savePreset(
    name: string,
    description: string,
    prompt = state.prompt,
    existingId?: string,
  ) {
    const existing = state.presets.find((preset) => preset.id === existingId);
    if (existing)
      Object.assign(existing, {
        name: name.trim(),
        description: description.trim(),
        prompt: prompt.trim(),
      });
    else
      state.presets.unshift({
        id: crypto.randomUUID(),
        name: name.trim(),
        description: description.trim(),
        prompt: prompt.trim(),
        category: "My presets",
        params: { ...state.params },
        image: sampleImages[chooseSample(prompt, 0)],
      });
    notify(
      existing ? t("toast.presetUpdated") : t("toast.presetSaved"),
    );
  }
  function removePreset(id: string) {
    state.presets = state.presets.filter((item) => item.id !== id);
    notify(t("toast.presetDeleted"));
  }
  function saveProvider(value: Provider) {
    const stored = { ...value, models: [...value.models] };
    if (backendAvailable() && stored.apiKey.trim()) {
      const key = stored.apiKey.trim();
      stored.apiKey = "";
      stored.hasKey = true;
      storeApiKey(value.id, key).catch(() => {
        const target = state.providers.find((item) => item.id === value.id);
        if (target) target.hasKey = false;
        notify(t("toast.keyStoreFail"), "error");
      });
    }
    const index = state.providers.findIndex((item) => item.id === value.id);
    if (index >= 0) state.providers[index] = stored;
    else state.providers.push(stored);
    if (
      state.params.providerId === value.id &&
      !value.models.includes(state.params.model)
    )
      updateParams({ model: value.defaultModel });
    notify(t("toast.providerSaved"));
  }
  function clearProviderKey(id: string) {
    const target = state.providers.find((item) => item.id === id);
    if (!target) return;
    target.hasKey = false;
    persistSideEffect(deleteApiKey(id));
    notify(t("toast.keyForgotten"), "info");
  }
  function addProvider() {
    const customs = state.providers.filter(
      (item) => item.kind === "custom",
    ).length;
    const provider: Provider = {
      id: `custom-${crypto.randomUUID().slice(0, 8)}`,
      name: `Custom Provider ${customs + 1}`,
      kind: "custom",
      description: "Bring your own image generation endpoint.",
      baseUrl: "https://",
      apiKey: "",
      models: ["custom-image-model"],
      defaultModel: "custom-image-model",
      status: "untested",
    };
    state.providers.push(provider);
    notify(t("toast.providerAdded"), "info");
    return provider;
  }
  function removeProvider(id: string) {
    const target = state.providers.find((item) => item.id === id);
    if (!target || target.kind !== "custom") return;
    state.providers = state.providers.filter((item) => item.id !== id);
    if (state.params.providerId === id && state.providers.length)
      selectProvider(state.providers[0].id);
    persistSideEffect(removeStoredProvider(id));
    notify(t("toast.providerRemoved", { name: target.name }), "info");
  }
  async function addReferences(files: File[]) {
    for (const file of files) {
      if (state.references.length >= 3) {
        notify(t("toast.refLimit"), "error");
        break;
      }
      if (!["image/jpeg", "image/png", "image/webp"].includes(file.type)) {
        notify(t("toast.refType"), "error");
        continue;
      }
      if (file.size > 10 * 1024 * 1024) {
        notify(t("toast.refSize"), "error");
        continue;
      }
      try {
        const src = await new Promise<string>((resolve, reject) => {
          const reader = new FileReader();
          reader.onload = () => resolve(String(reader.result));
          reader.onerror = () => reject(new Error("File could not be read"));
          reader.readAsDataURL(file);
        });
        const image = new Image();
        image.src = src;
        await image.decode();
        if (state.references.length >= 3) break;
        state.references.push({
          id: crypto.randomUUID(),
          name: file.name,
          src,
          strength: 65,
        });
        notify(t("toast.refAdded"));
      } catch {
        notify(t("toast.refUnreadable"), "error");
      }
    }
  }
  function removeReference(id: string) {
    state.references = state.references.filter((item) => item.id !== id);
  }
  function setReferenceStrength(id: string, strength: number) {
    const reference = state.references.find((item) => item.id === id);
    if (reference) reference.strength = strength;
  }
  function taskFrom(
    request: {
      prompt: string;
      params: GenerationParams;
      references: ReferenceImage[];
    },
    status: GenerationTask["status"],
  ): GenerationTask {
    return {
      id: crypto.randomUUID(),
      providerId: request.params.providerId,
      model: request.params.model,
      prompt: request.prompt,
      params: { ...request.params },
      status,
      createdAt: new Date().toISOString(),
      resultCount: 0,
    };
  }
  function updateTask(
    id: string,
    patch: Partial<GenerationTask>,
  ) {
    const task = state.tasks.find((item) => item.id === id);
    if (!task) return;
    Object.assign(task, patch);
    persistSideEffect(upsertTask({ ...task, params: { ...task.params } }));
  }
  function startNextTask() {
    const next = taskQueue.shift();
    if (next) void runTask(next.taskId, next.request);
  }
  async function generate(retry = false) {
    if (!state.ready) return;
    const request =
      retry && lastRequest
        ? lastRequest
        : {
            prompt: state.prompt.trim(),
            params: { ...state.params },
            references: state.references.map((item) => ({ ...item })),
          };
    const error = validateGeneration(request.prompt, request.params);
    if (error) {
      lastRequest = null;
      state.taskError = error;
      notify(error, "error");
      return;
    }
    lastRequest = request;
    if (state.generating) {
      if (taskQueue.length >= QUEUE_LIMIT) {
        notify(t("toast.queueFull"), "error");
        return;
      }
      const task = taskFrom(request, "pending");
      state.tasks.unshift(task);
      persistSideEffect(upsertTask({ ...task }));
      taskQueue.push({ taskId: task.id, request });
      notify(t("toast.queued"), "info");
      return;
    }
    state.taskError = "";
    const task = taskFrom(request, "running");
    state.tasks.unshift(task);
    void runTask(task.id, request);
  }
  async function runTask(
    taskId: string,
    request: {
      prompt: string;
      params: GenerationParams;
      references: ReferenceImage[];
    },
  ) {
    updateTask(taskId, { status: "running" });
    const taskProvider =
      state.providers.find(
        (item) => item.id === request.params.providerId,
      ) ?? provider.value;
    state.generating = true;
    state.progress = 0;
    const fail = state.failNext;
    state.failNext = false;
    const seed = request.params.seed
      ? Number(request.params.seed)
      : Math.floor(Math.random() * 4294967291);
    const requestId = taskId;
    const controller = new AbortController();
    activeJob = { id: requestId, controller };
    progressTimer = setInterval(() => {
      state.progress = Math.min(95, state.progress + 4);
    }, 180);
    try {
      const produced = await generateImages(
        {
          requestId,
          prompt: request.prompt,
          params: request.params,
          provider: taskProvider,
          references: request.references,
        },
        {
          failNext: fail,
          preferMock: state.preferMock,
          seed,
          signal: controller.signal,
        },
      );
      if (controller.signal.aborted || disposed) return;
      state.progress = 100;
      const batchId = crypto.randomUUID();
      const results: ImageResult[] = produced.map((item) => ({
        id: crypto.randomUUID(),
        src: item.src,
        path: item.path,
        thumb: item.thumb,
        title: request.prompt
          .split(/[,.]/)[0]
          .split(" ")
          .slice(0, 6)
          .join(" "),
        prompt: request.prompt,
        params: { ...request.params },
        createdAt: new Date().toISOString(),
        favorite: false,
        width: item.width,
        height: item.height,
        seed: item.seed,
        filter: item.filter,
        position: item.position,
        batchId,
        references: request.references,
      }));
      state.results = results;
      state.selectedId = results[0].id;
      state.history.unshift(...results);
      persistSideEffect(addHistory(results));
      updateTask(taskId, {
        status: "succeeded",
        finishedAt: new Date().toISOString(),
        resultCount: results.length,
      });
      if (usingDesktopBackend(state.preferMock)) {
        const target = state.providers.find(
          (item) => item.id === request.params.providerId,
        );
        if (target) target.status = "connected";
      }
      notify(tp("toast.ready", results.length));
    } catch (cause) {
      if (controller.signal.aborted || disposed) return;
      const backend = asBackendError(cause);
      const kind = t(`err.${backend.kind}`);
      state.taskError = backend.message;
      updateTask(taskId, {
        status: backend.kind === "cancelled" ? "cancelled" : "failed",
        errorKind: backend.kind,
        errorMessage: backend.message,
        finishedAt: new Date().toISOString(),
      });
      notify(`${kind} · ${backend.message}`, "error");
      if (backend.kind === "auth" || backend.kind === "config") {
        const target = state.providers.find(
          (item) => item.id === request.params.providerId,
        );
        if (target) target.status = "error";
      }
    } finally {
      clearInterval(progressTimer);
      activeJob = null;
      state.generating = false;
      startNextTask();
    }
  }
  function cancelGeneration() {
    if (!state.generating) return;
    const job = activeJob;
    activeJob = null;
    job?.controller.abort();
    if (job) {
      cancelDesktopGeneration(job.id);
      updateTask(job.id, {
        status: "cancelled",
        finishedAt: new Date().toISOString(),
      });
    }
    clearInterval(progressTimer);
    state.generating = false;
    state.progress = 0;
    notify(t("toast.canceled"), "info");
    startNextTask();
  }
  function cancelTask(id: string) {
    const index = taskQueue.findIndex((item) => item.taskId === id);
    if (index >= 0) {
      taskQueue.splice(index, 1);
      updateTask(id, {
        status: "cancelled",
        finishedAt: new Date().toISOString(),
      });
      notify(t("toast.canceled"), "info");
      return;
    }
    if (activeJob?.id === id) cancelGeneration();
  }
  function newSession() {
    if (state.generating) {
      notify(t("toast.busy"), "info");
      return;
    }
    state.prompt = "";
    state.references = [];
    state.results = [];
    state.selectedId = "";
    state.taskError = "";
    navigate("generate");
    notify(t("toast.newSession"));
  }
  function setFailNext(value: boolean) {
    state.failNext = value;
  }
  function setPreferMock(value: boolean) {
    state.preferMock = value;
    notify(
      value ? t("toast.mockOn") : t("toast.mockOff"),
      "info",
    );
  }
  function snapshot(): StoredWorkspace {
    return {
      version: 1,
      prompt: state.prompt,
      params: state.params,
      history: state.history,
      presets: state.presets,
      references: state.references,
      preferMock: state.preferMock,
      selectedId: state.selectedId,
      resultIds: state.results.map((image) => image.id),
      providers: state.providers.map(({ apiKey: _key, ...value }) => ({
        ...value,
        status: value.status === "testing" ? "untested" : value.status,
      })),
    };
  }
  async function persist() {
    state.saving = true;
    try {
      await saveWorkspace(snapshot());
      state.persistenceError = false;
    } catch {
      if (!state.persistenceError)
        notify(t("toast.persistFail"), "error");
      state.persistenceError = true;
    } finally {
      state.saving = false;
    }
  }
  watch(
    () => [
      state.prompt,
      state.params,
      state.history,
      state.presets,
      state.providers,
      state.references,
      state.selectedId,
      state.results,
    ],
    () => {
      if (!state.ready) return;
      clearTimeout(persistTimer);
      persistTimer = setTimeout(persist, 250);
    },
    { deep: true },
  );
  onMounted(async () => {
    try {
      state.tasks = await listTasks();
    } catch {
      state.tasks = [];
    }
    try {
      const saved = await loadWorkspace();
      if (saved && !disposed) {
        state.prompt =
          typeof saved.prompt === "string" ? saved.prompt : initialPrompt;
        state.params = { ...defaultParams, ...saved.params };
        state.history = Array.isArray(saved.history)
          ? saved.history
          : initialHistory;
        state.presets = Array.isArray(saved.presets)
          ? saved.presets
          : structuredClone(defaultPresets);
        state.references = saved.references ?? [];
        state.preferMock = saved.preferMock === true;
        if (saved.providers?.length)
          state.providers = saved.providers.map((value) => ({
            ...value,
            apiKey: "",
          }));
        const latest = state.history[0];
        state.results = Array.isArray(saved.resultIds)
          ? saved.resultIds
              .map((id) => state.history.find((image) => image.id === id))
              .filter((image): image is ImageResult => !!image)
          : latest
            ? state.history.filter((item) => item.batchId === latest.batchId)
            : [];
        state.selectedId = state.results.some(
          (image) => image.id === saved.selectedId,
        )
          ? saved.selectedId!
          : (state.results[0]?.id ?? "");
      }
    } catch {
      state.persistenceError = true;
      notify(t("toast.restoreFail"), "info");
    }
    state.ready = true;
  });
  onUnmounted(() => {
    disposed = true;
    activeJob?.controller.abort();
    if (activeJob) cancelDesktopGeneration(activeJob.id);
    clearInterval(progressTimer);
    clearTimeout(persistTimer);
    toastTimers.forEach(clearTimeout);
  });
  return {
    state: shallowReadonly(state),
    currentResult,
    provider,
    notify,
    dismissToast,
    navigate,
    setPrompt,
    updateParams,
    selectProvider,
    resetParams,
    selectImage,
    toggleFavorite,
    openHistory,
    reuseImage,
    applyPreset,
    savePreset,
    removePreset,
    saveProvider,
    clearProviderKey,
    addProvider,
    removeProvider,
    addReferences,
    removeReference,
    setReferenceStrength,
    generate,
    cancelGeneration,
    cancelTask,
    newSession,
    setFailNext,
    setPreferMock,
  };
}

const studioKey: InjectionKey<ReturnType<typeof createStudio>> =
  Symbol.for("imagen-studio");
export function provideStudio() {
  const studio = createStudio();
  provide(studioKey, studio);
  return studio;
}
export function useStudio() {
  const studio = inject(studioKey);
  if (!studio) throw new Error("Studio must be provided by App");
  return studio;
}

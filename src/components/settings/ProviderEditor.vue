<script setup lang="ts">
import { computed, onUnmounted, ref, shallowRef, watch } from "vue";
import {
  Eye,
  EyeOff,
  Check,
  LoaderCircle,
  PlugZap,
  AlertCircle,
  KeyRound,
  ShieldCheck,
  Trash2,
  RefreshCw,
} from "lucide-vue-next";
import type { Provider } from "../../types";
import { validateProvider } from "../../services/generation";
import {
  asBackendError,
  backendAvailable,
  describeError,
  testProviderConnection,
} from "../../services/backend";
import type { ConnectionTestResult } from "../../services/backend";
import { useI18n } from "../../i18n";
const props = defineProps<{ provider: Provider }>();
const emit = defineEmits<{
  save: [provider: Provider];
  dirty: [value: boolean];
  remove: [];
  forgetKey: [];
}>();
const draft = ref<Provider>({
  ...props.provider,
  models: [...props.provider.models],
});
const visibleKey = shallowRef(false);
const storedKey = shallowRef(!!props.provider.hasKey);
const testing = shallowRef(false);
const error = shallowRef("");
const tested = shallowRef(false);
const testDetail = shallowRef("");
const fetching = shallowRef(false);
const live = backendAvailable();
const { t } = useI18n();
const MOCK_MODELS = ["imagen-mock-xl", "imagen-mock-fast", "imagen-mock-turbo"];
const customModel = shallowRef(props.provider.defaultModel);
const customEntry = shallowRef(
  !!props.provider.defaultModel &&
    !props.provider.models.includes(props.provider.defaultModel),
);
let timer: ReturnType<typeof setTimeout> | undefined;
const dirty = computed(
  () =>
    JSON.stringify(draft.value) !== JSON.stringify(props.provider) ||
    customModel.value !== props.provider.defaultModel,
);
watch(dirty, (value) => emit("dirty", value), { immediate: true });
watch(
  () => props.provider,
  (provider) => {
    clearTimeout(timer);
    draft.value = { ...provider, models: [...provider.models] };
    storedKey.value = !!provider.hasKey;
    customModel.value = provider.defaultModel;
    customEntry.value =
      !!provider.defaultModel &&
      !provider.models.includes(provider.defaultModel);
    visibleKey.value = false;
    testing.value = false;
    fetching.value = false;
    tested.value = false;
    testDetail.value = "";
    error.value = "";
  },
  { deep: true },
);
function changed() {
  tested.value = false;
  draft.value.status = "untested";
  error.value = "";
}
async function test() {
  const validation = validateProvider(draft.value.baseUrl, customModel.value);
  if (validation) {
    error.value = validation;
    draft.value.status = "error";
    return;
  }
  testing.value = true;
  tested.value = false;
  testDetail.value = "";
  error.value = "";
  draft.value.status = "testing";
  if (live) {
    try {
      const result = await testProviderConnection({
        baseUrl: draft.value.baseUrl.trim(),
        apiKey: draft.value.apiKey,
        providerId: props.provider.id,
      });
      draft.value.status = "connected";
      tested.value = true;
      testDetail.value = result.models.length
        ? t("provider.testSuccess.models", {
            n: result.models.length,
            ms: result.latencyMs,
          })
        : t("provider.testSuccess.noModels", { ms: result.latencyMs });
    } catch (cause) {
      error.value = describeError(asBackendError(cause));
      draft.value.status = "error";
    } finally {
      testing.value = false;
    }
    return;
  }
  timer = setTimeout(() => {
    testing.value = false;
    if (/fail|invalid/i.test(draft.value.baseUrl)) {
      error.value = t("provider.mockTimeout");
      draft.value.status = "error";
    } else {
      draft.value.status = "connected";
      tested.value = true;
      testDetail.value = t("provider.testSuccess.mock", { ms: 120 });
    }
  }, 1200);
}
async function fetchModels() {
  const validation = validateProvider(
    draft.value.baseUrl,
    customModel.value.trim() || "model",
  );
  if (validation) {
    error.value = validation;
    draft.value.status = "error";
    return;
  }
  fetching.value = true;
  tested.value = false;
  testDetail.value = "";
  error.value = "";
  try {
    const result: ConnectionTestResult = live
      ? await testProviderConnection({
          baseUrl: draft.value.baseUrl.trim(),
          apiKey: draft.value.apiKey,
          providerId: props.provider.id,
        })
      : await new Promise<ConnectionTestResult>((resolve) =>
          setTimeout(
            () => resolve({ models: [...MOCK_MODELS], latencyMs: 120 }),
            900,
          ),
        );
    if (!result.models.length) {
      error.value = t("provider.noModels");
      draft.value.status = "error";
      return;
    }
    draft.value.models = result.models;
    if (!result.models.includes(customModel.value))
      customModel.value = result.models[0];
    customEntry.value = false;
    draft.value.status = "connected";
    tested.value = true;
    testDetail.value = t(
      live ? "provider.fetchSuccess" : "provider.fetchSuccess.mock",
      { n: result.models.length, ms: result.latencyMs },
    );
  } catch (cause) {
    error.value = describeError(asBackendError(cause));
    draft.value.status = "error";
  } finally {
    fetching.value = false;
  }
}
function pickModel(event: Event) {
  const model = (event.target as HTMLSelectElement).value;
  if (model === "__custom") {
    customEntry.value = true;
    customModel.value = "";
  } else {
    customModel.value = model;
  }
  changed();
}
function useModelList() {
  customEntry.value = false;
  if (!draft.value.models.includes(customModel.value))
    customModel.value = draft.value.models[0] ?? "";
  changed();
}
function save() {
  const validation = validateProvider(draft.value.baseUrl, customModel.value);
  if (validation) {
    error.value = validation;
    return;
  }
  const model = customModel.value.trim();
  const models = [...new Set([...draft.value.models, model])];
  emit("save", {
    ...draft.value,
    hasKey: storedKey.value || !!draft.value.apiKey.trim(),
    name: draft.value.name.trim() || "Custom Provider",
    baseUrl: draft.value.baseUrl.trim(),
    models,
    defaultModel: model,
  });
}
function reset() {
  draft.value = { ...props.provider, models: [...props.provider.models] };
  customModel.value = props.provider.defaultModel;
  error.value = "";
  tested.value = false;
}
onUnmounted(() => clearTimeout(timer));
</script>
<template>
  <form class="provider-editor" @submit.prevent="save">
    <div class="editor-heading">
      <div>
        <h3>{{ provider.name }}</h3>
        <p>{{ provider.description }}</p>
      </div>
      <span
        class="badge"
        :class="{ 'badge-success': draft.status === 'connected' }"
        ><span class="status-dot" :class="draft.status" />{{
          testing
            ? t("provider.testing")
            : draft.status === "connected"
              ? t("provider.connected")
              : draft.status === "error"
                ? t("provider.failed")
                : t("provider.untested")
        }}</span
      >
    </div>
    <div class="mock-banner">
      <ShieldCheck :size="16" /><span v-if="live">{{
        t("provider.banner.live")
      }}</span
      ><span v-else>{{ t("provider.banner.mock") }}</span>
    </div>
    <div class="editor-fields">
      <label v-if="provider.kind === 'custom'" class="field"
        ><span class="field-label">{{ t("provider.name") }}</span
        ><input
          v-model="draft.name"
          required
          maxlength="45"
          :disabled="testing"
          :aria-label="t('provider.name')"
          @input="changed" /></label
      ><label class="field"
        ><span class="field-label">{{ t("provider.baseUrl") }}</span
        ><input
          v-model="draft.baseUrl"
          type="url"
          required
          :disabled="testing"
          spellcheck="false"
          :aria-label="t('provider.baseUrl')"
          placeholder="https://api.example.com/v1"
          @input="changed"
        /><span class="field-hint">{{ t("provider.baseUrlHint") }}</span></label
      >
      <div class="field">
        <label for="provider-api-key" class="field-label"
          >{{ t("provider.apiKey")
          }}<span class="badge">{{
            storedKey ? t("provider.secureBadge") : t("provider.sessionOnly")
          }}</span></label
        >
        <div class="key-field">
          <KeyRound :size="14" /><input
            id="provider-api-key"
            v-model="draft.apiKey"
            :type="visibleKey ? 'text' : 'password'"
            :disabled="testing"
            autocomplete="off"
            spellcheck="false"
            :placeholder="
              storedKey && !draft.apiKey
                ? t('provider.keyPlaceholder.stored')
                : live
                  ? t('provider.keyPlaceholder.live')
                  : t('provider.keyPlaceholder.mock')
            "
            @input="changed"
          /><button
            type="button"
            class="icon-btn"
            :aria-label="
              visibleKey ? t('provider.hideKey') : t('provider.showKey')
            "
            @click="visibleKey = !visibleKey"
          >
            <EyeOff v-if="visibleKey" :size="15" /><Eye v-else :size="15" />
          </button>
        </div>
        <span class="field-hint">{{
          storedKey
            ? t("provider.keyHint.stored")
            : live
              ? t("provider.keyHint.live")
              : t("provider.keyHint.mock")
        }}</span
        ><button
          v-if="storedKey && !draft.apiKey"
          type="button"
          class="model-list-link"
          @click="emit('forgetKey')"
        >
          {{ t("provider.forgetKey") }}
        </button>
      </div>
      <div class="field">
        <span class="field-label"
          ><label for="provider-default-model">{{
            t("provider.defaultModel")
          }}</label
          ><button
            class="icon-btn"
            type="button"
            :disabled="testing || fetching"
            :aria-label="t('provider.fetchModels')"
            :data-tip="t('provider.fetchModels')"
            @click="fetchModels"
          >
            <LoaderCircle v-if="fetching" :size="13" class="spin" /><RefreshCw
              v-else
              :size="13"
          /></button></span
        ><select
          v-if="!customEntry"
          id="provider-default-model"
          :value="customModel"
          :disabled="testing || fetching"
          :aria-label="t('provider.defaultModel')"
          @change="pickModel"
        >
          <option v-for="model in draft.models" :key="model" :value="model">
            {{ model }}
          </option>
          <option value="__custom">{{
            t("provider.customOption")
          }}</option></select
        ><template v-else
          ><input
            id="provider-default-model"
            v-model="customModel"
            required
            :disabled="testing || fetching"
            spellcheck="false"
            :placeholder="t('provider.customModelPlaceholder')"
            @input="changed"
          /><button
            v-if="draft.models.length"
            type="button"
            class="model-list-link"
            @click="useModelList"
          >
            {{ t("provider.useModelList", { n: draft.models.length }) }}
          </button></template
        ><span class="field-hint">{{
          live ? t("provider.modelHint.live") : t("provider.modelHint.mock")
        }}</span>
      </div>
    </div>
    <div v-if="error" class="connection-feedback error" role="alert">
      <AlertCircle :size="15" /><span>{{ error }}</span>
    </div>
    <div v-else-if="tested" class="connection-feedback success" role="status">
      <Check :size="15" /><span>{{ testDetail }}</span>
    </div>
    <div class="connection-test">
      <button class="btn" type="button" :disabled="testing" @click="test">
        <LoaderCircle v-if="testing" :size="14" class="spin" /><PlugZap
          v-else
          :size="14"
        />{{
          testing ? t("provider.testingNow") : t("provider.test")
        }}</button
      ><span class="field-hint">{{
        live ? t("provider.testHint.live") : t("provider.testHint.mock")
      }}</span>
    </div>
    <div class="editor-footer">
      <button
        v-if="provider.kind === 'custom'"
        class="btn btn-danger"
        type="button"
        :disabled="testing"
        @click="emit('remove')"
      >
        <Trash2 :size="14" />{{ t("provider.delete") }}
      </button>
      <span class="unsaved-note">{{
        dirty ? t("provider.unsaved") : t("provider.saved")
      }}</span
      ><button
        class="btn"
        type="button"
        :disabled="!dirty || testing"
        @click="reset"
      >
        {{ t("provider.discard") }}</button
      ><button
        class="btn btn-primary"
        type="submit"
        :disabled="!dirty || testing"
      >
        <Check :size="14" />{{ t("provider.saveChanges") }}
      </button>
    </div>
  </form>
</template>
<style scoped>
.provider-editor {
  padding: 25px 28px;
  min-width: 0;
}
.editor-heading {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 15px;
}
.editor-heading h3 {
  font-size: 17px;
  font-weight: 600;
}
.editor-heading p {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 5px;
}
.editor-heading .badge {
  margin-top: 3px;
  flex-shrink: 0;
}
.mock-banner {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border-radius: 5px;
  color: var(--text-secondary);
  background: var(--input);
  border: 1px solid var(--border);
  font-size: 10px;
  margin-top: 23px;
}
.mock-banner > svg {
  color: var(--success);
}
.editor-fields {
  display: flex;
  flex-direction: column;
  gap: 22px;
  margin-top: 25px;
}
.editor-fields input {
  font-size: 12px;
  min-height: 37px;
}
.key-field {
  position: relative;
}
.key-field > svg {
  position: absolute;
  top: 12px;
  left: 11px;
  color: var(--text-muted);
}
.key-field input {
  width: 100%;
  padding-left: 33px;
  padding-right: 38px;
}
.key-field .icon-btn {
  position: absolute;
  top: 4px;
  right: 4px;
}
.field-hint {
  font-size: 10px;
}
.field-label .icon-btn[data-tip]::after {
  left: auto;
  right: 0;
  transform: none;
}
.editor-fields select {
  font-size: 12px;
  min-height: 37px;
}
.model-list-link {
  align-self: flex-start;
  color: var(--accent);
  font-size: 10px;
  padding: 0;
}
.model-list-link:hover {
  text-decoration: underline;
}
.connection-test {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 26px;
}
.connection-test .btn {
  font-size: 11px;
}
.connection-test > .field-hint {
  font-size: 9px;
  max-width: 170px;
}
.editor-footer {
  display: flex;
  align-items: center;
  gap: 8px;
  border-top: 1px solid var(--border);
  margin-top: 27px;
  padding-top: 18px;
}
.unsaved-note {
  margin-right: auto;
  font-size: 10px;
  color: var(--text-muted);
}
.editor-footer .btn {
  font-size: 11px;
}
.connection-feedback {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 9px 11px;
  border-radius: 5px;
  font-size: 11px;
  margin-top: 20px;
}
.connection-feedback.error {
  background: var(--danger-soft);
  color: var(--danger);
}
.connection-feedback.success {
  background: var(--success-soft);
  color: var(--success);
}
@media (max-width: 1050px) {
  .provider-editor {
    padding: 22px;
  }
  .connection-test {
    flex-wrap: wrap;
  }
  .connection-test > .field-hint {
    max-width: none;
  }
  .unsaved-note {
    display: none;
  }
  .editor-footer {
    justify-content: flex-end;
  }
}
@media (max-width: 600px) {
  .provider-editor {
    padding: 18px 14px;
  }
  .editor-heading {
    gap: 8px;
  }
  .editor-heading h3 {
    font-size: 15px;
  }
  .editor-heading p {
    font-size: 10px;
  }
}
</style>

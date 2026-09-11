<script setup lang="ts">
import {
  computed,
  shallowRef,
  watch,
  nextTick,
  useTemplateRef,
  onUnmounted,
} from "vue";
import {
  RotateCcw,
  ChevronDown,
  SlidersHorizontal,
  Shuffle,
  Info,
  X,
  Sparkles,
  CircleHelp,
} from "lucide-vue-next";
import type {
  AspectRatio,
  GenerationParams,
  Provider,
  ReferenceImage,
} from "../../types";
import { capabilitiesFor, dimensions } from "../../services/generation";
import ReferenceImages from "./ReferenceImages.vue";
import { useI18n } from "../../i18n";
const props = defineProps<{
  params: GenerationParams;
  providers: Provider[];
  references: ReferenceImage[];
  drawerOpen: boolean;
  live: boolean;
}>();
const { t, tp } = useI18n();
const emit = defineEmits<{
  update: [patch: Partial<GenerationParams>];
  provider: [id: string];
  reset: [];
  addReference: [files: File[]];
  removeReference: [id: string];
  referenceStrength: [id: string, value: number];
  close: [];
}>();
const advanced = shallowRef(false);
const panelElement = useTemplateRef<HTMLElement>("panelElement");
let previousFocus: HTMLElement | null = null;
const narrowWindow = window.matchMedia("(max-width: 850px)");
const closeOnResize = () => {
  if (!narrowWindow.matches && props.drawerOpen) emit("close");
};
narrowWindow.addEventListener("change", closeOnResize);
onUnmounted(() => narrowWindow.removeEventListener("change", closeOnResize));
watch(
  () => props.drawerOpen,
  async (open) => {
    if (open) {
      previousFocus = document.activeElement as HTMLElement;
      await nextTick();
      panelElement.value
        ?.querySelector<HTMLButtonElement>(".close-panel")
        ?.focus();
    } else previousFocus?.focus();
  },
);
function trapFocus(event: KeyboardEvent) {
  if (!props.drawerOpen || event.key !== "Tab") return;
  const elements = Array.from(
    panelElement.value?.querySelectorAll<HTMLElement>(
      'button:not(:disabled), input:not(:disabled), select:not(:disabled), [tabindex="0"]',
    ) ?? [],
  ).filter(
    (element) =>
      element.offsetParent !== null && !element.classList.contains("sr-only"),
  );
  const first = elements[0];
  const last = elements[elements.length - 1];
  if (event.shiftKey && document.activeElement === first) {
    event.preventDefault();
    last?.focus();
  } else if (!event.shiftKey && document.activeElement === last) {
    event.preventDefault();
    first?.focus();
  }
}
const provider = computed(
  () =>
    props.providers.find((item) => item.id === props.params.providerId) ??
    props.providers[0],
);
const caps = computed(() => capabilitiesFor(provider.value));
const size = computed(() =>
  dimensions(props.params.aspectRatio, props.params.resolution),
);
type Orientation = "landscape" | "portrait" | "square";
const ratioOptions: Record<
  Orientation,
  { value: AspectRatio; width: number; height: number }[]
> = {
  landscape: [
    { value: "16:9", width: 21, height: 12 },
    { value: "3:2", width: 19, height: 13 },
    { value: "4:3", width: 17, height: 13 },
  ],
  portrait: [
    { value: "9:16", width: 12, height: 21 },
    { value: "2:3", width: 13, height: 19 },
    { value: "3:4", width: 13, height: 17 },
  ],
  square: [{ value: "1:1", width: 15, height: 15 }],
};
const orientation = computed<Orientation>(() => {
  const aspect = props.params.aspectRatio;
  if (aspect === "1:1") return "square";
  return aspect === "9:16" || aspect === "2:3" || aspect === "3:4"
    ? "portrait"
    : "landscape";
});
function pickOrientation(next: Orientation) {
  if (orientation.value !== next)
    emit("update", { aspectRatio: ratioOptions[next][0].value });
}
function value(event: Event) {
  return (event.target as HTMLInputElement).value;
}
</script>
<template>
  <div v-if="drawerOpen" class="panel-backdrop" @click="emit('close')" />
  <aside
    ref="panelElement"
    class="parameter-panel"
    :class="{ 'drawer-open': drawerOpen }"
    :aria-label="t('params.title')"
    @keydown.esc="drawerOpen && emit('close')"
    @keydown="trapFocus"
  >
    <header class="panel-heading">
      <SlidersHorizontal :size="15" />
      <h2>{{ t("params.title") }}</h2>
      <button
        class="icon-btn reset-button"
        :aria-label="t('params.reset')"
        :data-tip="t('params.resetTip')"
        @click="emit('reset')"
      >
        <RotateCcw :size="14" /></button
      ><button
        class="icon-btn close-panel"
        :aria-label="t('params.close')"
        @click="emit('close')"
      >
        <X :size="17" />
      </button>
    </header>
    <div class="panel-scroll">
      <section class="parameter-section provider-section">
        <label class="field"
          ><span class="field-label"
            >{{ t("params.provider") }}
            <span class="provider-connection"
              ><span class="status-dot" :class="provider.status" />{{
                provider.status === "connected"
                  ? t("params.connected")
                  : t("params.notTested")
              }}</span
            ></span
          ><span class="provider-select-wrap"
            ><Sparkles :size="16" /><select
              :value="params.providerId"
              :aria-label="t('params.providerAria')"
              @change="emit('provider', value($event))"
            >
              <option v-for="item in providers" :key="item.id" :value="item.id">
                {{ item.name }}
              </option>
            </select></span
          ></label
        >
        <label class="field"
          ><span class="field-label">{{ t("params.model") }}</span
          ><select
            :value="params.model"
            :aria-label="t('params.modelAria')"
            @change="emit('update', { model: value($event) })"
          >
            <option
              v-for="model in provider.models"
              :key="model"
              :value="model"
            >
              {{ model }}
            </option></select
          ><span class="field-hint model-note">{{
            provider.kind === "openai"
              ? t("params.modelNote.openai")
              : provider.kind === "gemini"
                ? t("params.modelNote.gemini")
                : t("params.modelNote.other")
          }}</span></label
        >
        <p v-if="live" class="field-hint capability-note">
          {{ t("params.capabilityNote") }}
        </p>
      </section>
      <section class="parameter-section image-settings">
        <div class="field">
          <span class="field-label">{{ t("params.aspect") }}</span>
          <div
            class="segmented orientation-options"
            role="group"
            :aria-label="t('params.orientation')"
          >
            <button
              :class="{ selected: orientation === 'landscape' }"
              :aria-pressed="orientation === 'landscape'"
              @click="pickOrientation('landscape')"
            >
              {{ t("params.orientation.landscape") }}
            </button>
            <button
              :class="{ selected: orientation === 'portrait' }"
              :aria-pressed="orientation === 'portrait'"
              @click="pickOrientation('portrait')"
            >
              {{ t("params.orientation.portrait") }}
            </button>
            <button
              :class="{ selected: orientation === 'square' }"
              :aria-pressed="orientation === 'square'"
              @click="pickOrientation('square')"
            >
              1:1
            </button>
          </div>
          <div
            class="aspect-options"
            role="group"
            :aria-label="t('params.aspect')"
          >
            <button
              v-for="aspect in ratioOptions[orientation]"
              :key="aspect.value"
              :class="{ selected: params.aspectRatio === aspect.value }"
              :aria-label="`${t(`params.aspect.${aspect.value}`)} ${aspect.value}`"
              :aria-pressed="params.aspectRatio === aspect.value"
              @click="emit('update', { aspectRatio: aspect.value })"
            >
              <span
                class="aspect-shape"
                :style="{
                  width: `${aspect.width}px`,
                  height: `${aspect.height}px`,
                }"
              /><span>{{ aspect.value }}</span>
            </button>
          </div>
        </div>
        <label class="field"
          ><span class="field-label"
            >{{ t("params.resolution") }}
            <span class="mono muted"
              >{{ size.width }} × {{ size.height }}</span
            ></span
          ><select
            :value="params.resolution"
            :aria-label="t('params.resolution')"
            @change="
              emit('update', {
                resolution: value($event) as GenerationParams['resolution'],
              })
            "
          >
            <option value="1K">{{ t("params.resolution.1K") }}</option>
            <option value="2K">{{ t("params.resolution.2K") }}</option>
            <option value="4K">{{ t("params.resolution.4K") }}</option>
          </select
          ><span
            v-if="params.resolution !== '1K' && live"
            class="field-hint"
            >{{ t("params.resolutionNote") }}</span
          ></label
        >
        <div class="field">
          <span class="field-label"
            >{{ t("params.quality") }}
            <span
              :data-tip="t('params.qualityTip')"
              tabindex="0"
              :aria-label="t('params.qualityAria')"
              ><CircleHelp :size="13" class="muted" /></span
          ></span>
          <div class="segmented" role="group" :aria-label="t('params.quality')">
            <button
              v-for="quality in ['Standard', 'High', 'Auto'] as const"
              :key="quality"
              :class="{ selected: params.quality === quality }"
              :aria-pressed="params.quality === quality"
              @click="emit('update', { quality })"
            >
              {{ quality }}
            </button>
          </div>
        </div>
        <div class="field">
          <span class="field-label"
            >{{ t("params.count") }}
            <span class="muted small">{{ t("params.countPer") }}</span></span
          >
          <div
            class="number-options"
            role="group"
            :aria-label="t('params.count')"
          >
            <button
              v-for="count in 4"
              :key="count"
              :class="{ selected: params.count === count }"
              :aria-label="tp('params.countAria', count)"
              :aria-pressed="params.count === count"
              @click="emit('update', { count })"
            >
              {{ count }}
            </button>
          </div>
        </div>
      </section>
      <section class="parameter-section">
        <ReferenceImages
          :images="references"
          :unsupported="live && !caps.references"
          @add="emit('addReference', $event)"
          @remove="emit('removeReference', $event)"
          @strength="(id, strength) => emit('referenceStrength', id, strength)"
        />
      </section>
      <section class="advanced-section">
        <button
          class="advanced-toggle"
          :aria-expanded="advanced"
          aria-controls="advanced-settings"
          @click="advanced = !advanced"
        >
          <SlidersHorizontal :size="14" /><span>{{ t("params.advanced") }}</span
          ><ChevronDown :size="14" :class="{ rotated: advanced }" />
        </button>
        <div v-if="advanced" id="advanced-settings" class="advanced-content">
          <div class="field">
            <span class="field-label"
              ><label for="generation-seed">{{ t("params.seed") }}</label
              ><button
                class="icon-btn"
                :disabled="live && !caps.seed"
                :aria-label="t('params.seedRandom')"
                :data-tip="
                  live && !caps.seed
                    ? t('params.unsupported')
                    : t('params.seedRandom')
                "
                @click.prevent="
                  emit('update', {
                    seed: String(Math.floor(Math.random() * 4294967295)),
                  })
                "
              >
                <Shuffle :size="14" /></button></span
            ><input
              id="generation-seed"
              :value="params.seed"
              inputmode="numeric"
              :disabled="live && !caps.seed"
              :data-tip="live && !caps.seed ? t('params.unsupported') : undefined"
              :placeholder="t('params.seedPlaceholder')"
              maxlength="10"
              @input="emit('update', { seed: value($event) })"
            />
          </div>
          <label class="field"
            ><span class="field-label"
              >{{ t("params.guidance") }}
              <span class="mono">{{ params.guidance }}</span></span
            ><input
              type="range"
              :value="params.guidance"
              min="1"
              max="20"
              step="0.5"
              :disabled="live && !caps.guidance"
              :data-tip="
                live && !caps.guidance ? t('params.unsupported') : undefined
              "
              :aria-label="t('params.guidance')"
              @input="emit('update', { guidance: Number(value($event)) })"
            /><span class="field-hint">{{
              live && !caps.guidance
                ? t("params.unsupportedRecorded")
                : t("params.guidanceHint")
            }}</span></label
          >
          <label class="field"
            ><span class="field-label">{{ t("params.format") }}</span
            ><select
              :value="params.format"
              :aria-label="t('params.format')"
              @change="
                emit('update', {
                  format: value($event) as GenerationParams['format'],
                })
              "
            >
              <option>PNG</option>
              <option>JPEG</option>
              <option>WebP</option>
            </select></label
          >
        </div>
      </section>
      <div class="parameter-footnote">
        <Info :size="13" /><span
          >{{ t("params.footnote.apply") }}<br />{{
            live ? t("params.footnote.mode.live") : t("params.footnote.mode.mock")
          }}</span
        >
      </div>
    </div>
  </aside>
</template>
<style scoped>
.parameter-panel {
  width: 282px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: var(--panel);
  border-left: 1px solid var(--border);
  min-height: 0;
}
.panel-heading {
  height: 44px;
  min-height: 44px;
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 0 19px;
  border-bottom: 1px solid var(--border);
}
.panel-heading > svg {
  color: var(--text-muted);
}
.panel-heading h2 {
  font-size: 11px;
  font-weight: 600;
}
.reset-button {
  margin-left: auto;
}
.close-panel {
  display: none;
}
.panel-scroll {
  overflow-y: auto;
  overflow-x: hidden;
  flex: 1;
  min-height: 0;
}
.parameter-panel [data-tip]::after {
  left: auto;
  right: 0;
  transform: none;
}
.parameter-section {
  padding: 21px 20px;
  border-bottom: 1px solid var(--border);
}
.provider-section {
  display: flex;
  flex-direction: column;
  gap: 19px;
}
.provider-connection {
  color: var(--success);
  font-size: 9px;
  font-weight: 400;
  display: flex;
  align-items: center;
  gap: 5px;
}
.provider-select-wrap {
  position: relative;
  display: block;
}
.provider-select-wrap > svg {
  position: absolute;
  top: 10px;
  left: 10px;
  pointer-events: none;
}
.provider-select-wrap select {
  width: 100%;
  padding-left: 35px;
  font-size: 12px;
}
.field select {
  font-size: 11px;
}
.model-note {
  font-size: 9px;
}
.image-settings {
  display: flex;
  flex-direction: column;
  gap: 20px;
}
.orientation-options {
  width: 100%;
  margin-bottom: 8px;
}
.orientation-options button {
  flex: 1;
  font-size: 10px;
  padding: 6px 0;
}
.aspect-options {
  display: flex;
  gap: 7px;
}
.aspect-options button {
  flex: 1;
  border: 1px solid var(--border);
  border-radius: 5px;
  height: 62px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 7px;
  color: var(--text-secondary);
  font-size: 10px;
  background: var(--input);
}
.aspect-options button:hover {
  border-color: var(--border-strong);
}
.aspect-options button.selected {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-soft);
}
.aspect-shape {
  border: 1.4px solid currentColor;
  border-radius: 2px;
}
.small {
  font-size: 9px;
  font-weight: 400;
}
.number-options {
  display: flex;
  gap: 7px;
}
.number-options button {
  flex: 1;
  border-radius: 5px;
  border: 1px solid var(--border);
  height: 32px;
  font-size: 11px;
  color: var(--text-secondary);
  background: var(--input);
}
.number-options button:hover {
  border-color: var(--border-strong);
}
.number-options button.selected {
  color: var(--accent);
  background: var(--accent-soft);
  border-color: var(--accent);
}
.advanced-toggle {
  display: flex;
  align-items: center;
  gap: 9px;
  width: 100%;
  padding: 18px 20px;
  color: var(--text-secondary);
  font-size: 11px;
}
.advanced-toggle span {
  flex: 1;
  text-align: left;
}
.advanced-toggle:hover {
  color: var(--text);
  background: var(--hover);
}
.rotated {
  transform: rotate(180deg);
}
.advanced-content {
  display: flex;
  flex-direction: column;
  gap: 15px;
  padding: 0 20px 20px;
}
.capability-note {
  margin-top: 10px;
}
.parameter-footnote {
  padding: 8px 20px 22px;
  display: flex;
  gap: 8px;
  color: var(--text-muted);
  font-size: 9px;
  line-height: 1.8;
}
.parameter-footnote svg {
  margin-top: 2px;
}
.panel-backdrop {
  display: none;
}
@media (min-width: 1800px) {
  .parameter-panel {
    width: 304px;
  }
  .parameter-section {
    padding: 25px 23px;
  }
  .image-settings {
    gap: 25px;
  }
}
@media (max-height: 850px) {
  .parameter-section {
    padding-top: 16px;
    padding-bottom: 16px;
  }
  .image-settings {
    gap: 15px;
  }
  .provider-section {
    gap: 13px;
  }
}
@media (max-width: 1000px) {
  .parameter-panel {
    width: 254px;
  }
  .parameter-section {
    padding-left: 16px;
    padding-right: 16px;
  }
}
@media (max-width: 850px) {
  .parameter-panel {
    display: none;
  }
  .parameter-panel.drawer-open {
    display: flex;
    position: fixed;
    right: 0;
    top: 0;
    bottom: 25px;
    width: 290px;
    z-index: 42;
    box-shadow: var(--modal-shadow);
  }
  .close-panel {
    display: flex;
  }
  .panel-backdrop {
    display: block;
    position: fixed;
    inset: 0;
    background: var(--overlay);
    z-index: 40;
  }
}
</style>

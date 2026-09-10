<script setup lang="ts">
import { computed, shallowRef, watch } from "vue";
import { ChevronDown as ChevronDownFallback } from "lucide-vue-next";
import {
  Check,
  Image as ImageIcon,
  Star,
  Download,
  Maximize2,
  Columns2,
  ChevronLeft,
  ChevronRight,
  Minus,
  Plus,
  Scan,
  Info,
  X,
  Sparkles,
  LoaderCircle,
} from "lucide-vue-next";
import type { ImageResult } from "../../types";
import ResultImage from "../ui/ResultImage.vue";
import BaseDialog from "../ui/BaseDialog.vue";
import { useI18n } from "../../i18n";
const props = defineProps<{
  results: ImageResult[];
  selectedId: string;
  generating: boolean;
  progress: number;
  count: number;
  live: boolean;
}>();
const { t, tp } = useI18n();
const emit = defineEmits<{
  select: [id: string];
  favorite: [id: string];
  download: [image: ImageResult];
  reuse: [image: ImageResult];
  cancel: [];
}>();
const selected = computed(() =>
  props.results.find((image) => image.id === props.selectedId),
);
const selectedIndex = computed(() =>
  props.results.findIndex((image) => image.id === props.selectedId),
);
const compare = shallowRef(false);
const fullscreen = shallowRef(false);
const showInfo = shallowRef(false);
const zoom = shallowRef(100);
const compareImages = computed(() => {
  if (!selected.value) return [];
  return [
    selected.value,
    props.results.find((image) => image.id !== props.selectedId),
  ].filter((image): image is ImageResult => !!image);
});
watch(
  () => props.selectedId,
  () => {
    zoom.value = 100;
  },
);
watch(
  () => props.results.length,
  (length) => {
    if (length < 2) compare.value = false;
  },
);
function next(direction: number) {
  const image =
    props.results[
      (selectedIndex.value + direction + props.results.length) %
        props.results.length
    ];
  if (image) emit("select", image.id);
}
</script>
<template>
  <section class="canvas-workspace" :aria-label="t('canvas.aria')">
    <div class="canvas-toolbar">
      <div class="canvas-breadcrumb">
        <ImageIcon :size="14" /><span>{{ t("canvas.untitled") }}</span
        ><span class="breadcrumb-slash">/</span
        ><span class="secondary">{{
          selected ? t("canvas.output") : t("canvas.canvas")
        }}</span
        ><span v-if="results.length" class="badge output-count">{{
          tp("canvas.imageCount", results.length)
        }}</span
        >
      </div>
      <div class="canvas-tools">
        <button
          class="icon-btn"
          :class="{ 'is-active': compare }"
          :disabled="results.length < 2 || generating"
          :aria-label="t('canvas.compare')"
          :aria-pressed="compare"
          :data-tip="t('canvas.compare')"
          @click="compare = !compare"
        >
          <Columns2 :size="15" /></button
        ><button
          class="icon-btn"
          :disabled="!selected || generating"
          :aria-label="t('canvas.expandAria')"
          :data-tip="t('canvas.expand')"
          @click="fullscreen = true"
        >
          <Maximize2 :size="15" /></button
        ><span class="divider" /><button
          class="icon-btn"
          :class="{ 'is-accent': selected?.favorite }"
          :disabled="!selected"
          :aria-label="t('canvas.favorite')"
          :aria-pressed="selected?.favorite ?? false"
          :data-tip="t('canvas.favoriteTip')"
          @click="selected && emit('favorite', selected.id)"
        >
          <Star
            :size="15"
            :fill="selected?.favorite ? 'currentColor' : 'none'"
          /></button
        ><button
          class="icon-btn"
          :disabled="!selected || generating"
          :aria-label="t('canvas.download')"
          :data-tip="t('canvas.download')"
          @click="selected && emit('download', selected)"
        >
          <Download :size="15" />
        </button>
      </div>
    </div>
    <div
      class="image-stage"
      :class="{ 'is-generating': generating }"
      tabindex="0"
      :aria-label="t('canvas.stage')"
      @keydown.left.prevent="next(-1)"
      @keydown.right.prevent="next(1)"
    >
      <div
        v-if="!results.length && !generating"
        class="empty-state canvas-empty"
      >
        <span class="empty-icon"><ImageIcon :size="33" /></span>
        <h3>{{ t("canvas.emptyTitle") }}</h3>
        <p>{{ t("canvas.emptyBody") }}</p>
        <span class="empty-shortcut mono">{{
          t("canvas.emptyShortcut")
        }}</span>
      </div>
      <template v-else-if="selected">
        <div v-if="compare" class="compare-grid">
          <div
            v-for="(image, index) in compareImages"
            :key="image.id"
            class="compare-image"
            :style="{ aspectRatio: `${image.width} / ${image.height}` }"
          >
            <ResultImage
              :src="image.src"
              :alt="image.title"
              :filter="image.filter"
              :position="image.position"
            /><span class="compare-label"
              >{{
                index === 0
                  ? t("canvas.compareSelected")
                  : t("canvas.compareOther")
              }}
              · {{ image.seed }}</span
            >
          </div>
        </div>
        <div v-else class="image-scroll">
          <div
            class="image-frame"
            :style="{
              '--image-ratio': selected.width / selected.height,
              '--zoom': zoom / 100,
            }"
          >
            <ResultImage
              :src="selected.src"
              :alt="selected.title"
              :filter="selected.filter"
              :position="selected.position"
            />
          </div>
        </div>
        <button
          v-if="!compare && !generating && results.length > 1"
          class="image-nav previous"
          :aria-label="t('canvas.previousImage')"
          @click="next(-1)"
        >
          <ChevronLeft :size="19" /></button
        ><button
          v-if="!compare && !generating && results.length > 1"
          class="image-nav next"
          :aria-label="t('canvas.nextImage')"
          @click="next(1)"
        >
          <ChevronRight :size="19" />
        </button>
        <div v-if="showInfo" class="image-info">
          <div class="info-heading">
            <strong>{{ t("canvas.info.title") }}</strong
            ><button
              class="icon-btn"
              :aria-label="t('canvas.info.close')"
              @click="showInfo = false"
            >
              <X :size="13" />
            </button>
          </div>
          <dl>
            <dt>{{ t("canvas.info.model") }}</dt>
            <dd>{{ selected.params.model }}</dd>
            <dt>{{ t("canvas.info.dimensions") }}</dt>
            <dd>{{ selected.width }} × {{ selected.height }}</dd>
            <dt>{{ t("canvas.info.seed") }}</dt>
            <dd>{{ selected.seed }}</dd>
            <dt>{{ t("canvas.info.quality") }}</dt>
            <dd>{{ selected.params.quality }}</dd>
            <dt>{{ t("canvas.info.created") }}</dt>
            <dd>{{ new Date(selected.createdAt).toLocaleString() }}</dd>
          </dl>
          <button class="btn" @click="emit('reuse', selected)">
            {{ t("canvas.info.reuse") }}
          </button>
        </div>
        <div class="canvas-bottom">
          <span class="canvas-size mono"
            >{{ selected.width }} × {{ selected.height
            }}<span class="dot-separator">·</span
            >{{ selected.params.format }}</span
          >
          <div class="zoom-controls">
            <button
              class="icon-btn"
              :disabled="zoom <= 50 || compare"
              :aria-label="t('canvas.zoomOut')"
              @click="zoom = Math.max(50, zoom - 25)"
            >
              <Minus :size="13" /></button
            ><button
              class="zoom-label mono"
              :disabled="compare"
              :aria-label="t('canvas.fit')"
              @click="zoom = 100"
            >
              {{ zoom === 100 ? t("canvas.fitLabel") : `${zoom}%`
              }}<ChevronDownFallback :size="10" /></button
            ><button
              class="icon-btn"
              :disabled="zoom >= 200 || compare"
              :aria-label="t('canvas.zoomIn')"
              @click="zoom = Math.min(200, zoom + 25)"
            >
              <Plus :size="13" /></button
            ><span class="divider" /><button
              class="icon-btn"
              :aria-label="t('canvas.resetZoom')"
              :disabled="compare"
              @click="zoom = 100"
            >
              <Scan :size="14" />
            </button>
          </div>
          <button
            class="icon-btn canvas-info-toggle"
            :class="{ 'is-active': showInfo }"
            :aria-label="t('canvas.info.title')"
            :aria-expanded="showInfo"
            @click="showInfo = !showInfo"
          >
            <Info :size="14" />
          </button>
        </div>
      </template>
      <div
        v-if="generating"
        class="generation-overlay"
        role="status"
        aria-live="polite"
      >
        <span class="generation-orbit"><Sparkles :size="28" /></span>
        <h3>{{ t("canvas.generatingTitle") }}</h3>
        <p>
          {{
            progress < 28
              ? t("canvas.stage.early")
              : progress < 72
                ? t("canvas.stage.mid")
                : t("canvas.stage.late")
          }}
        </p>
        <div
          class="progress-track"
          role="progressbar"
          :aria-valuenow="progress"
          :aria-valuemin="0"
          :aria-valuemax="100"
          :aria-label="t('canvas.progress')"
        >
          <div :style="{ width: `${progress}%` }" />
        </div>
        <span class="progress-meta mono"
          ><LoaderCircle :size="11" class="spin" />{{ progress }}% ·
          {{ tp("canvas.imageCount", count) }} ·
          {{
            live ? t("canvas.progressMeta.live") : t("canvas.progressMeta.mock")
          }}</span
        ><button class="btn" @click="emit('cancel')">{{
          t("canvas.cancelGeneration")
        }}</button>
      </div>
    </div>
    <div v-if="results.length || generating" class="filmstrip">
      <span class="filmstrip-label eyebrow">{{ t("canvas.outputs") }}</span>
      <div class="thumbnails">
        <template v-if="generating"
          ><div
            v-for="index in count"
            :key="index"
            class="thumbnail skeleton" /></template
        ><button
          v-for="(image, index) in results"
          v-else
          :key="image.id"
          class="thumbnail"
          :class="{ selected: image.id === selectedId }"
          :aria-label="t('canvas.selectImage', { n: index + 1 })"
          :aria-pressed="image.id === selectedId"
          @click="emit('select', image.id)"
        >
          <ResultImage
            :src="image.src"
            :alt="image.title"
            :filter="image.filter"
            :position="image.position"
          /><span class="thumbnail-number">{{
            String(index + 1).padStart(2, "0")
          }}</span
          ><span v-if="image.id === selectedId" class="thumbnail-check"
            ><Check :size="9"
          /></span>
        </button>
      </div>
      <span class="filmstrip-meta"
        ><span v-if="!generating" class="ready-text"
          ><Check :size="12" />{{ t("canvas.complete") }}</span
        ><span class="mono">{{
          generating
            ? t("canvas.working")
            : t("canvas.indexOf", {
                index: selectedIndex + 1,
                total: results.length,
              })
        }}</span></span
      >
    </div>
    <BaseDialog
      v-if="fullscreen && selected"
      :title="selected.title"
      :description="t('canvas.fullscreenDesc')"
      wide
      @close="fullscreen = false"
      ><div
        class="fullscreen-image"
        :style="{ aspectRatio: `${selected.width} / ${selected.height}` }"
      >
        <ResultImage
          :src="selected.src"
          :alt="selected.title"
          :filter="selected.filter"
          :position="selected.position"
        />
      </div>
      <template #footer
        ><button class="btn" :disabled="results.length < 2" @click="next(-1)">
          <ChevronLeft :size="15" />{{ t("canvas.previous") }}</button
        ><button class="btn" :disabled="results.length < 2" @click="next(1)">
          {{ t("canvas.next") }}<ChevronRight :size="15" /></button
        ><button class="btn btn-primary" @click="emit('download', selected)">
          <Download :size="15" />{{ t("canvas.downloadAction") }}
        </button></template
      ></BaseDialog
    >
  </section>
</template>
<style scoped>
.canvas-workspace {
  display: flex;
  flex-direction: column;
  min-height: 0;
  flex: 1;
}
.canvas-toolbar {
  height: 44px;
  min-height: 44px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 19px 0 23px;
  gap: 10px;
  background: var(--panel);
}
.canvas-breadcrumb {
  display: flex;
  align-items: center;
  gap: 9px;
  font-size: 10px;
  color: var(--text-muted);
  white-space: nowrap;
}
.breadcrumb-slash {
  color: var(--border-strong);
}
.output-count {
  margin-left: 2px;
  font-size: 8px;
}
.canvas-tools {
  display: flex;
  align-items: center;
}
.image-stage {
  flex: 1;
  min-height: 150px;
  position: relative;
  overflow: hidden;
  background-color: var(--canvas);
  background-image: radial-gradient(var(--canvas-dot) 0.7px, transparent 0.7px);
  background-size: 12px 12px;
  container-type: size;
}
.image-scroll {
  display: flex;
  align-items: safe center;
  justify-content: safe center;
  position: absolute;
  inset: 0 0 36px;
  overflow: auto;
}
.image-frame {
  flex-shrink: 0;
  width: calc(
    min(100cqw - 66px, (100cqh - 64px) * var(--image-ratio)) * var(--zoom)
  );
  aspect-ratio: var(--image-ratio);
  box-shadow: var(--image-shadow);
  border-radius: 2px;
  overflow: hidden;
}
.image-nav {
  position: absolute;
  top: calc(50% - 18px);
  display: grid;
  place-items: center;
  width: 25px;
  height: 36px;
  border: 1px solid var(--border);
  border-radius: 5px;
  color: var(--text-secondary);
  background: var(--panel);
  opacity: 0;
  transition: opacity 120ms;
}
.image-stage:hover .image-nav,
.image-nav:focus-visible {
  opacity: 1;
}
.image-nav:hover {
  color: var(--text);
  background: var(--panel-raised);
}
.previous {
  left: 7px;
}
.next {
  right: 7px;
}
.canvas-bottom {
  display: flex;
  align-items: center;
  position: absolute;
  bottom: 9px;
  left: 18px;
  right: 18px;
  height: 28px;
}
.canvas-size {
  font-size: 9px;
  color: var(--text-muted);
}
.dot-separator {
  margin: 0 7px;
}
.zoom-controls {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 5px;
  height: 28px;
  padding: 0 2px;
}
.zoom-controls .icon-btn {
  width: 25px;
  height: 24px;
}
.zoom-controls .divider {
  margin: 5px 3px;
}
.zoom-label {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 3px;
  min-width: 34px;
  padding: 0 4px;
  color: var(--text-secondary);
  font-size: 9px;
}
.canvas-info-toggle {
  margin-left: auto;
}
.filmstrip {
  min-height: 85px;
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 13px 23px;
  background: var(--panel);
  border-top: 1px solid var(--border);
}
.filmstrip-label {
  font-size: 8px;
  writing-mode: vertical-rl;
  transform: rotate(180deg);
  letter-spacing: 0.15em;
}
.thumbnails {
  display: flex;
  align-items: center;
  gap: 10px;
  overflow-x: auto;
  padding: 3px;
  margin: -3px;
}
.thumbnail {
  position: relative;
  flex-shrink: 0;
  width: 83px;
  height: 57px;
  border-radius: 4px;
  overflow: hidden;
  padding: 0;
  border: 2px solid transparent;
  background: var(--input);
}
.thumbnail.selected {
  border-color: var(--accent);
  outline: 1px solid var(--accent);
  outline-offset: 2px;
}
.thumbnail:hover {
  border-color: var(--border-strong);
}
.thumbnail-number {
  position: absolute;
  bottom: 3px;
  left: 4px;
  color: #fff;
  text-shadow: 0 1px 3px #000;
  font-family: var(--font-mono);
  font-size: 8px;
}
.thumbnail-check {
  position: absolute;
  top: 3px;
  right: 3px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  display: grid;
  place-items: center;
  color: var(--accent-foreground);
  background: var(--accent);
}
.filmstrip-meta {
  margin-left: auto;
  font-size: 9px;
  color: var(--text-muted);
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 5px;
  white-space: nowrap;
}
.filmstrip-meta .mono {
  font-size: 8px;
}
.ready-text {
  display: inline-flex;
  gap: 4px;
  align-items: center;
  font-size: 8px;
}
.canvas-empty {
  height: 100%;
}
.empty-shortcut {
  font-size: 9px;
  margin-top: 10px;
  letter-spacing: 0.08em;
}
.generation-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  background: color-mix(in srgb, var(--canvas) 91%, transparent);
  backdrop-filter: blur(12px);
  z-index: 3;
}
.generation-orbit {
  display: grid;
  place-items: center;
  width: 61px;
  height: 61px;
  border: 1px solid var(--border-strong);
  border-radius: 16px;
  color: var(--accent);
  background: var(--panel);
  margin-bottom: 2px;
}
.generation-overlay h3 {
  font-size: 16px;
  font-weight: 500;
}
.generation-overlay p {
  font-size: 11px;
  color: var(--text-muted);
}
.progress-track {
  width: 200px;
  height: 3px;
  border-radius: 3px;
  background: var(--border);
  margin-top: 8px;
  overflow: hidden;
}
.progress-track > div {
  height: 100%;
  background: var(--accent);
  transition: width 180ms linear;
}
.progress-meta {
  font-size: 9px;
  display: flex;
  align-items: center;
  gap: 7px;
  color: var(--text-muted);
}
.generation-overlay .btn {
  margin-top: 9px;
  font-size: 10px;
}
.compare-grid {
  position: absolute;
  inset: 24px 22px 55px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  align-items: center;
}
.compare-image {
  aspect-ratio: 2/3;
  max-height: 100%;
  position: relative;
  overflow: hidden;
  border-radius: 3px;
  box-shadow: var(--image-shadow);
}
.compare-label {
  position: absolute;
  bottom: 10px;
  left: 10px;
  color: #fff;
  background: #1e2719b5;
  border-radius: 3px;
  padding: 4px 7px;
  font-size: 9px;
}
.image-info {
  position: absolute;
  bottom: 48px;
  right: 16px;
  width: 250px;
  background: var(--panel-raised);
  border: 1px solid var(--border-strong);
  border-radius: 7px;
  box-shadow: var(--popover-shadow);
  padding: 12px;
  z-index: 4;
}
.info-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 11px;
}
.image-info dl {
  display: grid;
  grid-template-columns: 70px 1fr;
  gap: 8px;
  margin: 10px 0 14px;
  font-size: 10px;
}
.image-info dt {
  color: var(--text-muted);
}
.image-info dd {
  margin: 0;
  overflow-wrap: anywhere;
}
.image-info .btn {
  width: 100%;
  font-size: 10px;
}
.fullscreen-image {
  max-height: 65dvh;
  width: auto;
  margin: auto;
}
@media (min-width: 1800px) {
  .filmstrip {
    min-height: 112px;
    padding: 16px 28px;
  }
  .thumbnail {
    width: 112px;
    height: 73px;
  }
  .image-frame {
    width: calc(
      min(100cqw - 100px, (100cqh - 108px) * var(--image-ratio)) * var(--zoom)
    );
  }
}
@media (max-width: 1280px) {
  .filmstrip-meta .ready-text {
    display: none;
  }
}
@media (max-width: 1000px) {
  .canvas-toolbar {
    padding: 0 13px;
  }
  .canvas-breadcrumb {
    gap: 6px;
  }
  .output-count {
    display: none;
  }
  .filmstrip {
    padding: 11px 16px;
    gap: 10px;
    min-height: 84px;
  }
  .thumbnail {
    width: 72px;
    height: 50px;
  }
  .thumbnails {
    gap: 7px;
  }
  .filmstrip-label {
    display: none;
  }
}
@media (max-width: 600px) {
  .canvas-breadcrumb {
    font-size: 9px;
  }
  .canvas-breadcrumb .breadcrumb-slash,
  .canvas-breadcrumb .secondary {
    display: none;
  }
  .canvas-tools .icon-btn {
    width: 26px;
  }
  .canvas-toolbar {
    min-height: 43px;
    height: 43px;
  }
  .filmstrip-meta {
    display: none;
  }
  .canvas-size {
    font-size: 7px;
  }
  .canvas-bottom {
    left: 10px;
    right: 8px;
  }
  .thumbnail {
    width: 68px;
  }
  .zoom-controls {
    left: auto;
    right: 29px;
    transform: none;
  }
  .image-frame {
    width: calc(
      min(100cqw - 40px, (100cqh - 66px) * var(--image-ratio)) * var(--zoom)
    );
  }
}
@media (max-height: 740px) {
  .filmstrip {
    min-height: 73px;
    padding-top: 8px;
    padding-bottom: 8px;
  }
  .thumbnail {
    height: 48px;
    width: 72px;
  }
  .generation-orbit {
    display: none;
  }
  .generation-overlay {
    gap: 7px;
  }
}
</style>

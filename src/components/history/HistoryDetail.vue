<script setup lang="ts">
import { RotateCcw, Star, Download, Copy } from "lucide-vue-next";
import type { ImageResult } from "../../types";
import BaseDialog from "../ui/BaseDialog.vue";
import ResultImage from "../ui/ResultImage.vue";
import { useI18n } from "../../i18n";
const { t, tp } = useI18n();
defineProps<{ image: ImageResult }>();
const emit = defineEmits<{
  close: [];
  reuse: [image: ImageResult];
  favorite: [id: string];
  download: [image: ImageResult];
  copy: [text: string];
}>();
</script>
<template>
  <BaseDialog
    :title="image.title"
    :description="new Date(image.createdAt).toLocaleString()"
    wide
    @close="emit('close')"
    ><div class="detail-layout">
      <div
        class="detail-image"
        :style="{ aspectRatio: `${image.width} / ${image.height}` }"
      >
        <ResultImage
          :src="image.src"
          :alt="image.title"
          :filter="image.filter"
          :position="image.position"
        />
      </div>
      <div class="detail-text">
        <div class="detail-label">
          <h3>{{ t("history.detail.prompt") }}</h3>
          <button
            class="icon-btn"
            :aria-label="t('history.detail.copy')"
            :data-tip="t('history.detail.copy')"
            @click="emit('copy', image.prompt)"
          >
            <Copy :size="13" />
          </button>
        </div>
        <p class="detail-prompt">{{ image.prompt }}</p>
        <div v-if="image.params.negativePrompt" class="negative">
          <h3>{{ t("history.detail.negative") }}</h3>
          <p>{{ image.params.negativePrompt }}</p>
        </div>
        <dl>
          <dt>{{ t("history.detail.model") }}</dt>
          <dd>{{ image.params.model }}</dd>
          <dt>{{ t("history.detail.dimensions") }}</dt>
          <dd>{{ image.width }} × {{ image.height }}</dd>
          <dt>{{ t("history.detail.quality") }}</dt>
          <dd>{{ image.params.quality }}</dd>
          <dt>{{ t("history.detail.seed") }}</dt>
          <dd class="mono">{{ image.seed }}</dd>
          <dt>{{ t("history.detail.referencesLabel") }}</dt>
          <dd>{{ tp("history.detail.references", image.references.length) }}</dd>
          <template v-if="image.path">
            <dt>{{ t("history.detail.file") }}</dt>
            <dd class="mono">{{ image.path.split(/[\\/]/).pop() }}</dd>
          </template>
        </dl>
        <span
          class="badge"
          :class="image.path ? 'badge-success' : 'badge-accent'"
          >{{
            image.path
              ? t("history.detail.badge.real")
              : t("history.detail.badge.mock")
          }}</span
        >
      </div>
    </div>
    <template #footer
      ><button
        class="btn btn-ghost"
        :class="{ 'favorite-active': image.favorite }"
        @click="emit('favorite', image.id)"
      >
        <Star :size="15" :fill="image.favorite ? 'currentColor' : 'none'" />{{
          image.favorite
            ? t("history.detail.favorited")
            : t("history.detail.favorite")
        }}</button
      ><button class="btn" @click="emit('download', image)">
        <Download :size="15" />{{ t("history.detail.download") }}</button
      ><button class="btn btn-primary" @click="emit('reuse', image)">
        <RotateCcw :size="15" />{{ t("history.detail.reuse") }}
      </button></template
    ></BaseDialog
  >
</template>
<style scoped>
.detail-layout {
  display: grid;
  grid-template-columns: minmax(0, 1.6fr) minmax(230px, 1fr);
  gap: 24px;
  align-items: start;
}
.detail-image {
  border-radius: 5px;
  overflow: hidden;
  max-height: 60vh;
}
.detail-text h3 {
  font-size: 11px;
  font-weight: 600;
}
.detail-label {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: -4px;
}
.detail-prompt {
  margin-top: 6px;
  font-size: 12px;
  line-height: 1.8;
  color: var(--text-secondary);
}
.detail-text dl {
  border-top: 1px solid var(--border);
  padding-top: 17px;
  margin: 20px 0;
  display: grid;
  grid-template-columns: 85px 1fr;
  gap: 11px;
  font-size: 11px;
}
.detail-text dt {
  color: var(--text-muted);
}
.detail-text dd {
  margin: 0;
  overflow-wrap: anywhere;
}
.favorite-active {
  color: var(--accent);
}
.negative {
  margin-top: 15px;
}
.negative p {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 6px;
}
@media (max-width: 700px) {
  .detail-layout {
    grid-template-columns: 1fr;
  }
  .detail-image {
    max-height: 40vh;
  }
  .detail-text dl {
    grid-template-columns: 85px 1fr 65px 1fr;
    font-size: 10px;
    gap: 9px;
  }
}
</style>

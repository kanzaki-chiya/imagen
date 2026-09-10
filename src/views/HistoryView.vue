<script setup lang="ts">
import { computed, shallowRef } from "vue";
import { useStudio } from "../composables/useStudio";
import type { ImageResult } from "../types";
import { exportImage } from "../services/exportImage";
import {
  backendAvailable,
  exportImageTo,
  openInFolder,
} from "../services/backend";
import { useI18n } from "../i18n";
const { t } = useI18n();
import HistoryToolbar from "../components/history/HistoryToolbar.vue";
import HistoryGallery from "../components/history/HistoryGallery.vue";
import HistoryDetail from "../components/history/HistoryDetail.vue";
const studio = useStudio();
const search = shallowRef("");
const favorites = shallowRef(false);
const model = shallowRef("all");
const sort = shallowRef("newest");
const layout = shallowRef<"grid" | "list">("grid");
const selected = shallowRef<ImageResult | null>(null);
const limit = shallowRef(24);
const models = computed(() => [
  ...new Set(studio.state.history.map((image) => image.params.model)),
]);
const filtered = computed(() =>
  studio.state.history
    .filter(
      (image) =>
        (!favorites.value || image.favorite) &&
        (model.value === "all" || image.params.model === model.value) &&
        `${image.title} ${image.prompt}`
          .toLowerCase()
          .includes(search.value.toLowerCase()),
    )
    .sort((a, b) =>
      sort.value === "newest"
        ? b.createdAt.localeCompare(a.createdAt)
        : a.createdAt.localeCompare(b.createdAt),
    ),
);
const groups = computed(() => {
  const groups = new Map<string, ImageResult[]>();
  const today = new Date().toDateString();
  const yesterday = new Date(Date.now() - 86400000).toDateString();
  for (const image of filtered.value.slice(0, limit.value)) {
    const date = new Date(image.createdAt);
    const label =
      date.toDateString() === today
        ? t("history.today")
        : date.toDateString() === yesterday
          ? t("history.yesterday")
          : date.toLocaleDateString([], {
              month: "long",
              day: "numeric",
              year: "numeric",
            });
    if (!groups.has(label)) groups.set(label, []);
    groups.get(label)!.push(image);
  }
  return [...groups].map(([label, images]) => ({ label, images }));
});
function reset() {
  search.value = "";
  favorites.value = false;
  model.value = "all";
}
function suggestedName(image: ImageResult): string {
  const extension = image.path?.split(".").pop() ?? "png";
  const title = (image.title || "imagen").replace(/[\\/:*?"<>|]/g, "-");
  return `${title.slice(0, 60)}-${image.id.slice(0, 8)}.${extension}`;
}
async function download(image: ImageResult) {
  try {
    if (image.path && backendAvailable()) {
      const target = await exportImageTo(image.path, suggestedName(image));
      if (target) studio.notify(t("toast.exported", { path: target }));
      return;
    }
    await exportImage(image);
    studio.notify(t("toast.downloadRequested"));
  } catch {
    studio.notify(t("toast.exportFail"), "error");
  }
}
async function openFolder(image: ImageResult) {
  if (!image.path) return;
  try {
    await openInFolder(image.path);
  } catch {
    studio.notify(t("toast.folderFail"), "error");
  }
}
async function copy(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    studio.notify(t("toast.promptCopied"));
  } catch {
    studio.notify(t("toast.clipboardFail"), "error");
  }
}
</script>
<template>
  <div class="history-view">
    <HistoryToolbar
      :search="search"
      :favorites="favorites"
      :model="model"
      :models="models"
      :sort="sort"
      :layout="layout"
      :total="studio.state.history.length"
      @search="search = $event"
      @favorites="favorites = !favorites"
      @model="model = $event"
      @sort="sort = $event"
      @layout="layout = $event"
    /><HistoryGallery
      :groups="groups"
      :layout="layout"
      @open="selected = $event"
      @favorite="studio.toggleFavorite"
      @reset="reset"
    />
    <div v-if="filtered.length > limit" class="load-more">
      <button class="btn" @click="limit += 24">{{ t("history.loadMore") }}</button>
    </div>
    <div v-if="filtered.length" class="history-end">
      <span />{{
        t("history.footer", {
          shown: Math.min(limit, filtered.length),
          total: filtered.length,
        })
      }}<span />
    </div>
    <HistoryDetail
      v-if="selected"
      :image="selected"
      @close="selected = null"
      @reuse="studio.reuseImage"
      @favorite="studio.toggleFavorite"
      @download="download"
      @open-folder="openFolder"
      @copy="copy"
    />
  </div>
</template>
<style scoped>
.history-view {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 25px 28px 35px;
}
.history-end {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15px;
  color: var(--text-muted);
  font-size: 10px;
  margin: 38px 0 10px;
}
.history-end > span {
  width: 34px;
  height: 1px;
  background: var(--border);
}
.load-more {
  text-align: center;
  margin-top: 25px;
}
@media (max-width: 650px) {
  .history-view {
    padding: 20px 17px;
  }
}
</style>

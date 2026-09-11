<script setup lang="ts">
import { computed, onMounted, shallowRef, watch } from "vue";
import { useStudio } from "../composables/useStudio";
import type { ImageResult } from "../types";
import {
  copyResultImage,
  exportImage,
  suggestedExportName,
} from "../services/exportImage";
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
import BaseDialog from "../components/ui/BaseDialog.vue";
const studio = useStudio();
const search = shallowRef("");
const view = shallowRef<"all" | "favorites" | "trash">("all");
const model = shallowRef("all");
const sort = shallowRef("newest");
const layout = shallowRef<"grid" | "list">("grid");
const selected = shallowRef<ImageResult | null>(null);
const purging = shallowRef<ImageResult | null>(null);
const emptyingTrash = shallowRef(false);
const limit = shallowRef(24);
const models = computed(() => [
  ...new Set(studio.state.history.map((image) => image.params.model)),
]);
watch(view, (value) => {
  if (value === "trash") void studio.refreshTrash();
});
onMounted(() => void studio.refreshTrash());
const filtered = computed(() =>
  studio.state.history
    .filter(
      (image) =>
        (view.value !== "favorites" || image.favorite) &&
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
const trashed = computed(() =>
  studio.state.trash
    .filter(
      (image) =>
        (model.value === "all" || image.params.model === model.value) &&
        `${image.title} ${image.prompt}`
          .toLowerCase()
          .includes(search.value.toLowerCase()),
    )
    .sort(
      (a, b) => (b.deletedAt ?? 0) - (a.deletedAt ?? 0),
    ),
);
function dayLabel(timestamp: number | string): string {
  const date = new Date(timestamp);
  const today = new Date().toDateString();
  const yesterday = new Date(Date.now() - 86400000).toDateString();
  if (date.toDateString() === today) return t("history.today");
  if (date.toDateString() === yesterday) return t("history.yesterday");
  return date.toLocaleDateString([], {
    month: "long",
    day: "numeric",
    year: "numeric",
  });
}
const groups = computed(() => {
  const groups = new Map<string, ImageResult[]>();
  const source = view.value === "trash" ? trashed.value : filtered.value;
  for (const image of source.slice(0, limit.value)) {
    const label = dayLabel(
      view.value === "trash"
        ? (image.deletedAt ?? image.createdAt)
        : image.createdAt,
    );
    if (!groups.has(label)) groups.set(label, []);
    groups.get(label)!.push(image);
  }
  return [...groups].map(([label, images]) => ({ label, images }));
});
const total = computed(() =>
  view.value === "trash" ? trashed.value.length : filtered.value.length,
);
function reset() {
  search.value = "";
  model.value = "all";
  if (view.value !== "trash") view.value = "all";
}
async function download(image: ImageResult) {
  try {
    if (image.path && backendAvailable()) {
      const target = await exportImageTo(
        image.path,
        suggestedExportName(image),
      );
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
async function copyImage(image: ImageResult) {
  try {
    await copyResultImage(image);
    studio.notify(t("toast.imageCopied"));
  } catch {
    studio.notify(t("toast.clipboardFail"), "error");
  }
}
function trashImage(image: ImageResult) {
  studio.trashImage(image.id);
  selected.value = null;
}
function restoreImage(image: ImageResult) {
  studio.restoreImage(image);
  selected.value = null;
}
function purgeImage() {
  if (purging.value) studio.purgeImage(purging.value.id);
  purging.value = null;
  selected.value = null;
}
async function emptyTrash() {
  studio.emptyTrash();
  emptyingTrash.value = false;
  selected.value = null;
}
</script>
<template>
  <div class="history-view">
    <HistoryToolbar
      :search="search"
      :view="view"
      :trash-count="studio.state.trash.length"
      :model="model"
      :models="models"
      :sort="sort"
      :layout="layout"
      :total="filtered.length"
      @search="search = $event"
      @view="view = $event"
      @empty-trash="emptyingTrash = true"
      @model="model = $event"
      @sort="sort = $event"
      @layout="layout = $event"
    /><HistoryGallery
      :groups="groups"
      :layout="layout"
      :trash="view === 'trash'"
      @open="selected = $event"
      @favorite="studio.toggleFavorite"
      @restore="restoreImage"
      @purge="purging = $event"
      @reset="reset"
    />
    <div v-if="total > limit" class="load-more">
      <button class="btn" @click="limit += 24">{{ t("history.loadMore") }}</button>
    </div>
    <div v-if="total" class="history-end">
      <span />{{
        t("history.footer", {
          shown: Math.min(limit, total),
          total,
        })
      }}<span />
    </div>
    <HistoryDetail
      v-if="selected"
      :image="selected"
      :trash="view === 'trash'"
      @close="selected = null"
      @reuse="studio.reuseImage"
      @favorite="studio.toggleFavorite"
      @download="download"
      @open-folder="openFolder"
      @copy="copy"
      @copy-image="copyImage"
      @trash="trashImage"
      @restore="restoreImage"
      @purge="purging = $event"
    /><BaseDialog
      v-if="purging"
      :title="t('history.purge.title')"
      :description="t('history.purge.desc', { title: purging.title })"
      @close="purging = null"
      ><p class="secondary">{{ t("history.purge.body") }}</p>
      <template #footer
        ><button class="btn" @click="purging = null">{{
          t("history.purge.keep")
        }}</button
        ><button class="btn btn-danger" @click="purgeImage">
          {{ t("history.purge.confirm") }}
        </button></template
      ></BaseDialog
    ><BaseDialog
      v-if="emptyingTrash"
      :title="t('history.emptyTrash.title')"
      :description="
        t('history.emptyTrash.desc', { n: studio.state.trash.length })
      "
      @close="emptyingTrash = false"
      ><p class="secondary">{{ t("history.emptyTrash.body") }}</p>
      <template #footer
        ><button class="btn" @click="emptyingTrash = false">{{
          t("history.emptyTrash.keep")
        }}</button
        ><button class="btn btn-danger" @click="emptyTrash">
          {{ t("history.emptyTrash.confirm") }}
        </button></template
      ></BaseDialog
    >
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

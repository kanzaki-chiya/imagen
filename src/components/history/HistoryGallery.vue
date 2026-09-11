<script setup lang="ts">
import { Star, ArrowUpRight, SearchX, Trash2, Undo2 } from "lucide-vue-next";
import type { ImageResult } from "../../types";
import ResultImage from "../ui/ResultImage.vue";
import { useI18n } from "../../i18n";
const { t, tp } = useI18n();
defineProps<{
  groups: { label: string; images: ImageResult[] }[];
  layout: "grid" | "list";
  trash?: boolean;
}>();
const emit = defineEmits<{
  open: [image: ImageResult];
  favorite: [id: string];
  restore: [image: ImageResult];
  purge: [image: ImageResult];
  reset: [];
}>();
</script>
<template>
  <div v-if="!groups.length" class="empty-state history-empty">
    <SearchX :size="34" />
    <template v-if="trash">
      <h3>{{ t("history.trashEmptyTitle") }}</h3>
      <p>{{ t("history.trashEmptyBody") }}</p>
    </template>
    <template v-else>
      <h3>{{ t("history.emptyTitle") }}</h3>
      <p>{{ t("history.emptyBody") }}</p>
      <button class="btn" @click="emit('reset')">{{ t("history.clearFilters") }}</button>
    </template>
  </div>
  <section
    v-for="group in groups"
    v-else
    :key="group.label"
    class="history-group"
  >
    <div class="group-heading">
      <h2>{{ group.label }}</h2>
      <span>{{ tp("history.groupCount", group.images.length) }}</span>
      <div class="group-line" />
    </div>
    <div class="history-grid" :class="{ 'list-layout': layout === 'list' }">
      <article
        v-for="image in group.images"
        :key="image.id"
        class="history-item"
      >
        <button
          class="history-image"
          :aria-label="t('history.view', { title: image.title })"
          @click="emit('open', image)"
        >
          <ResultImage
            :src="image.thumb ?? image.src"
            :alt="image.title"
            :filter="image.filter"
            :position="image.position"
            lazy
          /><span class="view-overlay"><ArrowUpRight :size="17" /></span
          ><span class="image-size mono"
            >{{ image.width }} × {{ image.height }}</span
          >
        </button>
        <div class="history-caption">
          <button class="image-title" @click="emit('open', image)">
            {{ image.title }}</button
          ><span class="image-model"
            >{{ image.params.model }}<span>·</span
            >{{
              new Date(image.createdAt).toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
              })
            }}</span
          >
          <p class="list-prompt">{{ image.prompt }}</p>
        </div>
        <div v-if="trash" class="trash-actions">
          <button
            class="favorite-button"
            :aria-label="t('history.restore', { title: image.title })"
            @click="emit('restore', image)"
          >
            <Undo2 :size="14" />
          </button>
          <button
            class="favorite-button purge"
            :aria-label="t('history.deleteForever', { title: image.title })"
            @click="emit('purge', image)"
          >
            <Trash2 :size="14" />
          </button>
        </div>
        <button
          v-else
          class="favorite-button"
          :class="{ favorited: image.favorite }"
          :aria-label="
            t(image.favorite ? 'history.favRemove' : 'history.favAdd', {
              title: image.title,
            })
          "
          :aria-pressed="image.favorite"
          @click="emit('favorite', image.id)"
        >
          <Star :size="14" :fill="image.favorite ? 'currentColor' : 'none'" />
        </button>
      </article>
    </div>
  </section>
</template>
<style scoped>
.history-group {
  padding: 24px 0 4px;
}
.group-heading {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 17px;
}
.group-heading h2 {
  font-size: 12px;
  font-weight: 600;
}
.group-heading > span {
  color: var(--text-muted);
  font-size: 10px;
}
.group-line {
  height: 1px;
  background: var(--border);
  flex: 1;
  margin-left: 7px;
}
.history-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 24px 17px;
}
.history-item {
  min-width: 0;
  position: relative;
}
.history-image {
  display: block;
  width: 100%;
  aspect-ratio: 3/2;
  padding: 0;
  border-radius: 6px;
  position: relative;
  overflow: hidden;
  border: 1px solid var(--border);
}
.view-overlay {
  position: absolute;
  top: 10px;
  right: 10px;
  width: 26px;
  height: 26px;
  display: grid;
  place-items: center;
  background: #f8faf2e6;
  color: #30382a;
  border-radius: 4px;
  opacity: 0;
  transform: translateY(3px);
  transition:
    opacity 120ms,
    transform 120ms;
}
.history-image:hover .view-overlay,
.history-image:focus-visible .view-overlay {
  opacity: 1;
  transform: none;
}
.image-size {
  position: absolute;
  bottom: 8px;
  left: 9px;
  color: #fff;
  text-shadow: 0 1px 4px #000;
  font-size: 8px;
  opacity: 0;
}
.history-image:hover .image-size {
  opacity: 1;
}
.history-caption {
  padding: 10px 24px 0 1px;
}
.image-title {
  display: block;
  font-size: 12px;
  font-weight: 500;
  padding: 0;
  text-align: left;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}
.image-title:hover {
  color: var(--accent);
}
.image-model {
  font-size: 9px;
  color: var(--text-muted);
  display: flex;
  gap: 6px;
  margin-top: 4px;
}
.favorite-button {
  position: absolute;
  right: 0;
  bottom: 20px;
  padding: 3px;
  color: var(--text-muted);
}
.favorite-button.favorited {
  color: var(--accent);
}
.favorite-button:hover {
  color: var(--accent);
}
.trash-actions {
  position: absolute;
  right: 0;
  bottom: 20px;
  display: flex;
  gap: 7px;
}
.trash-actions .favorite-button {
  position: static;
}
.favorite-button.purge:hover {
  color: var(--danger);
}
.list-prompt {
  display: none;
}
.history-empty {
  min-height: 400px;
}
.list-layout {
  display: flex;
  flex-direction: column;
  gap: 0;
}
.list-layout .history-item {
  display: flex;
  align-items: center;
  gap: 18px;
  padding: 14px 0;
  border-bottom: 1px solid var(--border);
}
.list-layout .history-image {
  width: 132px;
  flex-shrink: 0;
}
.list-layout .history-caption {
  flex: 1;
  padding: 0 30px 0 0;
  min-width: 0;
}
.list-layout .list-prompt {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  color: var(--text-secondary);
  font-size: 11px;
  margin-top: 7px;
  max-width: 800px;
}
.list-layout .favorite-button {
  bottom: auto;
  top: 20px;
}
.list-layout .trash-actions {
  bottom: auto;
  top: 20px;
}
@media (min-width: 1900px) {
  .history-grid:not(.list-layout) {
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: 26px 22px;
  }
}
@media (max-width: 1300px) {
  .history-grid {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}
@media (max-width: 850px) {
  .history-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
@media (max-width: 550px) {
  .history-grid {
    gap: 20px 11px;
  }
  .image-title {
    font-size: 11px;
  }
  .image-model {
    font-size: 8px;
    flex-wrap: wrap;
    gap: 2px;
  }
  .list-layout .history-image {
    width: 85px;
  }
  .list-layout .history-item {
    gap: 12px;
  }
}
</style>

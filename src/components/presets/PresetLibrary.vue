<script setup lang="ts">
import {
  ArrowUpRight,
  SlidersHorizontal,
  Pencil,
  Trash2,
  Bookmark,
} from "lucide-vue-next";
import type { Preset } from "../../types";
import ResultImage from "../ui/ResultImage.vue";
import { useI18n } from "../../i18n";
const { t } = useI18n();
defineProps<{ presets: Preset[] }>();
const emit = defineEmits<{
  apply: [preset: Preset];
  edit: [preset: Preset];
  remove: [preset: Preset];
}>();
</script>
<template>
  <div v-if="!presets.length" class="empty-state">
    <Bookmark :size="32" />
    <h3>{{ t("presets.emptyTitle") }}</h3>
    <p>{{ t("presets.emptyBody") }}</p>
  </div>
  <div v-else class="preset-grid">
    <article v-for="preset in presets" :key="preset.id" class="preset-item">
      <button
        class="preset-image"
        :aria-label="t('presets.apply', { name: preset.name })"
        @click="emit('apply', preset)"
      >
        <ResultImage :src="preset.image" :alt="preset.name" lazy /><span
          class="preset-category badge"
          >{{ t(`presets.cat.${preset.category}`) }}</span
        >
      </button>
      <div class="preset-content">
        <div class="preset-title-row">
          <h2>{{ preset.name }}</h2>
          <button
            class="icon-btn edit-preset"
            :aria-label="t('presets.edit', { name: preset.name })"
            @click="emit('edit', preset)"
          >
            <Pencil :size="13" /></button
          ><button
            v-if="preset.category === 'My presets'"
            class="icon-btn"
            :aria-label="t('presets.delete', { name: preset.name })"
            @click="emit('remove', preset)"
          >
            <Trash2 :size="13" />
          </button>
        </div>
        <p class="preset-description">
          {{ preset.description || t("presets.defaultDesc") }}
        </p>
        <p class="preset-prompt">{{ preset.prompt }}</p>
        <div class="preset-footer">
          <span
            ><SlidersHorizontal :size="12" />{{ preset.params.aspectRatio
            }}<span class="muted">·</span>{{ preset.params.quality }}</span
          ><button class="btn apply-preset" @click="emit('apply', preset)">
            {{ t("presets.use") }}<ArrowUpRight :size="13" />
          </button>
        </div>
      </div>
    </article>
  </div>
</template>
<style scoped>
.preset-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 24px;
}
.preset-item {
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: 7px;
  background: var(--panel);
}
.preset-image {
  display: block;
  position: relative;
  width: 100%;
  aspect-ratio: 1.85;
  padding: 0;
  overflow: hidden;
}
.preset-image :deep(img) {
  transition: transform 220ms;
}
.preset-image:hover :deep(img) {
  transform: scale(1.025);
}
.preset-category {
  position: absolute;
  left: 13px;
  bottom: 12px;
  background: #f7f9f0ed;
  color: #353b2f;
  font-size: 9px;
  border-radius: 4px;
  padding: 3px 7px;
}
.preset-content {
  padding: 16px;
}
.preset-title-row {
  display: flex;
  align-items: center;
  gap: 3px;
}
.preset-title-row h2 {
  flex: 1;
  font-size: 14px;
  font-weight: 600;
}
.preset-title-row .icon-btn {
  width: 23px;
  height: 25px;
}
.preset-description {
  font-size: 11px;
  margin-top: 5px;
  color: var(--text-muted);
  min-height: 34px;
}
.preset-prompt {
  margin-top: 14px;
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.8;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  height: 59px;
}
.preset-footer {
  margin-top: 20px;
  padding-top: 13px;
  border-top: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.preset-footer > span {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 9px;
  color: var(--text-muted);
}
.apply-preset {
  min-height: 29px;
  font-size: 10px;
  padding: 4px 9px;
}
.apply-preset:hover {
  color: var(--accent);
  border-color: var(--accent);
}
@media (min-width: 1800px) {
  .preset-grid {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }
}
@media (max-width: 1050px) {
  .preset-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 18px;
  }
}
@media (max-width: 600px) {
  .preset-grid {
    grid-template-columns: 1fr;
  }
}
</style>

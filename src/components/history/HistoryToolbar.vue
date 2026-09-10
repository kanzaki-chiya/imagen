<script setup lang="ts">
import { Search, Star, LayoutGrid, List, X } from "lucide-vue-next";
import { useI18n } from "../../i18n";
const { t } = useI18n();
defineProps<{
  search: string;
  favorites: boolean;
  model: string;
  models: string[];
  sort: string;
  layout: "grid" | "list";
  total: number;
}>();
const emit = defineEmits<{
  search: [value: string];
  favorites: [];
  model: [value: string];
  sort: [value: string];
  layout: [value: "grid" | "list"];
}>();
</script>
<template>
  <div class="history-toolbar">
    <div class="history-tabs">
      <button
        :class="{ active: !favorites }"
        @click="favorites && emit('favorites')"
      >
        {{ t("history.all") }}<span class="badge">{{ total }}</span></button
      ><button
        :class="{ active: favorites }"
        @click="!favorites && emit('favorites')"
      >
        <Star :size="13" />{{ t("history.favorites") }}
      </button>
    </div>
    <div class="history-controls">
      <div class="search-wrap">
        <Search :size="14" /><input
          :value="search"
          :aria-label="t('history.searchAria')"
          :placeholder="t('history.searchPlaceholder')"
          @input="emit('search', ($event.target as HTMLInputElement).value)"
        /><button
          v-if="search"
          class="clear-search"
          :aria-label="t('history.clearSearch')"
          @click="emit('search', '')"
        >
          <X :size="12" />
        </button>
      </div>
      <select
        :value="model"
        :aria-label="t('history.modelFilter')"
        @change="emit('model', ($event.target as HTMLSelectElement).value)"
      >
        <option value="all">{{ t("history.allModels") }}</option>
        <option v-for="item in models" :key="item" :value="item">
          {{ item }}
        </option></select
      ><select
        :value="sort"
        :aria-label="t('history.sort')"
        @change="emit('sort', ($event.target as HTMLSelectElement).value)"
      >
        <option value="newest">{{ t("history.newest") }}</option>
        <option value="oldest">{{ t("history.oldest") }}</option>
      </select>
      <div class="segmented layout-switch">
        <button
          :class="{ selected: layout === 'grid' }"
          :aria-label="t('history.gridView')"
          :aria-pressed="layout === 'grid'"
          @click="emit('layout', 'grid')"
        >
          <LayoutGrid :size="14" /></button
        ><button
          :class="{ selected: layout === 'list' }"
          :aria-label="t('history.listView')"
          :aria-pressed="layout === 'list'"
          @click="emit('layout', 'list')"
        >
          <List :size="14" />
        </button>
      </div>
    </div>
  </div>
</template>
<style scoped>
.history-toolbar {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  gap: 15px;
  align-items: center;
  padding: 0 0 22px;
  border-bottom: 1px solid var(--border);
}
.history-tabs {
  display: flex;
  align-items: center;
  gap: 20px;
}
.history-tabs button {
  display: flex;
  gap: 7px;
  align-items: center;
  color: var(--text-muted);
  font-size: 12px;
  padding: 5px 0;
}
.history-tabs button.active {
  color: var(--text);
  font-weight: 600;
}
.history-tabs .badge {
  font-size: 9px;
}
.history-controls {
  display: flex;
  align-items: center;
  gap: 9px;
}
.history-controls select {
  font-size: 10px;
  min-height: 32px;
  max-width: 144px;
}
.search-wrap {
  position: relative;
}
.search-wrap > svg {
  position: absolute;
  left: 9px;
  top: 10px;
  color: var(--text-muted);
}
.search-wrap input {
  padding-left: 30px;
  padding-right: 26px;
  width: 185px;
  font-size: 11px;
  min-height: 32px;
}
.clear-search {
  position: absolute;
  right: 5px;
  top: 8px;
  color: var(--text-muted);
}
.layout-switch {
  height: 31px;
}
.layout-switch button {
  padding: 3px 5px;
}
@media (max-width: 1000px) {
  .history-controls {
    flex-wrap: wrap;
  }
  .search-wrap input {
    width: 170px;
  }
}
@media (max-width: 600px) {
  .history-controls {
    width: 100%;
  }
  .search-wrap {
    width: 100%;
  }
  .search-wrap input {
    width: 100%;
  }
  .history-controls select {
    flex: 1;
    max-width: none;
  }
}
</style>

<script setup lang="ts">
import { computed, shallowRef } from "vue";
import { Search, Plus, Lightbulb } from "lucide-vue-next";
import { useStudio } from "../composables/useStudio";
import type { Preset } from "../types";
import PresetLibrary from "../components/presets/PresetLibrary.vue";
import PresetDialog from "../components/presets/PresetDialog.vue";
import BaseDialog from "../components/ui/BaseDialog.vue";
import { useI18n } from "../i18n";
const { t, tp } = useI18n();
const studio = useStudio();
const category = shallowRef("All presets");
const search = shallowRef("");
const showEditor = shallowRef(false);
const editing = shallowRef<Preset | undefined>();
const deleting = shallowRef<Preset | null>(null);
const categories = [
  "All presets",
  "Photography",
  "Illustration",
  "Product",
  "My presets",
];
const filtered = computed(() =>
  studio.state.presets.filter(
    (preset) =>
      (category.value === "All presets" ||
        preset.category === category.value) &&
      `${preset.name} ${preset.description} ${preset.prompt}`
        .toLowerCase()
        .includes(search.value.toLowerCase()),
  ),
);
function edit(preset?: Preset) {
  editing.value = preset;
  showEditor.value = true;
}
function save(name: string, description: string, prompt: string, id?: string) {
  studio.savePreset(name, description, prompt, id);
  showEditor.value = false;
}
function remove() {
  if (deleting.value) studio.removePreset(deleting.value.id);
  deleting.value = null;
}
</script>
<template>
  <div class="presets-view">
    <div class="presets-toolbar">
      <div class="preset-tabs" role="group" :aria-label="t('presets.all')">
        <button
          v-for="item in categories"
          :key="item"
          :class="{ selected: category === item }"
          :aria-pressed="category === item"
          @click="category = item"
        >
          {{ t(`presets.cat.${item}`) }}
        </button>
      </div>
      <div class="preset-actions">
        <div class="preset-search">
          <Search :size="14" /><input
            v-model="search"
            :aria-label="t('presets.searchAria')"
            :placeholder="t('presets.searchPlaceholder')"
          />
        </div>
        <button class="btn btn-primary" @click="edit()">
          <Plus :size="14" />{{ t("presets.new") }}
        </button>
      </div>
    </div>
    <div class="library-heading">
      <span class="eyebrow">{{
        category === "All presets"
          ? t("presets.toolkit")
          : t(`presets.cat.${category}`)
      }}</span
      ><span class="muted">{{ tp("presets.count", filtered.length) }}</span>
    </div>
    <PresetLibrary
      :presets="filtered"
      @apply="studio.applyPreset"
      @edit="edit"
      @remove="deleting = $event"
    />
    <div class="preset-tip">
      <Lightbulb :size="15" />
      <p>{{ t("presets.tip") }}</p>
    </div>
    <PresetDialog
      v-if="showEditor"
      :prompt="studio.state.prompt"
      :preset="editing"
      @close="showEditor = false"
      @save="save"
    /><BaseDialog
      v-if="deleting"
      :title="t('presets.delete.title')"
      :description="t('presets.delete.desc', { name: deleting.name })"
      @close="deleting = null"
      ><p class="secondary">{{ t("presets.delete.body") }}</p>
      <template #footer
        ><button class="btn" @click="deleting = null">{{
          t("presets.delete.keep")
        }}</button
        ><button class="btn btn-danger" @click="remove">
          {{ t("presets.delete.confirm") }}
        </button></template
      ></BaseDialog
    >
  </div>
</template>
<style scoped>
.presets-view {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 27px 28px;
}
.presets-toolbar {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
  border-bottom: 1px solid var(--border);
  padding-bottom: 22px;
}
.preset-tabs {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}
.preset-tabs button {
  font-size: 11px;
  color: var(--text-muted);
  padding: 7px 10px;
  border-radius: 5px;
}
.preset-tabs button.selected {
  background: var(--active);
  color: var(--text);
  font-weight: 500;
}
.preset-tabs button:hover {
  color: var(--text);
}
.preset-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}
.preset-actions .btn {
  font-size: 11px;
}
.preset-search {
  position: relative;
}
.preset-search > svg {
  position: absolute;
  left: 9px;
  top: 10px;
  color: var(--text-muted);
}
.preset-search input {
  padding-left: 30px;
  font-size: 11px;
  width: 170px;
}
.library-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 23px 0 19px;
}
.library-heading > .muted {
  font-size: 10px;
}
.preset-tip {
  display: flex;
  justify-content: center;
  gap: 9px;
  color: var(--text-muted);
  font-size: 10px;
  margin-top: 32px;
}
@media (max-width: 650px) {
  .presets-view {
    padding: 18px;
  }
  .preset-actions {
    width: 100%;
  }
  .preset-search {
    flex: 1;
  }
  .preset-search input {
    width: 100%;
  }
  .preset-tabs button {
    padding: 6px 8px;
    font-size: 10px;
  }
  .preset-tip {
    align-items: flex-start;
  }
}
</style>

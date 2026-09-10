<script setup lang="ts">
import { Monitor, Moon, Sun } from "lucide-vue-next";
import type { Theme } from "../../types";
import { useI18n } from "../../i18n";
const { t } = useI18n();
defineProps<{ modelValue: Theme; expanded?: boolean }>();
const emit = defineEmits<{ "update:modelValue": [theme: Theme] }>();
const themes = [
  { id: "light", icon: Sun },
  { id: "dark", icon: Moon },
  { id: "system", icon: Monitor },
] as const;
</script>
<template>
  <div
    class="segmented theme-switcher"
    :class="{ expanded }"
    role="group"
    :aria-label="t('theme.aria')"
  >
    <button
      v-for="theme in themes"
      :key="theme.id"
      :aria-label="
        t('theme.labelAria', { label: t(`theme.${theme.id}`) })
      "
      :aria-pressed="modelValue === theme.id"
      :class="{ selected: modelValue === theme.id }"
      :data-tip="expanded ? undefined : t(`theme.${theme.id}`)"
      @click="emit('update:modelValue', theme.id)"
    >
      <component :is="theme.icon" :size="14" /><span v-if="expanded">{{
        t(`theme.${theme.id}`)
      }}</span>
    </button>
  </div>
</template>
<style scoped>
.theme-switcher {
  padding: 2px;
  gap: 2px;
}
.theme-switcher button {
  padding: 5px 7px;
}
.theme-switcher.expanded button {
  min-width: 88px;
  padding: 8px 12px;
}
</style>

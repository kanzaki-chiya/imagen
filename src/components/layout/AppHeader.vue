<script setup lang="ts">
import {
  Plus,
  Check,
  LoaderCircle,
  SlidersHorizontal,
  CloudOff,
} from "lucide-vue-next";
import type { Theme, Workspace } from "../../types";
import ThemeSwitcher from "../ui/ThemeSwitcher.vue";
import { useI18n } from "../../i18n";
defineProps<{
  page: Workspace;
  theme: Theme;
  saving: boolean;
  persistenceError: boolean;
  generating: boolean;
}>();
const emit = defineEmits<{
  theme: [value: Theme];
  newSession: [];
  toggleParameters: [];
}>();
const { t } = useI18n();
</script>
<template>
  <header class="app-header">
    <div class="page-heading">
      <h1>{{ t(`page.${page}.title`) }}</h1>
      <p>{{ t(`page.${page}.sub`) }}</p>
    </div>
    <div class="header-actions">
      <span class="save-status"
        ><CloudOff v-if="persistenceError" :size="13" /><LoaderCircle
          v-else-if="saving"
          :size="13"
          class="spin"
        /><Check v-else :size="13" />{{
          persistenceError
            ? t("header.notSaved")
            : saving
              ? t("header.saving")
              : t("header.saved")
        }}</span
      >
      <ThemeSwitcher
        :model-value="theme"
        @update:model-value="emit('theme', $event)"
      />
      <button
        v-if="page === 'generate'"
        class="icon-btn parameters-toggle"
        :aria-label="t('header.toggleParams')"
        @click="emit('toggleParameters')"
      >
        <SlidersHorizontal :size="18" />
      </button>
      <button
        class="btn new-session"
        :disabled="generating"
        @click="emit('newSession')"
      >
        <Plus :size="15" /><span>{{ t("header.newSession") }}</span>
      </button>
    </div>
  </header>
</template>
<style scoped>
.app-header {
  height: 75px;
  min-height: 75px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
  padding: 0 27px;
  border-bottom: 1px solid var(--border);
  background: var(--panel);
}
.page-heading h1 {
  font-size: 20px;
  font-weight: 600;
  letter-spacing: -0.5px;
}
.page-heading p {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 4px;
}
.header-actions {
  display: flex;
  align-items: center;
  gap: 14px;
}
.save-status {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  font-size: 10px;
  color: var(--text-muted);
  white-space: nowrap;
}
.parameters-toggle {
  display: none;
}
@media (max-width: 1180px) {
  .app-header {
    height: 74px;
    min-height: 74px;
    padding: 0 20px;
  }
  .header-actions {
    gap: 9px;
  }
  .save-status {
    display: none;
  }
}
@media (max-width: 850px) {
  .parameters-toggle {
    display: inline-flex;
  }
}
@media (max-width: 600px) {
  .app-header {
    padding: 0 14px;
    height: 68px;
    min-height: 68px;
  }
  .page-heading h1 {
    font-size: 18px;
  }
  .page-heading p,
  .new-session span {
    display: none;
  }
  .header-actions {
    gap: 6px;
  }
  .new-session {
    padding: 6px;
  }
}
</style>

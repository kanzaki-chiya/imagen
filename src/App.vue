<script setup lang="ts">
import { computed, shallowRef, watch } from "vue";
import { CircleHelp, ChevronRight, LoaderCircle } from "lucide-vue-next";
import { provideStudio } from "./composables/useStudio";
import { useTheme } from "./composables/useTheme";
import { useWorkspaceNavigation } from "./composables/useWorkspaceNavigation";
import { useI18n } from "./i18n";
import { usingDesktopBackend } from "./services/backend";
import AppSidebar from "./components/layout/AppSidebar.vue";
import AppHeader from "./components/layout/AppHeader.vue";
import ToastStack from "./components/ui/ToastStack.vue";
import BaseDialog from "./components/ui/BaseDialog.vue";
import GenerateView from "./views/GenerateView.vue";
import HistoryView from "./views/HistoryView.vue";
import PresetsView from "./views/PresetsView.vue";
import SettingsView from "./views/SettingsView.vue";

const studio = provideStudio();
const { state, provider } = studio;
const { preference, setTheme } = useTheme();
const { t } = useI18n();
const live = computed(() => usingDesktopBackend(state.preferMock));
const queuedCount = computed(
  () => state.tasks.filter((task) => task.status === "pending").length,
);
const showShortcuts = shallowRef(false);
const drawerOpen = shallowRef(false);
useWorkspaceNavigation({
  current: () => state.page,
  navigate: studio.navigate,
  generate: () => studio.generate(),
  shortcuts: () => {
    showShortcuts.value = true;
  },
});
watch(
  () => state.page,
  () => {
    drawerOpen.value = false;
  },
);
const shortcutKeys = [
  "shortcuts.generate",
  "shortcuts.workspace.generate",
  "shortcuts.workspace.history",
  "shortcuts.workspace.presets",
  "shortcuts.workspace.settings",
  "shortcuts.switchImage",
  "shortcuts.closeDialog",
  "shortcuts.help",
] as const;
const shortcutCombos = [
  "Ctrl + Enter",
  "Ctrl + Alt + 1",
  "Ctrl + Alt + 2",
  "Ctrl + Alt + 3",
  "Ctrl + Alt + 4",
  "← / →",
  "Esc",
  "?",
];
</script>
<template>
  <div class="app">
    <div class="app-body" :inert="!state.ready" :aria-busy="!state.ready">
      <AppSidebar
        :page="state.page"
        :live="live"
        :history-count="state.history.length"
        :preset-count="state.presets.length"
        @navigate="studio.navigate"
        @new-session="studio.newSession"
        @shortcuts="showShortcuts = true"
      />
      <main class="app-main">
        <AppHeader
          :page="state.page"
          :theme="preference"
          :saving="state.saving"
          :persistence-error="state.persistenceError"
          :generating="state.generating"
          @theme="setTheme"
          @new-session="studio.newSession"
          @toggle-parameters="drawerOpen = !drawerOpen"
        /><GenerateView
          v-if="state.page === 'generate'"
          :drawer-open="drawerOpen"
          @close-drawer="drawerOpen = false"
        /><HistoryView v-else-if="state.page === 'history'" /><PresetsView
          v-else-if="state.page === 'presets'"
        /><SettingsView v-else :theme="preference" @theme="setTheme" />
      </main>
    </div>
    <footer class="statusbar">
      <div class="statusbar-left">
        <LoaderCircle v-if="state.generating" :size="10" class="spin" /><span
          v-else
          class="status-dot"
          :class="{ error: !!state.taskError }"
        /><span>{{
          state.generating
            ? t("status.generating", { progress: state.progress })
            : state.taskError
              ? t("status.needsAttention")
              : t("status.ready")
        }}</span
        ><button
          v-if="state.generating && state.page !== 'generate'"
          @click="studio.navigate('generate')"
        >
          {{ t("status.viewTask") }}<ChevronRight :size="10" />
        </button
        ><span v-if="queuedCount" class="statusbar-queue">{{
          t("status.queued", { n: queuedCount })
        }}</span>
      </div>
      <div class="statusbar-right">
        <span>{{ provider.name }}</span
        ><span class="statusbar-separator">/</span
        ><span>{{ state.params.model }}</span
        ><span class="statusbar-divider" /><button
          class="statusbar-help"
          :aria-label="t('shortcuts.help')"
          @click="showShortcuts = true"
        >
          <CircleHelp :size="11" /><span>{{ t("status.tagline") }}</span>
        </button>
      </div>
    </footer>
    <ToastStack
      :toasts="state.toasts"
      @dismiss="studio.dismissToast"
    /><BaseDialog
      v-if="showShortcuts"
      :title="t('shortcuts.title')"
      :description="t('shortcuts.sub')"
      @close="showShortcuts = false"
      ><div class="shortcut-list">
        <div v-for="(key, index) in shortcutKeys" :key="key">
          <span>{{ t(key) }}</span
          ><kbd>{{ shortcutCombos[index] }}</kbd>
        </div>
      </div>
      <template #footer
        ><button class="btn btn-primary" @click="showShortcuts = false">
          {{ t("shortcuts.gotIt") }}
        </button></template
      ></BaseDialog
    >
  </div>
</template>
<style scoped>
.app {
  height: 100dvh;
  min-height: 540px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.app-body {
  display: flex;
  flex: 1;
  min-height: 0;
}
.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
}
.statusbar {
  height: 25px;
  min-height: 25px;
  padding: 0 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
  border-top: 1px solid var(--border);
  background: var(--sidebar);
  color: var(--text-muted);
  font-size: 9px;
}
.statusbar-left,
.statusbar-right {
  display: flex;
  align-items: center;
  gap: 6px;
}
.statusbar-left > .status-dot {
  width: 5px;
  height: 5px;
}
.statusbar-left > button {
  display: inline-flex;
  align-items: center;
  color: var(--accent);
  font-size: 9px;
}
.statusbar-queue {
  color: var(--text-muted);
  font-size: 9px;
}
.statusbar-right {
  gap: 9px;
}
.statusbar-separator {
  color: var(--border-strong);
}
.statusbar-divider {
  height: 11px;
  width: 1px;
  background: var(--border);
  margin: 0 3px;
}
.statusbar-help {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
  font-size: 9px;
  padding: 0;
}
.statusbar-help:hover {
  color: var(--text);
}
.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: 0;
}
.shortcut-list > div {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 12px 0;
  border-bottom: 1px solid var(--border);
  font-size: 12px;
}
.shortcut-list > div:last-child {
  border-bottom: 0;
}
.shortcut-list kbd {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-secondary);
  background: var(--input);
  border: 1px solid var(--border);
  border-bottom: 2px solid var(--border-strong);
  border-radius: 4px;
  padding: 3px 6px;
}
@media (max-width: 800px) {
  .statusbar-help span {
    display: none;
  }
  .statusbar {
    padding: 0 10px;
    font-size: 8px;
  }
  .statusbar-right {
    gap: 6px;
  }
}
@media (max-width: 500px) {
  .statusbar-right > span:first-child,
  .statusbar-separator,
  .statusbar-divider,
  .statusbar-help {
    display: none;
  }
}
</style>

<script setup lang="ts">
import { onMounted, onUnmounted, shallowRef } from "vue";
import { Minus, Square, Copy, X } from "lucide-vue-next";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { backendAvailable } from "../../services/backend";
import { useI18n } from "../../i18n";
import BrandMark from "../ui/BrandMark.vue";

const { t } = useI18n();
const desktop = backendAvailable();
const appWindow = desktop ? getCurrentWindow() : null;
const maximized = shallowRef(false);
let unlisten: (() => void) | undefined;

onMounted(async () => {
  if (!appWindow) return;
  maximized.value = await appWindow.isMaximized();
  unlisten = await appWindow.onResized(async () => {
    maximized.value = await appWindow.isMaximized();
  });
});
onUnmounted(() => unlisten?.());

const minimize = () => void appWindow?.minimize();
const toggleMaximize = () => void appWindow?.toggleMaximize();
const close = () => void appWindow?.close();
</script>
<template>
  <div
    class="titlebar"
    data-tauri-drag-region
    @dblclick="toggleMaximize"
  >
    <div class="titlebar-brand">
      <BrandMark :size="14" /><span class="titlebar-name">imagen.</span>
    </div>
    <div class="titlebar-drag" data-tauri-drag-region />
    <div v-if="desktop" class="titlebar-controls">
      <button
        class="win-btn"
        :aria-label="t('titlebar.minimize')"
        :data-tip="t('titlebar.minimize')"
        @click="minimize"
      >
        <Minus :size="13" /></button
      ><button
        class="win-btn"
        :aria-label="
          maximized ? t('titlebar.restore') : t('titlebar.maximize')
        "
        :data-tip="maximized ? t('titlebar.restore') : t('titlebar.maximize')"
        @click="toggleMaximize"
      >
        <Copy v-if="maximized" :size="11" /><Square v-else :size="12" /></button
      ><button
        class="win-btn close"
        :aria-label="t('titlebar.close')"
        :data-tip="t('titlebar.close')"
        @click="close"
      >
        <X :size="14" />
      </button>
    </div>
  </div>
</template>
<style scoped>
.titlebar {
  height: 34px;
  min-height: 34px;
  display: flex;
  align-items: stretch;
  background: var(--panel);
  border-bottom: 1px solid var(--border);
  user-select: none;
  -webkit-user-select: none;
}
.titlebar-brand {
  display: flex;
  align-items: center;
  gap: 7px;
  padding: 0 12px;
  color: var(--text-secondary);
}
.titlebar-name {
  font-family: var(--font-display);
  font-size: 12px;
  letter-spacing: -0.3px;
  color: var(--text);
}
.titlebar-drag {
  flex: 1;
}
.titlebar-controls {
  display: flex;
  align-items: stretch;
}
.win-btn {
  width: 44px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
}
.win-btn:hover {
  background: var(--hover);
  color: var(--text);
}
.win-btn.close:hover {
  background: #e81123;
  color: #fff;
}
@media (max-width: 600px) {
  .win-btn {
    width: 38px;
  }
}
</style>

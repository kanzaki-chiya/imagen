<script setup lang="ts">
import {
  WandSparkles,
  History,
  Bookmark,
  Settings2,
  ChevronDown,
  Plus,
  Keyboard,
  HardDrive,
  ArrowUpRight,
  Image as ImageIcon,
} from "lucide-vue-next";
import type { Workspace } from "../../types";
import BrandMark from "../ui/BrandMark.vue";
import { useI18n } from "../../i18n";
defineProps<{
  page: Workspace;
  live: boolean;
  historyCount: number;
  presetCount: number;
}>();
const emit = defineEmits<{
  navigate: [page: Workspace];
  newSession: [];
  shortcuts: [];
}>();
const { t } = useI18n();
const links = [
  { id: "generate", icon: WandSparkles },
  { id: "history", icon: History },
  { id: "presets", icon: Bookmark },
  { id: "settings", icon: Settings2 },
] as const;
</script>
<template>
  <aside class="sidebar">
    <a
      class="brand"
      href="#generate"
      :aria-label="t('sidebar.brand')"
      @click.prevent="emit('navigate', 'generate')"
      ><BrandMark /><span>imagen<span class="brand-dot">.</span></span></a
    >
    <button class="workspace-select" @click="emit('navigate', 'settings')">
      <span class="workspace-avatar">P</span
      ><span class="workspace-name"
        >{{ t("sidebar.personalWorkspace")
        }}<span class="workspace-local">{{
          t("sidebar.localWorkspace")
        }}</span></span
      ><ChevronDown :size="13" class="workspace-chevron" />
    </button>
    <span class="eyebrow section-label">{{ t("sidebar.workspace") }}</span>
    <nav class="navigation" :aria-label="t('sidebar.nav')">
      <button
        v-for="link in links"
        :key="link.id"
        class="nav-item"
        :class="{ active: page === link.id }"
        :aria-current="page === link.id ? 'page' : undefined"
        :title="t(`nav.${link.id}`)"
        @click="emit('navigate', link.id)"
      >
        <component :is="link.icon" :size="18" /><span class="nav-label">{{
          t(`nav.${link.id}`)
        }}</span
        ><span v-if="link.id === 'history'" class="nav-count">{{
          historyCount
        }}</span
        ><span v-if="link.id === 'presets'" class="nav-count">{{
          presetCount
        }}</span>
      </button>
    </nav>
    <div class="sidebar-divider" />
    <div class="recent-heading">
      <span class="eyebrow">{{ t("sidebar.quickAccess") }}</span
      ><button
        class="icon-btn"
        :aria-label="t('sidebar.newSession')"
        :title="t('sidebar.newSession')"
        @click="emit('newSession')"
      >
        <Plus :size="15" />
      </button>
    </div>
    <button class="quick-link" @click="emit('navigate', 'history')">
      <ImageIcon :size="15" /><span>{{ t("sidebar.allGenerations") }}</span
      ><ArrowUpRight :size="12" class="quick-arrow" />
    </button>
    <button class="quick-link" @click="emit('navigate', 'presets')">
      <Bookmark :size="15" /><span>{{ t("sidebar.promptLibrary") }}</span>
    </button>
    <div class="sidebar-spacer" />
    <div class="local-note">
      <div class="local-note-title">
        <HardDrive :size="15" /><span>{{ t("sidebar.localNoteTitle") }}</span>
      </div>
      <p>
        {{ t("sidebar.localNoteBody1") }}<br />{{ t("sidebar.localNoteBody2") }}
      </p>
      <span class="local-note-rule" />
    </div>
    <button
      class="shortcut-button"
      :aria-label="t('sidebar.shortcuts')"
      @click="emit('shortcuts')"
    >
      <Keyboard :size="17" /><span>{{ t("sidebar.shortcuts") }}</span
      ><span class="mono">?</span>
    </button>
    <div class="sidebar-bottom">
      <span class="status-dot" /><span>{{
        live ? t("sidebar.liveEnvironment") : t("sidebar.mockEnvironment")
      }}</span
      ><span class="mono version">v0.1</span>
    </div>
  </aside>
</template>
<style scoped>
.sidebar {
  width: 192px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: var(--sidebar);
  border-right: 1px solid var(--border);
  padding: 0 13px;
}
.brand {
  height: 86px;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 9px;
  text-decoration: none;
  font-family: var(--font-display);
  font-weight: 600;
  font-size: 27px;
  letter-spacing: -0.9px;
}
.brand-dot {
  color: var(--accent);
}
.workspace-select {
  display: flex;
  align-items: center;
  gap: 9px;
  text-align: left;
  padding: 10px 6px;
  margin: 0 0 26px;
}
.workspace-select:hover {
  background: var(--hover);
  border-radius: 6px;
}
.workspace-avatar {
  display: grid;
  place-items: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  background: var(--active);
  border: 1px solid var(--border-strong);
  color: var(--text-secondary);
  font-size: 12px;
}
.workspace-name {
  flex: 1;
  font-size: 10px;
  font-weight: 600;
}
.workspace-local {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  font-weight: 400;
  margin-top: 2px;
}
.workspace-chevron {
  color: var(--text-muted);
}
.section-label {
  padding-left: 11px;
  margin-bottom: 12px;
}
.navigation {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
.nav-item {
  display: flex;
  align-items: center;
  gap: 11px;
  height: 39px;
  padding: 0 11px;
  border-radius: 6px;
  color: var(--text-secondary);
  text-align: left;
  font-size: 12px;
}
.nav-item:hover {
  background: var(--hover);
  color: var(--text);
}
.nav-item.active {
  color: var(--accent);
  background: var(--accent-soft);
  font-weight: 600;
}
.nav-label {
  flex: 1;
}
.nav-count {
  font-size: 10px;
  color: var(--text-muted);
}
.sidebar-divider {
  height: 1px;
  background: var(--border);
  margin: 25px 8px 13px;
}
.recent-heading {
  padding: 0 3px 0 11px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.quick-link {
  display: flex;
  gap: 10px;
  align-items: center;
  padding: 10px 11px;
  text-align: left;
  color: var(--text-secondary);
  font-size: 11px;
  border-radius: 6px;
}
.quick-link:hover {
  background: var(--hover);
  color: var(--text);
}
.quick-arrow {
  margin-left: auto;
  color: var(--text-muted);
}
.sidebar-spacer {
  flex: 1;
  min-height: 20px;
}
.local-note {
  padding: 14px 10px 12px;
}
.local-note-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 10px;
  color: var(--text-secondary);
}
.local-note p {
  font-size: 10px;
  color: var(--text-muted);
  line-height: 1.8;
  margin-top: 8px;
}
.local-note-rule {
  display: block;
  width: 25px;
  height: 2px;
  background: var(--border-strong);
  margin-top: 17px;
}
.shortcut-button {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 12px 9px;
  color: var(--text-secondary);
  font-size: 10px;
}
.shortcut-button .mono {
  margin-left: auto;
}
.shortcut-button:hover {
  color: var(--text);
}
.sidebar-bottom {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 9px;
  color: var(--text-muted);
  border-top: 1px solid var(--border);
  padding: 13px 8px;
}
.version {
  margin-left: auto;
  font-size: 9px;
}
@media (min-width: 1800px) {
  .sidebar {
    width: 210px;
  }
}
@media (max-width: 1180px) {
  .sidebar {
    width: 66px;
    padding: 0 10px;
  }
  .brand {
    justify-content: center;
    padding: 0;
    height: 74px;
  }
  .brand > span,
  .workspace-select,
  .section-label,
  .nav-label,
  .nav-count,
  .recent-heading,
  .quick-link,
  .local-note,
  .shortcut-button span,
  .sidebar-bottom span:not(.status-dot) {
    display: none;
  }
  .navigation {
    gap: 10px;
  }
  .nav-item {
    justify-content: center;
    padding: 0;
    height: 42px;
  }
  .sidebar-divider {
    margin: 25px 2px;
  }
  .shortcut-button,
  .sidebar-bottom {
    justify-content: center;
  }
}
</style>

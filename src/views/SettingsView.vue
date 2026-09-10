<script setup lang="ts">
import { shallowRef } from "vue";
import {
  PlugZap,
  Palette,
  HardDrive,
  Info,
  FlaskConical,
  Monitor,
  ShieldCheck,
  ChevronRight,
  Sparkles,
  ListTodo,
} from "lucide-vue-next";
import { useStudio } from "../composables/useStudio";
import { backendAvailable } from "../services/backend";
import type { Locale } from "../i18n";
import { useI18n } from "../i18n";
import type { Theme } from "../types";
import ProviderSettings from "../components/settings/ProviderSettings.vue";
import TaskList from "../components/settings/TaskList.vue";
import ThemeSwitcher from "../components/ui/ThemeSwitcher.vue";
import BrandMark from "../components/ui/BrandMark.vue";
defineProps<{ theme: Theme }>();
const emit = defineEmits<{ theme: [value: Theme] }>();
const { t, locale, setLocale } = useI18n();
const languages: { id: Locale; label: string }[] = [
  { id: "zh", label: "中文" },
  { id: "en", label: "English" },
];
const studio = useStudio();
const liveBackend = backendAvailable();
const section = shallowRef("providers");
const sections = [
  { id: "providers", icon: PlugZap },
  { id: "appearance", icon: Palette },
  { id: "workspace", icon: HardDrive },
  { id: "tasks", icon: ListTodo },
  { id: "about", icon: Info },
];
</script>
<template>
  <div class="settings-view">
    <nav class="settings-nav" :aria-label="t('settings.preferences')">
      <span class="eyebrow">{{ t("settings.preferences") }}</span
      ><button
        v-for="item in sections"
        :key="item.id"
        :class="{ selected: section === item.id }"
        @click="section = item.id"
      >
        <component :is="item.icon" :size="15" /><span>{{
          t(`settings.nav.${item.id}`)
        }}</span
        ><ChevronRight v-if="section === item.id" :size="12" />
      </button>
      <div class="settings-version">
        <span class="badge">{{ t("settings.versionBadge") }}</span>
        <p>
          {{ t("settings.versionNote1") }}<br />{{ t("settings.versionNote2") }}
        </p>
      </div>
    </nav>
    <div class="settings-scroll">
      <div class="settings-content">
        <ProviderSettings
          v-if="section === 'providers'"
          :providers="studio.state.providers"
          @save="studio.saveProvider"
        />
        <section v-else-if="section === 'appearance'" class="settings-section">
          <h2>{{ t("settings.appearance.title") }}</h2>
          <p>{{ t("settings.appearance.sub") }}</p>
          <div class="preference-row">
            <div>
              <h3>{{ t("settings.appearance.language") }}</h3>
              <p>{{ t("settings.appearance.languageDesc") }}</p>
            </div>
            <div
              class="segmented language-switcher"
              role="group"
              :aria-label="t('settings.appearance.language')"
            >
              <button
                v-for="item in languages"
                :key="item.id"
                :class="{ selected: locale === item.id }"
                :aria-pressed="locale === item.id"
                @click="setLocale(item.id)"
              >
                {{ item.label }}
              </button>
            </div>
          </div>
          <div class="preference-row">
            <div>
              <h3>{{ t("settings.appearance.theme") }}</h3>
              <p>{{ t("settings.appearance.themeDesc") }}</p>
            </div>
            <ThemeSwitcher
              :model-value="theme"
              expanded
              @update:model-value="emit('theme', $event)"
            />
          </div>
          <div class="theme-preview" :class="theme">
            <div class="mini-sidebar">
              <BrandMark :size="23" /><span /><span /><span />
            </div>
            <div class="mini-main">
              <div class="mini-toolbar" />
              <div class="mini-picture">
                <img
                  :src="studio.state.presets[0]?.image"
                  :alt="t('settings.appearance.previewAlt')"
                />
              </div>
              <div class="mini-prompt" />
              <div class="mini-action" />
            </div>
            <div class="mini-parameters"><span /><span /><span /><span /></div>
          </div>
          <div class="appearance-note">
            <Monitor :size="16" /><span>{{
              t("settings.appearance.note")
            }}</span>
          </div>
        </section>
        <section v-else-if="section === 'workspace'" class="settings-section">
          <h2>{{ t("settings.workspace.title") }}</h2>
          <p>{{ t("settings.workspace.sub") }}</p>
          <div class="preference-row">
            <div>
              <h3>{{ t("settings.workspace.storage") }}</h3>
              <p>{{ t("settings.workspace.storageDesc") }}</p>
            </div>
            <span class="badge badge-success"
              ><ShieldCheck :size="12" />{{
                studio.state.persistenceError
                  ? t("settings.workspace.unavailable")
                  : t("settings.workspace.available")
              }}</span
            >
          </div>
          <div class="workspace-data">
            <div>
              <span>{{ t("settings.workspace.savedImages") }}</span
              ><strong>{{ studio.state.history.length }}</strong>
            </div>
            <div>
              <span>{{ t("settings.workspace.presets") }}</span
              ><strong>{{ studio.state.presets.length }}</strong>
            </div>
            <div>
              <span>{{ t("settings.workspace.apiKeyStorage") }}</span
              ><strong>{{ t("settings.workspace.sessionOnly") }}</strong>
            </div>
          </div>
          <div class="preference-row">
            <div>
              <h3>
                <Sparkles :size="16" />{{ t("settings.workspace.backend") }}
              </h3>
              <p>
                {{
                  liveBackend
                    ? t("settings.workspace.backendDesc.live")
                    : t("settings.workspace.backendDesc.mock")
                }}
              </p>
            </div>
            <span
              class="badge"
              :class="{
                'badge-success': liveBackend && !studio.state.preferMock,
              }"
              >{{
                liveBackend
                  ? studio.state.preferMock
                    ? t("settings.workspace.backendMockOn")
                    : t("settings.workspace.backendLive")
                  : t("settings.workspace.backendMockOnly")
              }}</span
            >
          </div>
          <div v-if="liveBackend" class="preference-row">
            <div>
              <h3>{{ t("settings.workspace.preferMock") }}</h3>
              <p>{{ t("settings.workspace.preferMockDesc") }}</p>
            </div>
            <button
              class="switch"
              role="switch"
              :aria-label="t('settings.workspace.preferMock')"
              :aria-checked="studio.state.preferMock"
              :class="{ enabled: studio.state.preferMock }"
              @click="studio.setPreferMock(!studio.state.preferMock)"
            >
              <span />
            </button>
          </div>
          <div class="preference-row mock-settings">
            <div>
              <h3>
                <FlaskConical :size="16" />{{ t("settings.workspace.failNext") }}
              </h3>
              <p>{{ t("settings.workspace.failNextDesc") }}</p>
            </div>
            <button
              class="switch"
              role="switch"
              :aria-label="t('settings.workspace.failNext')"
              :aria-checked="studio.state.failNext"
              :class="{ enabled: studio.state.failNext }"
              @click="studio.setFailNext(!studio.state.failNext)"
            >
              <span />
            </button>
          </div>
          <div class="mock-explainer">
            <h3>{{ t("settings.workspace.mockTitle") }}</h3>
            <p>{{ t("settings.workspace.mockBody") }}</p>
            <button class="btn" @click="studio.navigate('generate')">
              {{ t("settings.workspace.backToGenerate")
              }}<ChevronRight :size="13" />
            </button>
          </div>
        </section>
        <TaskList v-else-if="section === 'tasks'" />
        <section v-else class="settings-section about-section">
          <BrandMark :size="48" />
          <h2>imagen.</h2>
          <p class="about-tagline">{{ t("settings.about.tagline") }}</p>
          <span class="badge">{{ t("settings.about.version") }}</span>
          <div class="about-copy">
            <p>{{ t("settings.about.p1") }}</p>
            <p>{{ t("settings.about.p2") }}</p>
          </div>
          <div class="about-tech">
            <span>Vue 3</span><span>TypeScript</span><span>Vite</span
            ><span>Tauri</span>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>
<style scoped>
.settings-view {
  display: flex;
  flex: 1;
  min-height: 0;
}
.settings-nav {
  width: 184px;
  flex-shrink: 0;
  padding: 30px 15px;
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  gap: 5px;
}
.settings-nav > .eyebrow {
  padding: 0 10px;
  margin-bottom: 15px;
}
.settings-nav > button {
  display: flex;
  align-items: center;
  gap: 9px;
  font-size: 11px;
  color: var(--text-muted);
  border-radius: 5px;
  padding: 10px;
  text-align: left;
}
.settings-nav > button.selected {
  background: var(--active);
  color: var(--text);
  font-weight: 500;
}
.settings-nav > button:hover {
  color: var(--text);
  background: var(--hover);
}
.settings-nav > button > span {
  flex: 1;
}
.settings-version {
  margin-top: auto;
  padding: 20px 10px 0;
}
.settings-version p {
  margin-top: 10px;
  font-size: 9px;
  color: var(--text-muted);
  line-height: 1.8;
}
.settings-scroll {
  flex: 1;
  overflow-y: auto;
  min-width: 0;
}
.settings-content {
  width: 100%;
  max-width: 1060px;
  padding: 33px 34px;
}
.settings-section h2 {
  font-size: 19px;
  font-weight: 600;
}
.settings-section > p {
  color: var(--text-muted);
  font-size: 12px;
  margin-top: 5px;
}
.preference-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 20px;
  border-top: 1px solid var(--border);
  padding: 24px 0;
  margin-top: 27px;
}
.preference-row h3 {
  font-size: 12px;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
}
.preference-row p {
  font-size: 10px;
  color: var(--text-muted);
  margin-top: 6px;
  line-height: 1.8;
}
.theme-preview {
  display: flex;
  width: 100%;
  height: 295px;
  border-radius: 8px;
  background: var(--panel);
  border: 1px solid var(--border);
  overflow: hidden;
}
.mini-sidebar {
  width: 65px;
  border-right: 1px solid var(--border);
  background: var(--sidebar);
  display: flex;
  align-items: center;
  flex-direction: column;
  gap: 15px;
  padding: 21px 10px;
}
.mini-sidebar > span {
  height: 5px;
  width: 27px;
  background: var(--border-strong);
  border-radius: 2px;
}
.mini-main {
  flex: 1;
  min-width: 0;
  position: relative;
  display: flex;
  flex-direction: column;
  padding: 14px;
  gap: 11px;
}
.mini-toolbar {
  height: 6px;
  width: 30%;
  border-radius: 2px;
  background: var(--border-strong);
}
.mini-picture {
  flex: 1;
  min-height: 0;
  background: var(--canvas);
  padding: 10px;
  display: flex;
  justify-content: center;
}
.mini-picture img {
  max-width: 100%;
  height: 100%;
  object-fit: cover;
}
.mini-prompt {
  height: 29px;
  border: 1px solid var(--border);
  background: var(--input);
  border-radius: 3px;
}
.mini-action {
  height: 15px;
  width: 67px;
  background: var(--accent);
  border-radius: 3px;
  align-self: flex-end;
}
.mini-parameters {
  width: 102px;
  border-left: 1px solid var(--border);
  padding: 20px 13px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.mini-parameters span {
  height: 27px;
  border: 1px solid var(--border);
  border-radius: 3px;
  background: var(--input);
}
.language-switcher {
  padding: 2px;
  gap: 2px;
}
.language-switcher button {
  min-width: 88px;
  padding: 8px 12px;
  font-size: 11px;
}
.appearance-note {
  display: flex;
  gap: 9px;
  align-items: center;
  color: var(--text-muted);
  font-size: 10px;
  margin-top: 20px;
}
.workspace-data {
  display: flex;
  flex-direction: column;
  border-top: 1px solid var(--border);
}
.workspace-data > div {
  display: flex;
  justify-content: space-between;
  gap: 18px;
  padding: 18px 0;
  border-bottom: 1px solid var(--border);
  font-size: 12px;
}
.workspace-data > div > span {
  color: var(--text-secondary);
}
.workspace-data strong {
  font-weight: 500;
}
.mock-settings {
  margin-top: 20px;
}
.switch {
  width: 34px;
  height: 20px;
  padding: 3px;
  border-radius: 12px;
  background: var(--border-strong);
  flex-shrink: 0;
}
.switch > span {
  display: block;
  width: 14px;
  height: 14px;
  background: var(--panel-raised);
  border-radius: 50%;
  transition: transform 130ms;
}
.switch.enabled {
  background: var(--accent);
}
.switch.enabled > span {
  transform: translateX(14px);
}
.mock-explainer {
  padding: 20px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--panel);
}
.mock-explainer h3 {
  font-size: 12px;
  font-weight: 600;
}
.mock-explainer p {
  color: var(--text-muted);
  line-height: 1.9;
  margin-top: 10px;
  font-size: 11px;
  max-width: 560px;
}
.mock-explainer .btn {
  margin-top: 18px;
  font-size: 10px;
}
.about-section {
  padding: 20px 10px;
}
.about-section h2 {
  margin-top: 22px;
  font-family: var(--font-display);
  font-size: 35px;
  letter-spacing: -1px;
}
.about-section .about-tagline {
  font-size: 16px;
  color: var(--text-secondary);
}
.about-section .badge {
  margin-top: 17px;
}
.about-copy {
  max-width: 420px;
  margin-top: 26px;
  font-size: 12px;
  color: var(--text-muted);
  line-height: 1.9;
}
.about-copy p + p {
  margin-top: 16px;
}
.about-tech {
  display: flex;
  flex-wrap: wrap;
  gap: 9px;
  margin-top: 28px;
}
.about-tech span {
  padding: 5px 9px;
  background: var(--input);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: 10px;
}
@media (max-width: 1280px) {
  .settings-nav {
    width: 155px;
    padding: 27px 10px;
  }
  .settings-content {
    padding: 28px 23px;
  }
}
@media (max-width: 950px) {
  .settings-nav {
    width: 55px;
    padding: 24px 7px;
  }
  .settings-nav > .eyebrow,
  .settings-nav > button > span,
  .settings-nav > button > svg:last-child:not(:first-child),
  .settings-version {
    display: none;
  }
  .settings-nav > button {
    padding: 10px;
    justify-content: center;
  }
  .settings-content {
    padding: 24px 20px;
  }
  .preference-row {
    align-items: flex-start;
    flex-wrap: wrap;
  }
}
@media (max-width: 600px) {
  .settings-view {
    flex-direction: column;
  }
  .settings-nav {
    width: 100%;
    padding: 8px 12px;
    border-right: 0;
    border-bottom: 1px solid var(--border);
    flex-direction: row;
  }
  .settings-nav > button {
    flex: 1;
  }
  .settings-content {
    padding: 22px 17px;
  }
  .theme-preview {
    height: 240px;
  }
  .mini-parameters {
    width: 58px;
    padding: 20px 8px;
  }
  .mini-sidebar {
    width: 46px;
  }
}
</style>

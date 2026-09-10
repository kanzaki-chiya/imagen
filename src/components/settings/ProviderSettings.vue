<script setup lang="ts">
import { computed, shallowRef } from "vue";
import { Sparkles, Link2, Server, ChevronRight, Plus } from "lucide-vue-next";
import type { Provider } from "../../types";
import { useStudio } from "../../composables/useStudio";
import ProviderEditor from "./ProviderEditor.vue";
import BaseDialog from "../ui/BaseDialog.vue";
import { useI18n } from "../../i18n";
const { t, tp } = useI18n();
const props = defineProps<{ providers: Provider[] }>();
const emit = defineEmits<{ save: [provider: Provider] }>();
const studio = useStudio();
const selectedId = shallowRef("openai");
const hasUnsavedChanges = shallowRef(false);
const pendingProvider = shallowRef("");
const removingProvider = shallowRef<Provider | null>(null);
function addProvider() {
  const provider = studio.addProvider();
  selectedId.value = provider.id;
  hasUnsavedChanges.value = true;
}
function requestRemove(provider: Provider) {
  removingProvider.value = provider;
}
function confirmRemove() {
  const provider = removingProvider.value;
  if (!provider) return;
  studio.removeProvider(provider.id);
  selectedId.value = props.providers[0]?.id ?? "";
  hasUnsavedChanges.value = false;
  removingProvider.value = null;
}
function selectProvider(id: string) {
  if (id === selectedId.value) return;
  if (hasUnsavedChanges.value) pendingProvider.value = id;
  else selectedId.value = id;
}
function discardAndSwitch() {
  hasUnsavedChanges.value = false;
  selectedId.value = pendingProvider.value;
  pendingProvider.value = "";
}
const selected = computed(
  () =>
    props.providers.find((item) => item.id === selectedId.value) ??
    props.providers[0],
);
</script>
<template>
  <section class="provider-settings">
    <div class="section-heading">
      <div>
        <h2>{{ t("settings.providers.title") }}</h2>
        <p>{{ t("settings.providers.sub") }}</p>
      </div>
      <span class="badge">{{
        tp("settings.providers.count", providers.length)
      }}</span>
    </div>
    <div class="provider-layout">
      <nav class="provider-list" :aria-label="t('settings.providers.title')">
        <button
          v-for="provider in providers"
          :key="provider.id"
          :class="{ selected: selectedId === provider.id }"
          :aria-label="t('settings.providers.configure', { name: provider.name })"
          :aria-current="selectedId === provider.id ? 'true' : undefined"
          @click="selectProvider(provider.id)"
        >
          <span class="provider-icon" :class="provider.kind"
            ><Link2 v-if="provider.kind === 'compatible'" :size="18" /><Server
              v-else-if="provider.kind === 'custom'"
              :size="18" /><Sparkles v-else :size="19" /></span
          ><span class="provider-list-text"
            ><strong>{{ provider.name }}</strong
            ><span
              ><span class="status-dot" :class="provider.status" />{{
                provider.status === "connected"
                  ? t("settings.providers.connected")
                  : provider.status === "error"
                    ? t("settings.providers.failed")
                    : t("settings.providers.untested")
              }}</span
            ></span
          ><ChevronRight :size="13" />
        </button>
        <button class="provider-add" @click="addProvider">
          <Plus :size="14" /><span>{{ t("settings.providers.add") }}</span>
        </button>
        <p class="provider-list-note">{{ t("settings.providers.listNote") }}</p>
      </nav>
      <ProviderEditor
        :key="selected.id"
        :provider="selected"
        @save="emit('save', $event)"
        @dirty="hasUnsavedChanges = $event"
        @remove="requestRemove(selected)"
        @forget-key="studio.clearProviderKey(selected.id)"
      />
    </div>
    <BaseDialog
      v-if="pendingProvider"
      :title="t('provider.switch.title')"
      :description="t('provider.switch.desc')"
      @close="pendingProvider = ''"
    >
      <p class="secondary">{{ t("provider.switch.body") }}</p>
      <template #footer
        ><button class="btn" @click="pendingProvider = ''">{{
          t("provider.switch.keep")
        }}</button
        ><button class="btn btn-danger" @click="discardAndSwitch">
          {{ t("provider.switch.discard") }}
        </button></template
      >
    </BaseDialog>
    <BaseDialog
      v-if="removingProvider"
      :title="t('provider.delete.title', { name: removingProvider.name })"
      :description="t('provider.delete.desc')"
      @close="removingProvider = null"
    >
      <p class="secondary">{{ t("provider.delete.body") }}</p>
      <template #footer
        ><button class="btn" @click="removingProvider = null">{{
          t("provider.delete.keep")
        }}</button
        ><button class="btn btn-danger" @click="confirmRemove">
          {{ t("provider.delete.confirm") }}
        </button></template
      >
    </BaseDialog>
  </section>
</template>
<style scoped>
.section-heading {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 25px;
}
.section-heading h2 {
  font-size: 19px;
  font-weight: 600;
  letter-spacing: -0.3px;
}
.section-heading p {
  color: var(--text-muted);
  font-size: 11px;
  margin-top: 5px;
}
.section-heading .badge {
  margin-top: 4px;
}
.provider-layout {
  display: grid;
  grid-template-columns: 214px minmax(0, 1fr);
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
  background: var(--panel);
}
.provider-list {
  border-right: 1px solid var(--border);
  padding: 14px 10px;
  background: var(--app);
}
.provider-list > button {
  display: flex;
  align-items: center;
  gap: 9px;
  width: 100%;
  padding: 12px 9px;
  text-align: left;
  border-radius: 5px;
  margin-bottom: 6px;
  border: 1px solid transparent;
}
.provider-list > button:hover {
  background: var(--hover);
}
.provider-list > button.selected {
  background: var(--panel-raised);
  border-color: var(--border);
}
.provider-icon {
  display: grid;
  place-items: center;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  background: var(--input);
  border: 1px solid var(--border);
  color: var(--text-secondary);
}
.provider-icon.gemini {
  color: var(--accent);
}
.provider-list-text {
  flex: 1;
  min-width: 0;
}
.provider-list-text strong {
  display: block;
  font-size: 10px;
  font-weight: 600;
}
.provider-list-text > span {
  display: flex;
  gap: 5px;
  align-items: center;
  font-size: 9px;
  color: var(--text-muted);
  margin-top: 4px;
}
.provider-list-text .status-dot {
  width: 4px;
  height: 4px;
}
.provider-list > button > svg {
  color: var(--text-muted);
  width: 11px;
}
.provider-add {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 7px;
  width: 100%;
  padding: 9px;
  margin-top: 2px;
  border: 1px dashed var(--border-strong);
  border-radius: 5px;
  color: var(--text-secondary);
  font-size: 10px;
}
.provider-add:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-soft);
}
.provider-list-note {
  border-top: 1px solid var(--border);
  margin: 16px 9px;
  padding-top: 15px;
  color: var(--text-muted);
  font-size: 10px;
  line-height: 1.8;
}
@media (max-width: 1200px) {
  .provider-layout {
    grid-template-columns: 185px minmax(0, 1fr);
  }
  .provider-list {
    padding: 10px 5px;
  }
  .provider-list > button {
    gap: 6px;
    padding: 10px 6px;
  }
  .provider-icon {
    width: 27px;
    height: 29px;
  }
  .provider-list-text strong {
    font-size: 9px;
  }
}
@media (max-width: 750px) {
  .provider-layout {
    grid-template-columns: 1fr;
  }
  .provider-list {
    display: grid;
    grid-template-columns: 1fr 1fr;
    border-right: 0;
    border-bottom: 1px solid var(--border);
    padding: 9px;
    gap: 5px;
  }
  .provider-list > button {
    margin: 0;
  }
  .provider-list-note {
    display: none;
  }
  .provider-list-text strong {
    font-size: 10px;
  }
}
</style>

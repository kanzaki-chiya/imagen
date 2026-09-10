<script setup lang="ts">
import { computed, shallowRef } from "vue";
import { useStudio } from "../composables/useStudio";
import { exportImage } from "../services/exportImage";
import { usingDesktopBackend } from "../services/backend";
import { useI18n } from "../i18n";
import type { ImageResult } from "../types";
import ImageCanvas from "../components/generate/ImageCanvas.vue";
import PromptComposer from "../components/generate/PromptComposer.vue";
import ParameterPanel from "../components/generate/ParameterPanel.vue";
import PresetDialog from "../components/presets/PresetDialog.vue";
defineProps<{ drawerOpen: boolean }>();
const emit = defineEmits<{ closeDrawer: [] }>();
const studio = useStudio();
const { state } = studio;
const { t } = useI18n();
const live = computed(() => usingDesktopBackend(state.preferMock));
const savingPreset = shallowRef(false);
async function download(image: ImageResult) {
  try {
    await exportImage(image);
    studio.notify(t("toast.downloadRequested"));
  } catch {
    studio.notify(t("toast.downloadFail"), "error");
  }
}
function enhance() {
  const addition =
    "Thoughtful framing, soft directional light, balanced tonal depth, fine photographic texture.";
  if (state.prompt.includes(addition)) {
    studio.notify(t("toast.alreadyEnhanced"), "info");
    return;
  }
  studio.setPrompt(
    `${state.prompt.trim().replace(/[,.]$/, "")}. ${addition}`.slice(0, 4000),
  );
  studio.notify(t("toast.enhanced"));
}
function savePreset(name: string, description: string, prompt: string) {
  studio.savePreset(name, description, prompt);
  savingPreset.value = false;
}
</script>
<template>
  <div class="generate-view">
    <div class="creation-column">
      <ImageCanvas
        :results="state.results"
        :selected-id="state.selectedId"
        :generating="state.generating"
        :progress="state.progress"
        :count="state.params.count"
        :live="live"
        @select="studio.selectImage"
        @favorite="studio.toggleFavorite"
        @download="download"
        @reuse="studio.reuseImage"
        @cancel="studio.cancelGeneration"
      /><PromptComposer
        :prompt="state.prompt"
        :negative-prompt="state.params.negativePrompt"
        :count="state.params.count"
        :generating="state.generating"
        :progress="state.progress"
        :error="state.taskError"
        :ready="state.ready"
        :live="live"
        @prompt="studio.setPrompt"
        @negative="studio.updateParams({ negativePrompt: $event })"
        @generate="studio.generate()"
        @cancel="studio.cancelGeneration"
        @retry="studio.generate(true)"
        @preset="savingPreset = true"
        @enhance="enhance"
      />
    </div>
    <ParameterPanel
      :params="state.params"
      :providers="state.providers"
      :references="state.references"
      :drawer-open="drawerOpen"
      :live="live"
      @update="studio.updateParams"
      @provider="studio.selectProvider"
      @reset="studio.resetParams"
      @add-reference="studio.addReferences"
      @remove-reference="studio.removeReference"
      @reference-strength="studio.setReferenceStrength"
      @close="emit('closeDrawer')"
    /><PresetDialog
      v-if="savingPreset"
      :prompt="state.prompt"
      @close="savingPreset = false"
      @save="savePreset"
    />
  </div>
</template>
<style scoped>
.generate-view {
  display: flex;
  flex: 1;
  min-height: 0;
  min-width: 0;
}
.creation-column {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
}
</style>

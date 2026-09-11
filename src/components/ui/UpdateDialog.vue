<script setup lang="ts">
import { computed } from "vue";
import { Download, RotateCw } from "lucide-vue-next";
import BaseDialog from "./BaseDialog.vue";
import { useUpdater } from "../../composables/useUpdater";
import { useStudio } from "../../composables/useStudio";
import { useI18n } from "../../i18n";
const { t } = useI18n();
const updater = useUpdater();
const studio = useStudio();
const show = computed(
  () => !!updater.available.value && !updater.dismissed.value,
);
// `show` already guarantees a pending update exists.
const info = computed(() => updater.available.value!);
const downloading = computed(() => updater.downloading.value);
const installed = computed(() => updater.readyToRestart.value);
const progress = computed(() => updater.progress.value);
async function install() {
  try {
    await updater.install();
  } catch {
    studio.notify(t("update.failed"), "error");
  }
}
function close() {
  if (!downloading.value) updater.dismiss();
}
</script>
<template>
  <BaseDialog
    v-if="show"
    :title="t('update.title')"
    :description="t('update.version', { version: info.version })"
    @close="close"
  >
    <p v-if="info.notes" class="update-notes">{{ info.notes }}</p>
    <p v-else class="update-notes muted">{{ t("update.noNotes") }}</p>
    <div
      v-if="downloading"
      class="update-progress"
      role="progressbar"
      :aria-valuenow="progress"
      aria-valuemin="0"
      aria-valuemax="100"
    >
      <div :style="{ width: `${progress}%` }" />
    </div>
    <p v-if="downloading" class="update-status">
      {{ t("update.downloading", { progress }) }}
    </p>
    <p v-else-if="installed" class="update-status">
      {{ t("update.ready") }}
    </p>
    <template #footer>
      <template v-if="!installed">
        <button class="btn" :disabled="downloading" @click="close">
          {{ t("update.later") }}</button
        ><button
          class="btn btn-primary"
          :disabled="downloading"
          @click="install"
        >
          <Download :size="15" />{{ t("update.install") }}
        </button>
      </template>
      <template v-else>
        <button class="btn" @click="close">{{ t("update.restartLater") }}</button
        ><button class="btn btn-primary" @click="updater.restartApp">
          <RotateCw :size="15" />{{ t("update.restart") }}
        </button>
      </template>
    </template>
  </BaseDialog>
</template>
<style scoped>
.update-notes {
  font-size: 12px;
  line-height: 1.8;
  color: var(--text-secondary);
  white-space: pre-wrap;
  max-height: 220px;
  overflow-y: auto;
}
.update-progress {
  height: 3px;
  border-radius: 3px;
  background: var(--border);
  overflow: hidden;
  margin-top: 18px;
}
.update-progress > div {
  height: 100%;
  background: var(--accent);
  transition: width 180ms linear;
}
.update-status {
  font-size: 10px;
  color: var(--text-muted);
  margin-top: 10px;
}
</style>

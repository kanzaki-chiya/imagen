<script setup lang="ts">
import { shallowRef } from "vue";
import { Bookmark } from "lucide-vue-next";
import type { Preset } from "../../types";
import BaseDialog from "../ui/BaseDialog.vue";
import { useI18n } from "../../i18n";
const { t } = useI18n();
const props = defineProps<{ prompt: string; preset?: Preset }>();
const emit = defineEmits<{
  close: [];
  save: [name: string, description: string, prompt: string, id?: string];
}>();
const name = shallowRef(props.preset?.name ?? "");
const description = shallowRef(props.preset?.description ?? "");
const text = shallowRef(props.preset?.prompt ?? props.prompt);
function save() {
  if (name.value.trim() && text.value.trim())
    emit("save", name.value, description.value, text.value, props.preset?.id);
}
</script>
<template>
  <BaseDialog
    :title="preset ? t('presets.dialog.editTitle') : t('presets.dialog.newTitle')"
    :description="t('presets.dialog.desc')"
    @close="emit('close')"
  >
    <form id="preset-form" class="preset-form" @submit.prevent="save">
      <label class="field"
        ><span class="field-label">{{ t("presets.dialog.name") }}</span
        ><input
          v-model="name"
          autofocus
          required
          maxlength="60"
          :placeholder="t('presets.dialog.namePlaceholder')"
      /></label>
      <label class="field"
        ><span class="field-label"
          >{{ t("presets.dialog.description") }}
          <span class="muted">{{ t("presets.dialog.optional") }}</span></span
        ><input
          v-model="description"
          maxlength="140"
          :placeholder="t('presets.dialog.descPlaceholder')"
      /></label>
      <label class="field"
        ><span class="field-label">{{ t("presets.dialog.prompt") }}</span
        ><textarea
          v-model="text"
          required
          maxlength="4000"
          rows="5"
          :placeholder="t('presets.dialog.promptPlaceholder')"
        />
      </label>
      <p class="field-hint">
        {{
          preset
            ? t("presets.dialog.hintEdit")
            : t("presets.dialog.hintNew")
        }}
      </p>
    </form>
    <template #footer
      ><button class="btn" @click="emit('close')">{{
        t("presets.dialog.cancel")
      }}</button
      ><button
        class="btn btn-primary"
        type="submit"
        form="preset-form"
        :disabled="!name.trim() || !text.trim()"
      >
        <Bookmark :size="14" />{{
          preset ? t("presets.dialog.saveChanges") : t("presets.dialog.save")
        }}
      </button></template
    >
  </BaseDialog>
</template>
<style scoped>
.preset-form {
  display: flex;
  flex-direction: column;
  gap: 18px;
}
</style>

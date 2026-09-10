<script setup lang="ts">
import { shallowRef, useTemplateRef } from "vue";
import { ImagePlus, Plus, X } from "lucide-vue-next";
import type { ReferenceImage } from "../../types";
import { useI18n } from "../../i18n";
defineProps<{ images: ReferenceImage[]; unsupported?: boolean }>();
const { t } = useI18n();
const emit = defineEmits<{
  add: [files: File[]];
  remove: [id: string];
  strength: [id: string, strength: number];
}>();
const fileInput = useTemplateRef<HTMLInputElement>("fileInput");
const dragging = shallowRef(false);
function change(event: Event) {
  const input = event.target as HTMLInputElement;
  emit("add", Array.from(input.files ?? []));
  input.value = "";
}
function drop(event: DragEvent) {
  dragging.value = false;
  emit("add", Array.from(event.dataTransfer?.files ?? []));
}
</script>
<template>
  <section class="references">
    <div class="reference-heading">
      <span class="field-label"
        >{{ t("refs.title") }}
        <span class="optional">{{ t("refs.optional") }}</span></span
      ><span v-if="images.length" class="mono muted"
        >{{ images.length }}/3</span
      >
    </div>
    <input
      ref="fileInput"
      class="sr-only"
      type="file"
      multiple
      accept="image/png,image/jpeg,image/webp"
      :aria-label="t('refs.uploadAria')"
      @change="change"
    />
    <p v-if="unsupported" class="field-hint">{{ t("refs.unsupported") }}</p>
    <button
      v-else-if="!images.length"
      class="drop-zone"
      :class="{ dragging }"
      @click="fileInput?.click()"
      @dragover.prevent="dragging = true"
      @dragleave.prevent="dragging = false"
      @drop.prevent="drop"
    >
      <span class="upload-icon"><ImagePlus :size="20" /></span
      ><span
        >{{ t("refs.drop") }} <strong>{{ t("refs.browse") }}</strong></span
      ><small>{{ t("refs.formats") }}</small>
    </button>
    <template v-if="images.length">
      <div class="reference-grid" @dragover.prevent @drop.prevent="drop">
        <div
          v-for="reference in images"
          :key="reference.id"
          class="reference-thumb"
        >
          <img :src="reference.src" :alt="reference.name" /><button
            :aria-label="t('refs.remove', { name: reference.name })"
            class="remove-reference"
            @click="emit('remove', reference.id)"
          >
            <X :size="12" />
          </button>
        </div>
        <button
          v-if="images.length < 3 && !unsupported"
          class="add-reference"
          :aria-label="t('refs.addAnother')"
          @click="fileInput?.click()"
        >
          <Plus :size="18" />
        </button>
      </div>
      <label
        v-for="reference in images"
        :key="`strength-${reference.id}`"
        class="strength"
        ><span :title="reference.name">{{ reference.name }}</span
        ><input
          type="range"
          min="0"
          max="100"
          :value="reference.strength"
          :disabled="unsupported"
          :aria-label="t('refs.strength', { name: reference.name })"
          @input="
            emit(
              'strength',
              reference.id,
              Number(($event.target as HTMLInputElement).value),
            )
          "
        /><small>{{ reference.strength }}%</small></label
      >
      <p class="field-hint">{{ t("refs.hint") }}</p>
    </template>
  </section>
</template>
<style scoped>
.reference-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}
.reference-heading .field-label {
  gap: 7px;
}
.optional {
  font-size: 10px;
  color: var(--text-muted);
  font-weight: 400;
}
.drop-zone {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 7px;
  width: 100%;
  height: 123px;
  border: 1px dashed var(--border-strong);
  border-radius: 6px;
  background: var(--input);
  font-size: 11px;
  color: var(--text-secondary);
}
.drop-zone:hover,
.drop-zone.dragging {
  border-color: var(--accent);
  background: var(--accent-soft);
}
.upload-icon {
  margin-bottom: 2px;
  color: var(--text-muted);
}
.drop-zone strong {
  color: var(--text);
  font-weight: 500;
}
.drop-zone small {
  font-size: 9px;
  color: var(--text-muted);
}
.reference-grid {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}
.reference-thumb,
.add-reference {
  width: calc((100% - 16px) / 3);
  aspect-ratio: 1;
  border: 1px solid var(--border);
  border-radius: 5px;
  position: relative;
}
.reference-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 4px;
}
.remove-reference {
  position: absolute;
  top: 3px;
  right: 3px;
  width: 18px;
  height: 18px;
  background: var(--panel-raised);
  border-radius: 3px;
  display: grid;
  place-items: center;
}
.add-reference {
  border-style: dashed;
  display: grid;
  place-items: center;
  color: var(--text-muted);
}
.add-reference:hover {
  border-color: var(--accent);
}
.strength {
  display: flex;
  align-items: center;
  gap: 7px;
  margin: 8px 0;
  font-size: 10px;
}
.strength > span {
  width: 65px;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}
.strength input {
  flex: 1;
  width: 50px;
}
.strength small {
  min-width: 24px;
  color: var(--text-muted);
}
</style>

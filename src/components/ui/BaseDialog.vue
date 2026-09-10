<script setup lang="ts">
import { onMounted, useTemplateRef } from "vue";
import { X } from "lucide-vue-next";
import { useI18n } from "../../i18n";
const { t } = useI18n();
withDefaults(
  defineProps<{ title: string; description?: string; wide?: boolean }>(),
  { description: "", wide: false },
);
const emit = defineEmits<{ close: [] }>();
const dialog = useTemplateRef<HTMLDialogElement>("dialog");
onMounted(() => dialog.value?.showModal());
function closeOutside(event: MouseEvent) {
  if (event.target === dialog.value) emit("close");
}
</script>
<template>
  <dialog
    ref="dialog"
    class="dialog"
    :class="{ wide }"
    aria-labelledby="dialog-title"
    @cancel.prevent="emit('close')"
    @click="closeOutside"
  >
    <div class="dialog-surface">
      <header class="dialog-header">
        <div>
          <h2 id="dialog-title">{{ title }}</h2>
          <p v-if="description" class="dialog-description">{{ description }}</p>
        </div>
        <button
          class="icon-btn"
          :aria-label="t('ui.closeDialog')"
          @click="emit('close')"
        >
          <X :size="18" />
        </button>
      </header>
      <div class="dialog-body"><slot /></div>
      <footer v-if="$slots.footer" class="dialog-footer">
        <slot name="footer" />
      </footer>
    </div>
  </dialog>
</template>
<style scoped>
.dialog {
  padding: 0;
  width: min(460px, calc(100vw - 32px));
  max-height: calc(100dvh - 48px);
  color: var(--text);
  background: var(--panel-raised);
  border: 1px solid var(--border-strong);
  border-radius: 10px;
  box-shadow: var(--modal-shadow);
  overflow: auto;
}
.dialog.wide {
  width: min(1050px, calc(100vw - 48px));
}
.dialog::backdrop {
  background: var(--overlay);
  backdrop-filter: blur(4px);
}
.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 14px;
  padding: 22px 22px 18px;
  border-bottom: 1px solid var(--border);
}
.dialog-header h2 {
  font-size: 17px;
  font-weight: 600;
}
.dialog-description {
  color: var(--text-muted);
  margin-top: 5px;
  font-size: 12px;
}
.dialog-body {
  padding: 22px;
}
.dialog-footer {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding: 15px 22px;
  border-top: 1px solid var(--border);
  background: var(--panel);
}
</style>

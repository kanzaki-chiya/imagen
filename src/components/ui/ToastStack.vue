<script setup lang="ts">
import { Check, AlertCircle, Info, X } from "lucide-vue-next";
import type { Toast } from "../../types";
import { useI18n } from "../../i18n";
const { t } = useI18n();
defineProps<{ toasts: Toast[] }>();
const emit = defineEmits<{ dismiss: [id: string] }>();
</script>
<template>
  <div class="toast-stack" aria-live="polite" aria-atomic="false">
    <div
      v-for="toast in toasts"
      :key="toast.id"
      class="toast"
      :class="toast.kind"
      :role="toast.kind === 'error' ? 'alert' : 'status'"
    >
      <component
        :is="
          toast.kind === 'success'
            ? Check
            : toast.kind === 'error'
              ? AlertCircle
              : Info
        "
        :size="17"
        class="toast-icon"
      />
      <span>{{ toast.message }}</span>
      <button
        class="icon-btn"
        :aria-label="t('ui.dismissToast')"
        @click="emit('dismiss', toast.id)"
      >
        <X :size="14" />
      </button>
    </div>
  </div>
</template>
<style scoped>
.toast-stack {
  position: fixed;
  bottom: 44px;
  left: 50%;
  transform: translateX(-50%);
  width: max-content;
  max-width: calc(100vw - 32px);
  z-index: 100;
  display: flex;
  flex-direction: column;
  gap: 7px;
  pointer-events: none;
}
.toast {
  display: flex;
  align-items: center;
  gap: 10px;
  min-height: 45px;
  max-width: 560px;
  padding: 7px 10px 7px 14px;
  background: var(--panel-raised);
  border: 1px solid var(--border-strong);
  border-radius: 8px;
  box-shadow: var(--popover-shadow);
  font-size: 12px;
  pointer-events: auto;
}
.toast-icon {
  color: var(--success);
}
.error .toast-icon {
  color: var(--danger);
}
.info .toast-icon {
  color: var(--text-secondary);
}
.toast span {
  flex: 1;
}
</style>

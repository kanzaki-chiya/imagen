<script setup lang="ts">
import { shallowRef, watch } from "vue";
import { ImageOff } from "lucide-vue-next";
import { useI18n } from "../../i18n";
const { t } = useI18n();
const props = withDefaults(
  defineProps<{
    src: string;
    alt: string;
    filter?: string;
    position?: string;
    lazy?: boolean;
  }>(),
  { filter: "none", position: "50% 50%", lazy: false },
);
const loaded = shallowRef(false);
const failed = shallowRef(false);
watch(
  () => props.src,
  () => {
    loaded.value = false;
    failed.value = false;
  },
);
</script>
<template>
  <div class="result-image" :class="{ skeleton: !loaded && !failed }">
    <img
      v-if="!failed"
      :src="src"
      :alt="alt"
      :style="{ filter, objectPosition: position, opacity: loaded ? 1 : 0 }"
      :loading="lazy ? 'lazy' : 'eager'"
      draggable="false"
      @load="loaded = true"
      @error="failed = true"
    />
    <div v-else class="image-error">
      <ImageOff :size="22" /><span>{{ t("ui.previewUnavailable") }}</span>
    </div>
  </div>
</template>
<style scoped>
.result-image {
  width: 100%;
  height: 100%;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
}
.result-image img {
  display: block;
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: opacity 140ms;
}
.image-error {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 8px;
  justify-content: center;
  align-items: center;
  background: var(--input);
  color: var(--text-muted);
  font-size: 11px;
}
</style>

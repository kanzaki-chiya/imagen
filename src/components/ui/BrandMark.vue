<script setup lang="ts">
withDefaults(defineProps<{ size?: number }>(), { size: 30 });
</script>
<template>
  <svg
    :width="size"
    :height="size"
    viewBox="0 0 36 36"
    fill="none"
    aria-hidden="true"
  >
    <rect width="36" height="36" rx="9" fill="var(--accent)" />
    <g
      stroke="var(--accent-foreground)"
      stroke-width="1.7"
      stroke-linejoin="round"
    >
      <path d="m18 6 10.5 6v12L18 30 7.5 24V12z" />
      <path d="M18 6v12l10.5 6M7.5 12 18 18v12m10.5-18L18 18 7.5 24" />
    </g>
  </svg>
</template>

import { computed, onUnmounted, shallowRef, watch } from "vue";
import type { Theme } from "../types";

export function useTheme() {
  const preference = shallowRef<Theme>("system");
  try {
    const saved = localStorage.getItem("imagen-theme");
    if (saved === "light" || saved === "dark" || saved === "system")
      preference.value = saved;
  } catch {
    /* System theme remains available when browser storage is restricted. */
  }
  const media = window.matchMedia("(prefers-color-scheme: dark)");
  const systemDark = shallowRef(media.matches);
  const resolved = computed(() =>
    preference.value === "system"
      ? systemDark.value
        ? "dark"
        : "light"
      : preference.value,
  );
  const onChange = (event: MediaQueryListEvent) => {
    systemDark.value = event.matches;
  };
  media.addEventListener("change", onChange);
  watch(
    [preference, resolved],
    () => {
      document.documentElement.dataset.theme = resolved.value;
      document.documentElement.style.colorScheme = resolved.value;
      try {
        localStorage.setItem("imagen-theme", preference.value);
      } catch {
        /* Non-essential preference. */
      }
    },
    { immediate: true },
  );
  onUnmounted(() => media.removeEventListener("change", onChange));
  return {
    preference,
    resolved,
    setTheme: (value: Theme) => {
      preference.value = value;
    },
  };
}

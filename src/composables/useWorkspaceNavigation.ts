import { onMounted, onUnmounted, watch } from "vue";
import type { Workspace } from "../types";
import { locale, t } from "../i18n";

const pages: Workspace[] = ["generate", "history", "presets", "settings"];
export function useWorkspaceNavigation(options: {
  current: () => Workspace;
  navigate: (page: Workspace) => void;
  generate: () => void;
  shortcuts: () => void;
}) {
  function fromHash() {
    const page = location.hash.replace("#", "") as Workspace;
    if (pages.includes(page)) options.navigate(page);
  }
  function keydown(event: KeyboardEvent) {
    if (document.querySelector("dialog[open]")) return;
    const target = event.target as HTMLElement;
    const editing =
      ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName) ||
      target.isContentEditable;
    if (event.ctrlKey || event.metaKey) {
      if (
        event.key === "Enter" &&
        !editing &&
        options.current() === "generate"
      ) {
        event.preventDefault();
        options.generate();
      }
      if (event.altKey && ["1", "2", "3", "4"].includes(event.key)) {
        event.preventDefault();
        options.navigate(pages[Number(event.key) - 1]);
      }
      return;
    }
    if (event.key === "?" && !editing) {
      event.preventDefault();
      options.shortcuts();
    }
  }
  watch([options.current, locale], () => {
    const page = options.current();
    if (location.hash !== `#${page}`) location.hash = page;
    document.title = `${t(`nav.${page}`)} — Imagen`;
  });
  onMounted(() => {
    fromHash();
    document.title = `${t(`nav.${options.current()}`)} — Imagen`;
    window.addEventListener("hashchange", fromHash);
    window.addEventListener("keydown", keydown);
  });
  onUnmounted(() => {
    window.removeEventListener("hashchange", fromHash);
    window.removeEventListener("keydown", keydown);
  });
}

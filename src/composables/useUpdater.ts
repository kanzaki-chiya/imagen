import { shallowRef } from "vue";
import {
  checkForUpdate,
  installUpdate,
  restartApp,
  type PendingUpdate,
} from "../services/updater";

export type UpdateCheckResult = "update" | "latest" | "failed" | "unavailable";

// Module-level state: the update dialog is shared between the automatic
// startup check and the manual check in Settings.
const available = shallowRef<PendingUpdate | null>(null);
const dismissed = shallowRef(false);
const checking = shallowRef(false);
const downloading = shallowRef(false);
const readyToRestart = shallowRef(false);
const progress = shallowRef(0);

export function useUpdater() {
  async function checkForUpdates(): Promise<UpdateCheckResult> {
    if (checking.value || downloading.value || readyToRestart.value)
      return "unavailable";
    checking.value = true;
    try {
      const found = await checkForUpdate();
      if (found) {
        available.value = found;
        dismissed.value = false;
        return "update";
      }
      return "latest";
    } catch {
      return "failed";
    } finally {
      checking.value = false;
    }
  }

  async function install() {
    if (!available.value || downloading.value) return;
    downloading.value = true;
    progress.value = 0;
    try {
      await installUpdate(({ downloaded, total }) => {
        progress.value = total
          ? Math.min(99, Math.round((downloaded / total) * 100))
          : 0;
      });
      progress.value = 100;
      readyToRestart.value = true;
    } catch {
      downloading.value = false;
      throw new Error("install failed");
    }
  }

  function dismiss() {
    dismissed.value = true;
  }

  return {
    available,
    dismissed,
    checking,
    downloading,
    readyToRestart,
    progress,
    checkForUpdates,
    install,
    dismiss,
    restartApp,
  };
}

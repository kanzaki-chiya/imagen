import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { backendAvailable } from "./backend";

export interface PendingUpdate {
  version: string;
  notes: string;
}

export interface DownloadProgress {
  downloaded: number;
  total: number | null;
}

let pending: Update | null = null;

/** Returns update metadata when a newer release is available. */
export async function checkForUpdate(): Promise<PendingUpdate | null> {
  if (!backendAvailable()) return null;
  const update = await check();
  if (!update) return null;
  pending = update;
  return { version: update.version, notes: update.body ?? "" };
}

/** Downloads and installs the pending update, reporting byte progress. */
export async function installUpdate(
  onProgress?: (progress: DownloadProgress) => void,
): Promise<void> {
  if (!pending) throw new Error("No pending update");
  let downloaded = 0;
  let total: number | null = null;
  await pending.downloadAndInstall((event) => {
    if (event.event === "Started") {
      total = event.data.contentLength ?? null;
    } else if (event.event === "Progress") {
      downloaded += event.data.chunkLength;
    }
    onProgress?.({ downloaded, total });
  });
}

/** Restarts into the newly installed version. */
export async function restartApp(): Promise<void> {
  await relaunch();
}

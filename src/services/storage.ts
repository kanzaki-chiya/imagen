import { invoke } from "@tauri-apps/api/core";
import { backendAvailable } from "./backend";
import type { GenerationTask, ImageResult, StoredWorkspace } from "../types";

const databaseName = "imagen-workspace";
const MIGRATION_KEY = "imagen:migrated-idb";

const desktop = backendAvailable();

function openDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(databaseName, 1);
    request.onupgradeneeded = () =>
      request.result.createObjectStore("workspace");
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

async function loadLegacy(): Promise<StoredWorkspace | undefined> {
  const db = await openDatabase();
  return new Promise((resolve, reject) => {
    const transaction = db.transaction("workspace", "readonly");
    const request = transaction.objectStore("workspace").get("current");
    request.onsuccess = () =>
      resolve(request.result?.version === 1 ? request.result : undefined);
    request.onerror = () => reject(request.error);
    transaction.oncomplete = () => db.close();
  });
}

async function saveLegacy(value: StoredWorkspace): Promise<void> {
  const db = await openDatabase();
  return new Promise((resolve, reject) => {
    const transaction = db.transaction("workspace", "readwrite");
    transaction
      .objectStore("workspace")
      .put(JSON.parse(JSON.stringify(value)), "current");
    transaction.oncomplete = () => {
      db.close();
      resolve();
    };
    transaction.onerror = () => {
      db.close();
      reject(transaction.error);
    };
  });
}

export async function loadWorkspace(): Promise<StoredWorkspace | undefined> {
  if (!desktop) return loadLegacy();
  let saved = await invoke<StoredWorkspace | undefined>("workspace_load");
  if (!saved && !localStorage.getItem(MIGRATION_KEY)) {
    localStorage.setItem(MIGRATION_KEY, "1");
    try {
      const legacy = await loadLegacy();
      if (legacy) {
        await invoke("workspace_import", { workspace: legacy });
        saved = await invoke<StoredWorkspace | undefined>("workspace_load");
      }
    } catch {
      // Migration is best-effort; a fresh workspace is still usable.
    }
  }
  return saved;
}

export async function saveWorkspace(value: StoredWorkspace): Promise<void> {
  if (!desktop) return saveLegacy(value);
  await invoke("workspace_save_state", { state: value });
  if (value.providers?.length)
    await invoke("providers_upsert", { providers: value.providers });
  if (value.presets?.length)
    await invoke("presets_upsert", { presets: value.presets });
}

export async function addHistory(images: ImageResult[]): Promise<void> {
  if (!desktop || !images.length) return;
  await invoke("history_add", { images });
}

export async function setHistoryFavorite(
  id: string,
  favorite: boolean,
): Promise<void> {
  if (!desktop) return;
  await invoke("history_set_favorite", { id, favorite });
}

export async function trashStoredImage(id: string): Promise<void> {
  if (!desktop) return;
  await invoke("history_remove", { id });
}

export async function restoreStoredImage(id: string): Promise<void> {
  if (!desktop) return;
  await invoke("history_restore", { id });
}

/** Permanently deletes the image file, thumbnail and history row. */
export async function purgeStoredImage(id: string): Promise<void> {
  if (!desktop) return;
  await invoke("history_purge", { id });
}

/** Permanently deletes everything in the trash; returns the count removed. */
export async function emptyStoredTrash(): Promise<number> {
  if (!desktop) return 0;
  return invoke<number>("history_empty_trash");
}

export async function listDeletedImages(): Promise<ImageResult[]> {
  if (!desktop) return [];
  return invoke<ImageResult[]>("history_deleted");
}

export async function removeStoredProvider(id: string): Promise<void> {
  if (!desktop) return;
  await invoke("providers_remove", { id });
}

export async function storeApiKey(
  providerId: string,
  key: string,
): Promise<void> {
  if (!desktop) return;
  await invoke("store_api_key", { providerId, key });
}

export async function deleteApiKey(providerId: string): Promise<void> {
  if (!desktop) return;
  await invoke("delete_api_key", { providerId });
}

export async function listTasks(): Promise<GenerationTask[]> {
  if (!desktop) return [];
  return invoke<GenerationTask[]>("tasks_list");
}

export async function upsertTask(task: GenerationTask): Promise<void> {
  if (!desktop) return;
  await invoke("tasks_upsert", { task });
}

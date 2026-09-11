import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import type {
  GenerationParams,
  Provider,
  ReferenceImage,
} from "../types";
import { chooseSample, dimensions } from "./generation";
import { sampleImages } from "../data/demo";
import { t } from "../i18n/index.ts";

export type BackendErrorKind =
  | "config"
  | "auth"
  | "rate_limit"
  | "network"
  | "server"
  | "invalid_response"
  | "cancelled";

export interface BackendErrorPayload {
  kind: BackendErrorKind;
  message: string;
  status?: number;
}

export interface GenerationJob {
  requestId: string;
  prompt: string;
  params: GenerationParams;
  provider: Provider;
  references: ReferenceImage[];
}

export interface ProducedImage {
  src: string;
  path?: string;
  thumb?: string;
  width: number;
  height: number;
  seed: string;
  filter: string;
  position: string;
}

export interface GenerateOptions {
  preferMock: boolean;
  seed: number;
  signal: AbortSignal;
}

export interface ConnectionTestResult {
  models: string[];
  latencyMs: number;
}

const MOCK_DURATION_MS = 4200;
const FILTERS = [
  "none",
  "saturate(.78) brightness(1.07)",
  "saturate(1.12) contrast(1.04)",
  "sepia(.14) brightness(.95)",
];
const POSITIONS = ["50% 50%", "34% 54%", "70% 50%", "50% 65%"];

export function backendAvailable(): boolean {
  return "__TAURI_INTERNALS__" in window;
}

export function usingDesktopBackend(preferMock: boolean): boolean {
  return backendAvailable() && !preferMock;
}

export function asBackendError(error: unknown): BackendErrorPayload {
  if (error instanceof DOMException && error.name === "AbortError")
    return { kind: "cancelled", message: t("validation.canceled") };
  if (typeof error === "object" && error !== null && "kind" in error) {
    const payload = error as BackendErrorPayload;
    if (typeof payload.kind === "string" && typeof payload.message === "string")
      return payload;
  }
  return {
    kind: "network",
    message: error instanceof Error ? error.message : String(error),
  };
}

function wait(milliseconds: number, signal: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      cleanup();
      resolve();
    }, milliseconds);
    const onAbort = () => {
      cleanup();
      reject(new DOMException("Aborted", "AbortError"));
    };
    const cleanup = () => {
      clearTimeout(timer);
      signal.removeEventListener("abort", onAbort);
    };
    if (signal.aborted) onAbort();
    else signal.addEventListener("abort", onAbort);
  });
}

export async function generateImages(
  job: GenerationJob,
  options: GenerateOptions,
): Promise<ProducedImage[]> {
  if (usingDesktopBackend(options.preferMock))
    return desktopGenerate(job, options);
  return mockGenerate(job, options);
}

async function mockGenerate(
  job: GenerationJob,
  options: GenerateOptions,
): Promise<ProducedImage[]> {
  await wait(MOCK_DURATION_MS, options.signal);
  const sample = chooseSample(job.prompt, options.seed);
  const size = dimensions(job.params.aspectRatio, job.params.resolution);
  return Array.from({ length: job.params.count }, (_, index) => ({
    src: sampleImages[sample],
    width: size.width,
    height: size.height,
    seed: String((options.seed + index) >>> 0),
    filter: FILTERS[(options.seed + index) % FILTERS.length],
    position: POSITIONS[index],
  }));
}

interface DesktopImage {
  path: string;
  thumb?: string;
  width: number;
  height: number;
}

async function desktopGenerate(
  job: GenerationJob,
  options: GenerateOptions,
): Promise<ProducedImage[]> {
  const images = await invoke<DesktopImage[]>("generate_images", {
    request: {
      requestId: job.requestId,
      providerId: job.provider.id,
      baseUrl: job.provider.baseUrl,
      apiKey: job.provider.apiKey,
      model: job.params.model,
      prompt: job.prompt,
      aspectRatio: job.params.aspectRatio,
      quality: job.params.quality,
      count: job.params.count,
    },
  });
  return images.map((image, index) => ({
    src: convertFileSrc(image.path),
    path: image.path,
    thumb: image.thumb ? convertFileSrc(image.thumb) : undefined,
    width: image.width,
    height: image.height,
    seed: String((options.seed + index) >>> 0),
    filter: "none",
    position: "50% 50%",
  }));
}

export function cancelDesktopGeneration(requestId: string) {
  if (!backendAvailable()) return;
  void invoke("cancel_generation", { requestId }).catch(() => {});
}

export function openInFolder(path: string): Promise<void> {
  return invoke("open_in_folder", { path });
}

/** Native save-as dialog; resolves to the target path or null when cancelled. */
export function exportImageTo(
  path: string,
  suggestedName: string,
): Promise<string | null> {
  return invoke<string | null>("export_image", { path, suggestedName });
}

export function testProviderConnection(provider: {
  baseUrl: string;
  apiKey: string;
  providerId?: string;
}): Promise<ConnectionTestResult> {
  return invoke<ConnectionTestResult>("test_connection", {
    request: {
      baseUrl: provider.baseUrl,
      apiKey: provider.apiKey,
      providerId: provider.providerId ?? "",
    },
  });
}

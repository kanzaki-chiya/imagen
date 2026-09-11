import type { ImageResult } from "../types";
import { backendAvailable, copyImageFile } from "./backend";

interface Drawable {
  source: CanvasImageSource;
  width: number;
  height: number;
  close?: () => void;
}

async function loadDrawable(src: string): Promise<Drawable> {
  try {
    const response = await fetch(src);
    if (response.ok) {
      const bitmap = await createImageBitmap(await response.blob());
      return {
        source: bitmap,
        width: bitmap.width,
        height: bitmap.height,
        close: () => bitmap.close(),
      };
    }
  } catch {
    // Fall back to a plain image element below.
  }
  const image = new Image();
  image.src = src;
  await image.decode();
  return { source: image, width: image.naturalWidth, height: image.naturalHeight };
}

/** Renders a result (with its crop and filter) into an encoded blob. */
export async function renderResult(result: ImageResult): Promise<Blob> {
  const drawable = await loadDrawable(result.src);
  const canvas = document.createElement("canvas");
  canvas.width = result.width;
  canvas.height = result.height;
  const context = canvas.getContext("2d");
  if (!context) throw new Error("Image export is unavailable");
  const ratio = Math.max(
    result.width / drawable.width,
    result.height / drawable.height,
  );
  const width = drawable.width * ratio;
  const height = drawable.height * ratio;
  const [x, y] = result.position
    .split(" ")
    .map((part) => parseFloat(part) / 100);
  context.filter = result.filter;
  context.drawImage(
    drawable.source,
    (result.width - width) * x,
    (result.height - height) * y,
    width,
    height,
  );
  drawable.close?.();
  const format = result.params.format.toLowerCase();
  const mime =
    format === "jpeg"
      ? "image/jpeg"
      : format === "webp"
        ? "image/webp"
        : "image/png";
  return new Promise<Blob>((resolve, reject) =>
    canvas.toBlob(
      (value) =>
        value ? resolve(value) : reject(new Error("Image export failed")),
      mime,
      0.95,
    ),
  );
}

/** `imagen-YYYYMMDD-HHMMSS-<id8>.<ext>` — sortable, unique, ASCII-safe. */
export function suggestedExportName(result: ImageResult): string {
  const date = new Date(result.createdAt);
  const pad = (value: number) => String(value).padStart(2, "0");
  const stamp = `${date.getFullYear()}${pad(date.getMonth() + 1)}${pad(
    date.getDate(),
  )}-${pad(date.getHours())}${pad(date.getMinutes())}${pad(
    date.getSeconds(),
  )}`;
  const extension =
    result.path?.split(".").pop() ??
    (result.params.format.toLowerCase() === "jpeg"
      ? "jpg"
      : result.params.format.toLowerCase());
  return `imagen-${stamp}-${result.id.slice(0, 8)}.${extension}`;
}

export async function exportImage(result: ImageResult) {
  const blob = await renderResult(result);
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = suggestedExportName(result);
  anchor.style.display = "none";
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  // Allow embedded browser download handlers time to consume the object URL.
  setTimeout(() => URL.revokeObjectURL(url), 60000);
}

/**
 * Copies the image to the system clipboard. Real generations copy the stored
 * file via the desktop backend; browser/mock results render through the
 * canvas and use the async clipboard API.
 */
export async function copyResultImage(result: ImageResult): Promise<void> {
  if (result.path && backendAvailable()) {
    await copyImageFile(result.path);
    return;
  }
  const drawable = await loadDrawable(result.src);
  const canvas = document.createElement("canvas");
  canvas.width = result.width;
  canvas.height = result.height;
  const context = canvas.getContext("2d");
  if (!context) throw new Error("Clipboard copy is unavailable");
  const ratio = Math.max(
    result.width / drawable.width,
    result.height / drawable.height,
  );
  const width = drawable.width * ratio;
  const height = drawable.height * ratio;
  const [x, y] = result.position
    .split(" ")
    .map((part) => parseFloat(part) / 100);
  context.filter = result.filter;
  context.drawImage(
    drawable.source,
    (result.width - width) * x,
    (result.height - height) * y,
    width,
    height,
  );
  drawable.close?.();
  // ClipboardItem only reliably supports PNG across browsers/webviews.
  const blob = await new Promise<Blob>((resolve, reject) =>
    canvas.toBlob(
      (value) =>
        value ? resolve(value) : reject(new Error("Clipboard copy failed")),
      "image/png",
    ),
  );
  await navigator.clipboard.write([
    new ClipboardItem({ "image/png": blob }),
  ]);
}

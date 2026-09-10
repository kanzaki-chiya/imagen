import type { ImageResult } from "../types";

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

export async function exportImage(result: ImageResult) {
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
  const blob = await new Promise<Blob>((resolve, reject) =>
    canvas.toBlob(
      (value) =>
        value ? resolve(value) : reject(new Error("Image export failed")),
      mime,
      0.95,
    ),
  );
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `imagen-${result.seed}.${format === "jpeg" ? "jpg" : format}`;
  anchor.style.display = "none";
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  // Allow embedded browser download handlers time to consume the object URL.
  setTimeout(() => URL.revokeObjectURL(url), 60000);
}

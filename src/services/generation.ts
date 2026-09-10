import type { AspectRatio, GenerationParams } from "../types.ts";
import { t } from "../i18n/index.ts";

export function dimensions(
  aspect: AspectRatio,
  resolution: GenerationParams["resolution"],
) {
  const sizes: Record<AspectRatio, [number, number]> = {
    "1:1": [1024, 1024],
    "3:2": [1536, 1024],
    "2:3": [1024, 1536],
    "16:9": [1536, 864],
  };
  const [width, height] = sizes[aspect];
  const scale = resolution === "4K" ? 4 : resolution === "2K" ? 2 : 1;
  return { width: width * scale, height: height * scale };
}

export function validateGeneration(
  prompt: string,
  params: GenerationParams,
): string | null {
  if (!prompt.trim()) return t("validation.emptyPrompt");
  if (prompt.length > 4000) return t("validation.longPrompt");
  if (!Number.isInteger(params.count) || params.count < 1 || params.count > 4)
    return t("validation.count");
  if (
    params.seed &&
    (!/^\d+$/.test(params.seed) || Number(params.seed) > 4294967295)
  )
    return t("validation.seed");
  return null;
}

export function validateProvider(
  baseUrl: string,
  model: string,
): string | null {
  try {
    const url = new URL(baseUrl);
    if (!["http:", "https:"].includes(url.protocol))
      return t("validation.providerProtocol");
    if (url.username || url.password)
      return t("validation.providerCredentials");
    if (url.search || url.hash) return t("validation.providerQuery");
  } catch {
    return t("validation.providerUrl");
  }
  return model.trim() ? null : t("validation.providerModel");
}

export function chooseSample(prompt: string, seed: number): number {
  if (/coast|ocean|sea|beach|cove|海/i.test(prompt)) return 1;
  if (/desert|dune|sand|terracotta|沙/i.test(prompt)) return 2;
  if (/product|botanic|ceramic|vase|plant|studio|植物|产品/i.test(prompt))
    return 3;
  if (/lake|mountain|forest|alpine|pine|湖|山|森林/i.test(prompt)) return 0;
  return seed % 4;
}

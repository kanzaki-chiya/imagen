import type { GenerationParams, ImageResult, Preset, Provider } from "../types";

const asset = (name: string) =>
  `${import.meta.env.BASE_URL}samples/${name}.png`;
export const sampleImages = [
  asset("alpine"),
  asset("coast"),
  asset("desert"),
  asset("botanical"),
];
export const initialPrompt =
  "A serene alpine lake surrounded by towering mountains and dense pine forests, soft morning mist drifting across the water, golden sunlight touching the peaks. Cinematic composition, natural colors, subtle film grain, photorealistic, incredibly detailed.";

export const defaultParams: GenerationParams = {
  providerId: "openai",
  model: "gpt-image-1",
  aspectRatio: "16:9",
  resolution: "1K",
  quality: "High",
  count: 4,
  seed: "",
  guidance: 7.5,
  negativePrompt: "",
  format: "PNG",
};

export const defaultProviders: Provider[] = [
  {
    id: "openai",
    name: "OpenAI",
    kind: "openai",
    description: "Versatile image generation and editing.",
    baseUrl: "https://api.openai.com/v1",
    apiKey: "",
    models: ["gpt-image-1", "dall-e-3"],
    defaultModel: "gpt-image-1",
    status: "connected",
  },
  {
    id: "compatible",
    name: "OpenAI Compatible",
    kind: "compatible",
    description: "Connect a gateway with an OpenAI-compatible interface.",
    baseUrl: "https://api.example.com/v1",
    apiKey: "",
    models: ["flux-1.1-pro", "flux-schnell"],
    defaultModel: "flux-1.1-pro",
    status: "untested",
  },
  {
    id: "gemini",
    name: "Gemini",
    kind: "gemini",
    description: "Explore multimodal image creation.",
    baseUrl: "https://generativelanguage.googleapis.com/v1beta",
    apiKey: "",
    models: ["gemini-2.5-flash-image", "imagen-3.0-generate-002"],
    defaultModel: "gemini-2.5-flash-image",
    status: "connected",
  },
  {
    id: "custom",
    name: "Custom Provider",
    kind: "custom",
    description: "Bring your own image generation endpoint.",
    baseUrl: "http://localhost:8188/v1",
    apiKey: "",
    models: ["custom-image-model"],
    defaultModel: "custom-image-model",
    status: "untested",
  },
];

export function createInitialHistory(): ImageResult[] {
  const titles = [
    "Alpine stillness",
    "Morning reflections",
    "Beyond the pines",
    "First light",
    "The quiet coast",
    "Sculpted by the wind",
    "A study in green",
    "Coastal afternoon",
  ];
  return titles.map((title, index) => ({
    id: `demo-${index}`,
    title,
    src: sampleImages[
      index < 4 ? 0 : index === 4 || index === 7 ? 1 : index === 5 ? 2 : 3
    ],
    prompt:
      index < 4
        ? initialPrompt
        : index === 5
          ? "Sculptural terracotta sand dunes at dawn, soft shadows, minimal composition, fine-art landscape photography."
          : index === 6
            ? "A sculptural green botanical leaf in a matte ceramic vase, quiet studio still life, soft directional light."
            : "A secluded Mediterranean cove, pale limestone cliffs, azure water and soft summer sunlight, editorial travel photography.",
    params: { ...defaultParams, count: 4 },
    createdAt: new Date(
      Date.now() - (index < 4 ? 120000 : 86400000 + index * 3600000),
    ).toISOString(),
    favorite: index === 0 || index === 5,
    width: 1536,
    height: 864,
    seed: String(284617 + index),
    filter: [
      "none",
      "saturate(.78) brightness(1.07)",
      "saturate(1.1) contrast(1.05)",
      "sepia(.12) brightness(.95)",
    ][index % 4],
    position: ["50% 50%", "34% 54%", "70% 50%", "50% 65%"][index % 4],
    batchId: index < 4 ? "demo-batch-1" : `demo-batch-${index}`,
    references: [],
  }));
}

export const defaultPresets: Preset[] = [
  {
    id: "cinematic",
    name: "Cinematic landscape",
    description: "Expansive scenes, atmospheric light, a touch of film.",
    prompt: initialPrompt,
    category: "Photography",
    params: { ...defaultParams },
    image: sampleImages[0],
  },
  {
    id: "editorial",
    name: "Coastal editorial",
    description: "Sun-washed tones and a slower kind of summer.",
    prompt:
      "A secluded Mediterranean cove, pale limestone cliffs, azure water and soft summer sunlight, editorial travel photography.",
    category: "Photography",
    params: { ...defaultParams, aspectRatio: "3:2" },
    image: sampleImages[1],
  },
  {
    id: "minimal",
    name: "Earth & form",
    description: "Sculptural shapes. Natural textures. Quiet compositions.",
    prompt:
      "Sculptural terracotta sand dunes at dawn, soft shadows, minimal composition, fine-art landscape photography.",
    category: "Illustration",
    params: { ...defaultParams, aspectRatio: "2:3" },
    image: sampleImages[2],
  },
  {
    id: "product",
    name: "Natural product studio",
    description: "Considered still life with soft, directional lighting.",
    prompt:
      "A sculptural green botanical leaf in a matte ceramic vase, quiet studio still life, soft directional light, premium product photography.",
    category: "Product",
    params: { ...defaultParams, aspectRatio: "1:1" },
    image: sampleImages[3],
  },
];

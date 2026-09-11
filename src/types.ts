export type Workspace = "generate" | "history" | "presets" | "settings";
export type Theme = "light" | "dark" | "system";
export type AspectRatio =
  | "1:1"
  | "3:2"
  | "4:3"
  | "16:9"
  | "9:16"
  | "2:3"
  | "3:4";
export type Quality = "Standard" | "High" | "Auto";
export type ConnectionStatus = "connected" | "untested" | "error" | "testing";

export interface Provider {
  id: string;
  name: string;
  kind: "openai" | "compatible" | "gemini" | "custom";
  description: string;
  baseUrl: string;
  apiKey: string;
  hasKey?: boolean;
  models: string[];
  defaultModel: string;
  status: ConnectionStatus;
}

export interface GenerationParams {
  providerId: string;
  model: string;
  aspectRatio: AspectRatio;
  resolution: "1K" | "2K" | "4K";
  quality: Quality;
  count: number;
  seed: string;
  guidance: number;
  negativePrompt: string;
  format: "PNG" | "JPEG" | "WebP";
}

export interface ReferenceImage {
  id: string;
  name: string;
  src: string;
  strength: number;
}

export interface ImageResult {
  id: string;
  src: string;
  path?: string;
  thumb?: string;
  title: string;
  prompt: string;
  params: GenerationParams;
  createdAt: string;
  favorite: boolean;
  width: number;
  height: number;
  seed: string;
  filter: string;
  position: string;
  batchId: string;
  references: ReferenceImage[];
}

export interface Preset {
  id: string;
  name: string;
  description: string;
  prompt: string;
  category: "Photography" | "Illustration" | "Product" | "My presets";
  params: GenerationParams;
  image: string;
}

export type TaskStatus =
  | "pending"
  | "running"
  | "succeeded"
  | "failed"
  | "cancelled"
  | "interrupted";

export interface GenerationTask {
  id: string;
  providerId: string;
  model: string;
  prompt: string;
  params: GenerationParams;
  status: TaskStatus;
  errorKind?: string;
  errorMessage?: string;
  createdAt: string;
  finishedAt?: string;
  resultCount: number;
}

export interface Toast {
  id: string;
  message: string;
  kind: "success" | "error" | "info";
}

export interface StoredWorkspace {
  version: 1;
  prompt: string;
  params: GenerationParams;
  history: ImageResult[];
  presets: Preset[];
  providers: (Omit<Provider, "apiKey"> & { hasKey?: boolean })[];
  references: ReferenceImage[];
  preferMock?: boolean;
  selectedId?: string;
  resultIds?: string[];
}

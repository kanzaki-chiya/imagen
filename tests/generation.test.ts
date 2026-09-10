import test from "node:test";
import assert from "node:assert/strict";
import {
  chooseSample,
  dimensions,
  validateGeneration,
  validateProvider,
} from "../src/services/generation.ts";
import type { GenerationParams } from "../src/types.ts";

const params: GenerationParams = {
  providerId: "openai",
  model: "gpt-image-1",
  aspectRatio: "3:2",
  resolution: "1K",
  quality: "High",
  count: 4,
  seed: "",
  guidance: 7.5,
  negativePrompt: "",
  format: "PNG",
};

test("dimensions preserve aspect ratio and scale for 2K and 4K", () => {
  assert.deepEqual(dimensions("3:2", "1K"), { width: 1536, height: 1024 });
  assert.deepEqual(dimensions("2:3", "2K"), { width: 2048, height: 3072 });
  assert.deepEqual(dimensions("16:9", "1K"), { width: 1536, height: 864 });
  assert.deepEqual(dimensions("1:1", "2K"), { width: 2048, height: 2048 });
  assert.deepEqual(dimensions("16:9", "4K"), { width: 6144, height: 3456 });
  assert.deepEqual(dimensions("1:1", "4K"), { width: 4096, height: 4096 });
});
test("prompt validation rejects empty or excessively long prompts", () => {
  assert.ok(validateGeneration("   ", params));
  assert.ok(validateGeneration("x".repeat(4001), params));
  assert.equal(validateGeneration("An alpine lake", params), null);
});
test("seed and batch boundaries are enforced including zero seed", () => {
  for (const seed of ["-1", "1.5", "abc", "4294967296"])
    assert.ok(validateGeneration("Lake", { ...params, seed }));
  for (const seed of ["", "0", "4294967295"])
    assert.equal(validateGeneration("Lake", { ...params, seed }), null);
  for (const count of [0, 5, 1.5, NaN])
    assert.ok(validateGeneration("Lake", { ...params, count }));
});
test("provider validation allows local endpoints but rejects credentials and unsafe protocols", () => {
  assert.equal(
    validateProvider("http://localhost:8188/v1", "demo-model"),
    null,
  );
  assert.equal(
    validateProvider("https://api.example.com/v1", "demo-model"),
    null,
  );
  for (const url of [
    "invalid",
    "javascript:alert(1)",
    "file:///x",
    "https://user:pass@example.com/v1",
    "https://example.com/?key=secret",
    "https://example.com/#key",
  ])
    assert.ok(validateProvider(url, "model"));
  assert.ok(validateProvider("https://example.com", "  "));
});
test("mock scene selection is deterministic and responds to prompt subjects", () => {
  assert.equal(chooseSample("A turquoise alpine lake", 3), 0);
  assert.equal(chooseSample("A Mediterranean coast", 0), 1);
  assert.equal(chooseSample("Sculptural desert dunes", 0), 2);
  assert.equal(chooseSample("Ceramic vase product studio", 0), 3);
  assert.equal(
    chooseSample("An abstract idea", 7),
    chooseSample("An abstract idea", 7),
  );
});

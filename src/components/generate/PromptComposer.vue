<script setup lang="ts">
import { shallowRef } from "vue";
import {
  WandSparkles,
  BookmarkPlus,
  ArrowUp,
  CircleStop,
  Type,
  Sparkles,
  CornerDownLeft,
  AlertCircle,
  RotateCcw,
} from "lucide-vue-next";
import { useI18n } from "../../i18n";
defineProps<{
  prompt: string;
  negativePrompt: string;
  count: number;
  generating: boolean;
  progress: number;
  error: string;
  ready: boolean;
  live: boolean;
}>();
const { t, tp } = useI18n();
const emit = defineEmits<{
  prompt: [value: string];
  negative: [value: string];
  generate: [];
  cancel: [];
  retry: [];
  preset: [];
  enhance: [];
}>();
const negativeTab = shallowRef(false);
const styles = ["Cinematic", "Photorealistic", "Natural light"];
function addStyle(prompt: string, style: string) {
  if (prompt.toLowerCase().includes(style.toLowerCase())) return;
  emit(
    "prompt",
    `${prompt.trim()}${prompt.trim() ? ", " : ""}${style.toLowerCase()}`,
  );
}
</script>
<template>
  <section class="prompt-composer" :aria-label="t('composer.aria')">
    <div class="prompt-heading">
      <div class="prompt-tabs" role="tablist" :aria-label="t('composer.tabAria')">
        <button
          role="tab"
          :aria-selected="!negativeTab"
          :class="{ selected: !negativeTab }"
          @click="negativeTab = false"
        >
          <Type :size="14" />{{ t("composer.tab.prompt") }}</button
        ><button
          role="tab"
          :aria-selected="negativeTab"
          :class="{ selected: negativeTab }"
          @click="negativeTab = true"
        >
          {{ t("composer.tab.negative")
          }}<span v-if="negativePrompt" class="negative-dot" />
        </button>
      </div>
      <button
        class="btn btn-ghost save-preset"
        :disabled="!prompt.trim()"
        @click="emit('preset')"
      >
        <BookmarkPlus :size="14" />{{ t("composer.savePreset") }}
      </button>
    </div>
    <div class="prompt-input-wrap" :class="{ 'has-error': error }">
      <textarea
        v-if="!negativeTab"
        id="prompt-input"
        class="prompt-input"
        :aria-label="t('composer.promptAria')"
        :value="prompt"
        maxlength="4000"
        :placeholder="t('composer.promptPlaceholder')"
        spellcheck="false"
        @input="emit('prompt', ($event.target as HTMLTextAreaElement).value)"
        @keydown.ctrl.enter.prevent="emit('generate')"
        @keydown.meta.enter.prevent="emit('generate')"
      /><textarea
        v-else
        class="prompt-input"
        :aria-label="t('composer.negativeAria')"
        :value="negativePrompt"
        maxlength="2000"
        :placeholder="t('composer.negativePlaceholder')"
        spellcheck="false"
        @input="emit('negative', ($event.target as HTMLTextAreaElement).value)"
      /><span class="character-count mono"
        >{{ negativeTab ? negativePrompt.length : prompt.length }} /
        {{ negativeTab ? "2,000" : "4,000" }}</span
      >
    </div>
    <div v-if="error" class="generation-error" role="alert">
      <AlertCircle :size="14" /><span>{{ error }}</span
      ><button class="btn btn-ghost" @click="emit('retry')">
        <RotateCcw :size="12" />{{ t("composer.retry") }}
      </button>
    </div>
    <div class="prompt-actions">
      <div class="prompt-tools">
        <button
          class="enhance-button"
          :disabled="!prompt.trim() || generating"
          :data-tip="t('composer.enhanceTip')"
          @click="emit('enhance')"
        >
          <WandSparkles :size="14" /><span>{{ t("composer.enhance") }}</span></button
        ><span class="tool-divider" /><button
          v-for="style in styles"
          :key="style"
          class="style-tag"
          :class="{
            included: prompt.toLowerCase().includes(style.toLowerCase()),
          }"
          :disabled="generating"
          @click="addStyle(prompt, style)"
        >
          {{ style }}
        </button>
      </div>
      <span class="mock-hint"
        ><span class="status-dot" />{{
          live ? t("composer.mockHint.live") : t("composer.mockHint.mock")
        }}</span
      >
    </div>
    <div class="generate-row">
      <span class="generation-help"
        ><Sparkles :size="13" /><span>{{ t("composer.help") }}</span></span
      ><span class="shortcut-hint">Ctrl <CornerDownLeft :size="11" /></span
      ><button
        v-if="generating"
        class="btn generate-button cancel-button"
        @click="emit('cancel')"
      >
        <CircleStop :size="16" />{{ t("composer.cancel") }}<span class="mono"
          >{{ progress }}%</span
        ></button
      ><button
        v-else
        class="btn btn-primary generate-button"
        :disabled="!prompt.trim() || !ready"
        @click="emit('generate')"
      >
        <Sparkles :size="16" />{{ tp("composer.generate", count)
        }}<ArrowUp :size="16" />
      </button>
    </div>
  </section>
</template>
<style scoped>
.prompt-composer {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  flex-shrink: 0;
  padding: 13px 23px 18px;
  background: var(--panel);
  border-top: 1px solid var(--border);
}
.prompt-heading {
  grid-column: 1 / -1;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 9px;
}
.prompt-tabs {
  display: flex;
  gap: 18px;
}
.prompt-tabs button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 3px 0;
  font-size: 11px;
  color: var(--text-muted);
}
.prompt-tabs button.selected {
  color: var(--text);
  font-weight: 600;
}
.negative-dot {
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: var(--accent);
}
.save-preset {
  min-height: 24px;
  font-size: 10px;
  color: var(--text-secondary);
  padding: 2px 4px;
}
.prompt-input-wrap {
  grid-column: 1 / -1;
  position: relative;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--input);
}
.prompt-input-wrap:focus-within {
  border-color: var(--accent);
}
.prompt-input-wrap.has-error {
  border-color: var(--danger);
}
.prompt-input {
  display: block;
  width: 100%;
  min-height: 83px;
  height: 83px;
  padding: 11px 12px 19px;
  background: transparent;
  border: 0;
  resize: none;
  font-size: 12px;
  line-height: 1.8;
  outline: none;
}
.character-count {
  position: absolute;
  bottom: 5px;
  right: 10px;
  color: var(--text-muted);
  font-size: 8px;
  pointer-events: none;
}
.prompt-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding-top: 12px;
}
.prompt-tools {
  display: flex;
  align-items: center;
  gap: 6px;
}
.enhance-button {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  color: var(--text-secondary);
  font-size: 10px;
  padding: 3px 2px;
}
.enhance-button:hover:not(:disabled) {
  color: var(--accent);
}
.tool-divider {
  height: 12px;
  width: 1px;
  background: var(--border);
  margin: 0 3px;
}
.style-tag {
  font-size: 9px;
  color: var(--text-muted);
  padding: 3px 6px;
  border-radius: 4px;
  background: var(--input);
  border: 1px solid var(--border);
}
.style-tag:hover:not(:disabled) {
  border-color: var(--border-strong);
  color: var(--text);
}
.style-tag.included {
  color: var(--text-secondary);
}
.mock-hint {
  display: none;
  align-items: center;
  gap: 5px;
  font-size: 8px;
  color: var(--text-muted);
  white-space: nowrap;
}
.mock-hint .status-dot {
  width: 4px;
  height: 4px;
}
.generate-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
  padding-left: 12px;
}
.generation-help {
  display: none;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
  font-size: 10px;
}
.shortcut-hint {
  margin-left: auto;
  display: inline-flex;
  align-items: center;
  gap: 5px;
  font-family: var(--font-mono);
  font-size: 9px;
  color: var(--text-muted);
}
.generate-button {
  min-height: 38px;
  padding: 8px 15px;
  font-size: 11px;
  gap: 8px;
  min-width: 175px;
}
.generate-button > svg:last-child {
  margin-left: 8px;
}
.cancel-button {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-soft);
}
.generation-error {
  grid-column: 1 / -1;
  display: flex;
  align-items: center;
  gap: 7px;
  font-size: 10px;
  color: var(--danger);
  background: var(--danger-soft);
  border-radius: 4px;
  padding: 6px 8px;
  margin-top: 8px;
}
.generation-error > span {
  flex: 1;
}
.generation-error .btn {
  padding: 2px 5px;
  min-height: 22px;
  font-size: 10px;
}
@media (min-width: 1800px) {
  .prompt-composer {
    padding: 17px 28px 22px;
  }
  .prompt-input {
    height: 100px;
    font-size: 13px;
  }
  .generate-button {
    min-height: 42px;
    font-size: 12px;
  }
  .prompt-tabs button {
    font-size: 12px;
  }
}
@media (max-width: 1000px) {
  .prompt-composer {
    padding-left: 16px;
    padding-right: 16px;
  }
  .mock-hint,
  .style-tag:last-child {
    display: none;
  }
  .generation-help {
    font-size: 9px;
  }
}
@media (max-width: 600px) {
  .prompt-composer {
    padding: 10px 13px 13px;
  }
  .generation-help span,
  .style-tag:last-child {
    display: none;
  }
  .prompt-tabs {
    gap: 12px;
  }
  .save-preset {
    font-size: 9px;
  }
  .prompt-tools {
    flex-wrap: wrap;
  }
  .style-tag,
  .tool-divider,
  .shortcut-hint {
    display: none;
  }
}
@media (max-height: 740px) {
  .prompt-composer {
    padding-top: 8px;
    padding-bottom: 12px;
  }
  .prompt-input {
    height: 68px;
    min-height: 68px;
  }
  .generate-row {
    margin-top: 10px;
  }
}
</style>

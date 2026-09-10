import { ref } from "vue";
import { en } from "./en.ts";
import { zh } from "./zh.ts";

export type Locale = "zh" | "en";
type Messages = Record<string, string>;

const STORAGE_KEY = "imagen:locale";
const catalogs: Record<Locale, Messages> = { zh, en };

function initial(): Locale {
  try {
    if (typeof localStorage === "undefined") return "zh";
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === "zh" || saved === "en") return saved;
  } catch {
    // Storage unavailable; fall through to the default.
  }
  return "zh";
}

function applyDocumentLang(value: Locale) {
  if (typeof document !== "undefined")
    document.documentElement.lang = value === "zh" ? "zh-CN" : "en";
}

export const locale = ref<Locale>(initial());
applyDocumentLang(locale.value);

export function setLocale(value: Locale) {
  locale.value = value;
  applyDocumentLang(value);
  try {
    if (typeof localStorage !== "undefined")
      localStorage.setItem(STORAGE_KEY, value);
  } catch {
    // Storage unavailable; the preference only lives for this session.
  }
}

export function t(
  key: string,
  params?: Record<string, string | number>,
): string {
  let message = catalogs[locale.value][key] ?? catalogs.en[key] ?? key;
  if (params)
    for (const [name, value] of Object.entries(params))
      message = message.replaceAll(`{${name}}`, String(value));
  return message;
}

export function tp(
  base: string,
  n: number,
  params?: Record<string, string | number>,
): string {
  const catalog = catalogs[locale.value];
  const key =
    n === 1 && `${base}.one` in catalog ? `${base}.one` : `${base}.other`;
  return t(key, { n, ...params });
}

export function useI18n() {
  return { locale, setLocale, t, tp };
}

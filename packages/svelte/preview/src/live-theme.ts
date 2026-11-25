import { themePresets, type ThemePreset, type ThemeScheme } from "@inflatable-cookie/poodle-core";

export interface LiveThemeSettings {
  scheme: ThemeScheme;
  base: string;
  baseBlend: number;
  accent: string;
  accentBlend: number;
  contrast: number;
  evenTint: boolean;
}

const defaults: LiveThemeSettings = {
  scheme: "dark",
  base: "mono",
  baseBlend: 0.25,
  accent: "mono",
  accentBlend: 0,
  contrast: 1,
  evenTint: true,
};

export function liveThemePreset(preset: ThemePreset, evenTint = true): LiveThemeSettings {
  const [, , scheme, plainBase, plainBlend, accent, accentBlend, contrast, evenBase] = preset;
  const [base, baseBlend] = (evenTint && evenBase) || [plainBase, plainBlend];
  return { scheme, base, baseBlend, accent, accentBlend, contrast, evenTint };
}

export function startLiveTheme(theme: string): LiveThemeSettings {
  const preset = themePresets.find(([id]) => id === theme);
  return preset ? liveThemePreset(preset) : { ...defaults };
}

function readColor(value: string | null): string {
  return value && /^#[0-9a-f]{6}$/i.test(value) ? value.toLowerCase() : "mono";
}

function readNumber(value: string | null, fallback: number, min: number, max: number): number {
  const number = value?.trim() ? Number(value) : NaN;
  return Number.isFinite(number) ? Math.min(max, Math.max(min, number)) : fallback;
}

export function readLiveTheme(params: URLSearchParams): LiveThemeSettings | null {
  const scheme = params.get("liveTheme");
  if (scheme !== "light" && scheme !== "dark") return null;

  return {
    scheme,
    base: readColor(params.get("base")),
    baseBlend: readNumber(params.get("baseBlend"), defaults.baseBlend, 0, 1),
    accent: readColor(params.get("accent")),
    accentBlend: readNumber(params.get("accentBlend"), defaults.accentBlend, 0, 1),
    contrast: readNumber(params.get("contrast"), defaults.contrast, 0.5, 2),
    evenTint: params.get("evenTint") !== "0",
  };
}

export function writeLiveTheme(params: URLSearchParams, settings: LiveThemeSettings): void {
  params.set("liveTheme", settings.scheme);
  params.set("base", settings.base);
  params.set("baseBlend", String(settings.baseBlend));
  params.set("accent", settings.accent);
  params.set("accentBlend", String(settings.accentBlend));
  params.set("contrast", String(settings.contrast));
  params.set("evenTint", settings.evenTint ? "1" : "0");
}

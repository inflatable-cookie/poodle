import type { ThemeSettings } from "./theme-resolver";

export type ThemePreset = readonly [
  id: string,
  name: string,
  scheme: NonNullable<ThemeSettings["scheme"]>,
  base: string,
  baseBlend: number,
  accent: string,
  accentBlend: number,
  contrast: number,
  evenTintBase?: readonly [base: string, baseBlend: number],
];

/** Preview preset settings, with a base override for the default even-tint mode. */
export const themePresets = [
  ["iceberg", "Iceberg", "light", "#4285c1", 0.53, "#1668d6", 0, 1, ["#6b839a", 0.53]],
  ["clay", "Clay", "light", "#c89514", 0.45, "#c96442", 0, 1, ["#967c4c", 0.45]],
  ["meadow", "Meadow", "light", "#56932b", 0.33, "#188f52", 0, 1, ["#718865", 0.33]],
  ["solarized-light", "Solarized Light", "light", "#f3c52d", 0.23, "#268bd2", 0, 1, ["#a27901", 0.27]],
  ["eclipse", "Eclipse", "dark", "#6384a3", 0.2, "#f0b24d", 0, 1],
  ["cobalt", "Cobalt", "dark", "mono", 0.13, "#5b9dff", 0, 1],
  ["graphite", "Graphite", "dark", "mono", 0.21, "#97d14d", 0, 1],
  ["hornet", "Hornet", "dark", "mono", 0, "#ffd400", 0, 1],
  ["midnight", "Midnight", "dark", "#5a7dcb", 0.17, "#6d8cff", 0, 1],
  ["forest", "Forest", "dark", "#5e8d69", 0.23, "#4dc98a", 0, 1],
  ["rose", "Rose", "dark", "#a3707b", 0.2, "#f65c8a", 0, 1],
  ["nord", "Nord", "dark", "#73809a", 0.98, "#88c0d0", 0, 1.35],
  ["solarized", "Solarized", "dark", "#038eaf", 0.61, "#b58900", 0, 1.4],
] as const satisfies readonly ThemePreset[];

// Framework-neutral theme catalogue: turns the generated `themes` metadata into
// a selectable list with resolved swatch colors (theme override, else the base
// semantic value). This is the data source a theme controller serves; the
// ThemeSelect component renders it. No framework, no DOM.

import { tokens } from "./generated/ts/index";
import { themes } from "./themes";

export type ThemeSwatch = {
  /** Page background. */
  canvas: string;
  /** Raised surface / card fill. */
  surface: string;
  /** Accent / brand color. */
  accent: string;
  /** Primary text. */
  text: string;
  /** Default border. */
  border: string;
};

export type ThemeOption = {
  /** `data-theme` value. */
  value: string;
  /** Human-readable label ("Loophole Studio"). */
  label: string;
  /** Theme description from the token metadata. */
  description: string;
  /** Representative colors for a mini preview. */
  swatch: ThemeSwatch;
};

const SWATCH_PATHS = {
  canvas: "color.background.canvas",
  surface: "color.background.surface",
  accent: "color.accent.base",
  text: "color.text.primary",
  border: "color.border.default",
} as const;

/** Resolve a base semantic token value (e.g. `color.accent.base`) to its hex.
 * Semantic tokens live under `tokens.semantic`. */
function baseColor(path: string): string {
  const value = ["semantic", ...path.split(".")].reduce<unknown>((node, key) => {
    if (node && typeof node === "object") return (node as Record<string, unknown>)[key];
    return undefined;
  }, tokens as unknown);
  return typeof value === "string" ? value : "";
}

/** "high-contrast" -> "High Contrast". */
function humanize(name: string): string {
  return name
    .split(/[-_]/)
    .map((word) => (word ? word.charAt(0).toUpperCase() + word.slice(1) : word))
    .join(" ");
}

/** The selectable Poodle themes with resolved swatch colors, in metadata order. */
export function themeOptions(): ThemeOption[] {
  return Object.entries(themes).map(([value, meta]) => {
    const overrides = (meta as { overrides?: Record<string, string> }).overrides ?? {};
    const pick = (path: string): string => overrides[path] ?? baseColor(path);
    return {
      value,
      label: humanize(value),
      description: (meta as { description?: string }).description ?? "",
      swatch: {
        canvas: pick(SWATCH_PATHS.canvas),
        surface: pick(SWATCH_PATHS.surface),
        accent: pick(SWATCH_PATHS.accent),
        text: pick(SWATCH_PATHS.text),
        border: pick(SWATCH_PATHS.border),
      },
    };
  });
}

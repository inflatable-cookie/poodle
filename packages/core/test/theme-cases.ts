import { themePresets, type ThemePreset } from "../src/theme-presets";
import type { ThemeScheme, ThemeSettings } from "../src/theme-resolver";

export const schemes = ["light", "dark"] as const;
export const tintModes = [true, false] as const;

export function presetSettings(
  preset: ThemePreset,
  scheme: ThemeScheme,
  evenTint: boolean,
): ThemeSettings {
  const [, , , plainBase, plainBlend, accent, accentBlend, contrast, alternate] = preset;
  const [base, baseBlend] = (evenTint && alternate) || [plainBase, plainBlend];
  return { scheme, base, baseBlend, accent, accentBlend, contrast, evenTint };
}

export function fixedRandom(seed: number) {
  return () => {
    seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0;
    return seed / 2 ** 32;
  };
}

export function randomTheme(random: () => number): ThemeSettings {
  const color = () => "#" + Math.floor(random() * 2 ** 24).toString(16).padStart(6, "0");
  const scheme = random() < 0.5 ? "light" : "dark";
  const evenTint = random() < 0.5;
  const base = color();
  const accent = color();
  const baseBlend = random();
  const accentBlend = random();
  const contrast = 0.5 + random() * 1.5;
  return { scheme, evenTint, base, accent, baseBlend, accentBlend, contrast };
}

export interface ThemeCase {
  id: string;
  settings: ThemeSettings;
}

export function themeCases(): ThemeCase[] {
  const cases: ThemeCase[] = [];
  const random = fixedRandom(0xc0ffee);
  for (const scheme of schemes) {
    for (const evenTint of tintModes) {
      const prefix = `${scheme}/${evenTint ? "even" : "plain"}`;
      for (const preset of themePresets) {
        cases.push({ id: `preset/${preset[0]}/${prefix}`, settings: presetSettings(preset, scheme, evenTint) });
      }
      for (const baseBlend of [0, 0.25, 0.5, 1]) {
        for (const accentBlend of [0, 0.25, 0.5, 1]) {
          for (const contrast of [0.5, 1, 1.35, 2]) {
            cases.push({
              id: `grid/${prefix}/${baseBlend}/${accentBlend}/${contrast}`,
              settings: { scheme, evenTint, base: "#e5652e", accent: "#2f6fed", baseBlend, accentBlend, contrast },
            });
          }
        }
      }
      const awkward: Record<string, ThemeSettings> = {
        "same-hue": { base: "#2f6fed", accent: "#2f6fed" },
        "opposite-hues": { base: "#e5487f", accent: "#4dc98a" },
        "vivid-primaries": { base: "#ff0000", accent: "#00ff00" },
        "yellow-blue": { base: "#0000ff", accent: "#ffff00" },
        "black-white": { base: "#000000", accent: "#ffffff" },
        "white-black": { base: "#ffffff", accent: "#000000" },
        "mid-grey": { base: "#808080", accent: "#808080" },
        "mid-lightness": { base: "#97654d", accent: "#787c91" },
        "rgb-input": { base: [0.5, 0.25, 0.75], accent: [1.2, -0.1, 0.4] },
        "short-hex": { base: "AbC", accent: " #dEf " },
        "invalid": { base: "#12345678", accent: "#zzzzzz", baseBlend: 7, accentBlend: -3, contrast: -1 },
        "clamp-exponents": { baseBlend: -1e-7, accentBlend: 1e21 },
        "mono": {},
      };
      for (const [id, settings] of Object.entries(awkward)) {
        cases.push({ id: `awkward/${prefix}/${id}`, settings: { scheme, evenTint, baseBlend: 0.5, accentBlend: 0.5, ...settings } });
      }
    }
  }
  for (let i = 0; i < 512; i++) {
    cases.push({ id: `seeded/${i}`, settings: randomTheme(random) });
  }
  return cases;
}

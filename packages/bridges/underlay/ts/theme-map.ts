import { controlSizes, densityModes, themes } from "../../../tokens/artifacts/ts/themes";

export type UnderlayThemeBridge = {
  underlayThemeId: string;
  flintThemeId: keyof typeof themes;
  cssFile: string;
  note?: string;
};

export type UnderlayModeBridge = {
  underlayModeId: string;
  flintModeId: string;
  selector: string;
};

// These IDs are bridge-local placeholders until Underlay provides its own
// canonical registration names. They keep the mapping explicit without leaking
// Flint naming directly into app code.
export const underlayThemeMap: UnderlayThemeBridge[] = [
  {
    underlayThemeId: "underlay-default",
    flintThemeId: "light",
    cssFile: "packages/tokens/artifacts/css/flint-theme-light.css",
  },
  {
    underlayThemeId: "underlay-night",
    flintThemeId: "dark",
    cssFile: "packages/tokens/artifacts/css/flint-theme-dark.css",
  },
  {
    underlayThemeId: "underlay-studio",
    flintThemeId: "loophole-studio",
    cssFile: "packages/tokens/artifacts/css/flint-theme-loophole-studio.css",
    note: "Reserved for workstation-oriented adoption pressure from Aura and Spark.",
  },
];

export const underlayDensityModeMap: UnderlayModeBridge[] = Object.entries(densityModes).map(
  ([flintModeId, value]) => ({
    underlayModeId: `underlay-density-${flintModeId}`,
    flintModeId,
    selector: value.selector,
  }),
);

export const underlayControlSizeMap: UnderlayModeBridge[] = Object.entries(controlSizes).map(
  ([flintModeId, value]) => ({
    underlayModeId: `underlay-control-size-${flintModeId}`,
    flintModeId,
    selector: value.selector,
  }),
);

export const canonicalFlintThemes = Object.keys(themes);

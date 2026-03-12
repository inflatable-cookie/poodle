import { controlSizes, densityModes, themes } from "../../../tokens/artifacts/ts/themes";

export type UnderlayThemeBridge = {
  underlayThemeId: string;
  pugThemeId: keyof typeof themes;
  cssFile: string;
  note?: string;
};

export type UnderlayModeBridge = {
  underlayModeId: string;
  pugModeId: string;
  selector: string;
};

// These IDs are bridge-local placeholders until Underlay provides its own
// canonical registration names. They keep the mapping explicit without leaking
// Pug naming directly into app code.
export const underlayThemeMap: UnderlayThemeBridge[] = [
  {
    underlayThemeId: "underlay-default",
    pugThemeId: "light",
    cssFile: "packages/tokens/artifacts/css/pug-theme-light.css",
  },
  {
    underlayThemeId: "underlay-night",
    pugThemeId: "dark",
    cssFile: "packages/tokens/artifacts/css/pug-theme-dark.css",
  },
  {
    underlayThemeId: "underlay-studio",
    pugThemeId: "loophole-studio",
    cssFile: "packages/tokens/artifacts/css/pug-theme-loophole-studio.css",
    note: "Reserved for workstation-oriented adoption pressure from Aura and Spark.",
  },
];

export const underlayDensityModeMap: UnderlayModeBridge[] = Object.entries(densityModes).map(
  ([pugModeId, value]) => ({
    underlayModeId: `underlay-density-${pugModeId}`,
    pugModeId,
    selector: value.selector,
  }),
);

export const underlayControlSizeMap: UnderlayModeBridge[] = Object.entries(controlSizes).map(
  ([pugModeId, value]) => ({
    underlayModeId: `underlay-control-size-${pugModeId}`,
    pugModeId,
    selector: value.selector,
  }),
);

export const canonicalPugThemes = Object.keys(themes);

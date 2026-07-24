import { controlSizes, densityModes, themes } from "../../../tokens/artifacts/ts/themes";

export type UnderlayThemeBridge = {
  underlayThemeId: string;
  poodleThemeId: keyof typeof themes;
  cssFile: string;
  note?: string;
};

export type UnderlayModeBridge = {
  underlayModeId: string;
  poodleModeId: string;
  selector: string;
};

// These IDs are bridge-local placeholders until Underlay provides its own
// canonical registration names. They keep the mapping explicit without leaking
// Poodle naming directly into app code.
export const underlayThemeMap: UnderlayThemeBridge[] = [
  {
    underlayThemeId: "underlay-default",
    poodleThemeId: "iceberg",
    cssFile: "packages/tokens/artifacts/css/poodle-theme-iceberg.css",
  },
  {
    underlayThemeId: "underlay-night",
    poodleThemeId: "dark",
    cssFile: "packages/tokens/artifacts/css/poodle-theme-dark.css",
  },
  {
    underlayThemeId: "underlay-studio",
    poodleThemeId: "graphite",
    cssFile: "packages/tokens/artifacts/css/poodle-theme-graphite.css",
    note: "Reserved for workstation-oriented adoption pressure from Aura and Spark.",
  },
];

export const underlayDensityModeMap: UnderlayModeBridge[] = Object.entries(densityModes).map(
  ([poodleModeId, value]) => ({
    underlayModeId: `underlay-density-${poodleModeId}`,
    poodleModeId,
    selector: value.selector,
  }),
);

export const underlayControlSizeMap: UnderlayModeBridge[] = Object.entries(controlSizes).map(
  ([poodleModeId, value]) => ({
    underlayModeId: `underlay-control-size-${poodleModeId}`,
    poodleModeId,
    selector: value.selector,
  }),
);

export const canonicalPoodleThemes = Object.keys(themes);

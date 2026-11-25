import { controlSizes, densityModes, styles, themes } from "./themes";

export type ThemeName = keyof typeof themes;
export type DensityModeName = keyof typeof densityModes;
export type ControlSizeName = keyof typeof controlSizes;
export type StyleName = keyof typeof styles;

export function applyThemeAttributes(
  element: HTMLElement,
  options: {
    theme?: ThemeName;
    style?: StyleName;
    density?: DensityModeName;
    controlSize?: ControlSizeName;
  },
): void {
  if (options.theme) {
    element.dataset.theme = options.theme;
  }

  if (options.style) {
    element.dataset.style = options.style;
  }

  if (options.density) {
    element.dataset.density = options.density;
  }

  if (options.controlSize) {
    element.dataset.controlSize = options.controlSize;
  }
}

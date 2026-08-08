import { controlSizes, densityModes, themes } from "./index";

export type ThemeName = keyof typeof themes;
export type DensityModeName = keyof typeof densityModes;
export type ControlSizeName = keyof typeof controlSizes;

export function applyThemeAttributes(
  element: HTMLElement,
  options: {
    theme?: ThemeName;
    density?: DensityModeName;
    controlSize?: ControlSizeName;
  },
): void {
  if (options.theme) {
    element.dataset.theme = options.theme;
  }

  if (options.density) {
    element.dataset.density = options.density;
  }

  if (options.controlSize) {
    element.dataset.controlSize = options.controlSize;
  }
}

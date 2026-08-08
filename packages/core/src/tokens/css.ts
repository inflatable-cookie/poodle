import { cssVars } from "./index";

export const colorVars = {
  backgroundCanvas: cssVars["color.background.canvas"],
  backgroundPanel: cssVars["color.background.panel"],
  textPrimary: cssVars["color.text.primary"],
  borderDefault: cssVars["color.border.default"],
  accentBase: cssVars["color.accent.base"],
} as const;

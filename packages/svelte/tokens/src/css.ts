import { cssVars } from "./index";

export const colorVars = {
  backgroundCanvas: cssVars["semantic.color.background.canvas"],
  backgroundPanel: cssVars["semantic.color.background.panel"],
  textPrimary: cssVars["semantic.color.text.primary"],
  borderDefault: cssVars["semantic.color.border.default"],
  accentBase: cssVars["semantic.color.accent.base"],
} as const;

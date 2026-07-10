/**
 * Color conversion utilities, now provided by the headless core; re-exported
 * so internal imports are unchanged.
 */
export {
  isValidHex,
  normalizeHex,
  hexToRgb,
  rgbToHex,
  rgbToHsl,
  hslToRgb,
  rgbToHsv,
  hsvToRgb,
  hexToHsv,
  hsvToHex,
  hexToHsl,
  hslToHex,
  hsvToHsl,
  hslToHsv,
  type RgbColor,
  type HslColor,
  type HsvColor,
} from "@poodle/headless";

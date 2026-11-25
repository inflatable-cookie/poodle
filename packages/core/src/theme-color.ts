// The resolver keeps every emitted channel quantised. Only the layer and gamut
// searches use unquantised linear sRGB.
import { hexToRgb, rgbToHex } from "./color";

export type ThemeColor = readonly [red: number, green: number, blue: number, alpha: number];
type Triple = readonly [number, number, number];

export function clamp(value: number, low: number, high: number): number {
  return Math.min(high, Math.max(low, value));
}

function byte(value: number): number {
  return Math.round(clamp(value, 0, 1) * 255);
}

export function rgb(r: number, g: number, b: number): ThemeColor {
  return [byte(r), byte(g), byte(b), 255];
}

export const WHITE = Object.freeze(rgb(1, 1, 1));
export const BLACK = Object.freeze(rgb(0, 0, 0));

export function alpha([r, g, b]: ThemeColor, a: number): ThemeColor {
  return [r, g, b, byte(a)];
}

export function hex([r, g, b, a]: ThemeColor): string {
  return rgbToHex(r, g, b, a / 255);
}

export function parseColor(value: unknown): ThemeColor | null {
  if (typeof value === "string") {
    const trimmed = value.trim();
    if (!/^#?([0-9a-f]{3}|[0-9a-f]{6})$/i.test(trimmed)) return null;
    const { r, g, b } = hexToRgb(trimmed);
    return [r, g, b, 255];
  }
  if (Array.isArray(value) && value.length >= 3) {
    const [r, g, b] = value;
    if (Number.isFinite(r) && Number.isFinite(g) && Number.isFinite(b) && value.every(Number.isFinite)) {
      return rgb(r, g, b);
    }
  }
  return null;
}

function toLinear(channel: number): number {
  return channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4;
}

function toGamma(channel: number): number {
  return channel <= 0.0031308 ? 12.92 * channel : 1.055 * channel ** (1 / 2.4) - 0.055;
}

export function linearLuminance([r, g, b]: Triple): number {
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

function luminance([r, g, b]: ThemeColor): number {
  return linearLuminance([toLinear(r / 255), toLinear(g / 255), toLinear(b / 255)]);
}

export function flare(y: number): number {
  return Math.cbrt(y + 0.01);
}

export function flareToLuminance(value: number): number {
  return value * value * value - 0.01;
}

export function contrast(fg: ThemeColor, bg: ThemeColor): number {
  const a = luminance(fg);
  const b = luminance(bg);
  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
}

export function over(fg: ThemeColor, bg: ThemeColor): ThemeColor {
  const a = fg[3] / 255;
  const channel = (front: number, back: number) => front / 255 * a + back / 255 * (1 - a);
  return rgb(channel(fg[0], bg[0]), channel(fg[1], bg[1]), channel(fg[2], bg[2]));
}

export function toLab([red, green, blue]: ThemeColor): Triple {
  const r = toLinear(red / 255);
  const g = toLinear(green / 255);
  const b = toLinear(blue / 255);
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
  const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  return [
    0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s,
    1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s,
    0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s,
  ];
}

function labToLinear(lightness: number, a: number, b: number): Triple {
  const l = (lightness + 0.3963377774 * a + 0.2158037573 * b) ** 3;
  const m = (lightness - 0.1055613458 * a - 0.0638541728 * b) ** 3;
  const s = (lightness - 0.0894841775 * a - 1.291485548 * b) ** 3;
  return [
    4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
    -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
    -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
  ];
}

export function vector(chroma: number, hue: number): readonly [number, number] {
  const radians = hue * Math.PI / 180;
  return [chroma * Math.cos(radians), chroma * Math.sin(radians)];
}

export function polar(a: number, b: number): readonly [number, number] {
  const hue = Math.atan2(b, a) * 180 / Math.PI;
  return [Math.hypot(a, b), hue < 0 ? hue + 360 : hue];
}

export function toLch(color: ThemeColor): Triple {
  const [lightness, a, b] = toLab(color);
  const [chroma, hue] = polar(a, b);
  // A grey reads back with rounding-noise chroma. Snap it before fitting.
  return chroma < 1e-6 ? [lightness, 0, 0] : [lightness, chroma, hue];
}

function inGamut(channels: Triple): boolean {
  return channels.every((c) => c >= -1e-7 && c <= 1 + 1e-7);
}

export function lchToLinear(l: number, chroma: number, hue: number): Triple {
  const lightness = clamp(l, 0, 1);
  const [a, b] = vector(1, hue);
  const at = (c: number) => labToLinear(lightness, c * a, c * b);
  // Out-of-gamut colours retain lightness and hue by losing chroma.
  const [r, g, blue] = at(highestIn((c) => inGamut(at(c)), 0, chroma) ?? 0);
  return [clamp(r, 0, 1), clamp(g, 0, 1), clamp(blue, 0, 1)];
}

export function lch(lightness: number, chroma: number, hue: number): ThemeColor {
  const [r, g, b] = lchToLinear(lightness, chroma, hue);
  return rgb(toGamma(r), toGamma(g), toGamma(b));
}

export function labDistance(x: ThemeColor, y: ThemeColor): number {
  const [l1, a1, b1] = toLab(x);
  const [l2, a2, b2] = toLab(y);
  return Math.hypot(l1 - l2, a1 - a2, b1 - b2);
}

export function flareDistance(x: ThemeColor, y: ThemeColor): number {
  const [, a1, b1] = toLab(x);
  const [, a2, b2] = toLab(y);
  return Math.hypot(flare(luminance(x)) - flare(luminance(y)), a1 - a2, b1 - b2);
}

// Both searches use a fixed 24 steps, with the satisfying endpoint retained.
export function lowestIn(test: (v: number) => boolean, low: number, high: number): number | null {
  if (!test(high)) return null;
  if (test(low)) return low;
  for (let i = 0; i < 24; i++) {
    const mid = (low + high) / 2;
    if (test(mid)) high = mid;
    else low = mid;
  }
  return high;
}

export function highestIn(test: (v: number) => boolean, low: number, high: number): number | null {
  if (!test(low)) return null;
  if (test(high)) return high;
  for (let i = 0; i < 24; i++) {
    const mid = (low + high) / 2;
    if (test(mid)) low = mid;
    else high = mid;
  }
  return low;
}

function minContrast(color: ThemeColor, backgrounds: readonly ThemeColor[]): number {
  let min = Infinity;
  for (const background of backgrounds) min = Math.min(min, contrast(color, background));
  return min;
}

// Yellows darken toward amber. The correction peaks at 105°.
function hueAt(hue: number, offset: number): number {
  if (offset >= 0) return hue;
  const weight = Math.max(0, 1 - Math.abs(hue - 105) / 35);
  return hue - 28 * weight * Math.min(1, -offset / 0.35);
}

function fitLightness(test: (l: number) => boolean, lightness: number, direction: 1 | -1): number | null {
  return direction > 0 ? lowestIn(test, lightness, 1) : highestIn(test, 0, lightness);
}

const FIT_MARGIN = 0.05;
const ON_STRONG = 7;
const ON_MIN = 4.5;
export const FILL_REACH = 0.12;

export function fitAway(start: ThemeColor, backgrounds: readonly ThemeColor[], target: number, direction: 1 | -1): ThemeColor {
  target += FIT_MARGIN;
  if (minContrast(start, backgrounds) >= target) return start;
  const [lightness, chroma, hue] = toLch(start);
  const at = (l: number) => lch(l, chroma, hueAt(hue, l - lightness));
  const found = fitLightness((l) => minContrast(at(l), backgrounds) >= target, lightness, direction);
  if (found === null) return direction > 0 ? WHITE : BLACK;
  return at(found);
}

export function fitAccentFill(start: ThemeColor) {
  const [lightness, chroma, hue] = toLch(start);
  const darkInk = lch(0.18, Math.min(chroma * 0.3, 0.04), hue);
  const at = (l: number) => lch(l, chroma, hueAt(hue, l - lightness));
  const strong = ON_STRONG + FIT_MARGIN;
  const floor = ON_MIN + FIT_MARGIN;
  let on = contrast(start, WHITE) >= contrast(start, darkInk) ? WHITE : darkInk;
  let fill = start;

  if (contrast(start, on) < strong) {
    // Choose the nearer exit from the mid-tones, with reach capped in OKLab.
    const exit = (ink: ThemeColor, dir: 1 | -1) => {
      const found = fitLightness((l) => contrast(at(l), ink) >= strong, lightness, dir);
      const color = found === null ? null : at(found);
      const room = dir > 0 ? 1 - lightness : lightness;
      const reach = highestIn((x) => labDistance(at(lightness + dir * x), start) <= FILL_REACH, 0, room);
      return { ink, dir, color, cost: color ? labDistance(color, start) : Infinity, reach: at(lightness + dir * (reach ?? 0)) };
    };
    const lighter = exit(darkInk, 1);
    const darker = exit(WHITE, -1);
    // At an equal cost within reach, the lighter exit comes first.
    const closest = lighter.cost <= darker.cost ? lighter : darker;
    if (closest.cost <= FILL_REACH && closest.color) {
      on = closest.ink;
      fill = closest.color;
    } else {
      const useLighter = lighter.cost < darker.cost ||
        (lighter.cost === darker.cost && contrast(lighter.reach, lighter.ink) >= contrast(darker.reach, darker.ink));
      const chosen = useLighter ? lighter : darker;
      on = chosen.ink;
      fill = chosen.reach;
      if (contrast(fill, on) < floor) {
        const found = fitLightness((l) => contrast(at(l), on) >= floor, lightness, chosen.dir);
        fill = found === null ? (chosen.dir > 0 ? WHITE : BLACK) : at(found);
      }
    }
  }

  const onContrast = contrast(fill, on);
  const keep = Math.min(onContrast, strong);
  const [fillL, fillChroma, fillHue] = toLch(fill);
  const shade = (offset: number) => lch(fillL + offset, fillChroma, hueAt(fillHue, offset));
  const holds = (offset: number) => contrast(shade(offset), on) >= keep;
  const hoverStep = 0.04;
  const activeStep = 0.08;
  // Keep the direction across hover and press. If darkening cannot clear
  // the contrast target for hover, test lighter shades in both states.
  const darken = fillL - hoverStep >= 0.03 && holds(-hoverStep);
  const lighterStep = (step: number) => highestIn((x) => fillL + x <= 0.985 && holds(x), 0, step) ?? 0;
  const hover = darken ? shade(-hoverStep) : shade(lighterStep(hoverStep));
  const press = darken
    ? -(highestIn((x) => fillL - x >= 0.03 && holds(-x), hoverStep, activeStep) ?? 0)
    : lighterStep(activeStep);

  return { fill, on, hover, active: shade(press), onContrast, strong: onContrast >= ON_STRONG };
}

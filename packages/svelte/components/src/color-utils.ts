/**
 * Color conversion utilities for the ColorPicker component.
 * Internal model uses HSV for the gradient pad; HSL/RGB/hex for display and I/O.
 */

export interface RgbColor {
  r: number; // 0–255
  g: number; // 0–255
  b: number; // 0–255
}

export interface HslColor {
  h: number; // 0–360
  s: number; // 0–100
  l: number; // 0–100
}

export interface HsvColor {
  h: number; // 0–360
  s: number; // 0–100
  v: number; // 0–100
}

// ── Hex validation ──────────────────────────────────────────────

const HEX_RE = /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/;

export function isValidHex(hex: string): boolean {
  return HEX_RE.test(hex);
}

/** Normalise shorthand (#RGB → #RRGGBB) and lowercase. */
export function normalizeHex(hex: string): string {
  const raw = hex.replace("#", "").toLowerCase();

  if (raw.length === 3) {
    return `#${raw[0]}${raw[0]}${raw[1]}${raw[1]}${raw[2]}${raw[2]}`;
  }

  return `#${raw}`;
}

// ── Hex ↔ RGB ───────────────────────────────────────────────────

export function hexToRgb(hex: string): RgbColor & { a?: number } {
  const n = normalizeHex(hex).replace("#", "");
  const r = parseInt(n.slice(0, 2), 16);
  const g = parseInt(n.slice(2, 4), 16);
  const b = parseInt(n.slice(4, 6), 16);

  if (n.length === 8) {
    const a = parseInt(n.slice(6, 8), 16) / 255;
    return { r, g, b, a };
  }

  return { r, g, b };
}

export function rgbToHex(r: number, g: number, b: number, a?: number): string {
  const toHex = (v: number) =>
    Math.round(clamp(v, 0, 255))
      .toString(16)
      .padStart(2, "0");

  const base = `#${toHex(r)}${toHex(g)}${toHex(b)}`;

  if (a !== undefined && a < 1) {
    return `${base}${toHex(a * 255)}`;
  }

  return base;
}

// ── RGB ↔ HSL ───────────────────────────────────────────────────

export function rgbToHsl(r: number, g: number, b: number): HslColor {
  const rn = r / 255;
  const gn = g / 255;
  const bn = b / 255;
  const max = Math.max(rn, gn, bn);
  const min = Math.min(rn, gn, bn);
  const d = max - min;
  const l = (max + min) / 2;

  if (d === 0) {
    return { h: 0, s: 0, l: Math.round(l * 100) };
  }

  const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  let h = 0;

  if (max === rn) {
    h = ((gn - bn) / d + (gn < bn ? 6 : 0)) / 6;
  } else if (max === gn) {
    h = ((bn - rn) / d + 2) / 6;
  } else {
    h = ((rn - gn) / d + 4) / 6;
  }

  return {
    h: Math.round(h * 360),
    s: Math.round(s * 100),
    l: Math.round(l * 100),
  };
}

export function hslToRgb(h: number, s: number, l: number): RgbColor {
  const sn = s / 100;
  const ln = l / 100;

  if (sn === 0) {
    const v = Math.round(ln * 255);
    return { r: v, g: v, b: v };
  }

  const hue2rgb = (p: number, q: number, t: number): number => {
    const tt = t < 0 ? t + 1 : t > 1 ? t - 1 : t;

    if (tt < 1 / 6) return p + (q - p) * 6 * tt;
    if (tt < 1 / 2) return q;
    if (tt < 2 / 3) return p + (q - p) * (2 / 3 - tt) * 6;

    return p;
  };

  const q = ln < 0.5 ? ln * (1 + sn) : ln + sn - ln * sn;
  const p = 2 * ln - q;
  const hn = h / 360;

  return {
    r: Math.round(hue2rgb(p, q, hn + 1 / 3) * 255),
    g: Math.round(hue2rgb(p, q, hn) * 255),
    b: Math.round(hue2rgb(p, q, hn - 1 / 3) * 255),
  };
}

// ── RGB ↔ HSV ───────────────────────────────────────────────────

export function rgbToHsv(r: number, g: number, b: number): HsvColor {
  const rn = r / 255;
  const gn = g / 255;
  const bn = b / 255;
  const max = Math.max(rn, gn, bn);
  const min = Math.min(rn, gn, bn);
  const d = max - min;
  const v = max;
  const s = max === 0 ? 0 : d / max;

  if (d === 0) {
    return { h: 0, s: 0, v: Math.round(v * 100) };
  }

  let h = 0;

  if (max === rn) {
    h = ((gn - bn) / d + (gn < bn ? 6 : 0)) / 6;
  } else if (max === gn) {
    h = ((bn - rn) / d + 2) / 6;
  } else {
    h = ((rn - gn) / d + 4) / 6;
  }

  return {
    h: Math.round(h * 360),
    s: Math.round(s * 100),
    v: Math.round(v * 100),
  };
}

export function hsvToRgb(h: number, s: number, v: number): RgbColor {
  const sn = s / 100;
  const vn = v / 100;
  const c = vn * sn;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = vn - c;

  let rn = 0;
  let gn = 0;
  let bn = 0;

  if (h < 60) {
    rn = c; gn = x; bn = 0;
  } else if (h < 120) {
    rn = x; gn = c; bn = 0;
  } else if (h < 180) {
    rn = 0; gn = c; bn = x;
  } else if (h < 240) {
    rn = 0; gn = x; bn = c;
  } else if (h < 300) {
    rn = x; gn = 0; bn = c;
  } else {
    rn = c; gn = 0; bn = x;
  }

  return {
    r: Math.round((rn + m) * 255),
    g: Math.round((gn + m) * 255),
    b: Math.round((bn + m) * 255),
  };
}

// ── Composite conversions ───────────────────────────────────────

export function hexToHsv(hex: string): HsvColor {
  const { r, g, b } = hexToRgb(hex);
  return rgbToHsv(r, g, b);
}

export function hsvToHex(h: number, s: number, v: number, a?: number): string {
  const { r, g, b } = hsvToRgb(h, s, v);
  return rgbToHex(r, g, b, a);
}

export function hexToHsl(hex: string): HslColor {
  const { r, g, b } = hexToRgb(hex);
  return rgbToHsl(r, g, b);
}

export function hslToHex(h: number, s: number, l: number, a?: number): string {
  const { r, g, b } = hslToRgb(h, s, l);
  return rgbToHex(r, g, b, a);
}

export function hsvToHsl(h: number, s: number, v: number): HslColor {
  const { r, g, b } = hsvToRgb(h, s, v);
  return rgbToHsl(r, g, b);
}

export function hslToHsv(h: number, s: number, l: number): HsvColor {
  const { r, g, b } = hslToRgb(h, s, l);
  return rgbToHsv(r, g, b);
}

// ── Utilities ───────────────────────────────────────────────────

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

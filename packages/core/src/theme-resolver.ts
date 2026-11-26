import {
  alpha, BLACK, clamp, FILL_REACH, fitAccentFill, fitAway, flare, flareDistance,
  flareToLuminance, hex, lch, lchToLinear, linearLuminance, lowestIn, over,
  parseColor, polar, toLab, toLch, vector, WHITE, type ThemeColor,
} from "./theme-color";

export type { ThemeColor } from "./theme-color";
export type ThemeScheme = "light" | "dark";
/** Hex sRGB, "mono", or normalized sRGB channels. */
export type ThemeColorInput = string | readonly [number, number, number];

export interface ThemeSettings {
  scheme?: ThemeScheme;
  base?: ThemeColorInput;
  baseBlend?: number;
  accent?: ThemeColorInput;
  accentBlend?: number;
  contrast?: number;
  evenTint?: boolean;
}

const STATUS = ["success", "warning", "danger", "info"] as const;
const SYNTAX = ["comment", "keyword", "string", "literal", "type", "callable", "property", "operator", "punctuation", "invalid"] as const;

type Status = (typeof STATUS)[number];
type Syntax = (typeof SYNTAX)[number];

export type ThemeRole =
  | `layer.${"canvas" | "surface" | "raised" | "overlay"}`
  | `text.${"primary" | "secondary" | "disabled"}`
  | `fill.${"rest" | "hover" | "active"}`
  | `select.${"neutral" | "accent"}`
  | `border.${"subtle" | "strong"}`
  | `accent.${"fill" | "fill-hover" | "fill-active" | "on" | "text" | "line" | "edge"}`
  | `status.${Status}.${"fg" | "bg"}`
  | `status.danger.${"solid" | "on" | "edge"}`
  | `syntax.${Syntax}`
  | "scrim" | "shadow";

export type ThemeRoles = Readonly<Record<ThemeRole, ThemeColor>>;

export interface ThemeReport {
  clamped: { input: keyof ThemeSettings; from: string; to: string; why: string }[];
  adjusted: { role: ThemeRole; from: string; to: string; why?: string }[];
  warnings: string[];
  notes: string[];
}

export interface ResolvedTheme {
  roles: ThemeRoles;
  report: ThemeReport;
}

const SCHEME = {
  light: {
    dir: -1, limit: 0.9,
    inkL: 0.2, textL: [0.24, 0.47, 0.72], borderStrongL: 0.64, derivedL: 0.52,
    fill: [0.045, 0.07, 0.1], select: [0.1, 0.14], borderSubtle: 0.09, statusBg: 0.1, scrim: 0.32, shadow: 0.1,
  },
  dark: {
    dir: 1, limit: 0.42,
    inkL: 0.97, textL: [0.95, 0.75, 0.5], borderStrongL: 0.56, derivedL: 0.79,
    fill: [0.055, 0.085, 0.12], select: [0.13, 0.18], borderSubtle: 0.1, statusBg: 0.14, scrim: 0.55, shadow: 0.45,
  },
} as const;

// Hue in degrees, chroma scale, lightness offset toward the layers.
const ANCHORS = {
  success: [150, 1, 0], warning: [80, 1.05, 0], danger: [27, 1, 0], info: [250, 1, 0],
  keyword: [300, 1, 0], string: [145, 0.95, 0], literal: [65, 1, 0],
  type: [195, 0.85, 0], callable: [255, 1, 0], property: [345, 0.9, 0],
  invalid: [25, 1.1, 0], comment: [250, 0.3, 0.1], operator: [0, 0, -0.06],
  punctuation: [0, 0, 0.06],
} as const satisfies Record<Status | Syntax, readonly [number, number, number]>;

const STEP = 0.024;
const TOP_CHROMA = 0.105;
const DEFAULT_BASE_BLEND = 0.25;
const HIERARCHY = 0.06;
const TEXT_CHROMA = [0.4, 0.6, 0.6] as const;
const TINT_PULL = 0.5;
const EDGE_NEAR = 0.1;

function inputField(input: unknown, name: keyof ThemeSettings): unknown {
  try {
    if (input !== null && typeof input === "object") return Reflect.get(input, name);
  } catch {
    // An unreadable property has the same default as a missing one.
  }
  return undefined;
}

function describeInput(value: unknown): string {
  try {
    return String(value);
  } catch {
    return "[unprintable]";
  }
}

function numberInput(value: unknown, fallback: number, min: number, max: number, input: keyof ThemeSettings, report: ThemeReport): number {
  if (value === undefined || value === null || value === "") return fallback;
  let number = NaN;
  try {
    if (typeof value !== "bigint") number = Number(value);
  } catch {
    // Failed coercions are reported as invalid numbers.
  }
  if (!Number.isFinite(number)) {
    report.clamped.push({ input, from: describeInput(value), to: String(fallback), why: "not a number" });
    return fallback;
  }
  const clamped = clamp(number, min, max);
  if (clamped !== number) {
    report.clamped.push({ input, from: String(number), to: String(clamped), why: `outside ${min} to ${max}` });
  }
  return clamped;
}

function colorInput(value: unknown, input: "base" | "accent", report: ThemeReport): ThemeColor | null {
  if (value === undefined || value === null || value === "mono") return null;
  try {
    const color = parseColor(value);
    if (color) return color;
  } catch {
    // Treat unreadable arrays like other invalid colours.
  }
  report.clamped.push({ input, from: describeInput(value), to: "mono", why: "not a colour" });
  return null;
}

function slopeMax(scheme: ThemeScheme): number {
  const lightness = Math.cbrt(flareToLuminance(SCHEME[scheme].limit));
  return TOP_CHROMA / (scheme === "dark" ? lightness : 1 - lightness);
}

type Slope = readonly [number, number];

function tintSlope(color: ThemeColor | null, scheme: ThemeScheme, even: boolean): Slope {
  if (!color) return [0, 0];
  const [lightness, a, b] = toLab(color);
  if (Math.hypot(a, b) < 1e-4) return [0, 0];
  // Even tint uses the dark slope, rescaled to the current scheme's cap.
  const from = even ? "dark" : scheme;
  const depth = Math.max(from === "dark" ? lightness : 1 - lightness, 1e-3);
  const ratio = slopeMax(scheme) / slopeMax(from);
  return [a / depth * ratio, b / depth * ratio];
}

function capSlope(slope: Slope, cap: number): Slope {
  const length = Math.hypot(...slope);
  return length > cap ? [slope[0] * cap / length, slope[1] * cap / length] : slope;
}

function weightedSlope(slope: Slope, cap: number, blend: number): Slope {
  const [a, b] = capSlope(slope, cap);
  const weight = blend * (2 - blend);
  return [a * weight, b * weight];
}

function layerRamp(scheme: ThemeScheme, depth: number, step: number, slope: number, hue: number) {
  const dark = scheme === "dark";
  const deep = dark ? flare(0) + step : flare(1) - step;
  const dim = dark ? SCHEME.dark.limit - 2 * step : SCHEME.light.limit + step;
  const surfaceFlare = deep + (dim - deep) * depth;
  const chromaAt = (lightness: number) => slope * (dark ? lightness : 1 - lightness);
  const yAt = (lightness: number) => linearLuminance(lchToLinear(lightness, chromaAt(lightness), hue));
  let clipped = false;

  const layerAt = (offset: number) => {
    const y = flareToLuminance(surfaceFlare + offset * step);
    if (y <= 1e-9) return { color: BLACK, lightness: 0 };
    if (y >= 1) return { color: WHITE, lightness: 1 };
    // Search with unquantised luminance, then round the emitted colour.
    const lightness = lowestIn((l) => yAt(l) >= y, 0, 1) ?? 1;
    const chroma = chromaAt(lightness);
    const color = lch(lightness, chroma, hue);
    clipped ||= chroma - toLch(color)[1] > 0.01;
    return { color, lightness };
  };
  const { color: canvas } = layerAt(-1);
  const { color: surface, lightness } = layerAt(0);
  const { color: raised } = layerAt(1);
  const { color: overlay } = layerAt(2);
  return { layers: [canvas, surface, raised, overlay] as const, chroma: chromaAt(lightness), clipped };
}

/** Preview theme resolver. Pure and independent of the current token build. */
export function resolveTheme(input?: ThemeSettings): ResolvedTheme;
export function resolveTheme(input?: unknown): ResolvedTheme {
  const report: ThemeReport = { clamped: [], adjusted: [], warnings: [], notes: [] };
  const schemeInput = inputField(input, "scheme");
  const scheme = schemeInput === "light" ? "light" : "dark";
  if (schemeInput !== undefined && schemeInput !== null && schemeInput !== "light" && schemeInput !== "dark") {
    report.clamped.push({ input: "scheme", from: describeInput(schemeInput), to: "dark", why: "not light or dark" });
  }
  const sc = SCHEME[scheme];
  const dir = sc.dir;
  const contrast = numberInput(inputField(input, "contrast"), 1, 0.5, 2, "contrast", report);
  const baseBlend = numberInput(inputField(input, "baseBlend"), DEFAULT_BASE_BLEND, 0, 1, "baseBlend", report);
  const accentBlend = numberInput(inputField(input, "accentBlend"), 0, 0, 1, "accentBlend", report);
  const base = colorInput(inputField(input, "base"), "base", report);
  const accent = colorInput(inputField(input, "accent"), "accent", report);
  const evenInput = inputField(input, "evenTint");
  const even = typeof evenInput === "boolean" ? evenInput : true;
  if (evenInput !== undefined && evenInput !== null && typeof evenInput !== "boolean") {
    report.clamped.push({ input: "evenTint", from: describeInput(evenInput), to: "true", why: "not true or false" });
  }

  // Both tint sources are weighted independently, then capped again as a sum.
  const cap = slopeMax(scheme);
  const baseSlope = tintSlope(base, scheme, even);
  const baseLength = Math.hypot(...baseSlope);
  if (baseLength > cap * 1.01) {
    report.clamped.push({ input: "base", from: `slope ${baseLength.toFixed(3)}`, to: cap.toFixed(3), why: `too saturated for a ${scheme} ground, muted` });
  }
  const [ba, bb] = weightedSlope(baseSlope, cap, baseBlend);
  const [aa, ab] = weightedSlope(tintSlope(accent, scheme, even), cap, accentBlend);
  const sum: Slope = [ba + aa, bb + ab];
  const [slope, hue] = polar(...capSlope(sum, cap));
  const baseTint = Math.hypot(ba, bb);
  const accentTint = Math.hypot(aa, ab);
  if (baseTint > 0.1 * cap && accentTint > 0.1 * cap && Math.hypot(...sum) < 0.5 * Math.max(baseTint, accentTint)) {
    report.notes.push("Base and accent hues cancel, so the neutrals lose most of their tint.");
  }

  const { layers, chroma, clipped } = layerRamp(scheme, Math.max(baseBlend, accentBlend), STEP * contrast, slope, hue);
  const [canvas, surface, raised, overlay] = layers;
  const [tintA, tintB] = vector(chroma, hue);
  if (clipped) report.notes.push("Some layers reach the edge of sRGB, so they carry less tint than the line asks for.");
  const dangerHue = ANCHORS.danger[0];
  const dangerHueDistance = Math.abs(((hue - dangerHue) % 360 + 540) % 360 - 180);
  if (chroma > 0.04 && dangerHueDistance < 30) {
    report.notes.push("The ground is close to the danger hue, so danger colours stand out less from it.");
  }

  const ink = lch(sc.inkL, chroma * 0.6, hue);
  const fillRest = alpha(ink, sc.fill[0]);
  const fillHover = alpha(ink, sc.fill[1]);
  const fillActive = alpha(ink, sc.fill[2]);
  const selectNeutral = alpha(ink, sc.select[0]);
  const picked = accent ?? ink;
  const accentLine = fitAway(picked, layers, 3, dir);
  const accentText = fitAway(picked, layers, 4.5, dir);
  if (accent && accentLine !== picked) report.adjusted.push({ role: "accent.line", from: hex(picked), to: hex(accentLine) });
  if (accent && accentText !== picked) report.adjusted.push({ role: "accent.text", from: hex(picked), to: hex(accentText) });

  const fill = fitAccentFill(picked);
  if (accent && fill.fill !== picked) {
    const inkDirection = fill.on[0] < 128 ? "lighter, so dark text" : "darker, so white text";
    const aim = fill.strong ? " reaches 7:1" : " gets as close to 7:1 as allowed";
    report.adjusted.push({ role: "accent.fill", from: hex(picked), to: hex(fill.fill), why: inkDirection + aim });
  }
  if (!fill.strong) {
    report.notes.push(`Text on the accent fill reaches ${fill.onContrast.toFixed(2)}:1, short of 7:1: reaching it would move the fill more than ΔE ${Math.round(FILL_REACH * 100)} from the picked colour.`);
  }
  const edge = layers.some((layer) => flareDistance(fill.fill, layer) < EDGE_NEAR);
  if (edge) report.notes.push("The accent fill is close to a layer, so filled controls get a hairline edge in the accent line colour.");
  const selectAccent = alpha(accentText, sc.select[1]);

  const vivid = clamp(accent ? toLch(picked)[1] : 0, 0.06, 0.15);
  const derived = (key: Status | Syntax) => {
    const [anchor, scale, offset] = ANCHORS[key];
    const [a, b] = vector(vivid * scale, anchor);
    const [c, h] = polar(a + TINT_PULL * tintA, b + TINT_PULL * tintB);
    return lch(sc.derivedL - dir * offset, c, h);
  };
  const status = (key: Status) => {
    const color = derived(key);
    return { color, background: alpha(color, sc.statusBg) };
  };
  const success = status("success");
  const warning = status("warning");
  const danger = status("danger");
  const info = status("info");

  // Fit text last, against the actual 8-bit composites on each layer.
  const tints = [fillRest, fillHover, fillActive, selectNeutral, selectAccent, success.background, warning.background, danger.background, info.background];
  const allBackgrounds = [...layers, ...layers.flatMap((layer) => tints.map((tint) => over(tint, layer)))];
  const lowerLayers = [canvas, surface, raised];
  const textStart = (i: 0 | 1 | 2) => lch(sc.textL[i], chroma * TEXT_CHROMA[i], hue);
  let primary = fitAway(fitAway(textStart(0), lowerLayers, 7, dir), allBackgrounds, 4.5, dir);
  const secondary = fitAway(textStart(1), allBackgrounds, 4.5, dir);
  const secondaryL = toLch(secondary)[0];
  if (dir * (toLch(primary)[0] - secondaryL) < HIERARCHY) {
    const liftedL = clamp(secondaryL + dir * (HIERARCHY + 0.004), 0, 1);
    if (dir * (liftedL - secondaryL) < HIERARCHY) {
      report.warnings.push(`No room for a text hierarchy: secondary text is too close to ${dir > 0 ? "white" : "black"}.`);
    }
    const lifted = lch(liftedL, chroma * TEXT_CHROMA[0], hue);
    report.adjusted.push({ role: "text.primary", from: hex(primary), to: hex(lifted), why: "hierarchy" });
    primary = lifted;
  }

  const statusText = ({ color, background }: ReturnType<typeof status>) =>
    fitAway(color, [...layers, ...layers.map((layer) => over(background, layer))], 4.5, dir);
  const syntaxText = (key: Syntax) => fitAway(derived(key), layers, 4.5, dir);
  const dangerText = statusText(danger);
  // The solid danger fill uses its anchor directly. Its foreground and tint
  // retain the same half pull as the other status colours.
  const dangerFill = fitAccentFill(lch(0.58, 0.19, dangerHue));
  const dangerEdge = layers.some((layer) => flareDistance(dangerFill.fill, layer) < EDGE_NEAR);
  if (dangerEdge) report.notes.push("The danger fill is close to a layer, so the danger button gets a hairline edge in the danger text colour.");

  return {
    roles: {
      "layer.canvas": canvas, "layer.surface": surface, "layer.raised": raised, "layer.overlay": overlay,
      "text.primary": primary, "text.secondary": secondary,
      "text.disabled": fitAway(textStart(2), lowerLayers, 3, dir),
      "fill.rest": fillRest, "fill.hover": fillHover, "fill.active": fillActive,
      "select.neutral": selectNeutral, "select.accent": selectAccent,
      "border.subtle": alpha(ink, sc.borderSubtle),
      "border.strong": fitAway(lch(sc.borderStrongL, chroma * 0.8, hue), layers, 3, dir),
      "accent.fill": fill.fill, "accent.fill-hover": fill.hover, "accent.fill-active": fill.active,
      "accent.on": fill.on, "accent.text": accentText, "accent.line": accentLine,
      "accent.edge": alpha(accentLine, edge ? 1 : 0),
      "status.success.fg": statusText(success), "status.success.bg": success.background,
      "status.warning.fg": statusText(warning), "status.warning.bg": warning.background,
      "status.danger.fg": dangerText, "status.danger.bg": danger.background,
      "status.info.fg": statusText(info), "status.info.bg": info.background,
      "status.danger.solid": dangerFill.fill, "status.danger.on": dangerFill.on,
      "status.danger.edge": alpha(dangerText, dangerEdge ? 1 : 0),
      "syntax.comment": syntaxText("comment"), "syntax.keyword": syntaxText("keyword"),
      "syntax.string": syntaxText("string"), "syntax.literal": syntaxText("literal"),
      "syntax.type": syntaxText("type"), "syntax.callable": syntaxText("callable"),
      "syntax.property": syntaxText("property"), "syntax.operator": syntaxText("operator"),
      "syntax.punctuation": syntaxText("punctuation"), "syntax.invalid": syntaxText("invalid"),
      scrim: alpha(dir > 0 ? BLACK : lch(0.18, chroma, hue), sc.scrim),
      shadow: alpha(dir > 0 ? BLACK : lch(0.22, chroma, hue), sc.shadow),
    },
    report,
  };
}

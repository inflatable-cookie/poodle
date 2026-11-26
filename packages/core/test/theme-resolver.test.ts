import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { runInNewContext } from "node:vm";

import { resolveTheme, themePresets, type ThemeRole, type ThemeSettings } from "../src/index";
import { contrast, hex, labDistance, over, toLch } from "../src/theme-color";
import { fixedRandom, presetSettings, randomTheme, schemes, themeCases, tintModes } from "./theme-cases";

import vectors from "../../contracts/headless/vectors/theme-resolver.json";

const layers = ["layer.canvas", "layer.surface", "layer.raised", "layer.overlay"] as const;
const tints = [
  "fill.rest", "fill.hover", "fill.active", "select.neutral", "select.accent",
  "status.success.bg", "status.warning.bg", "status.danger.bg", "status.info.bg",
] as const;
const neutral = [
  ...layers, "text.disabled", "fill.rest", "fill.hover", "fill.active",
  "select.neutral", "border.subtle", "border.strong", "scrim", "shadow",
] as const satisfies readonly ThemeRole[];

test("matches every generated role and report in the shared theme vectors", () => {
  const cases = themeCases();
  expect(vectors.vectors).toHaveLength(cases.length);
  const entries = new Map(vectors.vectors.map((vector) => [vector.id, vector]));
  expect(entries.size).toBe(cases.length);
  for (const { id, settings } of cases) {
    const actual = resolveTheme(settings);
    const vector = entries.get(id);
    if (!vector) throw new Error(`Missing theme vector ${id}`);
    expect<unknown>(vector.settings, id).toEqual(settings);
    expect(Object.keys(actual.roles).sort(), id).toEqual(vectors.roles);
    const colors = Object.entries(actual.roles).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0);
    expect(colors.map(([, color]) => hex(color)), id).toEqual(vector.colors);
    expect<unknown>(actual.report, id).toEqual(vector.report);
  }
}, 30000);

test("matches the checked-in prototype for every preset in both schemes and tint modes", () => {
  const prototype = readFileSync(new URL("../../../docs/knowledge/research/theming-model/index.html", import.meta.url), "utf8");
  const presetScript = prototype.match(/<script id="presets">([\s\S]*?)<\/script>/)?.[1];
  const resolverScript = prototype.match(/<script id="resolve-theme">([\s\S]*?)<\/script>/)?.[1];
  if (!presetScript || !resolverScript) throw new Error("Missing prototype scripts");
  // Evaluate only the two static, DOM-free scripts from the checked-in study.
  const original: {
    presets: (ThemeSettings & { id: string; name: string })[][],
    colors: Record<string, string>[],
  } = runInNewContext(`
    ${presetScript}
    ${resolverScript}
    (() => {
      const presets = [PoodlePresets.PRESETS, PoodlePresets.PRESETS_EVEN];
      const colors = presets.flatMap(list => list.flatMap(preset =>
        ["light", "dark"].map(scheme =>
          Object.fromEntries(Object.entries(PoodleTheme.resolveTheme({ ...preset, scheme }).roles)
            .map(([role, color]) => [role, PoodleTheme.hex(color)])))));
      return { presets, colors };
    })()
  `);
  const dangerFillRoles = new Set(["status.danger.solid", "status.danger.on", "status.danger.edge"]);
  const withoutDangerFill = (roles: Record<string, string>) =>
    Object.fromEntries(Object.entries(roles).filter(([role]) => !dangerFillRoles.has(role)));
  expect(original.colors).toHaveLength(52);
  let index = 0;
  for (const [mode, evenTint] of [false, true].entries()) {
    expect(original.presets[mode]).toEqual(themePresets.map((preset) => ({
      id: preset[0], name: preset[1], ...presetSettings(preset, preset[2], evenTint),
    })));
    for (const preset of themePresets) {
      for (const scheme of schemes) {
        const before = original.colors[index++];
        if (!before) throw new Error(`Missing prototype case for ${preset[0]}`);
        const { roles } = resolveTheme(presetSettings(preset, scheme, evenTint));
        const after = Object.fromEntries(Object.entries(roles).map(([role, color]) => [role, hex(color)]));
        expect(withoutDangerFill(after), `${preset[0]} ${scheme} even=${evenTint}`).toEqual(withoutDangerFill(before));
      }
    }
  }
});

test("the danger fill stays within 2° of its 27° anchor on yellow, green and brown grounds", () => {
  for (const scheme of schemes) {
    for (const settings of [true, false].flatMap((evenTint) =>
      ["#ffd400", "#30b446", "#9c5028"].map((color) => ({
        scheme, evenTint, base: color, baseBlend: 1, accent: color, accentBlend: 1,
      })))) {
      const { roles } = resolveTheme(settings);
      const fill = roles["status.danger.solid"];
      const hue = toLch(fill)[2];
      expect(Math.abs((hue - 27 + 540) % 360 - 180)).toBeLessThanOrEqual(2);
      expect(contrast(roles["status.danger.on"], fill)).toBeGreaterThanOrEqual(7);
      if (roles["status.danger.edge"][3]) {
        expect(roles["status.danger.edge"]).toEqual(roles["status.danger.fg"]);
      }
    }
  }
});

test("zero blends remove their hue, and swapping pairs preserves neutrals", () => {
  const random = fixedRandom(0xd1a1);
  for (let i = 0; i < 192; i++) {
    const settings = randomTheme(random);
    const withoutBase = resolveTheme({ ...settings, baseBlend: 0 }).roles;
    expect(withoutBase).toEqual(resolveTheme({ ...settings, base: "mono", baseBlend: 0 }).roles);
    const withoutAccent = resolveTheme({ ...settings, accentBlend: 0 }).roles;
    const monoAccent = resolveTheme({ ...settings, accent: "mono", accentBlend: 0 }).roles;
    const monoGround = resolveTheme({ ...settings, base: "mono", accentBlend: 0 }).roles;
    const before = resolveTheme(settings).roles;
    const swapped = resolveTheme({
      ...settings, base: settings.accent, baseBlend: settings.accentBlend,
      accent: settings.base, accentBlend: settings.baseBlend,
    }).roles;
    for (const role of neutral) {
      expect(withoutAccent[role], role).toEqual(monoAccent[role]);
      const [r, g, b] = monoGround[role];
      expect([r, g, b], role).toEqual([r, r, r]);
      expect(swapped[role], role).toEqual(before[role]);
    }
    // Primary and secondary text can move in lightness to clear accent tints.
    for (const role of ["text.primary", "text.secondary"] as const) {
      const [r, g, b] = monoGround[role];
      expect([r, g, b], role).toEqual([r, r, r]);
    }
  }
}, 30000);

test("both blends add tint gradually on leaving zero", () => {
  for (const scheme of schemes) {
    for (const evenTint of tintModes) {
      for (const color of ["#e5487f", "#2f6fed", "#ffd400", "#4dc98a", "#967c4c"]) {
        for (const source of ["base", "accent"] as const) {
          const blendKey = source === "base" ? "baseBlend" : "accentBlend";
          const settings = { scheme, evenTint, baseBlend: 0, accentBlend: 0 };
          const tintAt = (blend: number) => {
            const mono = resolveTheme({ ...settings, [blendKey]: blend }).roles["layer.surface"];
            const tinted = resolveTheme({ ...settings, [source]: color, [blendKey]: blend }).roles["layer.surface"];
            return labDistance(mono, tinted);
          };
          expect(tintAt(0)).toBe(0);
          // Allow one 8-bit step at the start, then require visible growth.
          expect(tintAt(0.01)).toBeLessThanOrEqual(0.007);
          expect(tintAt(0.25)).toBeGreaterThan(tintAt(0.01));
          expect(tintAt(0.5)).toBeGreaterThan(tintAt(0.25));
        }
      }
    }
  }
});

test("fitted text and accent lines clear their contrast targets after rounding", () => {
  for (const { id, settings } of themeCases()) {
    const { roles } = resolveTheme(settings);
    for (const [i, layer] of layers.entries()) {
      expect(contrast(roles["text.primary"], roles[layer]), id).toBeGreaterThanOrEqual(i < 3 ? 7 : 4.5);
      expect(contrast(roles["text.secondary"], roles[layer]), id).toBeGreaterThanOrEqual(4.5);
      expect(contrast(roles["accent.text"], roles[layer]), id).toBeGreaterThanOrEqual(4.5);
      expect(contrast(roles["accent.line"], roles[layer]), id).toBeGreaterThanOrEqual(3);
      for (const tint of tints) {
        const background = over(roles[tint], roles[layer]);
        expect(contrast(roles["text.primary"], background), id).toBeGreaterThanOrEqual(4.5);
        expect(contrast(roles["text.secondary"], background), id).toBeGreaterThanOrEqual(4.5);
      }
    }
    for (const role of ["accent.fill", "accent.fill-hover", "accent.fill-active"] as const) {
      expect(contrast(roles["accent.on"], roles[role]), id).toBeGreaterThanOrEqual(4.5);
    }
  }
}, 30000);

test("garbage settings always resolve to all 44 roles with valid RGBA bytes", () => {
  const random = fixedRandom(0x382);
  const junk = [
    undefined, null, NaN, Infinity, -1, 0.5, 3, "", "dark", "light", "#", "#12",
    "#abc", "#zzzzzz", "#12345678", "red", [1, 2], [0.5, 0.5, 0.5], [NaN, 0, 0],
    {}, true, false, "mono", Symbol("junk"), 1n, [, 0, 0], Object.create(null),
    new Proxy({}, { get() { throw new Error("unreadable"); } }),
  ];
  const pick = () => junk[Math.floor(random() * junk.length)];
  for (let i = 0; i < 256; i++) {
    const input = random() < 0.1 ? pick() : {
      scheme: pick(), base: pick(), baseBlend: pick(), accent: pick(),
      accentBlend: pick(), contrast: pick(), evenTint: pick(),
    };
    const actual: ReturnType<typeof resolveTheme> = Reflect.apply(resolveTheme, undefined, [input]);
    expect(Object.keys(actual.roles).sort()).toEqual(vectors.roles);
    expect(Object.values(actual.roles)).toHaveLength(44);
    for (const channels of Object.values(actual.roles)) {
      expect(channels).toHaveLength(4);
      expect(channels.every((c) => Number.isInteger(c) && c >= 0 && c <= 255)).toBe(true);
    }
  }
});

test("omitted and null settings resolve to defaults without spurious clamps", () => {
  const baseline = resolveTheme({ scheme: "dark", evenTint: true });
  const omitted = resolveTheme({});
  const bare = resolveTheme();
  const nulled = resolveTheme({
    scheme: null,
    base: null,
    baseBlend: null,
    accent: null,
    accentBlend: null,
    contrast: null,
    evenTint: null,
  } as unknown as ThemeSettings);

  for (const actual of [omitted, bare, nulled]) {
    expect(actual.roles).toEqual(baseline.roles);
    expect(actual.report.clamped).toEqual([]);
  }
});

import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";

import { previewThemeCssProperties, resolveTheme, themePresets, type ThemeSettings } from "../src/index";
import { cssVars } from "../src/tokens/generated/ts/index";
import { presetSettings } from "./theme-cases";

function* colorPaths(value: unknown, path = ""): Generator<string> {
  if (value === null || typeof value !== "object") return;
  if ("$type" in value) {
    if (value.$type === "color") yield path;
    return;
  }
  for (const [key, child] of Object.entries(value)) {
    yield* colorPaths(child, path ? `${path}.${key}` : key);
  }
}

test("covers the authored theme colours and every semantic colour fallback", () => {
  const properties = previewThemeCssProperties(resolveTheme({}).roles);
  const themes = new URL("../../tokens/schema/modes/themes/", import.meta.url);
  const files = readdirSync(themes).filter((file) => file.endsWith(".json"));
  expect(files.length).toBeGreaterThan(0);
  for (const file of files) {
    const source: unknown = JSON.parse(readFileSync(new URL(file, themes), "utf8"));
    for (const path of colorPaths(source)) {
      expect(Object.hasOwn(properties, `--poodle-${path.replaceAll(".", "-")}`), `${file}: ${path}`).toBe(true);
    }
  }

  const semanticColors = Object.entries(cssVars)
    .filter(([path]) => path.startsWith("color."))
    .map(([, property]) => property);
  expect(Object.keys(properties).sort()).toEqual(semanticColors.sort());
});

test("emits standalone CSS hex colours for all presets and contrast extremes", () => {
  const settings: ThemeSettings[] = [
    ...themePresets.flatMap((preset) => [presetSettings(preset, preset[2], true), presetSettings(preset, preset[2], false)]),
    { scheme: "dark", base: "mono", baseBlend: 0, accent: "#2f6fed", accentBlend: 0.5, contrast: 2 },
    { scheme: "light", base: "#967c4c", baseBlend: 0.45, accent: "#ffd400", contrast: 0.5 },
  ];
  for (const input of settings) {
    const properties = previewThemeCssProperties(resolveTheme(input).roles);
    for (const [property, color] of Object.entries(properties)) {
      expect(color, property).toMatch(/^#[0-9a-f]{6}([0-9a-f]{2})?$/);
    }
  }
});

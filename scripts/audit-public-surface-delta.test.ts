/**
 * g18.027 planted-negative coverage for `audit-public-surface-delta.ts`.
 *
 * The freeze report claims that removed exports, removed package entries,
 * removed props, removed recipe hooks and changed Rust signatures are
 * classified as breaking. These tests plant each class in a fixture and prove
 * the classifier reports it, so a later tool regression cannot silently turn a
 * break into an unreported row.
 */
import { expect, test } from "bun:test";

import {
  classify,
  diffInventory,
  iconIdentifierInventory,
  packageEntryInventory,
  parseCustomProperties,
  parseExportNames,
  parseManifest,
  parseRustPublicItems,
  parseSvelteProps,
  recipeHookInventory,
  summarize,
  type DeltaRow,
} from "./audit-public-surface-delta.ts";

const inventory = (entries: string[]): Record<string, string> =>
  Object.fromEntries(entries.map((entry) => [entry, entry]));

test("a removed package entry subpath is breaking", () => {
  const before = packageEntryInventory(
    parseManifest(
      JSON.stringify({
        exports: { ".": "./dist/index.js", "./editor": "./dist/editor.js" },
      }),
    ),
  );
  const after = packageEntryInventory(
    parseManifest(JSON.stringify({ exports: { ".": "./dist/index.js" } })),
  );
  const rows = diffInventory("package-entry", "pkg", before, after);
  expect(rows).toHaveLength(1);
  expect(rows[0]).toMatchObject({ name: "./editor", classification: "breaking" });
});

test("an added package entry subpath is additive", () => {
  const before = packageEntryInventory(
    parseManifest(JSON.stringify({ exports: { ".": "./dist/index.js" } })),
  );
  const after = packageEntryInventory(
    parseManifest(
      JSON.stringify({
        exports: { ".": "./dist/index.js", "./rich-text": "./dist/rich-text.js" },
      }),
    ),
  );
  expect(diffInventory("package-entry", "pkg", before, after)[0].classification).toBe("additive");
});

test("a removed root export is breaking and an added one is additive", () => {
  const before = inventory(["value sliderFallbackText", "type SliderAppearance"]);
  const after = inventory(["type SliderVariant"]);
  const rows = diffInventory("root-export", "@scope/core", before, after);
  expect(rows.filter((row) => row.classification === "breaking")).toHaveLength(2);
  expect(rows.find((row) => row.name === "type SliderVariant")?.classification).toBe("additive");
});

test("a changed Svelte prop default is breaking and an added prop is additive", () => {
  const before = { "Slider.svelte.variant": 'variant?: SliderVariant = "standard"' };
  const after = {
    "Slider.svelte.variant": 'variant?: SliderVariant = "block"',
    "Slider.svelte.appearance": "appearance?: SliderAppearance",
  };
  const rows = diffInventory("svelte-prop", "@scope/svelte", before, after);
  expect(rows.find((row) => row.name === "Slider.svelte.variant")?.classification).toBe("breaking");
  expect(rows.find((row) => row.name === "Slider.svelte.appearance")?.classification).toBe(
    "additive",
  );
});

test("a removed public recipe hook is breaking while a removed internal style var is not", () => {
  const hookRows = diffInventory(
    "css-recipe-hook",
    "@scope/core/styles",
    inventory(["--poodle-recipe-slider-block-fallback-text"]),
    {},
  );
  expect(hookRows[0].classification).toBe("breaking");

  const internalRows = diffInventory(
    "css-custom-property",
    "@scope/core/styles",
    inventory(["--poodle-slider-block-hit"]),
    {},
  );
  expect(internalRows[0].classification).toBe("internal-only");
});

test("a removed token custom property is breaking", () => {
  const rows = diffInventory(
    "css-custom-property",
    "packages/core/src/tokens/generated/css/poodle-tokens.css",
    inventory(["--poodle-color-syntax-keyword"]),
    {},
  );
  expect(rows[0].classification).toBe("breaking");
});

test("removed and reshaped Rust public items are breaking; added items are additive", () => {
  const removed = diffInventory(
    "rust-item",
    "pkg",
    inventory(["fn slider_fallback_text"]),
    {},
  );
  expect(removed[0].classification).toBe("breaking");

  const changed = diffInventory(
    "rust-item",
    "pkg",
    { "fn resolved_visible_text": "(value: f64, explicit: Option<&str>)" },
    { "fn resolved_visible_text": "(value: f64, min: f64, step: f64, explicit: Option<&str>)" },
  );
  expect(changed[0].classification).toBe("breaking");

  const reexport = diffInventory(
    "rust-item",
    "pkg",
    { "lib.rs::use slider": "slider::{ SliderAppearance, SliderSpec }" },
    { "lib.rs::use slider": "slider::{ SliderSpec }" },
  );
  expect(reexport[0].classification).toBe("breaking");

  const additive = diffInventory(
    "rust-item",
    "pkg",
    { "lib.rs::use slider": "slider::{ SliderSpec }" },
    { "lib.rs::use slider": "slider::{ SliderSpec, SliderVariant }" },
  );
  expect(additive[0].classification).toBe("additive");
});

test("classify covers every surface family", () => {
  const surfaces: DeltaRow["surface"][] = [
    "package-version",
    "package-entry",
    "package-dependency",
    "package-peer-dependency",
    "root-export",
    "subpath-export",
    "svelte-prop",
    "react-prop",
    "icon-identifier",
    "css-custom-property",
    "css-recipe-hook",
    "rust-item",
  ];
  for (const surface of surfaces) {
    expect(classify(surface, "scope", "name", "before", null).classification).toBeDefined();
    expect(classify(surface, "scope", "name", "before", "after").classification).toBeDefined();
    expect(classify(surface, "scope", "name", null, "after").classification).toBeDefined();
  }
});

test("parseExportNames reads blocks, default re-exports and direct declarations", () => {
  const names = parseExportNames(`
    export { default as Slider } from "./Slider.svelte";
    export type { SliderDirection, type SliderVariant } from "@scope/core";
    export type SliderAppearance = "track" | "block";
    export const SLIDER_MAX = 100;
    export function layoutSliderBlock(): void {}
  `);
  expect(names).toContain("value Slider");
  expect(names).toContain("type SliderDirection");
  expect(names).toContain("type SliderAppearance");
  expect(names).toContain("value SLIDER_MAX");
  expect(names).toContain("value layoutSliderBlock");
});

test("parseSvelteProps extracts declarations, defaults and $bindable", () => {
  const props = parseSvelteProps(`
    <script lang="ts">
      interface Props {
        value?: number;
        visibleLabel?: string | null;
      }
      let {
        value = $bindable(0),
        visibleLabel = null,
      }: Props = $props();
    </script>
  `);
  expect(props.find((prop) => prop.name === "value")?.default).toBe("0 (bindable)");
  expect(props.find((prop) => prop.name === "visibleLabel")?.default).toBe("null");
});

test("parseCustomProperties and recipeHookInventory separate declared values from hook seams", () => {
  const css = `
    --poodle-color-accent-base: #4f46e5;
    .x { color: var(--poodle-recipe-slider-track-fill, var(--poodle-color-accent-base)); }
  `;
  expect(parseCustomProperties(css)).toEqual({ "--poodle-color-accent-base": "#4f46e5" });
  expect(Object.keys(recipeHookInventory(css))).toEqual(["--poodle-recipe-slider-track-fill"]);
});

test("iconIdentifierInventory reads registry keys", () => {
  const source = `
    export const defaultLucideIconSet = {
      "between-horizontal-start": betweenHorizontalStart,
      "table": table,
    };
  `;
  expect(Object.keys(iconIdentifierInventory(source)).sort()).toEqual([
    "between-horizontal-start",
    "table",
  ]);
});

test("parseRustPublicItems reads struct fields, enum variants and fn signatures", () => {
  const source = `
    pub struct SliderSpec {
      pub value: f64,
      pub appearance: SliderAppearance,
    }

    pub enum SliderVariant {
      Standard,
      Embedded,
    }

    pub fn layout_slider_block(capsule_span: f32, selected_norm: f32) -> SliderBlockLayout {
  `;
  const items = parseRustPublicItems(source);
  expect(items["struct SliderSpec"]).toContain("appearance: SliderAppearance");
  expect(items["enum SliderVariant"]).toContain("Standard");
  expect(items["fn layout_slider_block"]).toContain("selected_norm: f32");
});

test("a new file present only on the head side is reported as additive", () => {
  const rows = diffInventory("css-custom-property", "styles", {}, inventory(["--poodle-new"]));
  expect(rows[0].classification).toBe("additive");
});

test("summarize counts every classification", () => {
  const rows: DeltaRow[] = [
    classify("root-export", "scope", "a", "a", null),
    classify("root-export", "scope", "b", null, "b"),
    classify("css-custom-property", "scope", "c", "c", null),
  ];
  expect(summarize(rows)).toEqual({ additive: 1, behavioral: 0, breaking: 1, "internal-only": 1 });
});

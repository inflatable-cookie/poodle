import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

/**
 * Source-bound proof for the g18.026 Slider-family foundation.
 *
 * The private rendering foundation (`slider-family.css`) is the only place the
 * shared block mechanics live. Both public component sheets import it and keep
 * only genuine single/range modifiers. This test fails if a Slider repair is
 * copied back into a component sheet instead of landing on the foundation.
 */

const stylesDir = join(import.meta.dir, "..", "src", "styles");
const read = (name: string): string => readFileSync(join(stylesDir, name), "utf8");

const family = read("slider-family.css");
const slider = read("slider.css");
const range = read("range-slider.css");

describe("slider-family style foundation", () => {
  test("both public sheets consume the one private foundation", () => {
    expect(slider.startsWith("/*")).toBe(true);
    expect(range.startsWith("/*")).toBe(true);
    expect(slider).toContain('@import "./slider-family.css";');
    expect(range).toContain('@import "./slider-family.css";');
  });

  test("the shared size ladder lives only on the foundation", () => {
    for (const [size, rem] of [
      ["xs", "1.5rem"],
      ["sm", "1.75rem"],
      ["md", "2.25rem"],
      ["lg", "2.75rem"],
      ["xl", "3.25rem"],
    ] as const) {
      expect(family).toContain(
        `:is(.poodle-slider, .poodle-range-slider)[data-variant="block"][data-size="${size}"] { --poodle-slider-family-block-height: ${rem}; }`,
      );
      expect(slider).not.toContain(`--poodle-slider-family-block-height: ${rem}`);
      expect(range).not.toContain(`--poodle-slider-family-block-height: ${rem}`);
    }
    expect(slider).not.toContain("--poodle-range-");
  });

  test("the shared capsule/track/inline/hit/handle rules are only on the foundation", () => {
    for (const shared of [
      ":is(.poodle-slider__capsule, .poodle-range-slider__capsule)",
      ":is(.poodle-slider__inline, .poodle-range-slider__inline)",
      ":is(.poodle-slider__hit, .poodle-range-slider__hit)",
      ":is(.poodle-slider__thumb, .poodle-range-slider__thumb)",
    ]) {
      expect(family).toContain(shared);
      expect(slider).not.toContain(shared);
      expect(range).not.toContain(shared);
    }
    // The handle primitive is defined once, with no private thickness/inset
    // ladder in either component sheet.
    expect(family).toContain("--poodle-slider-family-marker-inset: 0.25rem;");
    expect(family).toContain("--poodle-slider-family-marker-thickness: 0.25rem;");
    expect(slider).not.toContain("--poodle-slider-block-marker-thickness");
    expect(slider).not.toContain("--poodle-slider-block-marker-inset");
    expect(range).not.toContain("--poodle-range-slider-block-thumb");
    expect(range).not.toContain("--poodle-range-slider-block-marker-thickness");
  });

  test("the 44px targets and layout-neutral overflow stay shared", () => {
    expect(family).toContain("--poodle-slider-family-block-hit: 44px;");
    expect(family).toContain("pointer-events: auto;");
    expect(slider).not.toContain("--poodle-slider-block-hit");
    expect(range).not.toContain("--poodle-range-slider-block-hit");
  });

  test("each family supplies only its marker position through the shared variable", () => {
    expect(slider).toContain("--poodle-slider-family-marker: var(--poodle-slider-block-marker-position);");
    expect(range).toContain(
      "--poodle-slider-family-marker: var(--poodle-range-slider-block-marker-start);",
    );
    expect(range).toContain(
      "--poodle-slider-family-marker: var(--poodle-range-slider-block-marker-end);",
    );
    // The range handles are clamped inside the capsule by the shared bound.
    expect(range).toContain(
      "--poodle-range-slider-block-marker-start: clamp(\n      var(--poodle-slider-family-marker-offset),\n      var(--poodle-range-start),",
    );
    expect(range).toContain(
      "--poodle-range-slider-block-marker-end: clamp(\n      var(--poodle-slider-family-marker-offset),\n      var(--poodle-range-end),",
    );
  });

  test("the text layers paint above the shared handle in both families", () => {
    expect(family).toMatch(/:is\(\.poodle-slider__hit, \.poodle-range-slider__hit\) \{[\s\S]*?z-index: 2;/);
    expect(family).toMatch(/:is\(\.poodle-slider__inline, \.poodle-range-slider__inline\) \{[\s\S]*?z-index: 3;/);
    expect(family).toContain(
      ":is(.poodle-slider__inline--selected, .poodle-range-slider__inline--selected) {\n    z-index: 4;",
    );
  });

  test("forced colours resolve once on the foundation for both families", () => {
    expect(family).toContain(
      ":is(.poodle-slider__thumb, .poodle-range-slider__thumb) {\n      background: ButtonText;",
    );
    expect(range).not.toContain("ButtonFace");
    expect(slider).not.toContain("ButtonFace");
  });

  test("focus suppresses the UA outline on each family's own focusable", () => {
    // Slider focuses the block root; RangeSlider focuses each thumb hit.
    expect(slider).toContain(
      '.poodle-slider[data-variant="block"]:focus-visible {\n    outline: none;',
    );
    expect(range).toContain(
      '.poodle-range-slider[data-variant="block"] .poodle-range-slider__hit:focus-visible {\n    outline: none;',
    );
    // The ring is painted on the one shared handle in the foundation.
    expect(family).toContain(
      "0 0 0 0.1875rem var(--poodle-recipe-slider-block-focus-ring, var(--poodle-color-accent-focusRing));",
    );
  });
});

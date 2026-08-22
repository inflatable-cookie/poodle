/**
 * g15.046 — focused evidence for the Button visual fixture inventory.
 *
 * Every negative case is planted on a clone of the canonical file at run time.
 * A broken inventory is therefore never committed, and each assertion proves
 * the loader names the exact offender rather than failing generically.
 *
 * Run: `effigy test:visual-fixtures`
 */

import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { describe, expect, test } from "bun:test";

import {
  BUTTON_FIXTURE_NAMES,
  ButtonInventoryError,
  INVENTORY_PATH,
  INVENTORY_SCHEMA,
  REPORT_ROLES,
  integralNumber,
  SUPPORTED_CAPTURE_SCALES,
  loadButtonVisualInventory,
  parseButtonVisualInventory,
} from "./button-visual-inventory.ts";

/** Repo root, so the suite does not depend on the caller's working directory. */
const ROOT = resolve(import.meta.dir, "../../..");
const RUST_LOADER = resolve(ROOT, "packages/gpui/preview/tests/visual_fixture_inventory.rs");

type RawInventory = Record<string, unknown> & { fixtures: Record<string, unknown>[] };

function canonical(): RawInventory {
  return JSON.parse(readFileSync(INVENTORY_PATH, "utf8")) as RawInventory;
}

function rowAt(inventory: RawInventory, name: string): Record<string, unknown> {
  const row = inventory.fixtures.find((entry) => entry.name === name);
  if (row === undefined) throw new Error(`planting error: no fixture named '${name}'`);
  return row;
}

/** Plant a fault, parse, and return the reported problems. */
function problemsFor(mutate: (inventory: RawInventory) => void): string[] {
  const inventory = canonical();
  mutate(inventory);
  try {
    parseButtonVisualInventory(inventory);
  } catch (error) {
    if (error instanceof ButtonInventoryError) return error.problems;
    throw error;
  }
  throw new Error("expected the planted inventory to be rejected, but it parsed clean");
}

function expectProblem(problems: string[], needle: string): void {
  expect(problems.some((problem) => problem.includes(needle))).toBe(true);
}

describe("canonical inventory", () => {
  test("parses and holds exactly the 18 g15.046 identities", () => {
    const inventory = loadButtonVisualInventory();
    expect(inventory.schema).toBe(INVENTORY_SCHEMA);
    expect(inventory.component).toBe("button");
    expect(inventory.fixtures).toHaveLength(18);
    expect(inventory.fixtures.map((fixture) => fixture.name)).toEqual([...BUTTON_FIXTURE_NAMES]);
    expect(new Set(inventory.fixtures.map((fixture) => fixture.name)).size).toBe(18);
  });

  test("every row carries all resolved values; nothing is left to a runtime", () => {
    for (const fixture of loadButtonVisualInventory().fixtures) {
      for (const field of ["theme", "size", "density", "variant", "tone", "state"] as const) {
        expect(typeof fixture[field]).toBe("string");
        expect(fixture[field]).not.toBe("");
        expect(fixture[field]).not.toBe("inherit");
      }
      expect(Number.isInteger(fixture.viewport.width)).toBe(true);
      expect(Number.isInteger(fixture.viewport.height)).toBe(true);
      expect(SUPPORTED_CAPTURE_SCALES).toContain(fixture.scale);
      expect(fixture.landmarks[0]).toBe("root");
    }
  });

  test("declares the fixed environment on every row that does not override it", () => {
    const inventory = loadButtonVisualInventory();
    const base = inventory.fixtures.filter((fixture) => fixture.name !== "button/theme-iceberg");
    for (const fixture of base) expect(fixture.theme).toBe("eclipse");
    for (const fixture of inventory.fixtures) {
      expect(fixture.viewport).toEqual({ width: 240, height: 80 });
      expect(fixture.scale).toBe(2);
    }
    expect(rowAt(canonical(), "button/theme-iceberg").theme).toBe("iceberg");
  });

  test("names report roles and landmarks without recording expected output", () => {
    const inventory = loadButtonVisualInventory();
    expect(inventory.reportRoles).toEqual([...REPORT_ROLES]);
    const text = readFileSync(INVENTORY_PATH, "utf8");
    for (const banned of ["expected", "baseline", "threshold", "tolerance", "sha256", "#"]) {
      expect(text.includes(banned)).toBe(false);
    }
  });
});

describe("denominator faults are reported by exact name", () => {
  test("missing identity", () => {
    const problems = problemsFor((inventory) => {
      inventory.fixtures = inventory.fixtures.filter((row) => row.name !== "button/size-lg");
    });
    expectProblem(problems, "missing fixture name 'button/size-lg'");
  });

  test("extra identity", () => {
    const problems = problemsFor((inventory) => {
      const extra = structuredClone(rowAt(inventory, "button/tone-danger"));
      extra.name = "button/tone-info";
      inventory.fixtures.push(extra);
    });
    expectProblem(problems, "unknown fixture name 'button/tone-info'");
  });

  test("duplicate identity", () => {
    const problems = problemsFor((inventory) => {
      inventory.fixtures.push(structuredClone(rowAt(inventory, "button/variant-ghost")));
    });
    expectProblem(problems, "duplicate fixture name 'button/variant-ghost'");
  });
});

describe("domain faults are reported by exact name", () => {
  test("unknown tone", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/tone-danger").tone = "info";
    });
    expectProblem(problems, "fixture 'button/tone-danger': field 'tone' value 'info'");
  });

  test("unknown variant, including the legacy Rust compatibility arm", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/variant-primary").variant = "danger";
    });
    expectProblem(problems, "fixture 'button/variant-primary': field 'variant' value 'danger'");
  });

  test("unknown theme", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/theme-iceberg").theme = "iceberg-light";
    });
    expectProblem(problems, "fixture 'button/theme-iceberg': field 'theme' value 'iceberg-light'");
  });

  test("unknown control size", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/size-xl").size = "xxl";
    });
    expectProblem(problems, "fixture 'button/size-xl': field 'size' value 'xxl'");
  });

  test("interaction-produced state is not a fixture input", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/state-pressed").state = "hover";
    });
    expectProblem(problems, "fixture 'button/state-pressed': field 'state' value 'hover'");
  });

  test("icon outside the default registry", () => {
    const problems = problemsFor((inventory) => {
      (rowAt(inventory, "button/content-leading-icon").content as Record<string, unknown>).icon =
        "rocket-ship";
    });
    expectProblem(problems, "fixture 'button/content-leading-icon': content.icon 'rocket-ship'");
  });
});

describe("unresolved defaults are rejected", () => {
  test("null density", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/density-compact").density = null;
    });
    expectProblem(problems, "fixture 'button/density-compact': field 'density' is null");
  });

  test("inherit marker", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/rest-secondary").theme = "inherit";
    });
    expectProblem(
      problems,
      "fixture 'button/rest-secondary': field 'theme' is the unresolved-default marker 'inherit'",
    );
  });

  test("absent required field", () => {
    const problems = problemsFor((inventory) => {
      delete rowAt(inventory, "button/variant-ghost").tone;
    });
    expectProblem(problems, "fixture 'button/variant-ghost': missing required field 'tone'");
  });
});

describe("viewport and scale faults", () => {
  test("non-positive viewport side", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/size-xs").viewport = { width: 0, height: 80 };
    });
    expectProblem(problems, "fixture 'button/size-xs': viewport.width must be a positive whole");
  });

  test("fractional viewport side", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/size-sm").viewport = { width: 240, height: 80.5 };
    });
    expectProblem(problems, "fixture 'button/size-sm': viewport.height must be a positive whole");
  });

  test("viewport with an unknown key", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/size-lg").viewport = { width: 240, height: 80, dpr: 2 };
    });
    expectProblem(problems, "fixture 'button/size-lg': viewport has unknown key 'dpr'");
  });

  test("row scale outside the inventory capture scales", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/state-loading").scale = 1;
    });
    expectProblem(problems, "fixture 'button/state-loading': scale 1 is not one of");
  });

  test("inventory capture scale the GPUI seam cannot produce", () => {
    const problems = problemsFor((inventory) => {
      inventory.captureScales = [3];
    });
    expectProblem(problems, "inventory captureScales entry 3 is outside the supported set");
  });
});

describe("shape faults keep the format Button-specific", () => {
  test("a generic props bag is an unknown field", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/rest-secondary").props = { variant: "secondary" };
    });
    expectProblem(problems, "fixture 'button/rest-secondary': unknown field 'props'");
  });

  test("a second component is rejected", () => {
    const problems = problemsFor((inventory) => {
      inventory.component = "icon-button";
    });
    expectProblem(problems, "inventory component must be 'button'");
  });

  test("a wrong schema discriminator is rejected", () => {
    const problems = problemsFor((inventory) => {
      inventory.schema = "poodle.component-visual-inventory.v1";
    });
    expectProblem(problems, `inventory schema must be '${INVENTORY_SCHEMA}'`);
  });

  test("a non-object root is rejected", () => {
    expect(() => parseButtonVisualInventory("[]")).toThrow(ButtonInventoryError);
    expect(() => parseButtonVisualInventory(null)).toThrow(ButtonInventoryError);
  });

  test("content keys must match the content kind", () => {
    const missingAria = problemsFor((inventory) => {
      delete (rowAt(inventory, "button/content-icon-only").content as Record<string, unknown>)
        .ariaLabel;
    });
    expectProblem(missingAria, "fixture 'button/content-icon-only': content 'icon-only' is missing 'ariaLabel'");

    const strayIcon = problemsFor((inventory) => {
      (rowAt(inventory, "button/rest-secondary").content as Record<string, unknown>).icon = "play";
    });
    expectProblem(strayIcon, "fixture 'button/rest-secondary': content 'label' has unknown key 'icon'");
  });

  test("landmarks must match the content shape and state", () => {
    const noSpinner = problemsFor((inventory) => {
      rowAt(inventory, "button/state-loading").landmarks = ["root", "content"];
    });
    expectProblem(noSpinner, "fixture 'button/state-loading': landmarks must be exactly [root, content, spinner]");

    const noIcon = problemsFor((inventory) => {
      rowAt(inventory, "button/content-leading-icon").landmarks = ["root", "content"];
    });
    expectProblem(
      noIcon,
      "fixture 'button/content-leading-icon': landmarks must be exactly [root, content, icon]",
    );
  });
});

/**
 * JSON has one number type, so `2` and `2.0` are the same value and both
 * loaders must agree about them. The accepted-spelling case is planted on the
 * canonical *text*, because after `JSON.parse` TypeScript cannot tell the two
 * spellings apart — which is exactly why the rule has to be stated once and
 * applied identically in Rust. Mirrors
 * `numeric_spelling_is_normalized_consistently` in the Rust suite.
 */
describe("numeric spelling is normalized, numeric domain is not", () => {
  test("integral decimal spellings are accepted on every numeric path", () => {
    const text = readFileSync(INVENTORY_PATH, "utf8")
      .replace('"captureScales": [2]', '"captureScales": [2.0]')
      .replaceAll('"scale": 2', '"scale": 2.0')
      .replaceAll('"width": 240', '"width": 240.0')
      .replaceAll('"height": 80', '"height": 80.0');

    expect(text).toContain('"captureScales": [2.0]');
    expect(text).toContain('"scale": 2.0');
    expect(text).toContain('"width": 240.0');
    expect(text).toContain('"height": 80.0');

    const inventory = parseButtonVisualInventory(JSON.parse(text));
    expect(inventory.fixtures).toHaveLength(18);
    expect(inventory.fixtures.every((fixture) => fixture.scale === 2)).toBe(true);
  });

  test("fractional scale is still rejected", () => {
    const row = problemsFor((inventory) => {
      rowAt(inventory, "button/size-xs").scale = 2.5;
    });
    expectProblem(row, "fixture 'button/size-xs': scale 2.5 is not one of");

    const declared = problemsFor((inventory) => {
      inventory.captureScales = [2.5];
    });
    expectProblem(declared, "inventory captureScales entry 2.5 is outside the supported set");
  });

  test("negative numbers are still rejected", () => {
    const viewport = problemsFor((inventory) => {
      rowAt(inventory, "button/size-sm").viewport = { width: -240, height: 80 };
    });
    expectProblem(viewport, "fixture 'button/size-sm': viewport.width must be a positive whole");

    const scale = problemsFor((inventory) => {
      rowAt(inventory, "button/size-lg").scale = -2;
    });
    expectProblem(scale, "fixture 'button/size-lg': scale -2 is not one of");
  });

  test("a numeric string is not a number", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/variant-ghost").scale = "2";
    });
    expectProblem(problems, `fixture 'button/variant-ghost': scale "2" is not one of`);
  });

  test("integers beyond the shared exact range are rejected", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/theme-iceberg").viewport = { width: 1e16, height: 80 };
    });
    expectProblem(
      problems,
      "fixture 'button/theme-iceberg': viewport.width must be a positive whole",
    );
  });

  test("the rule itself: spelling is irrelevant, domain is not", () => {
    expect(integralNumber(2)).toBe(2);
    expect(integralNumber(2.0)).toBe(2);
    expect(integralNumber(0)).toBe(0);
    expect(integralNumber(2.5)).toBeNull();
    expect(integralNumber(-2)).toBeNull();
    expect(integralNumber(Number.NaN)).toBeNull();
    expect(integralNumber(Number.POSITIVE_INFINITY)).toBeNull();
    expect(integralNumber(Number.MAX_SAFE_INTEGER)).toBe(Number.MAX_SAFE_INTEGER);
    expect(integralNumber(Number.MAX_SAFE_INTEGER + 2)).toBeNull();
    expect(integralNumber("2")).toBeNull();
    expect(integralNumber(null)).toBeNull();
  });
});

/**
 * Arrays are compared element by element, never joined and never filtered.
 * A join accepts a collapsed element; a filter accepts an inserted non-string.
 * Both would let this loader and the Rust one disagree about the same bytes.
 */
describe("declared arrays must match element by element", () => {
  test("collapsed report role", () => {
    const problems = problemsFor((inventory) => {
      inventory.reportRoles = ["fill border", "text", "shadow", "focus-ring"];
    });
    expectProblem(
      problems,
      "inventory reportRoles must be exactly [fill, border, text, shadow, focus-ring] (5 entries)",
    );
  });

  test("non-string report role in place", () => {
    const problems = problemsFor((inventory) => {
      inventory.reportRoles = ["fill", "border", 3, "shadow", "focus-ring"];
    });
    expectProblem(problems, "inventory reportRoles entry 2 must be the string 'text', got 3");
  });

  test("non-string report role inserted", () => {
    const problems = problemsFor((inventory) => {
      inventory.reportRoles = ["fill", "border", "text", "shadow", "focus-ring", null];
    });
    expectProblem(
      problems,
      "inventory reportRoles must be exactly [fill, border, text, shadow, focus-ring] (5 entries)",
    );
  });

  test("report roles that are not an array", () => {
    const problems = problemsFor((inventory) => {
      inventory.reportRoles = "fill border text shadow focus-ring";
    });
    expectProblem(problems, "inventory reportRoles must be an array of strings");
  });

  test("collapsed landmark names its fixture", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/rest-secondary").landmarks = ["root content"];
    });
    expectProblem(
      problems,
      "fixture 'button/rest-secondary': landmarks must be exactly [root, content] (2 entries)",
    );
  });

  test("non-string landmark in place names its fixture", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/state-loading").landmarks = ["root", "content", 7];
    });
    expectProblem(
      problems,
      "fixture 'button/state-loading': landmarks entry 2 must be the string 'spinner', got 7",
    );
  });

  test("non-string landmark inserted names its fixture", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/content-leading-icon").landmarks = [
        "root",
        "content",
        "icon",
        null,
      ];
    });
    expectProblem(
      problems,
      "fixture 'button/content-leading-icon': landmarks must be exactly [root, content, icon] (3 entries)",
    );
  });

  test("landmarks that are not an array names its fixture", () => {
    const problems = problemsFor((inventory) => {
      rowAt(inventory, "button/theme-iceberg").landmarks = "root content";
    });
    expectProblem(
      problems,
      "fixture 'button/theme-iceberg': landmarks must be an array of strings",
    );
  });
});

describe("authority boundary", () => {
  test("the Rust loader reads this same checked-in file", () => {
    const rust = readFileSync(RUST_LOADER, "utf8");
    expect(rust.includes("test/visual/fixtures/button-visual-inventory.json")).toBe(true);
    for (const name of BUTTON_FIXTURE_NAMES) expect(rust.includes(name)).toBe(true);
  });

  test("no published package source imports the inventory", () => {
    const hits = Array.from(new Bun.Glob("packages/**/*.{ts,tsx,svelte,rs}").scanSync(ROOT))
      .filter((path) => !path.includes("node_modules") && !path.includes("/tests/"))
      .filter((path) => readFileSync(resolve(ROOT, path), "utf8").includes("button-visual-inventory"));
    expect(hits).toEqual([]);
  });
});

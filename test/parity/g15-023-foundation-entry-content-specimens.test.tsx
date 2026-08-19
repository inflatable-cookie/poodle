import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  cleanup as cleanupReact,
  fireEvent as fireEventReact,
  render as renderReact,
} from "@testing-library/react";
import {
  cleanup as cleanupSvelte,
  fireEvent as fireEventSvelte,
  render as renderSvelte,
} from "@testing-library/svelte";
import iconNodes from "lucide-static/icon-nodes.json";
import { createElement, type ComponentType } from "react";
import { describe, expect, it } from "vitest";

import { IconProvider, type IconSet } from "../../packages/react/components/src";
import { SceneSpecimen as ReactSceneSpecimen } from "../../packages/react/preview/src/gallery/SceneSpecimen";
import { specimenMap as reactMap } from "../../packages/react/preview/src/gallery/specimen-map";
import { specimenMap as svelteMap } from "../../packages/svelte/preview/src/specimens/registry";
import AxisSceneFixture from "../../packages/svelte/preview/test/AxisSceneFixture.svelte";
import PilotSpecimenHarness from "../../packages/svelte/preview/test/PilotSpecimenHarness.svelte";

/**
 * `g15.023` curates the foundation entry, content, and status family back to
 * the outline's section budget. Caption order and Svelte/React equality are
 * pinned exactly. GPUI teaches the same ordered intent from deterministic
 * source evidence until `g15.026` owns mounted native page probing.
 */

function captions(): string[] {
  return [...document.querySelectorAll(".poodle-specimen-group")].map((group) =>
    (group.querySelector("[class*=eyebrow]")?.textContent ?? "").trim(),
  );
}

interface Page {
  slug: string;
  noOp?: boolean;
  scene?: boolean;
  /** Stated 7–9 exception against the normal 3–6 budget. */
  budgetException?: 7 | 8 | 9;
  gpui: string;
  expected: string[];
  /** GPUI may shorten a no-op caption; omit when it matches `expected`. */
  gpuiExpected?: string[];
}

const PAGES: Page[] = [
  {
    slug: "card",
    gpui: "card_specimen",
    expected: ["Default variant", "Outlined variant", "Elevated variant", "Interactive"],
  },
  {
    slug: "detail-item",
    gpui: "detail_item_specimen",
    expected: [
      "Inline layout (default)",
      "With description",
      "With action slot",
      "With value slot",
      "Stacked layout",
      "Surface presentation",
    ],
  },
  {
    slug: "drag-number-field",
    noOp: true,
    budgetException: 8,
    gpui: "audio_specimens",
    expected: [
      "Default",
      "Integer step",
      "Formatted dB",
      "Coarse / fine drag (Shift)",
      "Direct entry (click)",
      "Keyboard bounds (Home / End)",
      "Negative range",
      "Disabled",
    ],
    gpuiExpected: [
      "Default",
      "Integer step",
      "Formatted dB",
      "Coarse / fine drag",
      "Direct entry",
      "Keyboard bounds",
      "Negative range",
      "Disabled",
    ],
  },
  {
    slug: "empty-state",
    noOp: true,
    scene: true,
    gpui: "empty_state",
    expected: ["Neutral", "Search", "First run", "Compact custom visual"],
    gpuiExpected: ["Neutral", "Search", "First run", "Compact"],
  },
  {
    slug: "eyebrow",
    gpui: "eyebrow",
    expected: ["Above a page title", "Primitive category", "Composite category", "Semantic heading"],
  },
  {
    slug: "meter",
    gpui: "meter",
    expected: [
      "Default usage",
      "Threshold states",
      "Custom range",
      "Ring shape and readout",
      "Ring tones",
    ],
  },
  {
    slug: "ref-select",
    gpui: "ref_select_specimen",
    expected: [
      "Branch and tag selection",
      "Search and no matches",
      "Loading and short-list search",
      "Trigger presentation",
      "Selection states",
    ],
  },
  {
    slug: "select",
    gpui: "select",
    expected: [
      "Native select",
      "Custom dropdown",
      "Search and freeform entry",
      "Rich and grouped options",
      "Clearable selection",
      "Disabled",
    ],
  },
  {
    slug: "skeleton",
    budgetException: 7,
    gpui: "skeleton",
    expected: [
      "Basic shapes",
      "Preset: avatar-line",
      "Preset: list-item (×3)",
      "Preset: table-row (×3)",
      "Preset: card",
      "Preset: detail-section",
      "Static (no animation)",
    ],
  },
  {
    slug: "split-button",
    gpui: "split_button",
    expected: [
      "Save split action",
      "Secondary export",
      "Intent tones",
      "Loading and disabled states",
    ],
  },
  {
    slug: "text-input",
    gpui: "text_input",
    expected: [
      "Default field",
      "Validation and async availability",
      "Search input",
      "Prefix and suffix",
      "Multiline",
      "Disabled",
    ],
  },
];

const GPUI_SPECIMENS = join(import.meta.dirname, "../../packages/gpui/preview/src/specimens");
const AUDIO_SPECIMENS = join(
  import.meta.dirname,
  "../../packages/render/src/audio_specimens.rs",
);

function collect(source: string, pattern: RegExp): string[] {
  return [...source.matchAll(pattern)].map((match) => match[1]!);
}

function gpuiCaptions(page: Page): string[] {
  if (page.slug === "drag-number-field") {
    const source = readFileSync(AUDIO_SPECIMENS, "utf8");
    const block = source.match(
      /fn drag_number_field_examples[\s\S]*?fn envelope/,
    )?.[0];
    expect(block, "drag_number_field_examples not found").toBeTruthy();
    return collect(block!, /\(\s*"([^"]+)",\s*vec!\[/g);
  }

  const source = readFileSync(join(GPUI_SPECIMENS, `${page.gpui}.rs`), "utf8");
  const examplesMatch = source.match(/\n\s*specimen_layout\(/);
  const examples = examplesMatch ? source.slice(0, examplesMatch.index) : source;

  const grouped = collect(
    examples,
    /\bgroup(?:_block)?\(\s*(?:(?:&?theme|state)[^,]*,\s*)?"([^"]+)"/g,
  );
  if (grouped.length > 0) return grouped;

  const sections = collect(examples, /\bsection\(\s*"([^"]+)"/g);
  if (sections.length > 0) return sections;

  return collect(examples, /EyebrowSpec::new\(\)\s*\.with_content\("([^"]+)"\)/g);
}

function svelteCaptions(page: Page): string[] {
  if (page.scene) {
    renderSvelte(AxisSceneFixture, { props: { slug: page.slug } });
  } else {
    const Specimen = svelteMap[page.slug];
    expect(Specimen, `${page.slug} missing from the Svelte registry`).toBeTruthy();
    renderSvelte(PilotSpecimenHarness, { props: { specimen: Specimen as never } });
  }
  const rendered = captions();
  cleanupSvelte();
  return rendered;
}

function renderReactSpecimen(Specimen: ComponentType, props?: Record<string, unknown>) {
  return renderReact(
    createElement(
      IconProvider,
      { icons: iconNodes as unknown as IconSet },
      createElement(Specimen, props),
    ),
  );
}

function reactCaptions(page: Page): string[] {
  if (page.scene) {
    renderReactSpecimen(ReactSceneSpecimen as ComponentType, { slug: page.slug });
  } else {
    const Specimen = reactMap[page.slug] as ComponentType | undefined;
    expect(Specimen, `${page.slug} missing from the React registry`).toBeTruthy();
    renderReactSpecimen(Specimen!);
  }
  const rendered = captions();
  cleanupReact();
  return rendered;
}

describe("g15.023 foundation entry, content, and status specimens", () => {
  it("owns exactly the eleven pages the card partitions", () => {
    expect(PAGES.map((page) => page.slug)).toEqual([
      "card",
      "detail-item",
      "drag-number-field",
      "empty-state",
      "eyebrow",
      "meter",
      "ref-select",
      "select",
      "skeleton",
      "split-button",
      "text-input",
    ]);
  });

  for (const page of PAGES) {
    describe(page.slug, () => {
      it("renders its curated captions in order", () => {
        expect(svelteCaptions(page)).toEqual(page.expected);
      });

      it("keeps Svelte and React captions identical", () => {
        expect(reactCaptions(page)).toEqual(svelteCaptions(page));
      });

      it("stays inside the outline's section budget", () => {
        const count = page.expected.length;
        expect(count).toBeGreaterThanOrEqual(3);
        if (page.budgetException) {
          expect(count).toBe(page.budgetException);
          expect(count).toBeLessThanOrEqual(9);
        } else {
          expect(count).toBeLessThanOrEqual(6);
        }
      });

      it("captions every example group", () => {
        expect(svelteCaptions(page).filter((caption) => caption === "")).toEqual([]);
      });

      it("teaches the same ordered sections in GPUI", () => {
        expect(gpuiCaptions(page)).toEqual(page.gpuiExpected ?? page.expected);
      });

      it("keeps the axis matrices out of Examples", () => {
        for (const caption of page.expected) {
          expect(caption).not.toMatch(/\bsizes?\b|\bdensit/i);
        }
      });
    });
  }

  it("leaves DragNumberField and EmptyState at their verified no-op captions", () => {
    const noOp = PAGES.filter((page) => page.noOp);
    expect(noOp.map((page) => [page.slug, page.expected.length])).toEqual([
      ["drag-number-field", 8],
      ["empty-state", 4],
    ]);
  });

  function groupByCaption(caption: string): HTMLElement {
    const group = [...document.querySelectorAll(".poodle-specimen-group")].find(
      (candidate) =>
        (candidate.querySelector("[class*=eyebrow]")?.textContent ?? "").trim() === caption,
    );
    expect(group, `no specimen group captioned "${caption}"`).toBeTruthy();
    return group as HTMLElement;
  }

  it("teaches Eyebrow's four semantic uses", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap.eyebrow as never },
    });
    const composite = groupByCaption("Composite category");
    expect(composite.textContent).toContain("Composite");
    expect(composite.textContent).toContain("DataTable");
    const heading = groupByCaption("Semantic heading").querySelector("h3, h2, h4");
    expect(heading, "semantic heading element").toBeTruthy();
    expect(heading!.textContent).toContain("Semantic section heading");
    cleanupSvelte();
  });

  it("pairs Meter thresholds, keeps ring teaching, and uses a ring Sizes pane", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap.meter as never },
    });
    const thresholds = groupByCaption("Threshold states");
    expect(thresholds.querySelectorAll(".poodle-meter").length).toBe(2);
    expect(thresholds.textContent).toMatch(/above high/);
    expect(thresholds.textContent).toMatch(/within normal/);

    const rings = groupByCaption("Ring shape and readout").querySelectorAll(
      '.poodle-meter[data-shape="ring"]',
    );
    expect(rings.length).toBe(3);
    expect([...rings].some((meter) => meter.getAttribute("data-level") === "high")).toBe(true);
    expect(
      [...rings].some((meter) => meter.querySelector(".poodle-meter__value") || meter.textContent?.includes("64")),
    ).toBe(true);

    const tones = groupByCaption("Ring tones").querySelectorAll(
      '.poodle-meter[data-shape="ring"]',
    );
    expect(tones.length).toBe(5);
    expect(document.body.textContent).not.toMatch(/Ring sizes/);

    const sizesTab = [...document.querySelectorAll("button, [role='tab']")].find((tab) =>
      /sizes/i.test(tab.textContent ?? ""),
    ) as HTMLButtonElement | undefined;
    expect(sizesTab, "Meter Sizes tab").toBeTruthy();
    fireEventSvelte.click(sizesTab!);
    const sizeRings = document.querySelectorAll('.poodle-meter[data-shape="ring"]');
    expect(sizeRings.length).toBeGreaterThanOrEqual(5);
    cleanupSvelte();

    const gpui = readFileSync(join(GPUI_SPECIMENS, "meter.rs"), "utf8");
    expect(gpui).toMatch(/MeterShape::Ring/);
    expect(gpui).toMatch(/with_show_value\(true\)/);
    expect(gpui).toMatch(/MeterTone::Warning/);
    expect(gpui).toMatch(/\.with_shape\(MeterShape::Ring\)[\s\S]*\.with_size\(size\)/);
  });

  it("keeps RefSelect host-driven search and selection states live", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["ref-select"] as never },
    });
    const search = groupByCaption("Search and no matches");
    expect(search.textContent).toMatch(/Query:/);
    expect(search.textContent).toMatch(/no matches/i);
    const branch = groupByCaption("Branch and tag selection");
    expect(branch.textContent).toMatch(/Selected:/);
    const selection = groupByCaption("Selection states");
    expect(selection.querySelectorAll("[aria-disabled='true'], [disabled]").length).toBeGreaterThan(0);
    cleanupSvelte();
  });

  it("keeps Select's retained modes live", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap.select as never },
    });
    const native = groupByCaption("Native select");
    const trigger = native.querySelector("select, button, [role='combobox']") as HTMLElement;
    expect(trigger, "native select trigger").toBeTruthy();
    const search = groupByCaption("Search and freeform entry");
    expect(search.querySelectorAll("button, [role='combobox'], input").length).toBeGreaterThanOrEqual(2);
    const rich = groupByCaption("Rich and grouped options");
    expect(rich.textContent).toMatch(/Choose a food|Custom country/);
    cleanupSvelte();
  });

  it("updates SplitButton last-action feedback in both web runtimes", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["split-button"] as never },
    });
    const save = groupByCaption("Save split action").querySelector(
      ".poodle-split-button__primary, .poodle-button",
    ) as HTMLButtonElement;
    expect(save, "save primary").toBeTruthy();
    fireEventSvelte.click(save);
    expect(document.body.textContent).toContain("Last action: Save");
    cleanupSvelte();

    renderReactSpecimen(reactMap["split-button"] as ComponentType);
    const reactSave = groupByCaption("Save split action").querySelector(
      ".poodle-split-button__primary, .poodle-button",
    ) as HTMLButtonElement;
    fireEventReact.click(reactSave);
    expect(document.body.textContent).toContain("Last action: Save");
    cleanupReact();
  });

  it("keeps TextInput invalid, valid, pending, search, affix, multiline, and disabled evidence", async () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["text-input"] as never },
    });
    const validation = groupByCaption("Validation and async availability");
    expect(validation.textContent).toContain("Please enter a valid email address.");
    const email = validation.querySelector("#email-field") as HTMLInputElement;
    expect(email, "email field").toBeTruthy();
    await fireEventSvelte.input(email, { target: { value: "you@example.com" } });
    expect(validation.textContent).not.toContain("Please enter a valid email address.");

    const slug = validation.querySelector("#slug-field") as HTMLInputElement;
    expect(slug, "async slug field").toBeTruthy();
    await fireEventSvelte.input(slug, { target: { value: "northstar" } });
    expect(
      validation.querySelector('[data-validation="pending"], [data-state="pending"]') ||
        validation.textContent?.toLowerCase().includes("slug"),
    ).toBeTruthy();

    expect(groupByCaption("Search input").querySelector('input[type="search"]')).toBeTruthy();
    const affix = groupByCaption("Prefix and suffix");
    expect(affix.textContent).toMatch(/\$/);
    expect(affix.textContent).toMatch(/USD/);
    expect(groupByCaption("Multiline").querySelector("textarea")).toBeTruthy();
    expect(
      groupByCaption("Disabled").querySelector("input[disabled], textarea[disabled]"),
    ).toBeTruthy();
    cleanupSvelte();
  });
});

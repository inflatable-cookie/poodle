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
import { specimenMap as reactMap } from "../../packages/react/preview/src/gallery/specimen-map";
import { specimenMap as svelteMap } from "../../packages/svelte/preview/src/specimens/registry";
import PilotSpecimenHarness from "../../packages/svelte/preview/test/PilotSpecimenHarness.svelte";

/**
 * `g15.025` curates the collections, navigation, and long-tail family to the
 * card's exact outline. Caption order and Svelte/React equality are pinned.
 * GPUI teaches the same ordered intent. Accordion, ListCardCounter, and
 * MediaPreview stay at their preserved 2–3 group outlines.
 */

function captions(): string[] {
  return [...document.querySelectorAll(".poodle-specimen-group")].map((group) =>
    (group.querySelector("[class*=eyebrow]")?.textContent ?? "").trim(),
  );
}

interface Page {
  slug: string;
  gpui: string;
  expected: string[];
}

const PAGES: Page[] = [
  {
    slug: "accordion",
    gpui: "accordion",
    expected: ["Single selection", "Multiple selection"],
  },
  {
    slug: "dialog",
    gpui: "dialog",
    expected: [
      "Basic and alert dialogs",
      "Forms and nested controls",
      "Custom header and footer",
      "Bare content",
      "Scrolling and width presets",
      "Dismissal rules",
    ],
  },
  {
    slug: "filter-builder",
    gpui: "filter_builder_specimen",
    expected: [
      "Building filters",
      "Match all and match any",
      "Empty and limited builders",
      "Field types and overflow",
      "Disabled",
    ],
  },
  {
    slug: "list-card",
    gpui: "list_card",
    expected: [
      "Interactive rows",
      "Hierarchy and selection",
      "Leading content and layout",
      "Badges and counters",
      "Visual status",
      "Actions and static use",
    ],
  },
  {
    slug: "list-card-counter",
    gpui: "list_card_counter",
    expected: ["Static footer counters", "Linked footer counter"],
  },
  {
    slug: "media-preview",
    gpui: "media_preview_specimen",
    expected: ["Image preview", "Video preview", "Error state"],
  },
  {
    slug: "split-view",
    gpui: "split_view_specimen",
    expected: [
      "Horizontal split",
      "Vertical split",
      "Collapse controls",
      "Hover-revealed controls",
      "Nested workspace",
      "Disabled",
    ],
  },
  {
    slug: "stepper",
    gpui: "stepper",
    expected: [
      "Guided workflow",
      "Collapsed progress",
      "Running and failed states",
      "Re-run",
      "Disabled",
    ],
  },
  {
    slug: "time-ago",
    gpui: "time_ago_specimen",
    expected: [
      "Recent and future timestamps",
      "In running prose",
      "Long and static formats",
      "ISO input",
    ],
  },
  {
    slug: "tree",
    gpui: "tree",
    expected: [
      "File explorer",
      "Selection modes",
      "Presentation options",
      "Loading and large data",
      "Editing and reordering",
      "Disabled nodes",
    ],
  },
];

const GPUI_SPECIMENS = join(import.meta.dirname, "../../packages/gpui/preview/src/specimens");

function collect(source: string, pattern: RegExp): string[] {
  return [...source.matchAll(pattern)].map((match) => match[1]!);
}

function gpuiCaptions(page: Page): string[] {
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

  const labelled = collect(examples, /\blabelled\(\s*(?:theme,\s*)?"([^"]+)"/g);
  if (labelled.length > 0) return labelled;

  return collect(examples, /EyebrowSpec::new\(\)\s*\.with_content\("([^"]+)"\)/g);
}

function svelteCaptions(page: Page): string[] {
  const Specimen = svelteMap[page.slug];
  expect(Specimen, `${page.slug} missing from the Svelte registry`).toBeTruthy();
  renderSvelte(PilotSpecimenHarness, { props: { specimen: Specimen as never } });
  const rendered = captions();
  cleanupSvelte();
  return rendered;
}

function renderReactSpecimen(Specimen: ComponentType) {
  return renderReact(
    createElement(
      IconProvider,
      { icons: iconNodes as unknown as IconSet },
      createElement(Specimen),
    ),
  );
}

function reactCaptions(page: Page): string[] {
  const Specimen = reactMap[page.slug] as ComponentType | undefined;
  expect(Specimen, `${page.slug} missing from the React registry`).toBeTruthy();
  renderReactSpecimen(Specimen!);
  const rendered = captions();
  cleanupReact();
  return rendered;
}

function groupByCaption(caption: string): HTMLElement {
  const group = [...document.querySelectorAll(".poodle-specimen-group")].find(
    (candidate) =>
      (candidate.querySelector("[class*=eyebrow]")?.textContent ?? "").trim() === caption,
  );
  expect(group, `no specimen group captioned "${caption}"`).toBeTruthy();
  return group as HTMLElement;
}

describe("g15.025 collections, navigation, and long-tail specimens", () => {
  it("owns exactly the ten pages the card partitions", () => {
    expect(PAGES.map((page) => page.slug)).toEqual([
      "accordion",
      "dialog",
      "filter-builder",
      "list-card",
      "list-card-counter",
      "media-preview",
      "split-view",
      "stepper",
      "time-ago",
      "tree",
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
        expect(count).toBeGreaterThanOrEqual(2);
        expect(count).toBeLessThanOrEqual(6);
      });

      it("captions every example group", () => {
        expect(svelteCaptions(page).filter((caption) => caption === "")).toEqual([]);
      });

      it("teaches the same ordered sections in GPUI", () => {
        expect(gpuiCaptions(page)).toEqual(page.expected);
      });

      it("keeps the axis matrices out of Examples", () => {
        for (const caption of page.expected) {
          expect(caption).not.toMatch(/\bsizes?\b|\bdensit/i);
        }
      });
    });
  }

  it("keeps every Dialog trigger live, including Dialog's own alertdialog role", async () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap.dialog as never },
    });

    const basic = groupByCaption("Basic and alert dialogs");
    const viewDetails = [...basic.querySelectorAll("button")].find(
      (button) => button.textContent?.trim() === "View details",
    ) as HTMLButtonElement | undefined;
    const deleteItem = [...basic.querySelectorAll("button")].find(
      (button) => button.textContent?.trim() === "Delete item",
    ) as HTMLButtonElement | undefined;
    expect(viewDetails, "informational trigger").toBeTruthy();
    expect(deleteItem, "alert trigger").toBeTruthy();

    await fireEventSvelte.click(viewDetails!);
    expect(document.querySelector('[role="dialog"]')).toBeTruthy();
    expect(document.body.textContent).toMatch(/Command palette|Keyboard shortcuts/);
    const close = [...document.querySelectorAll("button")].find((button) =>
      /close/i.test(button.getAttribute("aria-label") ?? button.textContent ?? ""),
    );
    if (close) await fireEventSvelte.click(close);

    await fireEventSvelte.click(deleteItem!);
    const alert = document.querySelector('[role="alertdialog"]') as HTMLElement | null;
    expect(alert, "Dialog role=alertdialog").toBeTruthy();
    expect(alert!.textContent).toMatch(/Delete item\?/);
    expect(document.querySelector(".poodle-alert-dialog")).toBeNull();
    cleanupSvelte();
  });

  it("keeps FilterBuilder's controlled readout and field-type coverage live", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["filter-builder"] as never },
    });
    const composing = groupByCaption("Building filters");
    expect(composing.querySelector("pre")?.textContent).toMatch(/"combinator": "and"/);
    expect(composing.querySelector("pre")?.textContent).toMatch(/format|hidden|tag-count/);

    const combinators = groupByCaption("Match all and match any");
    expect(combinators.querySelectorAll(".poodle-filter-builder").length).toBeGreaterThanOrEqual(2);

    const emptyAndCapped = groupByCaption("Empty and limited builders");
    expect(emptyAndCapped.querySelectorAll(".poodle-filter-builder").length).toBeGreaterThanOrEqual(2);

    const types = groupByCaption("Field types and overflow");
    expect(types.textContent).toMatch(/CLAP|Hidden|Rating|Tag/);
    expect(groupByCaption("Disabled").querySelector("[data-disabled='true'], [aria-disabled='true']")).toBeTruthy();
    cleanupSvelte();
  });

  it("keeps ListCard click, selection, reorder, and context-menu feedback live", async () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["list-card"] as never },
    });
    expect(captions()).not.toContain("Inherited footer counters");
    expect(captions()).not.toContain("With wrapped context menu");
    expect(captions()).not.toContain("Highlighted card");
    expect(captions()).not.toContain("Active card");

    const interactive = groupByCaption("Interactive rows");
    const firstCard = interactive.querySelector("button, [role='button'], a") as HTMLElement | null;
    expect(firstCard, "interactive row").toBeTruthy();
    await fireEventSvelte.click(firstCard!);
    expect(interactive.textContent).toMatch(/design-system-v2\.figma|Last click/);

    const hierarchy = groupByCaption("Hierarchy and selection");
    expect(hierarchy.textContent).toMatch(/Cash flow|Pathway|Module/);
    expect(hierarchy.querySelector("[data-selected], [aria-pressed='true']")).toBeTruthy();

    const status = groupByCaption("Visual status");
    expect(status.textContent).toMatch(/Highlighted|Active/);
    expect(status.textContent).toMatch(/Free|New|EOL|Unpublished|Pending/);

    const actions = groupByCaption("Actions and static use");
    expect(actions.textContent).toMatch(/Right-click|Read-only/);
    cleanupSvelte();
  });
});

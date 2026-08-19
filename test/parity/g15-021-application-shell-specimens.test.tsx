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
 * `g15.021` curates the application-shell family to the outline's section
 * budget, wires decorative controls, and maps HistoryCenter fixture IDs to
 * reader language. Caption order and Svelte/React equality are pinned exactly;
 * behavior assertions cover the stories the captions claim.
 */

/** Captions a reader can actually see on the Examples tab, in DOM order. */
function captions(): string[] {
  return [...document.querySelectorAll(".poodle-specimen-group")].map((group) =>
    (group.querySelector("[class*=eyebrow]")?.textContent ?? "").trim(),
  );
}

interface Page {
  slug: string;
  /** `true` when the curation left the page untouched. */
  noOp?: boolean;
  /** File stem under `packages/gpui/preview/src/specimens`. */
  gpui: string;
  expected: string[];
}

const PAGES: Page[] = [
  {
    slug: "action-discovery-panel",
    gpui: "action_discovery_panel",
    expected: [
      "Grouped actions",
      "Descriptions, badges, and shortcuts",
      "Loading and empty states",
    ],
  },
  {
    slug: "detail-section",
    gpui: "detail_section_specimen",
    expected: [
      "Project details",
      "Section actions",
      "Described detail rows",
      "Two-column details",
    ],
  },
  {
    slug: "detail-section-group",
    gpui: "detail_section_group_specimen",
    noOp: true,
    expected: ["Grid layout", "Stack layout", "Column cap"],
  },
  {
    slug: "detail-shell",
    gpui: "detail_shell",
    expected: [
      "Layout structure",
      "Multi-section layout with header",
      "Loading state",
      "Error state",
    ],
  },
  {
    slug: "dock-region",
    gpui: "dock_region",
    expected: [
      "Expanded side dock",
      "Collapse and edge placement",
      "Tab strip presentation",
      "Move panels between docks",
      "Static panel stacks",
    ],
  },
  {
    slug: "history-center",
    gpui: "history_center_specimen",
    expected: [
      "Linear history",
      "Choosing between continuations",
      "Nested continuation runs",
      "Single continuation and run boundaries",
      "Rename and manage a continuation",
      "Failure and incomplete metadata",
    ],
  },
  {
    slug: "page-header",
    gpui: "page_header_specimen",
    expected: [
      "Page title and summary",
      "Navigation and actions",
      "Hierarchy and count",
      "Contextual status",
      "Operational metadata",
    ],
  },
];

const GPUI_SPECIMENS = join(
  import.meta.dirname,
  "../../packages/gpui/preview/src/specimens",
);

function gpuiCaptions(stem: string): string[] {
  const source = readFileSync(join(GPUI_SPECIMENS, `${stem}.rs`), "utf8");
  const examplesMatch = source.match(/\n\s*specimen_layout\(/);
  const examples = examplesMatch
    ? source.slice(0, examplesMatch.index)
    : source;
  const labels: string[] = [];
  for (const match of examples.matchAll(/\bgroup(?:_block)?\(\s*"([^"]+)"/g)) {
    labels.push(match[1]!);
  }
  return labels;
}

function svelteCaptions(slug: string): string[] {
  const Specimen = svelteMap[slug];
  expect(Specimen, `${slug} missing from the Svelte registry`).toBeTruthy();
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

function reactCaptions(slug: string): string[] {
  const Specimen = reactMap[slug] as ComponentType | undefined;
  expect(Specimen, `${slug} missing from the React registry`).toBeTruthy();
  renderReactSpecimen(Specimen!);
  const rendered = captions();
  cleanupReact();
  return rendered;
}

describe("g15.021 application-shell specimens", () => {
  it("owns exactly the seven pages the card partitions", () => {
    expect(PAGES.map((page) => page.slug)).toEqual([
      "action-discovery-panel",
      "detail-section",
      "detail-section-group",
      "detail-shell",
      "dock-region",
      "history-center",
      "page-header",
    ]);
  });

  for (const page of PAGES) {
    describe(page.slug, () => {
      it("renders its curated captions in order", () => {
        expect(svelteCaptions(page.slug)).toEqual(page.expected);
      });

      it("keeps Svelte and React captions identical", () => {
        expect(reactCaptions(page.slug)).toEqual(svelteCaptions(page.slug));
      });

      it("stays inside the outline's 3-6 section budget", () => {
        const count = page.expected.length;
        expect(count).toBeGreaterThanOrEqual(3);
        expect(count).toBeLessThanOrEqual(6);
      });

      it("captions every example group", () => {
        expect(svelteCaptions(page.slug).filter((caption) => caption === "")).toEqual([]);
      });

      it("teaches the same ordered sections in GPUI", () => {
        expect(gpuiCaptions(page.gpui)).toEqual(page.expected);
      });

      it("keeps the axis matrices out of Examples", () => {
        for (const caption of page.expected) {
          expect(caption).not.toMatch(/\bsizes?\b|\bdensit/i);
        }
      });
    });
  }

  it("leaves DetailSectionGroup at its verified no-op captions", () => {
    const noOp = PAGES.find((page) => page.noOp);
    expect(noOp?.expected).toEqual(["Grid layout", "Stack layout", "Column cap"]);
  });

  function groupByCaption(caption: string): HTMLElement {
    const group = [...document.querySelectorAll(".poodle-specimen-group")].find(
      (candidate) =>
        (candidate.querySelector("[class*=eyebrow]")?.textContent ?? "").trim() === caption,
    );
    expect(group, `no specimen group captioned "${caption}"`).toBeTruthy();
    return group as HTMLElement;
  }

  it("proves ActionDiscoveryPanel selection updates visible host feedback", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["action-discovery-panel"] as never },
    });
    const group = groupByCaption("Grouped actions");
    const save = group.querySelector(".poodle-list-card") as HTMLElement;
    expect(save?.textContent).toContain("Save");
    fireEventSvelte.click(save);
    expect(group.textContent).toContain("Selected action: Save");
    cleanupSvelte();

    renderReactSpecimen(reactMap["action-discovery-panel"] as ComponentType);
    const reactGroup = groupByCaption("Grouped actions");
    const reactSave = reactGroup.querySelector(".poodle-list-card") as HTMLElement;
    fireEventReact.click(reactSave);
    expect(reactGroup.textContent).toContain("Selected action: Save");
    cleanupReact();
  });

  it("proves DetailSection, DetailShell, and PageHeader actions update visible feedback", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["detail-section"] as never },
    });
    const billing = groupByCaption("Section actions");
    const edit = billing.querySelector(".poodle-button") as HTMLButtonElement;
    fireEventSvelte.click(edit);
    expect(billing.textContent).toContain("Last action: Edit billing");
    cleanupSvelte();

    renderReactSpecimen(reactMap["detail-shell"] as ComponentType);
    const shell = groupByCaption("Multi-section layout with header");
    const editShell = [...shell.querySelectorAll(".poodle-button")].find(
      (button) => button.textContent?.trim() === "Edit",
    ) as HTMLButtonElement;
    fireEventReact.click(editShell);
    expect(shell.textContent).toContain("Last action: Edit project");
    expect(shell.textContent).toContain("Light");
    cleanupReact();

    renderReactSpecimen(reactMap["page-header"] as ComponentType);
    const nav = groupByCaption("Navigation and actions");
    const upload = nav.querySelector('[aria-label="Upload"]') as HTMLButtonElement;
    fireEventReact.click(upload);
    expect(nav.textContent).toContain("Last action: Upload");
    cleanupReact();
  });

  it("maps HistoryCenter's nine fixture stories into six reader questions", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["history-center"] as never },
    });
    const labels = captions();
    expect(labels).toEqual([
      "Linear history",
      "Choosing between continuations",
      "Nested continuation runs",
      "Single continuation and run boundaries",
      "Rename and manage a continuation",
      "Failure and incomplete metadata",
    ]);
    expect(document.querySelectorAll("[data-part='trigger']").length).toBeGreaterThanOrEqual(9);
    cleanupSvelte();
  });

  it("keeps DockRegion's iconless, pass-through, collapse, transfer, and static groups", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["dock-region"] as never },
    });
    const expanded = groupByCaption("Expanded side dock");
    expect(expanded.textContent).toMatch(/icon-less|Inspector/i);
    const tabs = groupByCaption("Tab strip presentation");
    expect(tabs.textContent).toMatch(/tabActiveEdge|tabReorderable|tabVariant/i);
    const collapse = groupByCaption("Collapse and edge placement");
    expect(collapse.querySelectorAll(".poodle-dock-region").length).toBeGreaterThanOrEqual(3);
    const transfer = groupByCaption("Move panels between docks");
    expect(transfer.querySelectorAll(".poodle-dock-region").length).toBe(2);
    const staticStacks = groupByCaption("Static panel stacks");
    expect(staticStacks.textContent).toMatch(/Meter Strip|Toolbar/i);
    cleanupSvelte();
  });
});

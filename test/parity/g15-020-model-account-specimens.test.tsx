import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  cleanup as cleanupReact,
  render as renderReact,
} from "@testing-library/react";
import {
  cleanup as cleanupSvelte,
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
 * `g15.020` curated the model-connection and account-lifecycle family back to
 * the outline's section budget. The two headline risks are silent regrowth and
 * one runtime drifting from the other, so both are pinned exactly: the ordered
 * caption list per page, and Svelte/React equality across the whole set.
 *
 * LicenceActivation and LicenceSeats were already within budget. They are
 * asserted here unchanged so a later card cannot quietly inflate them either.
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
    slug: "licence-activation",
    gpui: "licence_activation",
    noOp: true,
    expected: [
      "Embedded account activation",
      "External account activation",
      "Key activation",
      "Pending and disabled",
      "Host copy",
    ],
  },
  {
    slug: "licence-seats",
    gpui: "licence_seats",
    noOp: true,
    expected: [
      "Mixed labels",
      "Unnamed machines",
      "This machine only",
      "Pending release",
      "Direct release",
      "Empty authority",
    ],
  },
  {
    slug: "licence-status",
    gpui: "licence_status",
    expected: [
      "Active",
      "In grace",
      "Use window expired",
      "Lease lapsed",
      "Clock refused",
    ],
  },
  {
    slug: "model-catalogue-editor",
    gpui: "model_catalogue_editor_specimen",
    expected: [
      "Shown and hidden models",
      "Reorder and visibility controls",
      "Host mark, actions, and row metadata",
      "Loading and pending",
      "Empty catalogue",
      "Unavailable, error, and session-negotiated",
    ],
  },
  {
    slug: "model-connection-card",
    gpui: "model_connection_card_specimen",
    expected: [
      "Ready and enabled",
      "Readiness and preference states",
      "Host mark, badges, actions, and closed accessory",
      "Open details with catalogue",
      "Narrow summary wrapping",
    ],
  },
  {
    slug: "model-connection-picker",
    gpui: "model_connection_picker_specimen",
    expected: [
      "Grouped catalogue",
      "Search results",
      "Catalogue states and host lock",
      "Host provider marks and footer",
      "Narrow layout",
    ],
  },
  {
    slug: "model-connection-setup",
    gpui: "model_connection_setup_specimen",
    expected: [
      "Choose a connection",
      "Configure: API key",
      "Auto-detected local route",
      "OAuth in progress",
      "Local endpoint",
      "Validation and pending",
    ],
  },
  {
    slug: "model-picker",
    gpui: "model_picker_specimen",
    expected: [
      "Cross-provider default",
      "Axis control forms",
      "Variants and emphasis",
      "What the trigger shows",
      "Nothing selected, and disabled",
    ],
  },
];

const GPUI_SPECIMENS = join(
  import.meta.dirname,
  "../../packages/gpui/preview/src/specimens",
);

/**
 * GPUI caption order, read from the specimen source.
 *
 * `g15.026` owns the headless page probe. Until it lands, the narrowest
 * deterministic evidence available is the ordered caption list each GPUI
 * specimen declares through its local `group`/`section` helper — the same
 * helpers that feed the native `Eyebrow`.
 */
function gpuiCaptions(stem: string): string[] {
  const source = readFileSync(join(GPUI_SPECIMENS, `${stem}.rs`), "utf8");
  const labels: string[] = [];
  for (const match of source.matchAll(/\bgroup\(\s*(?:&?theme|state)[^,]*,\s*"([^"]+)"/g)) {
    labels.push(match[1]!);
  }
  for (const match of source.matchAll(/\bsection\(\s*"([^"]+)"/g)) {
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

describe("g15.020 model-connection and account-lifecycle specimens", () => {
  it("owns exactly the eight pages the card partitions", () => {
    expect(PAGES.map((page) => page.slug)).toEqual([
      "licence-activation",
      "licence-seats",
      "licence-status",
      "model-catalogue-editor",
      "model-connection-card",
      "model-connection-picker",
      "model-connection-setup",
      "model-picker",
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
        // 7-9 needs a stated reason and 10+ is a curation defect. No page in
        // this family has a stated reason, so the budget is the hard bound.
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

  it("leaves the two already-curated licence pages at their verified counts", () => {
    const noOp = PAGES.filter((page) => page.noOp);
    expect(noOp.map((page) => [page.slug, page.expected.length])).toEqual([
      ["licence-activation", 5],
      ["licence-seats", 6],
    ]);
  });

  /** The `.poodle-specimen-group` whose visible caption is `caption`. */
  function groupByCaption(caption: string): HTMLElement {
    const group = [...document.querySelectorAll(".poodle-specimen-group")].find(
      (candidate) =>
        (candidate.querySelector("[class*=eyebrow]")?.textContent ?? "").trim() === caption,
    );
    expect(group, `no specimen group captioned "${caption}"`).toBeTruthy();
    return group as HTMLElement;
  }

  /**
   * The card's contract-critical story for ModelConnectionSetup: a route whose
   * option sets `requiresConfiguration: false` is added straight from `choose`.
   * Asserting the caption alone let an earlier draft render this section on
   * `configure`, which teaches the opposite of what it claims.
   */
  function assertDirectAddStory(runtime: string): void {
    const group = groupByCaption("Auto-detected local route");
    const setups = [...group.querySelectorAll(".poodle-model-connection-setup")];
    expect(setups, runtime).toHaveLength(2);

    for (const setup of setups) {
      expect(setup.getAttribute("data-stage"), `${runtime}: no configure stage is emitted`).toBe(
        "choose",
      );
      expect(
        setup.querySelector(".poodle-model-connection-setup__configuration"),
        `${runtime}: a skipped credential step must render no configuration surface`,
      ).toBeNull();

      const actions = [
        ...setup.querySelectorAll(".poodle-model-connection-setup__actions .poodle-button"),
      ].map((button) => (button.textContent ?? "").trim());
      // Add, not Continue; and no Back, because there is nowhere to go back to.
      expect(actions, runtime).toEqual(["Cancel", "Add connection"]);
    }

    const codexOptions = setups.map(
      (setup) =>
        setup.querySelector('[data-model-connection-option="codex-app"]') as HTMLButtonElement,
    );
    expect(
      codexOptions.map((option) => option.getAttribute("data-availability")),
      `${runtime}: detection outcome is visible on the option`,
    ).toEqual(["available", "unavailable"]);
    expect(
      codexOptions.map((option) => option.textContent?.replace(/\s+/g, " ").trim()),
      `${runtime}: visible availability copy stays honest`,
    ).toEqual([
      expect.stringContaining("Available"),
      expect.stringContaining("Unavailable"),
    ]);
    expect(
      codexOptions.map((option) =>
        option.querySelector(".poodle-status-indicator")?.getAttribute("aria-label"),
      ),
      `${runtime}: host detail remains available to assistive technology`,
    ).toEqual([
      "Available",
      "Not detected",
    ]);
    expect(
      codexOptions.map((option) => option.disabled),
      `${runtime}: the missing route is not selectable`,
    ).toEqual([false, true]);

    // Detected: the host approved the add, so Add is live from `choose`.
    const detected = setups[0]!.querySelector(
      ".poodle-model-connection-setup__actions .poodle-button:last-of-type",
    ) as HTMLButtonElement;
    expect(detected.disabled, `${runtime}: detected route submits from choose`).toBe(false);

    // Missing: nothing was found, so Add stays disabled - still without a
    // configure stage, which is the point of the pairing.
    const missing = setups[1]!.querySelector(
      ".poodle-model-connection-setup__actions .poodle-button:last-of-type",
    ) as HTMLButtonElement;
    expect(missing.disabled, `${runtime}: undetected route cannot be added`).toBe(true);
  }

  it("proves ModelConnectionSetup skips the credential step in both web runtimes", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["model-connection-setup"] as never },
    });
    assertDirectAddStory("svelte");
    cleanupSvelte();

    renderReactSpecimen(reactMap["model-connection-setup"] as ComponentType);
    assertDirectAddStory("react");
    cleanupReact();
  });

  it("keeps GPUI's auto-detected route on the choose stage too", () => {
    const source = readFileSync(
      join(GPUI_SPECIMENS, "model_connection_setup_specimen.rs"),
      "utf8",
    );
    // Both outcomes stay on Choose. Their option sets carry the actual host
    // detection result, rather than invisible configure-stage feedback.
    expect(source).toMatch(
      /let direct_add = \|options: Vec<ModelConnectionOption>, value: &str\| \{\s*ModelConnectionSetupSpec::new\(\)\s*\.with_options\(options\)\s*\.with_stage\(ModelConnectionSetupStage::Choose\)/,
    );
    expect(source).toMatch(
      /fn missing_options\(\)[\s\S]*ModelConnectionAvailability::Unavailable,\s*"Not detected"/,
    );
    expect(source).toMatch(/direct_add\(interactive_options\(\), "codex-app"\)/);
    expect(source).toMatch(/direct_add\(missing_options\(\), "codex-app"\)/);
    expect(source).not.toMatch(/configure\("codex-app"\)/);
  });

  it("seeds GPUI's open-details example open, as both web runtimes do", () => {
    const source = readFileSync(
      join(GPUI_SPECIMENS, "model_connection_card_specimen.rs"),
      "utf8",
    );
    expect(source).toMatch(/const CARD_LIVE_ID: &str = "conn-openai-work";/);
    // `card_is_open(id, default)` mirrors `card_is_enabled`: seeded open, and
    // still yielding to the host map once the reader toggles the disclosure.
    expect(source).toMatch(/\.with_open\(host\.card_is_open\(CARD_LIVE_ID, true\)\)/);
  });

  it("proves the contract-critical stories the curation had to keep", () => {
    // Every licence usability state keeps its own surface.
    const licence = svelteCaptions("licence-status");
    expect(licence).toHaveLength(5);

    // LicenceStatus: both trust bases and every use/update null-value pairing
    // survive the reduction to five fixtures.
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["licence-status"] as never },
    });
    const rows = [...document.querySelectorAll(".poodle-licence-status")].map(
      (status) => (status.textContent ?? "").replace(/\s+/g, " "),
    );
    expect(rows).toHaveLength(6);
    expect(rows.some((row) => row.includes("verified on this machine"))).toBe(true);
    expect(rows.some((row) => row.includes("confirmed"))).toBe(true);
    const noEnd = rows.filter((row) => (row.match(/No end date/g) ?? []).length === 2);
    expect(noEnd.length, "a licence with neither window bounded").toBe(1);
    cleanupSvelte();
  });
});

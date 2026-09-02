import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import PilotSpecimenHarness from "./PilotSpecimenHarness.svelte";
import ButtonSpecimen from "../src/specimens/ButtonSpecimen.svelte";
import RangeSliderSpecimen from "../src/specimens/RangeSliderSpecimen.svelte";
import TabsSpecimen from "../src/specimens/TabsSpecimen.svelte";

/**
 * Structural guards for the three `g15.011` pilot pages.
 *
 * The audit's two headline defects were invisible to every existing gate: an
 * example group whose caption does not render, and an axis matrix expanded
 * inside `Examples`. Both are structural, so they are checked structurally
 * here rather than by screenshot — specimen screenshots are not parity tests.
 */

/** Captions a reader can actually see, in order. */
function captions(): string[] {
  return [...document.querySelectorAll(".poodle-specimen-group")].map((group) =>
    (group.querySelector("[class*=eyebrow]")?.textContent ?? "").trim(),
  );
}

const PILOTS = [
  { name: "Button", component: ButtonSpecimen, expected: 6 },
  { name: "RangeSlider", component: RangeSliderSpecimen, expected: 6 },
  { name: "Tabs", component: TabsSpecimen, expected: 6 },
] as const;

describe("g15.011 pilot specimens", () => {
  for (const pilot of PILOTS) {
    describe(pilot.name, () => {
      it("captions every example group", () => {
        render(PilotSpecimenHarness, { props: { specimen: pilot.component } });
        const rendered = captions();
        expect(rendered.length).toBe(pilot.expected);
        expect(rendered.filter((caption) => caption === "")).toEqual([]);
      });

      it("keeps Examples within the section budget", () => {
        render(PilotSpecimenHarness, { props: { specimen: pilot.component } });
        // The outline's budget is 3-6 sections; 10+ is a curation defect.
        expect(captions().length).toBeLessThanOrEqual(6);
      });

      it("opens on Examples and offers both axis tabs", () => {
        render(PilotSpecimenHarness, { props: { specimen: pilot.component } });
        const tabs = screen
          .getAllByRole("tab")
          .map((tab) => (tab.textContent ?? "").trim());
        expect(tabs.slice(0, 3)).toEqual(["Examples", "Sizes", "Densities"]);
        // g14.008 rejected the executable corpus; g14.021 removed its tab.
        expect(tabs).not.toContain("Conformance");
      });

      it("keeps size and density matrices out of Examples", () => {
        render(PilotSpecimenHarness, { props: { specimen: pilot.component } });
        for (const caption of captions()) {
          expect(caption).not.toMatch(/\bsizes?\b|\bdensit/i);
        }
      });
    });
  }

  it("RangeSlider teaches the vertical orientation the contract covers", () => {
    render(PilotSpecimenHarness, { props: { specimen: RangeSliderSpecimen } });
    expect(
      document.querySelector('[data-orientation="vertical"]'),
    ).not.toBeNull();
  });

  it("Button teaches aria-expanded as disclosure state, not the chevron", () => {
    render(PilotSpecimenHarness, { props: { specimen: ButtonSpecimen } });
    // The contract separates the visual chevron from `ariaExpanded`, the
    // state a screen reader hears. A chevron alone is not disclosure evidence.
    const disclosure = document.querySelector(
      ".poodle-button[aria-expanded]",
    ) as HTMLElement | null;
    expect(disclosure).not.toBeNull();
    expect(disclosure?.getAttribute("aria-expanded")).toBe("false");

    const chevronOnly = [...document.querySelectorAll(".poodle-button")].filter(
      (b) =>
        b.querySelector('[class*="chevron"]') && !b.hasAttribute("aria-expanded"),
    );
    expect(chevronOnly.length).toBeGreaterThan(0);
  });

  it("Button shows every tone the contract defines", () => {
    render(PilotSpecimenHarness, { props: { specimen: ButtonSpecimen } });
    const labels = [...document.querySelectorAll(".poodle-button")].map(
      (button) => (button.textContent ?? "").trim(),
    );
    // default / danger / success / warning, one variant each — not the grid.
    for (const label of ["Default", "Delete", "Approve", "Override"]) {
      expect(labels).toContain(label);
    }
  });
});

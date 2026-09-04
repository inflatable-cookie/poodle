import { readFileSync } from "node:fs";
import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import TabsFillLayoutHarness from "./TabsFillLayoutHarness.svelte";

/**
 * The g16.102 fill-layout seam proofs.
 *
 * The consumer evidence this replaces: three products overrode
 * `.poodle-tabs`/`.poodle-tabs__panel` through `:global` to force
 * `height: 100%`, `grid-template-rows: auto minmax(0, 1fr)`, and a scrollable
 * panel. `layout="fill"` must produce that same computed layout with no
 * internal selectors touched.
 *
 * happy-dom cannot cascade stylesheets at computed-value time (the
 * DockRegionTabPassThroughs underline-hook suite records the same
 * limitation), so the CSS half of the contract is asserted on the shipped
 * stylesheet declarations themselves.
 */
const tabsCss = readFileSync(
  new URL("../../../core/src/styles/tabs.css", `file://${import.meta.dirname}/`),
  "utf8",
);

describe("Tabs fill layout (svelte)", () => {
  it("emits data-layout=fill and renders the oracle scenario: a 300px host with long panel content", () => {
    const { container } = render(TabsFillLayoutHarness, {
      props: { layout: "fill", hostHeight: "300px" },
    });

    const root = container.querySelector<HTMLElement>(".poodle-tabs")!;
    expect(root.getAttribute("data-layout")).toBe("fill");

    // The panel and the strip are both present: the fill grid rows apply to
    // a real panel, and the strip is the `auto` row above it.
    expect(container.querySelector<HTMLElement>(".poodle-tabs__panel")).not.toBeNull();
    expect(container.querySelectorAll<HTMLElement>("[data-testid='panel-row']")).toHaveLength(40);
  });

  it("auto keeps the natural-height grid and the data-layout default", () => {
    const { container } = render(TabsFillLayoutHarness, { props: {} });
    const root = container.querySelector<HTMLElement>(".poodle-tabs")!;
    expect(root.getAttribute("data-layout")).toBe("auto");
  });
});

describe("Tabs fill layout rules (tabs.css)", () => {
  it("fill sizes the root to its container with strip + scrolling panel rows", () => {
    expect(tabsCss).toContain(`.poodle-tabs[data-layout="fill"] {
    height: 100%;
    grid-template-rows: auto minmax(0, 1fr);
  }`);
  });

  it("the fill panel may shrink to zero and owns scrolling", () => {
    expect(tabsCss).toContain(`.poodle-tabs[data-layout="fill"] .poodle-tabs__panel {
    min-height: 0;
    overflow: auto;
  }`);
  });

  it("vertical fill stretches one row instead of row tracks", () => {
    expect(tabsCss).toContain(`.poodle-tabs[data-layout="fill"][data-orientation="vertical"] {
    grid-template-rows: minmax(0, 1fr);
  }`);
    expect(tabsCss).toContain(
      `.poodle-tabs[data-layout="fill"][data-orientation="vertical"] .poodle-tabs__panel {
    align-self: stretch;
  }`,
    );
  });

  it("auto is unchanged: every root row-track rule is gated on the fill attribute", () => {
    // happy-dom cannot cascade stylesheets, so gating is asserted on rule
    // structure: any rule that sets root row tracks must be fill-gated.
    const rules = tabsCss.match(/\.poodle-tabs[^{]*\{[^}]*\}/g) ?? [];
    const rowTrackRules = rules.filter((rule) => rule.includes("grid-template-rows"));
    expect(rowTrackRules).toHaveLength(2);
    for (const rule of rowTrackRules) {
      expect(rule).toContain('data-layout="fill"');
    }
  });

  it("declares the panel padding hook with the historical padding as default", () => {
    expect(tabsCss).toContain(
      "--poodle-tabs-panel-padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);",
    );
    expect(tabsCss).toContain("padding: var(--poodle-tabs-panel-padding);");
  });
});

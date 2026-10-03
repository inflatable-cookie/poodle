import { readFileSync } from "node:fs";
import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Tabs } from "../src/Tabs";

/**
 * The g16.102 fill-layout seam proofs, React side. Mirrors the Svelte suite:
 * `layout` emits `data-layout`, and the CSS half of the contract is asserted
 * on the shipped stylesheet declarations because happy-dom cannot cascade
 * stylesheets at computed-value time (the DockRegionTabPassThroughs
 * underline-hook suite records the same limitation).
 */
const tabsCss = readFileSync(
  new URL("../../../core/src/styles/tabs.css", `file://${import.meta.dirname}/`),
  "utf8",
);
const tokenCss = readFileSync(
  new URL("../../../tokens/artifacts/css/poodle-tokens.css", `file://${import.meta.dirname}/`),
  "utf8",
);

const items = [
  { value: "mix", label: "Mix" },
  { value: "master", label: "Master" },
];

describe("Tabs fill layout (react)", () => {
  it("emits data-layout=fill and renders the oracle scenario: a 300px host with long panel content", () => {
    const { container } = render(
      <div style={{ height: "300px" }}>
        <Tabs layout="fill" ariaLabel="Fill layout" items={items}>
          {(value) => (
            <div>
              <p>Panel for {value}</p>
              {Array.from({ length: 40 }, (_, row) => (
                <div key={row}>Fill panel row {row}</div>
              ))}
            </div>
          )}
        </Tabs>
      </div>,
    );

    const root = container.querySelector<HTMLElement>(".poodle-tabs");
    expect(root?.getAttribute("data-layout")).toBe("fill");
    expect(root?.getAttribute("data-panel-scroll")).toBe("panel");
    expect(container.querySelector<HTMLElement>(".poodle-tabs__panel")).not.toBeNull();
    expect(container.textContent).toContain("Fill panel row 39");
  });

  it("auto keeps the natural-height grid and the data-layout default", () => {
    const { container } = render(
      <Tabs ariaLabel="Auto layout" items={items}>
        {(value) => <p>Panel for {value}</p>}
      </Tabs>,
    );
    const root = container.querySelector<HTMLElement>(".poodle-tabs");
    expect(root?.getAttribute("data-layout")).toBe("auto");
    expect(root?.getAttribute("data-panel-scroll")).toBe("panel");
  });

  it("supports a flex child without an explicit parent block size and lets content own scrolling", () => {
    const { container } = render(
      <div style={{ display: "flex", flexDirection: "column", height: "auto" }}>
        <Tabs layout="fill" panelScroll="content" ariaLabel="Fill layout" items={items}>
          {(value) => (
            <div data-testid="content-scroll" style={{ maxHeight: "150px", overflow: "auto", minHeight: 0 }}>
              <p>Panel for {value}</p>
              {Array.from({ length: 40 }, (_, row) => (
                <div key={row}>Fill panel row {row}</div>
              ))}
            </div>
          )}
        </Tabs>
      </div>,
    );

    const root = container.querySelector<HTMLElement>(".poodle-tabs")!;
    const panel = container.querySelector<HTMLElement>(".poodle-tabs__panel")!;
    const content = container.querySelector<HTMLElement>("[data-testid='content-scroll']")!;
    const host = container.firstElementChild as HTMLElement;
    expect(host.style.display).toBe("flex");
    expect(host.style.flexDirection).toBe("column");
    expect(host.style.height).toBe("auto");
    expect(root.getAttribute("data-panel-scroll")).toBe("content");
    expect(panel.contains(content)).toBe(true);
    expect(content.style.overflow).toBe("auto");
  });
});

describe("Tabs fill layout rules (tabs.css)", () => {
  it("fill sizes the root to its container with strip + scrolling panel rows", () => {
    expect(tabsCss).toContain(`.poodle-tabs[data-layout="fill"] {
    height: 100%;
    flex: 1 1 auto;
    min-height: 0;
    grid-template-rows: auto minmax(0, 1fr);
  }`);
  });

  it("the fill panel may shrink to zero and owns scrolling", () => {
    expect(tabsCss).toContain(`.poodle-tabs[data-layout="fill"] .poodle-tabs__panel {
    min-height: 0;
    overflow: auto;
  }`);
  });

  it("content scroll ownership leaves overflow to the consumer's content", () => {
    expect(tabsCss).toContain(`.poodle-tabs[data-layout="fill"][data-panel-scroll="content"] .poodle-tabs__panel {
    overflow: visible;
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

  it("routes each Tabs chrome token through its rendered CSS property", () => {
    expect(tabsCss).toContain("gap: var(--poodle-tabs-gap);");
    expect(tabsCss).toContain("padding: var(--poodle-tabs-list-padding);");
    expect(tabsCss).toContain("background: var(--poodle-tabs-list-background);");
    expect(tabsCss).toContain("background: var(--poodle-recipe-tabs-block-list-fill, var(--poodle-tabs-list-background));");
    expect(tabsCss).toContain("border-bottom: var(--poodle-tabs-list-border);");
    expect(tabsCss).toContain("border-right: var(--poodle-tabs-list-border);");
    expect(tokenCss).toContain("--poodle-tabs-gap: var(--poodle-space-stack-md);");
    expect(tokenCss).toContain("--poodle-tabs-list-padding: 0;");
    expect(tokenCss).toContain("--poodle-tabs-list-background: color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent);");
    expect(tokenCss).toContain("--poodle-tabs-list-border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent);");
  });
});

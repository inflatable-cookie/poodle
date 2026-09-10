import { readFileSync } from "node:fs";
import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Tabs } from "../src/Tabs";

/**
 * Card inactive surfaces (g18.004). Mirrors the Svelte suite: the fill lives
 * on the item wrapper so a closable tab's close button sits inside the card.
 * happy-dom cannot resolve recipe var() chains, so the CSS half is the
 * shipped declaration; core's cascade suite computes the winning background.
 */
const tabsCss = readFileSync(
  new URL("../../../core/src/styles/tabs.css", `file://${import.meta.dirname}/`),
  "utf8",
);

const CARD_FILL =
  "var(--poodle-recipe-tabs-card-item-fill, var(--poodle-color-background-surface))";

const items = [
  { value: "mix", label: "Mix" },
  { value: "master", label: "Master", disabled: true },
  { value: "notes", label: "Notes", closable: true },
];

describe("Tabs card item surfaces (react)", () => {
  it("the card-item fill is a recipe hook on the item wrapper, not the tab button", () => {
    expect(tabsCss).toContain(`.poodle-tabs[data-variant="card"] .poodle-tabs__item {
    border-radius: var(--poodle-radius-control);
    background: ${CARD_FILL};
  }`);
    expect(tabsCss).toContain(
      `.poodle-tabs[data-variant="card"][data-active-fill="none"] .poodle-tabs__item[data-selected="true"]`,
    );
    expect(tabsCss).not.toContain(`.poodle-tabs[data-variant="pill"] .poodle-tabs__item {
    border-radius: 999px;
    background: ${CARD_FILL};
  }`);
  });

  it("the closable item wrapper encloses the tab button and the close affordance", () => {
    const { container } = render(<Tabs items={items} defaultValue="mix" />);
    const notes = [...container.querySelectorAll<HTMLElement>(".poodle-tabs__item")].find((item) =>
      item.querySelector('.poodle-tabs__tab[data-value="notes"]'),
    );
    expect(notes).toBeTruthy();
    expect(notes!.querySelector(".poodle-tabs__tab")).not.toBeNull();
    expect(notes!.querySelector(".poodle-tabs__close")).not.toBeNull();
    expect(notes!.querySelector(".poodle-tabs__tab")!.contains(notes!.querySelector(".poodle-tabs__close"))).toBe(
      false,
    );
  });

  it("disabled inactive cards still render as item wrappers under the card variant", () => {
    const { container } = render(<Tabs items={items} defaultValue="mix" variant="card" />);
    const root = container.querySelector(".poodle-tabs")!;
    expect(root.getAttribute("data-variant")).toBe("card");
    const master = [...container.querySelectorAll<HTMLElement>(".poodle-tabs__item")].find((item) =>
      item.querySelector('.poodle-tabs__tab[data-value="master"]'),
    );
    expect(master).toBeTruthy();
    expect(master!.getAttribute("data-selected")).toBe("false");
    expect((master!.querySelector(".poodle-tabs__tab") as HTMLButtonElement).disabled).toBe(true);
  });
});

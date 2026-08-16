import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Card from "../src/Card.svelte";
import CardRegionsHarness from "./CardRegionsHarness.svelte";

describe("Card (svelte)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-card") as HTMLElement;

  it("projects variant, layout, interactive, and selected data attributes", () => {
    const root = rootOf(
      render(Card, {
        props: { variant: "elevated", layout: "horizontal", interactive: true, selected: true },
      }).container,
    );
    expect(root.dataset.variant).toBe("elevated");
    expect(root.dataset.layout).toBe("horizontal");
    expect(root.dataset.interactive).toBe("true");
    expect(root.dataset.selected).toBe("true");
  });

  it("renders media, header, body, and footer regions from snippets", () => {
    const { container } = render(CardRegionsHarness, {
      props: {
        showMedia: true,
        showHeader: true,
        showFooter: true,
        headerText: "Title",
        bodyText: "Body copy",
        footerText: "Actions",
      },
    });
    expect(container.querySelector(".poodle-card__media img")).not.toBeNull();
    expect(container.querySelector(".poodle-card__header .harness-header")?.textContent).toBe(
      "Title",
    );
    expect(container.querySelector(".poodle-card__body .harness-body")?.textContent).toBe(
      "Body copy",
    );
    expect(container.querySelector(".poodle-card__footer .harness-footer")?.textContent).toBe(
      "Actions",
    );
  });

  it("omits media, header, and footer regions when their snippets are absent", () => {
    const { container } = render(CardRegionsHarness, {
      props: { bodyText: "Only body" },
    });
    expect(container.querySelector(".poodle-card__media")).toBeNull();
    expect(container.querySelector(".poodle-card__header")).toBeNull();
    expect(container.querySelector(".poodle-card__footer")).toBeNull();
    expect(container.querySelector(".poodle-card__body .harness-body")?.textContent).toBe(
      "Only body",
    );
  });
});

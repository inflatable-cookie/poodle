import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ListGrid from "../src/ListGrid.svelte";
import { asSnippet } from "./snippet";

describe("ListGrid (svelte)", () => {
  it("emits the auto-fill column template with the default gap", () => {
    const { container } = render(ListGrid, {
      props: { minItemWidth: 12, maxColumns: null, children: asSnippet(() => "items") },
    });
    const content = container.querySelector(".poodle-list-grid__content") as HTMLElement;
    expect(content.style.gridTemplateColumns).toContain("minmax(min(12em, 100%), 1fr)");
    expect(content.style.gap).toBe("1.25rem");
  });

  it("caps the column count with a maxColumns formula", () => {
    const { container } = render(ListGrid, {
      props: { minItemWidth: "8rem", maxColumns: 3, gap: 16, children: asSnippet(() => "items") },
    });
    const content = container.querySelector(".poodle-list-grid__content") as HTMLElement;
    expect(content.style.gridTemplateColumns).toContain(
      "max(8rem, calc((100% - (3 - 1) * 16px) / 3))",
    );
    expect(content.style.gap).toBe("16px");
  });

  it("renders compact variant as a single column", () => {
    const { container } = render(ListGrid, {
      props: { variant: "compact", children: asSnippet(() => "items") },
    });
    const content = container.querySelector(".poodle-list-grid__content") as HTMLElement;
    expect(content.style.gridTemplateColumns).toBe("1fr");
    expect(content.style.gap).toBe("0.5rem");
  });

  it("renders the actions header only when an actions snippet is present", () => {
    const withActions = render(ListGrid, {
      props: { actions: asSnippet(() => "Toolbar"), children: asSnippet(() => "items") },
    });
    expect(withActions.container.querySelector(".poodle-list-grid__header")).not.toBeNull();

    const plain = render(ListGrid, { props: { children: asSnippet(() => "items") } });
    expect(plain.container.querySelector(".poodle-list-grid__header")).toBeNull();
  });
});

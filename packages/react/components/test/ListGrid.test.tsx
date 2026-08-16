import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { ListGrid } from "../src/ListGrid";

describe("ListGrid (react)", () => {
  it("emits the auto-fill column template with the default gap", () => {
    const { container } = render(<ListGrid minItemWidth={12} maxColumns={null}>items</ListGrid>);
    const content = container.querySelector(".poodle-list-grid__content") as HTMLElement;
    expect(content.style.gridTemplateColumns).toContain("minmax(min(12em, 100%), 1fr)");
    expect(content.style.gap).toBe("1.25rem");
  });

  it("caps the column count with a maxColumns formula", () => {
    const { container } = render(<ListGrid minItemWidth="8rem" maxColumns={3} gap={16}>items</ListGrid>);
    const content = container.querySelector(".poodle-list-grid__content") as HTMLElement;
    expect(content.style.gridTemplateColumns).toContain(
      "max(8rem, calc((100% - (3 - 1) * 16px) / 3))",
    );
    expect(content.style.gap).toBe("16px");
  });

  it("renders compact variant as a single column", () => {
    const { container } = render(<ListGrid variant="compact">items</ListGrid>);
    const content = container.querySelector(".poodle-list-grid__content") as HTMLElement;
    expect(content.style.gridTemplateColumns).toBe("1fr");
    expect(content.style.gap).toBe("0.5rem");
  });

  it("renders the actions header only when actions are present", () => {
    const withActions = render(<ListGrid actions={<span>Toolbar</span>}>items</ListGrid>);
    expect(withActions.container.querySelector(".poodle-list-grid__header")).not.toBeNull();

    const plain = render(<ListGrid>items</ListGrid>);
    expect(plain.container.querySelector(".poodle-list-grid__header")).toBeNull();
  });
});

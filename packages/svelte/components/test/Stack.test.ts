import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Stack from "../src/Stack.svelte";

describe("Stack (svelte)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-stack") as HTMLElement;

  it("applies direction and gap scale as inline styles", () => {
    const root = rootOf(render(Stack, { props: { direction: "row", gap: "lg" } }).container);
    expect(root.style.flexDirection).toBe("row");
    expect(root.style.gap).toBe("var(--poodle-space-panel-x)");
  });

  it("defaults column align to stretch and row align to center", () => {
    const column = rootOf(render(Stack, {}).container);
    expect(column.style.alignItems).toBe("stretch");

    const row = rootOf(render(Stack, { props: { direction: "row" } }).container);
    expect(row.style.alignItems).toBe("center");
  });

  it("maps justify and wrap props onto the flex container", () => {
    const root = rootOf(
      render(Stack, { props: { justify: "between", wrap: true } }).container,
    );
    expect(root.style.justifyContent).toBe("space-between");
    expect(root.style.flexWrap).toBe("wrap");
  });

  it("projects the semantic role and merges a caller class", () => {
    const root = rootOf(
      render(Stack, { props: { asRole: "region", ariaLabel: "Toolbar", class: "my-stack" } })
        .container,
    );
    expect(root.getAttribute("role")).toBe("region");
    expect(root.getAttribute("aria-label")).toBe("Toolbar");
    expect(root.className).toContain("my-stack");
  });
});

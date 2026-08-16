import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Grid from "../src/Grid.svelte";

describe("Grid (svelte)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-grid") as HTMLElement;

  it("applies column tracks and the gap scale as inline styles", () => {
    const root = rootOf(render(Grid, { props: { columns: "1fr 2fr", gap: "sm" } }).container);
    expect(root.style.gridTemplateColumns).toBe("1fr 2fr");
    expect(root.style.gap).toBe("var(--poodle-space-inline-sm)");
  });

  it("omits row tracks when rows is null and applies them when set", () => {
    const plain = rootOf(render(Grid, {}).container);
    expect(plain.style.gridTemplateRows).toBe("");

    const withRows = rootOf(render(Grid, { props: { rows: "auto 1fr" } }).container);
    expect(withRows.style.gridTemplateRows).toBe("auto 1fr");
  });

  it("projects the semantic role and label only when opted in", () => {
    const plain = rootOf(render(Grid, {}).container);
    expect(plain.getAttribute("role")).toBeNull();

    const region = rootOf(
      render(Grid, { props: { asRole: "region", ariaLabel: "Tiles" } }).container,
    );
    expect(region.getAttribute("role")).toBe("region");
    expect(region.getAttribute("aria-label")).toBe("Tiles");
  });
});

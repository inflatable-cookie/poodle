import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Box from "../src/Box.svelte";

describe("Box (svelte)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-box") as HTMLElement;

  it("maps the padding scale to space tokens", () => {
    const root = rootOf(render(Box, { props: { padding: "lg" } }).container);
    expect(root.style.padding).toBe("var(--poodle-space-panel-x)");
    const none = rootOf(render(Box, {}).container);
    expect(["0", "0px"]).toContain(none.style.padding);
  });

  it("applies explicit width and height constraints inline", () => {
    const root = rootOf(
      render(Box, { props: { width: "12rem", height: "6rem" } }).container,
    );
    expect(root.style.width).toBe("12rem");
    expect(root.style.height).toBe("6rem");
  });

  it("projects the semantic role and label only when opted in", () => {
    const plain = rootOf(render(Box, {}).container);
    expect(plain.getAttribute("role")).toBeNull();

    const region = rootOf(
      render(Box, { props: { asRole: "region", ariaLabel: "Tools" } }).container,
    );
    expect(region.getAttribute("role")).toBe("region");
    expect(region.getAttribute("aria-label")).toBe("Tools");
  });
});

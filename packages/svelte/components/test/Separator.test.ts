import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Separator from "../src/Separator.svelte";

describe("Separator (svelte)", () => {
  it("hides decorative separators from assistive technology", () => {
    const { container } = render(Separator);
    const root = container.querySelector(".poodle-separator")!;
    expect(root.getAttribute("aria-hidden")).toBe("true");
    expect(root.getAttribute("role")).toBeNull();
    expect(root.getAttribute("aria-orientation")).toBeNull();
  });

  it("exposes a semantic separator with orientation", () => {
    const { container } = render(Separator, { props: { decorative: false, orientation: "vertical" } });
    const root = container.querySelector(".poodle-separator")!;
    expect(root.getAttribute("role")).toBe("separator");
    expect(root.getAttribute("aria-orientation")).toBe("vertical");
    expect(root.getAttribute("aria-hidden")).toBeNull();
  });

  it("lands orientation and tone on data attributes", () => {
    const { container } = render(Separator, { props: { tone: "default", orientation: "vertical" } });
    const root = container.querySelector(".poodle-separator")!;
    expect(root.getAttribute("data-orientation")).toBe("vertical");
    expect(root.getAttribute("data-tone")).toBe("default");

    const defaults = render(Separator);
    const defaultRoot = defaults.container.querySelector(".poodle-separator")!;
    expect(defaultRoot.getAttribute("data-orientation")).toBe("horizontal");
    expect(defaultRoot.getAttribute("data-tone")).toBe("subtle");
  });

  it("never enters the tab order", () => {
    const { container } = render(Separator, { props: { decorative: false } });
    const root = container.querySelector(".poodle-separator")!;
    expect(root.getAttribute("tabindex")).toBeNull();
  });
});

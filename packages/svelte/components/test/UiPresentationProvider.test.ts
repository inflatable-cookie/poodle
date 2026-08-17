import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import UiPresentationProvider from "../src/UiPresentationProvider.svelte";
import UiPresentationHarness from "./UiPresentationHarness.svelte";
import { asSnippet } from "./snippet";

describe("UiPresentationProvider (svelte)", () => {
  it("sets the four presentation CSS custom properties from props", () => {
    const { container } = render(UiPresentationProvider, {
      props: { density: "compact", sizeScale: "sm", children: asSnippet(() => "content") },
    });
    const root = container.querySelector(".poodle-ui-presentation-provider") as HTMLElement;
    expect(root.style.getPropertyValue("--poodle-size-control-height")).toBe("1.75rem");
    expect(root.style.getPropertyValue("--poodle-space-control-x")).toBe("0.5rem");
    expect(root.style.getPropertyValue("--poodle-space-panel-x")).toBe("0.75rem");
    expect(root.style.getPropertyValue("--poodle-space-panel-y")).toBe("0.5rem");
  });

  it("resolves the default density and size scale", () => {
    const { container } = render(UiPresentationProvider, {
      props: { children: asSnippet(() => "content") },
    });
    const root = container.querySelector(".poodle-ui-presentation-provider") as HTMLElement;
    expect(root.style.getPropertyValue("--poodle-size-control-height")).toBe("2.25rem");
    expect(root.style.getPropertyValue("--poodle-space-control-x")).toBe("0.75rem");
  });

  it("stays layout- and accessibility-neutral around its children", () => {
    const { container } = render(UiPresentationHarness, { props: {} });
    const root = container.querySelector(".poodle-ui-presentation-provider") as HTMLElement;
    // `display: contents` lives in the shared stylesheet keyed on this class,
    // so the class is the observable hook for the neutrality requirement; the
    // test DOM does not apply the sheet.
    expect(root.className).toBe("poodle-ui-presentation-provider");
    expect(root.getAttribute("role")).toBeNull();
    expect(Array.from(root.attributes).map((a) => a.name)).toEqual(["class", "style"]);
    // The wrapper contributes no box of its own: the control is a direct child.
    expect(root.firstElementChild?.classList.contains("poodle-button")).toBe(true);
  });

  it("propagates semantic size resolution to descendant controls", () => {
    const { container } = render(UiPresentationHarness, {
      props: { density: "comfortable", sizeScale: "lg" },
    });
    const button = container.querySelector(".poodle-button") as HTMLElement;
    // sizeRole defaults to "control", so sizeScale lg resolves to lg.
    expect(button.dataset.size).toBe("lg");
  });
});
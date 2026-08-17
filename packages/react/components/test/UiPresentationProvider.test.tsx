import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Button, UiPresentationProvider } from "../src";

describe("UiPresentationProvider (react)", () => {
  it("sets the four presentation CSS custom properties from props", () => {
    const { container } = render(
      <UiPresentationProvider density="compact" sizeScale="sm">
        content
      </UiPresentationProvider>,
    );
    const root = container.querySelector(".poodle-ui-presentation-provider") as HTMLElement;
    expect(root.style.getPropertyValue("--poodle-size-control-height")).toBe("1.75rem");
    expect(root.style.getPropertyValue("--poodle-space-control-x")).toBe("0.5rem");
    expect(root.style.getPropertyValue("--poodle-space-panel-x")).toBe("0.75rem");
    expect(root.style.getPropertyValue("--poodle-space-panel-y")).toBe("0.5rem");
  });

  it("resolves the default density and size scale", () => {
    const { container } = render(<UiPresentationProvider>content</UiPresentationProvider>);
    const root = container.querySelector(".poodle-ui-presentation-provider") as HTMLElement;
    expect(root.style.getPropertyValue("--poodle-size-control-height")).toBe("2.25rem");
    expect(root.style.getPropertyValue("--poodle-space-control-x")).toBe("0.75rem");
  });

  it("stays layout- and accessibility-neutral around its children", () => {
    const { container } = render(
      <UiPresentationProvider>
        <Button ariaLabel="Action">Action</Button>
      </UiPresentationProvider>,
    );
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
    const { container } = render(
      <UiPresentationProvider density="comfortable" sizeScale="lg">
        <Button ariaLabel="Action">Action</Button>
      </UiPresentationProvider>,
    );
    const button = container.querySelector(".poodle-button") as HTMLElement;
    // sizeRole defaults to "control", so sizeScale lg resolves to lg.
    expect(button.dataset.size).toBe("lg");
  });
});
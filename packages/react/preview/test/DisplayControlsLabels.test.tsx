import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { UiPresentationProvider } from "@inflatable-cookie/poodle-react";
import { DisplayControls } from "../src/gallery/DisplayControls";
import { previewShell } from "../src/generated/preview-shell";

/**
 * The preview DisplayControls' accessible labels come from the scene, as
 * the visible Eyebrow labels do — never hard-coded in the shell. The scene
 * is the capability contract (card 035 R3/R4); renaming a control there
 * must move the accessible name with the visible one.
 */
function sceneLabel(kind: string): string {
  const control = previewShell.controls.find((entry) => entry.kind === kind) as
    | { label: string }
    | undefined;
  if (!control) throw new Error(`scene has no ${kind} control`);
  return control.label;
}

describe("DisplayControls scene labels (react)", () => {
  it("names every control from the scene", () => {
    const { container } = render(
      <UiPresentationProvider sizeScale="sm" density="compact">
        <DisplayControls
          theme="eclipse"
          density="compact"
          controlSize="sm"
          search=""
          contrast={0.5}
        />
      </UiPresentationProvider>,
    );

    // One assertion per scene-driven control: the accessible name equals
    // the scene label the visible Eyebrow already prints.
    const groups = [...container.querySelectorAll(".poodle-toggle-group")];
    expect(groups).toHaveLength(2);
    expect(groups[0]?.getAttribute("aria-label")).toBe(sceneLabel("density"));
    expect(groups[1]?.getAttribute("aria-label")).toBe(sceneLabel("size"));

    const slider = container.querySelector('[role="slider"]') as HTMLElement;
    expect(slider.getAttribute("aria-label")).toBe(sceneLabel("contrast"));

    const search = container.querySelector("input.poodle-text-input__control") as HTMLElement;
    expect(search.getAttribute("aria-label")).toBe(sceneLabel("search"));

    // And the visible labels agree: each group's Eyebrow prints the same
    // scene label its control announces.
    for (const group of container.querySelectorAll(".poodle-display-controls__group")) {
      const eyebrow = group.querySelector(".poodle-eyebrow")?.textContent?.trim();
      const control = group.querySelector("[aria-label]") as HTMLElement | null;
      expect(control?.getAttribute("aria-label")).toBe(eyebrow);
    }
  });
});

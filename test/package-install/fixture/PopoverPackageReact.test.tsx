import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import "@inflatable-cookie/poodle-core/styles/button.css";
import "@inflatable-cookie/poodle-core/styles/popover.css";
import {
  Button,
  Popover,
  type PopoverTriggerState,
} from "@inflatable-cookie/poodle-react";

describe("packed @inflatable-cookie/poodle-react Popover", () => {
  it("threads expanded/controls/disabled to the real control", () => {
    const view = render(
      <Popover
        triggerIsInteractive
        ariaLabel="Packed popover"
        trigger={(state: PopoverTriggerState) => (
          <Button
            ariaExpanded={state.expanded}
            controls={state.controls}
            disabled={state.disabled}
          >
            Packed trigger
          </Button>
        )}
      >
        <p>Packed surface content</p>
      </Popover>,
    );

    const wrapper = view.container.querySelector(
      ".poodle-popover__trigger",
    ) as HTMLElement;
    expect(wrapper.getAttribute("role")).toBeNull();
    expect(wrapper.getAttribute("tabindex")).toBeNull();

    const trigger = wrapper.querySelector("button.poodle-button") as HTMLButtonElement;
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(trigger.getAttribute("aria-controls")).toBeNull();

    fireEvent.click(trigger);

    const surface = document.body.querySelector(
      '.poodle-popover__surface[data-part="surface"]',
    ) as HTMLElement;
    expect(surface).not.toBeNull();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("aria-controls")).toBe(surface.id);

    view.unmount();
  });
});

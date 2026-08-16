import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Collapsible } from "../src/Collapsible";

describe("Collapsible (react)", () => {
  it("renders content open from defaultOpen and toggles via the trigger", () => {
    const { container, getByRole } = render(
      <Collapsible title="Details" defaultOpen>
        Content
      </Collapsible>,
    );
    const trigger = getByRole("button");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(container.querySelector(".poodle-collapsible__content")).not.toBeNull();

    fireEvent.click(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-collapsible__content")).toBeNull();
  });

  it("keeps content closed until the trigger is pressed", () => {
    const { container, getByRole } = render(<Collapsible title="Details">Content</Collapsible>);
    const trigger = getByRole("button");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-collapsible__content")).toBeNull();

    fireEvent.click(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
  });

  it("reports open changes and refuses to toggle when disabled", () => {
    const onOpenChange = vi.fn();
    const { getByRole } = render(<Collapsible title="Details" onOpenChange={onOpenChange} />);
    const trigger = getByRole("button");

    fireEvent.click(trigger);
    expect(onOpenChange).toHaveBeenCalledWith(true);

    const disabled = render(<Collapsible title="Details" disabled />);
    const disabledTrigger = disabled.container.querySelector("button") as HTMLButtonElement;
    expect(disabledTrigger.disabled).toBe(true);
    fireEvent.click(disabledTrigger);
    expect(disabledTrigger.getAttribute("aria-expanded")).toBe("false");
  });

  it("projects the open state from a controlled prop", () => {
    const { container, getByRole } = render(<Collapsible open={false}>Content</Collapsible>);
    expect(getByRole("button").getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-collapsible__content")).toBeNull();
  });
});

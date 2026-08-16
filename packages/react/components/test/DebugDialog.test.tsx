import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { DebugDialog } from "../src/DebugDialog";

describe("DebugDialog (react)", () => {
  it("renders nothing when the value is null", () => {
    const { container } = render(<DebugDialog value={null} />);
    expect(container.querySelector("button")).toBeNull();
  });

  it("renders the trigger button only when a value is present", async () => {
    const { container } = render(<DebugDialog value={{ status: "ok" }} triggerLabel="Inspect" />);
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Inspect"),
    ) as HTMLButtonElement;
    expect(trigger).not.toBeNull();

    fireEvent.click(trigger);
    await waitFor(() => {
      expect(document.querySelector(".poodle-dialog__surface")).not.toBeNull();
    });
  });

  it("serialises the value as pretty-printed JSON in the code block", async () => {
    const { container } = render(
      <DebugDialog value={{ a: 1, nested: { b: 2 } }} triggerLabel="Inspect" />,
    );
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Inspect"),
    ) as HTMLButtonElement;
    fireEvent.click(trigger);
    await waitFor(() => {
      expect(document.querySelector(".poodle-dialog__surface")).not.toBeNull();
    });

    const surface = document.querySelector(".poodle-dialog__surface") as HTMLElement;
    expect(surface.textContent).toContain('"a": 1');
    expect(surface.textContent).toContain('"nested": {');
  });
});
import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import DebugDialog from "../src/DebugDialog.svelte";

function flush(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

describe("DebugDialog (svelte)", () => {
  it("renders nothing when the value is null", () => {
    const { container } = render(DebugDialog, { props: { value: null } });
    expect(container.querySelector("button")).toBeNull();
  });

  it("renders the trigger button only when a value is present", async () => {
    const { container } = render(DebugDialog, {
      props: { value: { status: "ok" }, triggerLabel: "Inspect" },
    });
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Inspect"),
    ) as HTMLButtonElement;
    expect(trigger).not.toBeNull();

    await fireEvent.click(trigger);
    await flush();
    expect(document.querySelector(".poodle-dialog__surface")).not.toBeNull();
  });

  it("serialises the value as pretty-printed JSON in the code block", async () => {
    const { container } = render(DebugDialog, {
      props: { value: { a: 1, nested: { b: 2 } }, triggerLabel: "Inspect" },
    });
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Inspect"),
    ) as HTMLButtonElement;
    await fireEvent.click(trigger);
    await flush();

    const surface = document.querySelector(".poodle-dialog__surface") as HTMLElement;
    expect(surface.textContent).toContain('"a": 1');
    expect(surface.textContent).toContain('"nested": {');
  });
});
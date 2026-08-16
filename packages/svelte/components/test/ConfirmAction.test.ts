import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ConfirmAction from "../src/ConfirmAction.svelte";
import ConfirmActionTriggerHarness from "./ConfirmActionTriggerHarness.svelte";

// The AlertDialog inside ConfirmAction portals to the theme root, so its
// surface is not reachable from the render container — same pattern as the
// AlertDialog suites.
function flush(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

describe("ConfirmAction (svelte)", () => {
  it("renders the default trigger button with the trigger label", () => {
    const { container } = render(ConfirmAction, {
      props: { title: "Delete record?", triggerLabel: "Delete record" },
    });
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete record"),
    ) as HTMLButtonElement;
    expect(trigger).not.toBeNull();
    expect(container.querySelector(".poodle-confirm-action__trigger")).toBeNull();
  });

  it("opens the alert dialog from the default trigger and confirms", async () => {
    const onConfirm = vi.fn().mockResolvedValue(undefined);
    const { container } = render(ConfirmAction, {
      props: { title: "Delete record?", description: "Permanent.", onConfirm },
    });
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete"),
    ) as HTMLButtonElement;
    await fireEvent.click(trigger);
    await flush();

    const surface = document.querySelector(".poodle-dialog__surface") as HTMLElement;
    expect(surface.getAttribute("role")).toBe("alertdialog");
    expect(surface.textContent).toContain("Delete record?");

    const confirm = [...document.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Confirm"),
    ) as HTMLButtonElement;
    await fireEvent.click(confirm);
    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("runs onCancel when the dialog is dismissed through the cancel action", async () => {
    const onCancel = vi.fn();
    const { container } = render(ConfirmAction, {
      props: { title: "Delete record?", onCancel },
    });
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete"),
    ) as HTMLButtonElement;
    await fireEvent.click(trigger);
    await flush();

    const cancel = [...document.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Cancel"),
    ) as HTMLButtonElement;
    await fireEvent.click(cancel);
    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it("renders a custom trigger when the snippet is provided", async () => {
    const { container } = render(ConfirmActionTriggerHarness, { props: { title: "Clear all?" } });
    const wrapper = container.querySelector(".poodle-confirm-action__trigger") as HTMLElement;
    expect(wrapper.getAttribute("role")).toBe("presentation");
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Clear filters"),
    ) as HTMLButtonElement;
    await fireEvent.click(trigger);
    await flush();
    expect(document.querySelector(".poodle-dialog__surface")).not.toBeNull();
  });

  it("derives a danger tone for the default trigger", () => {
    const { container } = render(ConfirmAction, {
      props: { title: "Delete record?", triggerLabel: "Delete record", tone: "danger" },
    });
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete record"),
    ) as HTMLButtonElement;
    expect(trigger.getAttribute("data-tone")).toBe("danger");
  });
});
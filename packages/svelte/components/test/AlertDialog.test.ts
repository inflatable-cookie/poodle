import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import AlertDialog from "../src/AlertDialog.svelte";

// The Dialog inside AlertDialog portals to the theme root, so its surface is
// not reachable from the render container — same pattern as the Dialog suites.
function flush(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

describe("AlertDialog (svelte)", () => {
  const renderAlertDialog = (props: Record<string, unknown> = {}) =>
    render(AlertDialog, {
      props: { title: "Delete entry?", open: true, ...props },
    });

  it("opens as an alertdialog with the given title and item detail", async () => {
    renderAlertDialog({ itemLabel: "Entry", itemValue: "Q3 report" });
    await flush();

    const surface = document.querySelector(".poodle-dialog__surface") as HTMLElement;
    expect(surface.getAttribute("role")).toBe("alertdialog");
    expect(surface.textContent).toContain("Delete entry?");
    expect(document.querySelector(".poodle-alert-dialog__item-detail")?.textContent).toContain(
      "Entry:",
    );
  });

  it("runs onConfirm and closes on the confirm button", async () => {
    const onConfirm = vi.fn().mockResolvedValue(undefined);
    const onOpenChange = vi.fn();
    renderAlertDialog({ onConfirm, onOpenChange });
    await flush();

    const confirm = [...document.querySelectorAll("button")].find((el) =>
      el.textContent?.includes("Confirm"),
    ) as HTMLButtonElement;
    await fireEvent.click(confirm);

    expect(onConfirm).toHaveBeenCalledTimes(1);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("runs onCancel and closes on the cancel button", async () => {
    const onCancel = vi.fn();
    const onOpenChange = vi.fn();
    renderAlertDialog({ onCancel, onOpenChange });
    await flush();

    const cancel = [...document.querySelectorAll("button")].find((el) =>
      el.textContent?.includes("Cancel"),
    ) as HTMLButtonElement;
    await fireEvent.click(cancel);

    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("shows the working state and holds the dialog while onConfirm is pending", async () => {
    let resolveConfirm: (() => void) | undefined;
    const onConfirm = vi.fn().mockImplementation(
      () => new Promise<void>((resolve) => (resolveConfirm = resolve)),
    );
    renderAlertDialog({ onConfirm });
    await flush();

    const confirm = [...document.querySelectorAll("button")].find((el) =>
      el.textContent?.includes("Confirm"),
    ) as HTMLButtonElement;
    await fireEvent.click(confirm);

    expect(confirm.textContent).toContain("Working…");
    expect(confirm.disabled).toBe(true);

    resolveConfirm?.();
    await flush();
  });
});

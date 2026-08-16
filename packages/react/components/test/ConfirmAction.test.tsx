import { act, fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ConfirmAction } from "../src/ConfirmAction";

describe("ConfirmAction (react)", () => {
  it("renders the default trigger button with the trigger label", () => {
    const { container } = render(
      <ConfirmAction title="Delete record?" triggerLabel="Delete record" />,
    );
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete record"),
    ) as HTMLButtonElement;
    expect(trigger).not.toBeNull();
    expect(container.querySelector(".poodle-confirm-action__trigger")).toBeNull();
  });

  it("opens the alert dialog from the default trigger and confirms", async () => {
    const onConfirm = vi.fn().mockResolvedValue(undefined);
    const { container } = render(
      <ConfirmAction title="Delete record?" description="Permanent." onConfirm={onConfirm} />,
    );
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete"),
    ) as HTMLButtonElement;
    fireEvent.click(trigger);
    await waitFor(() => {
      expect(document.querySelector(".poodle-dialog__surface")).not.toBeNull();
    });

    const surface = document.querySelector(".poodle-dialog__surface") as HTMLElement;
    expect(surface.getAttribute("role")).toBe("alertdialog");
    expect(surface.textContent).toContain("Delete record?");

    const confirm = [...document.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Confirm"),
    ) as HTMLButtonElement;
    // handleConfirm awaits the async callback before closing, so the close
    // update lands outside the synchronous click — wrap it in act.
    await act(async () => {
      fireEvent.click(confirm);
    });
    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("runs onCancel when the dialog is dismissed through the cancel action", async () => {
    const onCancel = vi.fn();
    const { container } = render(<ConfirmAction title="Delete record?" onCancel={onCancel} />);
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete"),
    ) as HTMLButtonElement;
    fireEvent.click(trigger);
    await waitFor(() => {
      expect(document.querySelector(".poodle-dialog__surface")).not.toBeNull();
    });

    const cancel = [...document.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Cancel"),
    ) as HTMLButtonElement;
    fireEvent.click(cancel);
    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it("renders a custom trigger when the snippet is provided", async () => {
    const { container } = render(
      <ConfirmAction title="Clear all?" trigger={<button>Clear filters</button>} />,
    );
    const wrapper = container.querySelector(".poodle-confirm-action__trigger") as HTMLElement;
    expect(wrapper.getAttribute("role")).toBe("presentation");
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Clear filters"),
    ) as HTMLButtonElement;
    fireEvent.click(trigger);
    await waitFor(() => {
      expect(document.querySelector(".poodle-dialog__surface")).not.toBeNull();
    });
  });

  it("derives a danger tone for the default trigger", () => {
    const { container } = render(
      <ConfirmAction title="Delete record?" triggerLabel="Delete record" tone="danger" />,
    );
    const trigger = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Delete record"),
    ) as HTMLButtonElement;
    expect(trigger.getAttribute("data-tone")).toBe("danger");
  });
});
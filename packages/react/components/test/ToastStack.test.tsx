import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ToastStack } from "../src/ToastStack";
import type { ToastItem } from "../src/types";

const items: ToastItem[] = [
  { id: "t1", title: "Changes saved", message: "Your settings were updated.", tone: "success" },
  { id: "t2", title: "New version", actionLabel: "Update", tone: "info" },
];

describe("ToastStack (react)", () => {
  it("renders a polite live-region list with per-toast tones", () => {
    const { container } = render(<ToastStack items={items} ariaLabel="Alerts" />);
    const stack = container.querySelector(".poodle-toast-stack") as HTMLElement;
    expect(stack.tagName).toBe("UL");
    expect(stack.getAttribute("aria-label")).toBe("Alerts");
    expect(stack.getAttribute("aria-live")).toBe("polite");

    const toasts = [...container.querySelectorAll(".poodle-toast")];
    expect(toasts.length).toBe(2);
    expect(toasts[0].dataset.tone).toBe("success");
    expect(toasts[0].textContent).toContain("Changes saved");
    expect(toasts[0].textContent).toContain("Your settings were updated.");
  });

  it("escalates danger toasts to assertive announcements", () => {
    const { container } = render(
      <ToastStack items={[{ id: "t3", title: "Deploy failed", tone: "danger" }]} />,
    );
    const toast = container.querySelector(".poodle-toast") as HTMLElement;
    expect(toast.dataset.tone).toBe("danger");
    expect(toast.getAttribute("aria-live")).toBe("assertive");
  });

  it("reports dismissal with the toast id", () => {
    const onDismiss = vi.fn();
    const { container } = render(<ToastStack items={items} onDismiss={onDismiss} />);
    const dismiss = container.querySelector(
      'button[aria-label="Dismiss Changes saved"]',
    ) as HTMLButtonElement;
    fireEvent.click(dismiss);
    expect(onDismiss).toHaveBeenCalledWith("t1");
  });

  it("keeps a controlled toast live when dismiss has no removal callback", () => {
    const { container } = render(<ToastStack items={items} />);
    const dismiss = container.querySelector(
      'button[aria-label="Dismiss Changes saved"]',
    ) as HTMLButtonElement;
    const nextDismiss = container.querySelector(
      'button[aria-label="Dismiss New version"]',
    ) as HTMLButtonElement;
    dismiss.focus();

    fireEvent.click(dismiss);

    const toast = container.querySelector(".poodle-toast") as HTMLElement;
    expect(document.activeElement).toBe(nextDismiss);
    expect(toast.dataset.motion).toBe("settled");
    expect(toast.getAttribute("aria-live")).toBe("polite");
    expect(toast.hasAttribute("aria-hidden")).toBe(false);
    expect(toast.hasAttribute("inert")).toBe(false);
    expect(dismiss.getAttribute("tabindex")).not.toBe("-1");
  });

  it("keeps a controlled toast live when onDismiss does not remove it", () => {
    const onDismiss = vi.fn();
    const { container } = render(<ToastStack items={items} onDismiss={onDismiss} />);
    const dismiss = container.querySelector(
      'button[aria-label="Dismiss Changes saved"]',
    ) as HTMLButtonElement;
    dismiss.focus();

    fireEvent.click(dismiss);

    const toast = container.querySelector(".poodle-toast") as HTMLElement;
    expect(onDismiss).toHaveBeenCalledWith("t1");
    expect(toast.dataset.motion).toBe("settled");
    expect(toast.getAttribute("aria-live")).toBe("polite");
    expect(toast.hasAttribute("aria-hidden")).toBe(false);
    expect(toast.hasAttribute("inert")).toBe(false);
    expect(dismiss.getAttribute("tabindex")).not.toBe("-1");
  });

  it("reports the action with the toast id", () => {
    const onAction = vi.fn();
    const { container } = render(<ToastStack items={items} onAction={onAction} />);
    const action = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Update"),
    ) as HTMLButtonElement;
    fireEvent.click(action);
    expect(onAction).toHaveBeenCalledWith("t2");
  });

  it("renders nothing for an empty items list", () => {
    const { container } = render(<ToastStack items={[]} />);
    expect(container.querySelectorAll(".poodle-toast").length).toBe(0);
  });

  it("omitted items settle without a default-array render loop", () => {
    const { container } = render(<ToastStack />);
    expect(container.querySelector(".poodle-toast-stack")).not.toBeNull();
    expect(container.querySelectorAll(".poodle-toast").length).toBe(0);
  }, 2000);
});

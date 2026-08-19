import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import RemediationBanner from "../src/RemediationBanner.svelte";

describe("RemediationBanner (svelte)", () => {
  it("defaults to tint and projects every tone/fill combination", () => {
    const tones = ["neutral", "info", "success", "warning", "danger", "pending"] as const;
    for (const tone of tones) {
      for (const fill of ["tint", "solid"] as const) {
        const { container } = render(RemediationBanner, {
          props: { tone, fill, title: "Recovery", message: "Try again" },
        });
        const root = container.querySelector(".poodle-remediation-banner") as HTMLElement;
        expect(root.dataset.tone).toBe(tone);
        expect(root.dataset.fill).toBe(fill);
        if (tone === "pending") {
          expect(container.querySelector(".poodle-spinner")?.getAttribute("data-tone")).toBe(
            fill === "solid" ? "current" : "accent",
          );
        }
      }
    }
  });

  it("keeps announcement, action, disabled, and dismiss behavior intact", async () => {
    const onAction = vi.fn();
    const onDismiss = vi.fn();
    const { container } = render(RemediationBanner, {
      props: {
        fill: "solid",
        announceMode: "assertive",
        title: "Recovery",
        message: "Try again",
        primaryAction: { id: "retry", label: "Retry", variant: "primary", isDisabled: false },
        secondaryAction: { id: "skip", label: "Skip", variant: "secondary", isDisabled: true },
        isDismissible: true,
        onAction,
        onDismiss,
      },
    });
    const root = container.querySelector(".poodle-remediation-banner") as HTMLElement;
    expect(root.dataset.fill).toBe("solid");
    expect(root.getAttribute("role")).toBe("alert");
    expect(root.getAttribute("aria-live")).toBe("assertive");
    const buttons = container.querySelectorAll("button");
    expect(buttons).toHaveLength(3);
    expect((buttons[1] as HTMLButtonElement).disabled).toBe(true);

    await fireEvent.click(buttons[0]);
    await fireEvent.click(buttons[2]);
    expect(onAction).toHaveBeenCalledWith("retry");
    expect(onDismiss).toHaveBeenCalledTimes(1);
  });
});

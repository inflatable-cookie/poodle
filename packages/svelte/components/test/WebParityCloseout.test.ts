import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import RemediationBanner from "../src/RemediationBanner.svelte";
import StateTile from "../src/StateTile.svelte";
import ValidationSummary from "../src/ValidationSummary.svelte";

describe("web parity closeout (svelte)", () => {
  it("omits inactive validation entries and renders nothing when none remain", () => {
    const entries = [
      { fieldId: "name", label: "Name", message: "Required", validationState: "invalid" as const },
      { fieldId: "slug", label: "Slug", message: "Checking", validationState: "pending" as const },
      { fieldId: "email", label: "Email", message: "Ready", validationState: "valid" as const },
    ];
    const summary = render(ValidationSummary, { props: { entries } });
    expect(summary.getByRole("status").textContent).toContain("Name");
    expect(summary.queryByText("Slug")).toBeNull();
    expect(summary.queryByText("Email")).toBeNull();

    expect(render(ValidationSummary, { props: { entries: [] } }).container.querySelector(".poodle-validation-summary")).toBeNull();
  });

  it("keeps a custom StateTile trend readable", () => {
    const { container, getByText } = render(StateTile, { props: { label: "Capacity", value: "72%", trend: "steady" } });
    expect(getByText("steady")).toBeTruthy();
    expect(container.querySelector(".poodle-state-tile")?.getAttribute("role")).toBeNull();
  });

  it("reports remediation action and dismiss commands", async () => {
    const onAction = vi.fn();
    const onDismiss = vi.fn();
    const view = render(RemediationBanner, {
      props: {
        title: "Save failed",
        message: "Try again.",
        primaryAction: { id: "retry", label: "Retry", variant: "primary", isDisabled: false },
        isDismissible: true,
        onAction,
        onDismiss,
      },
    });
    await fireEvent.click(view.getByRole("button", { name: "Retry" }));
    await fireEvent.click(view.getByRole("button", { name: "Dismiss" }));
    expect(onAction).toHaveBeenCalledWith("retry");
    expect(onDismiss).toHaveBeenCalledOnce();
  });
});

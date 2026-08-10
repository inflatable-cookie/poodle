import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { RemediationBanner } from "../src/RemediationBanner";
import { StateTile } from "../src/StateTile";
import { ValidationSummary } from "../src/ValidationSummary";

describe("web parity closeout (react)", () => {
  it("omits inactive validation entries and renders nothing when none remain", () => {
    const entries = [
      { fieldId: "name", label: "Name", message: "Required", validationState: "invalid" as const },
      { fieldId: "slug", label: "Slug", message: "Checking", validationState: "pending" as const },
      { fieldId: "email", label: "Email", message: "Ready", validationState: "valid" as const },
    ];
    const summary = render(<ValidationSummary entries={entries} />);
    expect(summary.getByRole("status").textContent).toContain("Name");
    expect(summary.queryByText("Slug")).toBeNull();
    expect(summary.queryByText("Email")).toBeNull();
    expect(render(<ValidationSummary entries={[]} />).container.firstChild).toBeNull();
  });

  it("keeps a custom StateTile trend readable", () => {
    const view = render(<StateTile label="Capacity" value="72%" trend="steady" />);
    expect(view.getByText("steady")).toBeTruthy();
    expect(view.container.querySelector(".poodle-state-tile")?.getAttribute("role")).toBeNull();
  });

  it("reports remediation action and dismiss commands", () => {
    const onAction = vi.fn();
    const onDismiss = vi.fn();
    const view = render(
      <RemediationBanner
        title="Save failed"
        message="Try again."
        primaryAction={{ id: "retry", label: "Retry", variant: "primary", isDisabled: false }}
        isDismissible
        onAction={onAction}
        onDismiss={onDismiss}
      />,
    );
    fireEvent.click(view.getByRole("button", { name: "Retry" }));
    fireEvent.click(view.getByRole("button", { name: "Dismiss" }));
    expect(onAction).toHaveBeenCalledWith("retry");
    expect(onDismiss).toHaveBeenCalledOnce();
  });
});

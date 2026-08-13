import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { UpdateCenter } from "../src/UpdateCenter";

const offer = {
  presence: "attention" as const,
  status: { kind: "ready" as const },
  availability: { state: "offer" as const, version: "1.4.0", reason: "staged" as const, notes: null },
};

describe("UpdateCenter (react)", () => {
  it("renders nothing at all when presence is hidden", () => {
    const { container } = render(<UpdateCenter presence="hidden" />);

    expect(container.querySelector(".poodle-update-center")).toBeNull();
    expect(screen.queryByRole("button")).toBeNull();
  });

  it("draws the eye when presence is attention", () => {
    const { container } = render(<UpdateCenter {...offer} />);

    expect(screen.getByRole("button", { name: "Updates" })).toBeTruthy();
    expect(container.querySelector(".poodle-update-center__indicator")).toBeTruthy();
  });

  it("shows an unremarkable trigger when presence is quiet", () => {
    const { container } = render(<UpdateCenter {...offer} presence="quiet" />);

    expect(screen.getByRole("button", { name: "Updates" })).toBeTruthy();
    expect(container.querySelector(".poodle-update-center__indicator")).toBeNull();
  });

  it("shows the offer inside the popover", () => {
    render(<UpdateCenter {...offer} />);

    fireEvent.click(screen.getByRole("button", { name: "Updates" }));

    expect(screen.getByRole("dialog", { name: "Updates" })).toBeTruthy();
    expect(screen.getByText("Version 1.4.0 is available")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Install and restart" })).toBeTruthy();
  });

  it("swaps the icon for a determinate progress ring while downloading", () => {
    const { container } = render(
      <UpdateCenter presence="quiet" progress={{ state: "downloading", fraction: 0.42 }} />,
    );

    const ring = container.querySelector(".poodle-update-center__ring");
    expect(ring).toBeTruthy();
    expect(ring?.getAttribute("data-indeterminate")).toBe("false");
    expect(container.querySelector(".poodle-icon")).toBeNull();
    expect(container.querySelector(".poodle-update-center__ring-fill")?.getAttribute("stroke-dasharray")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Downloading update, 42%" })).toBeTruthy();
  });

  it("swaps the icon for an indeterminate ring when the fraction is null", () => {
    const { container } = render(
      <UpdateCenter presence="quiet" progress={{ state: "downloading", fraction: null }} />,
    );

    const ring = container.querySelector(".poodle-update-center__ring");
    expect(ring?.getAttribute("data-indeterminate")).toBe("true");
    expect(container.querySelector(".poodle-update-center__ring-fill")?.getAttribute("stroke-dasharray")).toBeNull();
    expect(screen.getByRole("button", { name: "Downloading update" })).toBeTruthy();
  });
});

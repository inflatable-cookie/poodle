import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { PageLoading } from "../src/PageLoading";

describe("PageLoading (react)", () => {
  it("renders a polite status region with the default label", () => {
    const { container } = render(<PageLoading visible />);
    const root = container.querySelector(".poodle-page-loading") as HTMLElement;
    expect(root.getAttribute("role")).toBe("status");
    expect(root.getAttribute("aria-live")).toBe("polite");
    expect(root.getAttribute("aria-label")).toBe("Loading");
    expect(root.querySelector(".poodle-page-loading__spinner")).not.toBeNull();
  });

  it("renders nothing when not visible", () => {
    const { container } = render(<PageLoading visible={false} />);
    expect(container.querySelector(".poodle-page-loading")).toBeNull();
  });

  it("renders the backdrop only in overlay presentation", () => {
    const overlay = render(<PageLoading presentation="overlay" />);
    expect(overlay.container.querySelector(".poodle-page-loading__backdrop")).not.toBeNull();

    const inline = render(<PageLoading presentation="inline" />);
    expect(inline.container.querySelector(".poodle-page-loading__backdrop")).toBeNull();
    expect(
      inline.container.querySelector(".poodle-page-loading")?.getAttribute("data-presentation"),
    ).toBe("inline");
  });

  it("renders the progress bar when a value is provided and omits it when indeterminate", () => {
    const determinate = render(<PageLoading value={40} max={100} />);
    expect(determinate.container.querySelector(".poodle-page-loading__progress")).not.toBeNull();

    const indeterminate = render(<PageLoading value={null} />);
    expect(indeterminate.container.querySelector(".poodle-page-loading__progress")).toBeNull();
  });

  it("renders the message and the cancel button when enabled", () => {
    const onCancel = vi.fn();
    const { container } = render(
      <PageLoading message="Uploading files..." canCancel onCancel={onCancel} />,
    );
    expect(container.querySelector(".poodle-page-loading__message")?.textContent).toContain(
      "Uploading files...",
    );
    const cancel = container.querySelector(".poodle-page-loading__cancel") as HTMLButtonElement;
    fireEvent.click(cancel);
    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it("omits the cancel button unless canCancel is true", () => {
    const { container } = render(<PageLoading message="Uploading..." />);
    expect(container.querySelector(".poodle-page-loading__cancel")).toBeNull();
  });
});
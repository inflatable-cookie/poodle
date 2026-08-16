import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";

import ErrorBoundaryHarness from "./ErrorBoundaryHarness.svelte";
import { resetBomb } from "./ErrorBoundaryBomb.svelte";

afterEach(() => {
  resetBomb();
});

describe("ErrorBoundary (svelte)", () => {
  it("renders children normally when nothing throws", () => {
    const { container } = render(ErrorBoundaryHarness, { props: { shouldThrow: false } });
    expect(container.querySelector(".harness-ok")).not.toBeNull();
  });

  it("replaces the failed subtree with an EmptyState showing the error message", () => {
    const { container } = render(ErrorBoundaryHarness, {
      props: { shouldThrow: true, title: "Failed to load" },
    });
    expect(container.querySelector(".harness-ok")).toBeNull();
    const empty = container.querySelector(".poodle-empty-state") as HTMLElement;
    expect(empty).not.toBeNull();
    expect(empty.textContent).toContain("Failed to load");
    expect(empty.textContent).toContain("render exploded");
  });

  it("re-renders children after the retry action resets the boundary", async () => {
    const { container } = render(ErrorBoundaryHarness, {
      props: { shouldThrow: true, retryLabel: "Retry now" },
    });
    const retry = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Retry now"),
    ) as HTMLButtonElement;
    await fireEvent.click(retry);
    expect(container.querySelector(".harness-ok")).not.toBeNull();
  });
});
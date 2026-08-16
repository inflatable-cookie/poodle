import { Component, type ReactNode } from "react";
import { fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ErrorBoundary } from "../src/ErrorBoundary";

// The boundary's retry re-renders children; the child keeps throwing while
// the external flag is set, so the test can clear the failure before retrying.
const failureState = { active: false };

class ThrowingChild extends Component<{ shouldThrow: boolean }> {
  render(): ReactNode {
    if (this.props.shouldThrow && failureState.active) {
      throw new Error("render exploded");
    }
    return <span className="harness-ok">ok</span>;
  }
}

afterEach(() => {
  failureState.active = false;
  vi.restoreAllMocks();
});

// React logs a console.error when an error boundary catches a throw; the
// global setup fails the test on any console.error, so silence it here.
function silenceBoundaryErrors(): void {
  vi.spyOn(console, "error").mockImplementation(() => {});
}

describe("ErrorBoundary (react)", () => {
  it("renders children normally when nothing throws", () => {
    const { container } = render(
      <ErrorBoundary>
        <span className="harness-ok">ok</span>
      </ErrorBoundary>,
    );
    expect(container.querySelector(".harness-ok")).not.toBeNull();
  });

  it("replaces the failed subtree with an EmptyState showing the error message", () => {
    silenceBoundaryErrors();
    failureState.active = true;
    const { container } = render(
      <ErrorBoundary title="Failed to load">
        <ThrowingChild shouldThrow />
      </ErrorBoundary>,
    );
    expect(container.querySelector(".harness-ok")).toBeNull();
    const empty = container.querySelector(".poodle-empty-state") as HTMLElement;
    expect(empty).not.toBeNull();
    expect(empty.textContent).toContain("Failed to load");
    expect(empty.textContent).toContain("render exploded");
  });

  it("re-renders children after the retry action resets the boundary", () => {
    silenceBoundaryErrors();
    failureState.active = true;
    const { container } = render(
      <ErrorBoundary retryLabel="Retry now">
        <ThrowingChild shouldThrow />
      </ErrorBoundary>,
    );
    expect(container.querySelector(".harness-ok")).toBeNull();
    const retry = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Retry now"),
    ) as HTMLButtonElement;
    // Clear the underlying failure, then retry: the reset re-renders the child.
    failureState.active = false;
    fireEvent.click(retry);
    expect(container.querySelector(".harness-ok")).not.toBeNull();
  });
});
import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ErrorBoundarySpecimen from "../src/specimens/ErrorBoundarySpecimen.svelte";

function resetButton(container: HTMLElement): HTMLButtonElement {
  return [...container.querySelectorAll("button")].find((button) =>
    button.textContent?.includes("Reset boundary"),
  ) as HTMLButtonElement;
}

function throwAgainButton(container: HTMLElement): HTMLButtonElement {
  return [...container.querySelectorAll("button")].find((button) =>
    button.textContent?.includes("Throw again"),
  ) as HTMLButtonElement;
}

describe("g15.031 ErrorBoundary specimen", () => {
  it("shows recovered content after Reset boundary and re-enters error on Throw again", async () => {
    const { container } = render(ErrorBoundarySpecimen);

    expect(container.textContent).toContain("Preview failed");

    await fireEvent.click(resetButton(container));
    expect(container.textContent).toContain("Recovered child content");
    expect(container.textContent).not.toContain("Preview failed");

    await fireEvent.click(throwAgainButton(container));
    expect(container.textContent).toContain("Preview failed");
  });

  it("starts in the error state after remount and across a second instance", () => {
    const first = render(ErrorBoundarySpecimen);
    expect(first.container.textContent).toContain("Preview failed");
    first.unmount();

    const remounted = render(ErrorBoundarySpecimen);
    expect(remounted.container.textContent).toContain("Preview failed");
    remounted.unmount();

    const left = render(ErrorBoundarySpecimen);
    const right = render(ErrorBoundarySpecimen);
    expect(left.container.textContent).toContain("Preview failed");
    expect(right.container.textContent).toContain("Preview failed");
  });
});

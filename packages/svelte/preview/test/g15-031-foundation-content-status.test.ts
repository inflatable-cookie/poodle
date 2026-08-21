import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ErrorBoundarySpecimen from "../src/specimens/ErrorBoundarySpecimen.svelte";

describe("g15.031 ErrorBoundary specimen", () => {
  it("shows recovered content after Reset boundary and re-enters error on Throw again", async () => {
    const { container } = render(ErrorBoundarySpecimen);

    expect(container.textContent).toContain("Preview failed");

    const reset = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Reset boundary"),
    ) as HTMLButtonElement;
    await fireEvent.click(reset);
    expect(container.textContent).toContain("Recovered child content");
    expect(container.textContent).not.toContain("Preview failed");

    const throwAgain = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Throw again"),
    ) as HTMLButtonElement;
    await fireEvent.click(throwAgain);
    expect(container.textContent).toContain("Preview failed");
  });
});

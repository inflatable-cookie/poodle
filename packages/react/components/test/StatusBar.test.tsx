import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { StatusBar } from "../src/StatusBar";

describe("StatusBar (react)", () => {
  it("renders a footer landmark with the Status fallback label", () => {
    const { container } = render(<StatusBar />);
    const root = container.querySelector("footer.poodle-status-bar") as HTMLElement;
    expect(root).not.toBeNull();
    expect(root.getAttribute("aria-label")).toBe("Status");
  });

  it("resolves aria-label from summary then ariaLabel", () => {
    const viaSummary = render(<StatusBar summary="3 items selected" />);
    expect(
      viaSummary.container.querySelector("footer")?.getAttribute("aria-label"),
    ).toBe("3 items selected");

    const viaLabel = render(<StatusBar ariaLabel="Connection status" />);
    expect(
      viaLabel.container.querySelector("footer")?.getAttribute("aria-label"),
    ).toBe("Connection status");
  });

  it("shows summary text in the leading region when no leading node is given", () => {
    const { container } = render(<StatusBar summary="Ready" />);
    const leading = container.querySelector(".poodle-status-bar__leading") as HTMLElement;
    expect(leading.textContent).toBe("Ready");
    expect(container.querySelector(".poodle-status-bar__trailing")).toBeNull();
  });

  it("renders the leading node over the summary and the trailing region on demand", () => {
    const { container } = render(
      <StatusBar summary="Ready" leading={<span>main</span>} trailing={<span>Ln 42, Col 18</span>} />,
    );
    const leading = container.querySelector(".poodle-status-bar__leading") as HTMLElement;
    expect(leading.textContent).toContain("main");
    expect(leading.textContent).not.toContain("Ready");
    expect(container.querySelector(".poodle-status-bar__trailing")?.textContent).toContain(
      "Ln 42, Col 18",
    );
  });

  it("omits the trailing region until the trailing node is present", () => {
    const onlyLeading = render(<StatusBar leading={<span>main</span>} />);
    expect(onlyLeading.container.querySelector(".poodle-status-bar__trailing")).toBeNull();
  });

  it("applies the chrome modifier and projects size and density", () => {
    const { container } = render(<StatusBar chrome size="lg" density="compact" />);
    const root = container.querySelector("footer") as HTMLElement;
    expect(root.classList.contains("poodle-status-bar--chrome")).toBe(true);
    expect(root.dataset.size).toBe("lg");
    expect(root.dataset.density).toBe("compact");
  });
});
import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import StatusBar from "../src/StatusBar.svelte";
import StatusBarSnippetsHarness from "./StatusBarSnippetsHarness.svelte";
import { asSnippet } from "./snippet";

describe("StatusBar (svelte)", () => {
  it("renders a footer landmark with the Status fallback label", () => {
    const { container } = render(StatusBar, {});
    const root = container.querySelector("footer.poodle-status-bar") as HTMLElement;
    expect(root).not.toBeNull();
    expect(root.getAttribute("aria-label")).toBe("Status");
  });

  it("resolves aria-label from summary then ariaLabel", () => {
    const viaSummary = render(StatusBar, { props: { summary: "3 items selected" } });
    expect(
      viaSummary.container.querySelector("footer")?.getAttribute("aria-label"),
    ).toBe("3 items selected");

    const viaLabel = render(StatusBar, { props: { ariaLabel: "Connection status" } });
    expect(
      viaLabel.container.querySelector("footer")?.getAttribute("aria-label"),
    ).toBe("Connection status");
  });

  it("shows summary text in the leading region when no leading snippet is given", () => {
    const { container } = render(StatusBar, { props: { summary: "Ready" } });
    const leading = container.querySelector(".poodle-status-bar__leading") as HTMLElement;
    expect(leading.textContent).toBe("Ready");
    expect(container.querySelector(".poodle-status-bar__trailing")).toBeNull();
  });

  it("renders the leading snippet over the summary and the trailing region on demand", () => {
    const { container } = render(StatusBarSnippetsHarness);
    const leading = container.querySelector(".poodle-status-bar__leading") as HTMLElement;
    expect(leading.textContent).toContain("main");
    expect(leading.textContent).not.toContain("Ready");
    expect(container.querySelector(".poodle-status-bar__trailing")?.textContent).toContain(
      "Ln 42, Col 18",
    );
  });

  it("omits the trailing region until the trailing snippet has content", () => {
    const onlyLeading = render(StatusBar, {
      props: { leading: asSnippet(() => "main") },
    });
    expect(onlyLeading.container.querySelector(".poodle-status-bar__trailing")).toBeNull();
  });

  it("applies the chrome modifier and projects size and density", () => {
    const { container } = render(StatusBar, {
      props: { chrome: true, size: "lg", density: "compact" },
    });
    const root = container.querySelector("footer") as HTMLElement;
    expect(root.classList.contains("poodle-status-bar--chrome")).toBe(true);
    expect(root.dataset.size).toBe("lg");
    expect(root.dataset.density).toBe("compact");
  });
});
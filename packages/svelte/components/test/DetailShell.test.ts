import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import DetailShell from "../src/DetailShell.svelte";
import { asSnippet } from "./snippet";

describe("DetailShell (svelte)", () => {
  it("renders the body region when ready and shows the title fallback header", () => {
    const { container } = render(DetailShell, {
      props: { title: "Member", children: asSnippet(() => "<p>content</p>") },
    });
    expect(container.querySelector(".poodle-detail-shell__body")).not.toBeNull();
    const heading = container.querySelector(".poodle-detail-shell__header h2") as HTMLElement;
    expect(heading.textContent).toBe("Member");
  });

  it("replaces the body with a loading state region showing the spinner", () => {
    const { container } = render(DetailShell, {
      props: { title: "Member", state: "loading" },
    });
    expect(container.querySelector(".poodle-detail-shell__body")).toBeNull();
    const stateRegion = container.querySelector(".poodle-detail-shell__state") as HTMLElement;
    expect(stateRegion.dataset.state).toBe("loading");
    expect(stateRegion.querySelector(".poodle-detail-shell__spinner")).not.toBeNull();
  });

  it("renders the state title and message in the error state", () => {
    const { container } = render(DetailShell, {
      props: {
        title: "Member",
        state: "error",
        stateTitle: "Failed to load",
        stateMessage: "Try again.",
      },
    });
    const stateRegion = container.querySelector(".poodle-detail-shell__state") as HTMLElement;
    expect(stateRegion.dataset.state).toBe("error");
    expect(stateRegion.textContent).toContain("Failed to load");
    expect(stateRegion.textContent).toContain("Try again.");
  });

  it("uses the stateContent snippet instead of the default state display", () => {
    const { container } = render(DetailShell, {
      props: {
        title: "Member",
        state: "error",
        stateContent: asSnippet(() => "<p>custom</p>"),
      },
    });
    const stateRegion = container.querySelector(".poodle-detail-shell__state") as HTMLElement;
    // Raw thunks materialize as comment nodes under happy-dom; the observable
    // contract behaviour is that the default strong/p copy is replaced.
    expect(stateRegion.querySelector("strong")).toBeNull();
    expect(stateRegion.querySelector("p")).toBeNull();
  });

  it("projects the scroll mode and applies the accessible label", () => {
    const { container } = render(DetailShell, {
      props: { scrollMode: "shell", ariaLabel: "Member detail" },
    });
    const root = container.querySelector(".poodle-detail-shell") as HTMLElement;
    expect(root.dataset.scrollMode).toBe("shell");
    expect(root.getAttribute("aria-label")).toBe("Member detail");
  });
});

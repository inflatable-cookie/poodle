import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { PickerShell } from "../src/PickerShell";

describe("PickerShell (react)", () => {
  it("renders the section with title, description, and counts", () => {
    const { container } = render(
      <PickerShell title="Select a component" description="Browse components." resultCount={12} selectionCount={2} />,
    );
    const section = container.querySelector(".poodle-picker-shell") as HTMLElement;
    expect(section.getAttribute("data-variant")).toBe("inline");
    expect(section.getAttribute("data-state")).toBe("ready");
    expect(section.querySelector("h3")?.textContent).toBe("Select a component");
    expect(section.querySelector(".poodle-picker-shell__description")?.textContent).toContain(
      "Browse components.",
    );
    expect(section.textContent).toContain("12 results");
    expect(section.textContent).toContain("2 selected");
  });

  it("renders the body only in the ready state", () => {
    const ready = render(
      <PickerShell title="Pick" state="ready">
        <div>candidates</div>
      </PickerShell>,
    );
    expect(ready.container.querySelector(".poodle-picker-shell__body")).not.toBeNull();
    expect(ready.container.querySelector(".poodle-picker-shell__state")).toBeNull();

    const loading = render(<PickerShell title="Pick" state="loading" />);
    expect(loading.container.querySelector(".poodle-picker-shell__body")).toBeNull();
    expect(loading.container.querySelector(".poodle-picker-shell__state")).not.toBeNull();
  });

  it("shows the spinner in the loading state fallback", () => {
    const { container } = render(<PickerShell title="Pick" state="loading" />);
    expect(container.querySelector(".poodle-picker-shell__spinner")).not.toBeNull();
  });

  it("renders explicit state title and message in the state area", () => {
    const { container } = render(
      <PickerShell
        title="Pick"
        state="no-results"
        stateTitle="No matches"
        stateMessage="Try a different term."
      />,
    );
    const state = container.querySelector(".poodle-picker-shell__state") as HTMLElement;
    expect(state.textContent).toContain("No matches");
    expect(state.textContent).toContain("Try a different term.");
  });

  it("renders the sr-only status live region when statusText is provided", () => {
    const { container } = render(
      <PickerShell title="Pick" statusText="3 candidates available" statusId="picker-status" />,
    );
    const status = container.querySelector("#picker-status") as HTMLElement;
    expect(status.getAttribute("role")).toBe("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.classList.contains("poodle-sr-only")).toBe(true);
  });

  it("renders the toolbar, selection, and footer regions when provided", () => {
    const { container } = render(
      <PickerShell
        title="Pick"
        toolbar={<input />}
        selection={<div>chips</div>}
        footer={<button>Confirm</button>}
      />,
    );
    expect(container.querySelector(".poodle-picker-shell__toolbar")).not.toBeNull();
    expect(container.querySelector(".poodle-picker-shell__selection")).not.toBeNull();
    expect(container.querySelector(".poodle-picker-shell__footer")).not.toBeNull();
  });
});
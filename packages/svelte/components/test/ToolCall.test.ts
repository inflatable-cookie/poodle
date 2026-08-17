import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ToolCall from "../src/ToolCall.svelte";

describe("ToolCall (svelte)", () => {
  it("renders the kind, detail, and status with an accessible name omitting success", () => {
    const { container } = render(ToolCall, {
      props: { id: "tc-1", label: "Ran command", detail: "npm test", status: "success", output: "ok" },
    });
    const root = container.querySelector(".poodle-tool-call") as HTMLElement;
    expect(root.dataset.status).toBe("success");
    expect(container.querySelector(".poodle-tool-call__label")?.textContent).toBe("Ran command");
    expect(container.querySelector(".poodle-tool-call__detail")?.textContent).toBe("npm test");
    const trigger = container.querySelector(".poodle-tool-call__trigger") as HTMLElement;
    expect(trigger.getAttribute("aria-label")).toBe("Ran command: npm test");
  });

  it("carries the status in the accessible name when not success", () => {
    const { container } = render(ToolCall, {
      props: { id: "tc-1", label: "Ran command", detail: "npm test", status: "error", output: "ok" },
    });
    const trigger = container.querySelector(".poodle-tool-call__trigger") as HTMLElement;
    expect(trigger.getAttribute("aria-label")).toBe("Ran command: npm test, error");
  });

  it("renders a non-interactive row (div, no disclosure, no tab stop) when there is no output", () => {
    const { container } = render(ToolCall, {
      props: { id: "tc-1", label: "Searched", status: "success" },
    });
    const root = container.querySelector(".poodle-tool-call") as HTMLElement;
    expect(root.dataset.interactive).toBe("false");
    expect(container.querySelector(".poodle-tool-call__trigger")?.tagName).toBe("DIV");
    expect(container.querySelector(".poodle-tool-call__disclosure")).toBeNull();
    expect(container.querySelector(".poodle-tool-call__output")).toBeNull();
  });

  it("renders output as a button with aria-expanded and reveals the code block lazily", async () => {
    const onToggle = vi.fn();
    const { container } = render(ToolCall, {
      props: {
        id: "tc-1",
        label: "Ran command",
        output: "const x = 1;",
        outputLanguage: "ts",
        onToggle,
      },
    });
    const root = container.querySelector(".poodle-tool-call") as HTMLElement;
    expect(root.dataset.interactive).toBe("true");
    const trigger = container.querySelector(".poodle-tool-call__trigger") as HTMLButtonElement;
    expect(trigger.tagName).toBe("BUTTON");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-tool-call__output")).toBeNull();

    await fireEvent.click(trigger);
    expect(onToggle).toHaveBeenCalledWith("tc-1");
    expect(container.querySelector(".poodle-tool-call__output .poodle-code")).not.toBeNull();
  });

  it("resolves the icon from the label and allows an override", () => {
    const derived = render(ToolCall, { props: { id: "tc-1", label: "Ran command" } });
    expect(derived.container.querySelector(".poodle-tool-call__icon svg")).not.toBeNull();

    const overridden = render(ToolCall, {
      props: { id: "tc-1", label: "Custom thing", icon: "search" },
    });
    expect(overridden.container.querySelector(".poodle-tool-call__icon svg")).not.toBeNull();
  });

  it("keeps the detail in the accessible name untruncated", () => {
    const longDetail = "npm run very-long-command ".repeat(20).trim();
    const { container } = render(ToolCall, {
      props: { id: "tc-1", label: "Ran command", detail: longDetail, output: "ok" },
    });
    const trigger = container.querySelector(".poodle-tool-call__trigger") as HTMLElement;
    expect(trigger.getAttribute("aria-label")).toContain(longDetail);
  });
});
import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ToolCallGroup from "../src/ToolCallGroup.svelte";

const calls = [
  { id: "tc-1", label: "Ran command", detail: "one", status: "success" as const },
  { id: "tc-2", label: "Ran command", detail: "two", status: "success" as const },
  { id: "tc-3", label: "Ran command", detail: "three", status: "success" as const },
];

describe("ToolCallGroup (svelte)", () => {
  it("renders only the newest call while collapsed with the hidden count", () => {
    const { container } = render(ToolCallGroup, { props: { id: "run-1", calls } });
    const root = container.querySelector(".poodle-tool-call-group") as HTMLElement;
    expect(root.dataset.count).toBe("3");
    const rendered = Array.from(container.querySelectorAll(".poodle-tool-call"));
    expect(rendered.length).toBe(1);
    expect(rendered[0].textContent).toContain("three");
    expect(container.querySelector(".poodle-tool-call-group__toggle")?.textContent).toContain(
      "+2 previous tool calls",
    );
  });

  it("expands to list every call in order and reports the toggle", async () => {
    const onToggle = vi.fn();
    const { container } = render(ToolCallGroup, { props: { id: "run-1", calls, onToggle } });
    const toggle = container.querySelector(".poodle-tool-call-group__toggle") as HTMLButtonElement;

    await fireEvent.click(toggle);
    expect(onToggle).toHaveBeenCalledWith("run-1");
    const rendered = Array.from(container.querySelectorAll(".poodle-tool-call"));
    expect(rendered.length).toBe(3);
    expect(rendered[2].textContent).toContain("three");
  });

  it("omits the toggle entirely for a single-call run", () => {
    const { container } = render(ToolCallGroup, {
      props: { id: "run-1", calls: calls.slice(0, 1) },
    });
    expect(container.querySelector(".poodle-tool-call-group__toggle")).toBeNull();
  });

  it("marks the run as failed when any call failed, outranking running", () => {
    const mixed = render(ToolCallGroup, {
      props: {
        id: "run-1",
        calls: [
          { id: "tc-1", label: "Ran command", status: "running" as const },
          { id: "tc-2", label: "Ran command", status: "error" as const },
        ],
      },
    });
    const mixedRoot = mixed.container.querySelector(".poodle-tool-call-group") as HTMLElement;
    expect(mixedRoot.dataset.status).toBe("error");
    const mixedToggle = mixed.container.querySelector(".poodle-tool-call-group__toggle") as HTMLElement;
    expect(mixedToggle.getAttribute("aria-label")).toBe(
      "+1 previous tool calls, contains a failure",
    );

    const running = render(ToolCallGroup, {
      props: {
        id: "run-1",
        calls: [
          { id: "tc-1", label: "Ran command", status: "running" as const },
          { id: "tc-2", label: "Ran command", status: "success" as const },
        ],
      },
    });
    const runningRoot = running.container.querySelector(".poodle-tool-call-group") as HTMLElement;
    expect(runningRoot.dataset.status).toBe("running");
  });

  it("forwards a call's disclosure toggle with the call id", async () => {
    const onCallToggle = vi.fn();
    const { container } = render(ToolCallGroup, {
      props: {
        id: "run-1",
        calls: [{ id: "tc-1", label: "Ran command", output: "ok", status: "success" as const }],
        onCallToggle,
      },
    });
    const call = container.querySelector(".poodle-tool-call__trigger") as HTMLButtonElement;
    await fireEvent.click(call);
    expect(onCallToggle).toHaveBeenCalledWith("tc-1");
  });
});
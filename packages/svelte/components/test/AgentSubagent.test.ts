import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import AgentSubagent from "../src/AgentSubagent.svelte";

const running = {
  id: "child-1",
  label: "Scout",
  status: "running" as const,
  activityLine: "Checking vectors",
};

describe("AgentSubagent (svelte)", () => {
  it("renders the running badge verbatim with the activity line and a dots spinner", () => {
    const { container } = render(AgentSubagent, { props: { item: running } });
    const root = container.querySelector(".poodle-agent-subagent") as HTMLElement;
    expect(root.dataset.status).toBe("running");
    expect(container.querySelector(".poodle-agent-subagent__badge")?.textContent).toBe("Running");
    expect(container.querySelector(".poodle-agent-subagent__activity-line")?.textContent).toBe(
      "Checking vectors",
    );
    expect(container.querySelector(".poodle-spinner")).not.toBeNull();
  });

  it("renders the terminal summary instead of the activity line once settled", () => {
    const { container } = render(AgentSubagent, {
      props: {
        item: { ...running, status: "completed", summary: "Found three stale vectors" },
      },
    });
    expect(container.querySelector(".poodle-agent-subagent__badge")?.textContent).toBe("Completed");
    expect(container.querySelector(".poodle-agent-subagent__summary")?.textContent).toBe(
      "Found three stale vectors",
    );
    expect(container.querySelector(".poodle-agent-subagent__activity-line")).toBeNull();
    expect(container.querySelector(".poodle-spinner")).toBeNull();
  });

  it("renders the unknown status literally without a spinner", () => {
    const { container } = render(AgentSubagent, {
      props: { item: { ...running, status: "unknown", activityLine: undefined } },
    });
    expect(container.querySelector(".poodle-agent-subagent__badge")?.textContent).toBe("Unknown");
    expect(container.querySelector(".poodle-spinner")).toBeNull();
  });

  it("shows the disclosure only when there is detail and reports the toggle", async () => {
    const onToggle = vi.fn();
    const noDetail = render(AgentSubagent, { props: { item: running } });
    expect(noDetail.container.querySelector('[data-kind="toggle"]')).toBeNull();

    const { container } = render(AgentSubagent, {
      props: { item: running, detailLines: ["First line"], onToggle },
    });
    const toggle = container.querySelector('[data-kind="toggle"]') as HTMLButtonElement;
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-agent-subagent__detail")).toBeNull();

    await fireEvent.click(toggle);
    expect(onToggle).toHaveBeenCalledWith(true);
    expect(container.querySelectorAll(".poodle-agent-subagent__detail li").length).toBe(1);
  });

  it("renders the click-through only when a handler exists", async () => {
    const onOpenChild = vi.fn();
    const noHandler = render(AgentSubagent, { props: { item: running } });
    expect(noHandler.container.querySelector('[data-kind="open"]')).toBeNull();

    const { container } = render(AgentSubagent, {
      props: { item: running, onOpenChild },
    });
    const open = container.querySelector('[data-kind="open"]') as HTMLButtonElement;
    expect(open.textContent).toBe("Open child work");
    await fireEvent.click(open);
    expect(onOpenChild).toHaveBeenCalledOnce();
  });

  it("renders nothing when no item is provided", () => {
    const { container } = render(AgentSubagent, {});
    expect(container.querySelector(".poodle-agent-subagent")).toBeNull();
  });
});
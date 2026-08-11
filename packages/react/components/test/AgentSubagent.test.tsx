import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AgentSubagent } from "../src/AgentSubagent";
import { AgentTranscript } from "../src/AgentTranscript";

const running = {
  id: "child-1",
  label: "Scout",
  status: "running" as const,
  activityLine: "Checking vectors",
  summary: "Found three stale vectors",
};

describe("AgentSubagent (react)", () => {
  it("renders running and terminal presentation from the shared status helpers", () => {
    const { container, rerender } = render(<AgentSubagent item={running} />);
    expect(container.querySelector(".poodle-spinner")).not.toBeNull();
    expect(container.textContent).toContain("Running");
    expect(container.textContent).toContain("Checking vectors");

    rerender(<AgentSubagent item={{ ...running, status: "completed" }} />);
    expect(container.querySelector(".poodle-spinner")).toBeNull();
    expect(container.textContent).toContain("Completed");
    expect(container.textContent).toContain("Found three stale vectors");
  });

  it("owns uncontrolled disclosure and reports both actions", () => {
    const onToggle = vi.fn();
    const onOpenChild = vi.fn();
    const { getByRole, queryByText } = render(
      <AgentSubagent
        item={running}
        detailLines={["First line"]}
        onToggle={onToggle}
        onOpenChild={onOpenChild}
      />,
    );

    expect(queryByText("First line")).toBeNull();
    fireEvent.click(getByRole("button", { name: "Show activity" }));
    expect(queryByText("First line")).not.toBeNull();
    expect(onToggle).toHaveBeenCalledWith(true);
    fireEvent.click(getByRole("button", { name: "Open child work" }));
    expect(onOpenChild).toHaveBeenCalledOnce();
  });

  it("omits controls that have no action", () => {
    const { queryByRole } = render(<AgentSubagent item={running} />);
    expect(queryByRole("button")).toBeNull();
  });

  it("renders inside AgentTranscript and forwards child navigation", () => {
    const onOpenChild = vi.fn();
    const { getByRole, getByText } = render(
      <AgentTranscript
        virtualized={false}
        items={[{ kind: "subagent-group", id: "group-1", subagent: running }]}
        onOpenChild={onOpenChild}
      />,
    );

    expect(getByText("Scout")).not.toBeNull();
    fireEvent.click(getByRole("button", { name: "Open child work" }));
    expect(onOpenChild).toHaveBeenCalledWith("child-1");
  });
});

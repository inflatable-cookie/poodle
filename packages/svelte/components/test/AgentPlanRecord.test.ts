import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import AgentPlanRecord from "../src/AgentPlanRecord.svelte";

describe("AgentPlanRecord (svelte)", () => {
  const plan = "# Step 1\n\nDo the thing.";

  it("shows the status badge with fallback wording and collapsed summary by default", () => {
    const { container } = render(AgentPlanRecord, { props: { plan, status: "accepted" } });
    const root = container.querySelector(".poodle-agent-plan-record") as HTMLElement;
    expect(root.dataset.status).toBe("accepted");
    expect(container.querySelector(".poodle-agent-plan-record__badge")?.textContent).toBe("Accepted");
    expect(container.querySelector(".poodle-agent-plan-record__summary")?.textContent).toBe(
      "# Step 1 Do the thing.",
    );
    expect(container.querySelector(".poodle-agent-plan-record__body")).toBeNull();
  });

  it("renders the full plan through markdown when expanded, never both summary and plan", async () => {
    const onToggle = vi.fn();
    const { container } = render(AgentPlanRecord, {
      props: { plan, status: "revised", expanded: true, onToggle },
    });
    const body = container.querySelector(".poodle-agent-plan-record__body") as HTMLElement;
    expect(body).not.toBeNull();
    expect(body.querySelector("h1")).not.toBeNull();
    expect(container.querySelector(".poodle-agent-plan-record__summary")).toBeNull();

    await fireEvent.click(container.querySelector(".poodle-agent-plan-record__toggle") as HTMLElement);
    expect(onToggle).toHaveBeenCalledWith(false);
  });

  it("overrides the badge wording and renders the decided-at meta", () => {
    const { container } = render(AgentPlanRecord, {
      props: { plan, status: "dismissed", decisionLabel: "Not now", decidedAt: "2026-08-16" },
    });
    expect(container.querySelector(".poodle-agent-plan-record__badge")?.textContent).toBe("Not now");
    expect(container.querySelector(".poodle-agent-plan-record__meta")?.textContent).toBe(
      "2026-08-16",
    );
  });

  it("truncates a long plan to the summary budget with an ellipsis in budget", () => {
    const longPlan = "word ".repeat(80).trim();
    const { container } = render(AgentPlanRecord, {
      props: { plan: longPlan, status: "accepted", summaryMaxLength: 40 },
    });
    const summary = container.querySelector(".poodle-agent-plan-record__summary")?.textContent ?? "";
    expect(summary.endsWith("…")).toBe(true);
    expect(summary.length).toBeLessThanOrEqual(40);
  });

  it("exposes the disclosure state through aria-expanded and the toggle label", () => {
    const collapsed = render(AgentPlanRecord, { props: { plan, status: "accepted" } });
    const collapsedToggle = collapsed.container.querySelector(
      ".poodle-agent-plan-record__toggle",
    ) as HTMLButtonElement;
    expect(collapsedToggle.getAttribute("aria-expanded")).toBe("false");
    expect(collapsedToggle.textContent).toBe("Show plan");

    const expanded = render(AgentPlanRecord, {
      props: { plan, status: "accepted", expanded: true },
    });
    const expandedToggle = expanded.container.querySelector(
      ".poodle-agent-plan-record__toggle",
    ) as HTMLButtonElement;
    expect(expandedToggle.getAttribute("aria-expanded")).toBe("true");
    expect(expandedToggle.textContent).toBe("Hide plan");
  });
});
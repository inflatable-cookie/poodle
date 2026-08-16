import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AgentQuestion } from "../src/AgentQuestion";

const single = [
  {
    id: "q1",
    header: "Scope",
    prompt: "What should the change cover?",
    options: [
      { value: "docs", label: "Docs" },
      { value: "code", label: "Code" },
    ],
  },
];

const batch = [
  ...single,
  { id: "q2", prompt: "Second?", options: [{ value: "yes", label: "Yes" }] },
];

describe("AgentQuestion (react)", () => {
  it("renders a single-select question as a radiogroup labelled by the prompt", () => {
    const { container } = render(<AgentQuestion questions={single} />);
    const options = container.querySelector(".poodle-agent-question__options") as HTMLElement;
    expect(options.getAttribute("role")).toBe("radiogroup");
    const promptId = container.querySelector(".poodle-agent-question__prompt")?.id;
    expect(options.getAttribute("aria-labelledby")).toBe(promptId);
    const buttons = Array.from(container.querySelectorAll(".poodle-agent-question__option"));
    expect(buttons.map((b) => b.getAttribute("role"))).toEqual(["radio", "radio"]);
  });

  it("submits on a single-select click and reports the selection change", () => {
    const onSubmit = vi.fn();
    const onSelectionChange = vi.fn();
    const { container } = render(
      <AgentQuestion questions={single} onSubmit={onSubmit} onSelectionChange={onSelectionChange} />,
    );
    const option = Array.from(container.querySelectorAll(".poodle-agent-question__option")).find(
      (el) => el.textContent?.includes("Docs"),
    ) as HTMLElement;

    fireEvent.click(option);
    expect(onSelectionChange).toHaveBeenCalledWith(["docs"]);
    expect(onSubmit).toHaveBeenCalledWith(
      expect.objectContaining({ questionId: "q1", outcome: "selected", values: ["docs"] }),
    );
  });

  it("renders multi-select as a group with checkboxes and never submits on click", () => {
    const onSubmit = vi.fn();
    const { container } = render(
      <AgentQuestion
        questions={[{ id: "q1", prompt: "Pick tags", allowMultiple: true, options: [{ value: "a", label: "A" }, { value: "b", label: "B" }] }]}
        onSubmit={onSubmit}
      />,
    );
    const options = container.querySelector(".poodle-agent-question__options") as HTMLElement;
    expect(options.getAttribute("role")).toBe("group");
    const buttons = Array.from(container.querySelectorAll(".poodle-agent-question__option"));
    expect(buttons.map((b) => b.getAttribute("role"))).toEqual(["checkbox", "checkbox"]);
    expect(container.querySelectorAll(".poodle-agent-question__option-check").length).toBe(2);

    fireEvent.click(buttons[0]);
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("clears the selection when override text is present", () => {
    const onSelectionChange = vi.fn();
    const { container } = render(
      <AgentQuestion questions={single} selections={["docs"]} override="write it" onSelectionChange={onSelectionChange} />,
    );
    // The host owns selection state: the component reports the clear instead of
    // mutating the prop, and a host that echoes it back drops the selection.
    expect(onSelectionChange).toHaveBeenCalledWith([]);
    expect(container.querySelector(".poodle-agent-question__prompt")?.textContent).toContain(
      "What should the change cover?",
    );
  });

  it("shows batch progress only when there is more than one question", () => {
    const singleRender = render(<AgentQuestion questions={single} />);
    expect(singleRender.container.querySelector(".poodle-agent-question__progress")).toBeNull();

    const batchRender = render(<AgentQuestion questions={batch} activeIndex={1} />);
    const label = batchRender.container.querySelector(".poodle-agent-question__progress-label");
    expect(label?.textContent).toBe("2 of 2");
  });

  it("resolves dismissal as declined when dismissible", () => {
    const onDismiss = vi.fn();
    const onSubmit = vi.fn();
    const { container } = render(
      <AgentQuestion questions={single} dismissible onDismiss={onDismiss} onSubmit={onSubmit} />,
    );
    const dismiss = container.querySelector(".poodle-agent-question__dismiss") as HTMLElement;
    expect(dismiss.textContent).toBe("Skip this question");

    fireEvent.click(dismiss);
    expect(onDismiss).toHaveBeenCalledWith("q1");
    expect(onSubmit).toHaveBeenCalledWith(
      expect.objectContaining({ questionId: "q1", outcome: "declined", values: [] }),
    );
  });

  it("does not render the dismiss control when not dismissible", () => {
    const { container } = render(<AgentQuestion questions={single} />);
    expect(container.querySelector(".poodle-agent-question__dismiss")).toBeNull();
  });
});
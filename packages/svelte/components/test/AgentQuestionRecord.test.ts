import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import AgentQuestionRecord from "../src/AgentQuestionRecord.svelte";

const question = {
  id: "q1",
  prompt: "Which scope?",
  options: [
    { value: "docs", label: "Docs" },
    { value: "code", label: "Code" },
  ],
};

describe("AgentQuestionRecord (svelte)", () => {
  it("shows every option for a selected outcome and marks the chosen one", () => {
    const { container } = render(AgentQuestionRecord, {
      props: {
        question,
        answer: { questionId: "q1", outcome: "selected", values: ["code"], text: "" },
      },
    });
    const root = container.querySelector(".poodle-agent-question-record") as HTMLElement;
    expect(root.dataset.outcome).toBe("selected");
    const options = Array.from(root.querySelectorAll(".poodle-agent-question-record__option"));
    expect(options.length).toBe(2);
    expect(options.map((o) => o.getAttribute("data-chosen"))).toEqual(["false", "true"]);
    expect(options[1].getAttribute("aria-label")).toBe("chosen: Code");
  });

  it("renders the typed answer for an override outcome with no option list", () => {
    const { container } = render(AgentQuestionRecord, {
      props: {
        question,
        answer: { questionId: "q1", outcome: "override", values: [], text: "Do it my way" },
      },
    });
    expect(container.querySelector(".poodle-agent-question-record")?.getAttribute("data-outcome")).toBe(
      "override",
    );
    expect(container.querySelector(".poodle-agent-question-record__answer")?.textContent).toBe(
      "Do it my way",
    );
    expect(container.querySelector(".poodle-agent-question-record__options")).toBeNull();
  });

  it("renders the declined label for a declined outcome", () => {
    const { container } = render(AgentQuestionRecord, {
      props: {
        question,
        answer: { questionId: "q1", outcome: "declined", values: [], text: "" },
        declinedLabel: "Skipped",
      },
    });
    expect(container.querySelector(".poodle-agent-question-record__answer")?.textContent).toBe(
      "Skipped",
    );
  });

  it("hides the option list when showOptions is false", () => {
    const { container } = render(AgentQuestionRecord, {
      props: {
        question,
        answer: { questionId: "q1", outcome: "selected", values: ["code"], text: "" },
        showOptions: false,
      },
    });
    expect(container.querySelector(".poodle-agent-question-record__options")).toBeNull();
    expect(container.querySelector(".poodle-agent-question-record__answer")?.textContent).toBe("Code");
  });

  it("renders the header as an eyebrow when present", () => {
    const { container } = render(AgentQuestionRecord, {
      props: {
        question: { ...question, header: "Scope" },
        answer: { questionId: "q1", outcome: "selected", values: ["code"], text: "" },
      },
    });
    expect(container.querySelector(".poodle-eyebrow")?.textContent).toBe("Scope");
  });
});
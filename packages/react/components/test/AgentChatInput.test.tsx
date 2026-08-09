import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AgentChatInput } from "../src/AgentChatInput";

// Mirrors packages/svelte/components/test/AgentChatInput.test.ts — the submit
// gating and status semantics must behave identically in both frameworks.
describe("AgentChatInput (react)", () => {
  const editorOf = (container: HTMLElement) =>
    container.querySelector(".poodle-agent-chat-input__editor") as HTMLTextAreaElement;
  const actionOf = (container: HTMLElement) =>
    container.querySelector(".poodle-agent-chat-input__action") as HTMLButtonElement;

  it("disables the action while the editor is empty and enables it with text", () => {
    expect(actionOf(render(<AgentChatInput value="" />).container).disabled).toBe(true);
    expect(actionOf(render(<AgentChatInput value="   " />).container).disabled).toBe(true);
    expect(actionOf(render(<AgentChatInput value="hi" />).container).disabled).toBe(false);
    expect(actionOf(render(<AgentChatInput value="" allowEmptySubmit />).container).disabled).toBe(
      false,
    );
  });

  it("hosts a live question and lets its selection enable an empty editor", () => {
    const onSubmit = vi.fn();
    const { container, rerender } = render(
      <AgentChatInput
        value=""
        status="questioning"
        question={<div data-testid="question">Choose a target</div>}
        onSubmit={onSubmit}
      />,
    );

    expect(container.querySelector('[data-testid="question"]')).not.toBeNull();
    expect(editorOf(container).placeholder).toBe(
      "Type your own answer, or leave this blank to use the selected option",
    );
    expect(actionOf(container).disabled).toBe(true);

    rerender(
      <AgentChatInput
        value=""
        status="questioning"
        question={<div data-testid="question">Choose a target</div>}
        questionCanSubmit
        onSubmit={onSubmit}
      />,
    );

    expect(actionOf(container).disabled).toBe(false);
    fireEvent.click(actionOf(container));
    expect(onSubmit).toHaveBeenCalledWith("");
  });

  it("does not render the question region outside questioning status", () => {
    const { container } = render(
      <AgentChatInput value="" question={<div data-testid="question">Choose a target</div>} />,
    );

    expect(container.querySelector('[data-testid="question"]')).toBeNull();
    expect(editorOf(container).placeholder).toBe("Send a message");
  });

  it("hosts plan review without changing ordinary submit gating", () => {
    const { container } = render(
      <AgentChatInput
        value=""
        status="reviewing-plan"
        plan={<div data-testid="plan">Review this plan</div>}
      />,
    );

    expect(container.querySelector('[data-testid="plan"]')).not.toBeNull();
    expect(editorOf(container).placeholder).toBe(
      "Describe what to change, or decide the plan above",
    );
    expect(actionOf(container).disabled).toBe(true);
  });

  it("submits on Enter and inserts a newline on Shift+Enter", () => {
    const onSubmit = vi.fn();
    const { container } = render(<AgentChatInput value="ship it" onSubmit={onSubmit} />);
    const editor = editorOf(container);

    fireEvent.keyDown(editor, { key: "Enter", shiftKey: true });
    expect(onSubmit).not.toHaveBeenCalled();

    fireEvent.keyDown(editor, { key: "Enter" });
    expect(onSubmit).toHaveBeenCalledWith("ship it");
  });

  it("only submits on Cmd/Ctrl+Enter when submitOnEnter is false", () => {
    const onSubmit = vi.fn();
    const { container } = render(
      <AgentChatInput value="ship it" submitOnEnter={false} onSubmit={onSubmit} />,
    );
    const editor = editorOf(container);

    fireEvent.keyDown(editor, { key: "Enter" });
    expect(onSubmit).not.toHaveBeenCalled();

    fireEvent.keyDown(editor, { key: "Enter", metaKey: true });
    expect(onSubmit).toHaveBeenCalledTimes(1);
  });

  it("routes the action and Escape to stop while busy, never to submit", () => {
    const onSubmit = vi.fn();
    const onStop = vi.fn();
    const { container } = render(
      <AgentChatInput value="ship it" status="busy" onSubmit={onSubmit} onStop={onStop} />,
    );
    const action = actionOf(container);

    expect(action.dataset.state).toBe("stop");
    expect(action.getAttribute("aria-label")).toBe("Stop");
    expect(action.disabled).toBe(false);

    fireEvent.click(action);
    fireEvent.keyDown(editorOf(container), { key: "Enter" });
    fireEvent.keyDown(editorOf(container), { key: "Escape" });

    expect(onStop).toHaveBeenCalledTimes(2);
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("renders an image attachment as a thumbnail tile, files as chips", () => {
    const { container } = render(
      <AgentChatInput
        value=""
        attachments={[
          { id: "img", label: "diagram.png", kind: "image", thumbnailUrl: "/thumb.png" },
          { id: "doc", label: "notes.md", kind: "document", icon: "file-text" },
        ]}
      />,
    );
    const items = container.querySelectorAll<HTMLElement>(".poodle-agent-chat-input__attachment");

    expect(items[0].dataset.variant).toBe("thumbnail");
    const thumb = items[0].querySelector(
      ".poodle-agent-chat-input__attachment-thumb",
    ) as HTMLImageElement;
    expect(thumb.getAttribute("src")).toBe("/thumb.png");
    // The filename still reaches assistive tech, and hover shows it.
    expect(thumb.getAttribute("alt")).toBe("diagram.png");
    expect(items[0].getAttribute("title")).toBe("diagram.png");
    expect(items[0].querySelector(".poodle-agent-chat-input__attachment-label")).toBeNull();

    expect(items[1].dataset.variant).toBe("chip");
    expect(items[1].querySelector(".poodle-agent-chat-input__attachment-thumb")).toBeNull();
    expect(items[1].querySelector(".poodle-agent-chat-input__attachment-label")?.textContent).toBe(
      "notes.md",
    );

    // Both keep a removal control with the same accessible name shape.
    expect(container.querySelectorAll('[aria-label^="Remove "]')).toHaveLength(2);
  });

  it("renders the context ring only when a limit is supplied", () => {
    const without = render(<AgentChatInput value="" />).container;
    expect(without.querySelector(".poodle-agent-chat-input__context")).toBeNull();

    const { container } = render(
      <AgentChatInput value="" contextUsed={180_000} contextLimit={200_000} />,
    );
    const meter = container.querySelector(".poodle-meter") as HTMLElement;
    expect(meter.dataset.shape).toBe("ring");
    // 90% of the limit is past the 0.8 warn threshold.
    expect(meter.dataset.level).toBe("high");
    expect(meter.getAttribute("aria-label")).toBe("Context used, 90%");
  });

  it("does not clear the editor on submit", () => {
    const { container } = render(<AgentChatInput value="keep me" />);
    fireEvent.click(actionOf(container));
    expect(editorOf(container).value).toBe("keep me");
  });
});

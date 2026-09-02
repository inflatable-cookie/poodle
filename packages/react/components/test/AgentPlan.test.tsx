import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AgentChatInput } from "../src/AgentChatInput";
import { AgentPlan } from "../src/AgentPlan";

// Mirrors packages/svelte/components/test/AgentChatInputPlan.svelte.test.ts —
// the regression there crashed the preview specimen with `e.replace is not a
// function` from marked's lexer when the specimen's plan markdown string was
// shadowed by the snippet name. React composes through the `plan` prop, so the
// case here mounts the contract composition — AgentPlan through the composer's
// plan region while `status="reviewing-plan"` — and asserts the plan renders as
// markdown with working decision controls.
describe("AgentChatInput plan region (react)", () => {
  const plan = ["## Proposed plan", "", "1. Add the surface", "2. Wire the callbacks"].join("\n");

  it("renders AgentPlan through the plan prop while reviewing-plan", async () => {
    const { container } = render(
      <AgentChatInput status="reviewing-plan" plan={<AgentPlan plan={plan} />} />,
    );

    const region = container.querySelector(".poodle-agent-chat-input__plan") as HTMLElement;
    expect(region).not.toBeNull();

    const planEl = region.querySelector(".poodle-agent-plan") as HTMLElement;
    expect(planEl).not.toBeNull();
    expect(planEl.dataset.status).toBe("pending");
    // The markdown went through the lexer: the heading is a real element, not
    // raw text, and the list items rendered.
    await waitFor(() => {
      expect(planEl.querySelector("h2")?.textContent).toBe("Proposed plan");
      expect(planEl.querySelectorAll("li")).toHaveLength(2);
    });
  });

  it("routes the decision controls to the host callbacks", () => {
    const onAccept = vi.fn();
    const onRevise = vi.fn();
    const onDismiss = vi.fn();
    const { container } = render(
      <AgentChatInput
        status="reviewing-plan"
        plan={<AgentPlan plan={plan} onAccept={onAccept} onRevise={onRevise} onDismiss={onDismiss} />}
      />,
    );

    fireEvent.click(container.querySelector('[data-variant="primary"]') as HTMLElement);
    fireEvent.click(container.querySelector('[data-variant="secondary"]') as HTMLElement);
    fireEvent.click(container.querySelector('[data-variant="ghost"]') as HTMLElement);

    expect(onAccept).toHaveBeenCalledTimes(1);
    expect(onRevise).toHaveBeenCalledTimes(1);
    expect(onDismiss).toHaveBeenCalledTimes(1);
  });

  it("keeps the editor an ordinary send channel while a plan waits", () => {
    const onSubmit = vi.fn();
    const { container } = render(
      <AgentChatInput status="reviewing-plan" plan={<AgentPlan plan={plan} />} onSubmit={onSubmit} />,
    );
    const editor = container.querySelector(".poodle-agent-chat-input__editor") as HTMLTextAreaElement;

    // Unlike `questioning`, an empty editor does not submit — the plan does
    // not relax the gate, because the turn is already complete.
    expect(
      (container.querySelector(".poodle-agent-chat-input__action") as HTMLButtonElement).disabled,
    ).toBe(true);

    fireEvent.input(editor, { target: { value: "drop step 2" } });
    fireEvent.keyDown(editor, { key: "Enter" });
    expect(onSubmit).toHaveBeenCalledWith("drop step 2");
  });
});
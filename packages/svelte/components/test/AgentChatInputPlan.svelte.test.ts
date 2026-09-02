import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import AgentChatInputPlanHarness from "./AgentChatInputPlanHarness.svelte";

// Regression: the preview specimen crashed with `e.replace is not a function`
// from marked's lexer because the specimen's `plan` markdown string was
// shadowed by the `{#snippet plan()}` name, so the snippet function reached
// `AgentMessage.markdown`. This mounts the contract composition — AgentPlan
// through the composer's `plan` snippet while `status="reviewing-plan"` — and
// asserts the plan renders as markdown with working decision controls.
describe("AgentChatInput plan region (svelte)", () => {
  const plan = ["## Proposed plan", "", "1. Add the surface", "2. Wire the callbacks"].join("\n");

  it("renders AgentPlan through the plan snippet while reviewing-plan", async () => {
    const { container } = render(AgentChatInputPlanHarness, { props: { plan } });

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

  it("routes the decision controls to the host callbacks", async () => {
    const onAccept = vi.fn();
    const onRevise = vi.fn();
    const onDismiss = vi.fn();
    const { container } = render(AgentChatInputPlanHarness, {
      props: { plan, onAccept, onRevise, onDismiss },
    });

    await fireEvent.click(container.querySelector('[data-variant="primary"]') as HTMLElement);
    await fireEvent.click(container.querySelector('[data-variant="secondary"]') as HTMLElement);
    await fireEvent.click(container.querySelector('[data-variant="ghost"]') as HTMLElement);

    expect(onAccept).toHaveBeenCalledTimes(1);
    expect(onRevise).toHaveBeenCalledTimes(1);
    expect(onDismiss).toHaveBeenCalledTimes(1);
  });

  it("keeps the editor an ordinary send channel while a plan waits", async () => {
    const onSubmit = vi.fn();
    const { container } = render(AgentChatInputPlanHarness, { props: { plan, onSubmit } });
    const editor = container.querySelector(
      ".poodle-agent-chat-input__editor",
    ) as HTMLTextAreaElement;

    // Unlike `questioning`, an empty editor does not submit — the plan does
    // not relax the gate, because the turn is already complete.
    expect(
      (container.querySelector(".poodle-agent-chat-input__action") as HTMLButtonElement).disabled,
    ).toBe(true);

    await fireEvent.input(editor, { target: { value: "drop step 2" } });
    await fireEvent.keyDown(editor, { key: "Enter" });
    expect(onSubmit).toHaveBeenCalledWith("drop step 2");
  });
});

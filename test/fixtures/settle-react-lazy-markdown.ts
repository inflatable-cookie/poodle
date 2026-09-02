import { waitFor } from "@testing-library/dom";
import { act } from "@testing-library/react";

function markdownPending(root: ParentNode): boolean {
  for (const body of root.querySelectorAll(
    ".poodle-agent-plan__body, .poodle-agent-plan-record__body",
  )) {
    if (!body.querySelector(".poodle-agent-message")) return true;
  }
  return false;
}

/** AgentPlan / expanded AgentPlanRecord load AgentMessage asynchronously. */
export async function settleLazyMarkdown(
  root: ParentNode = document.body,
): Promise<void> {
  if (markdownPending(root)) {
    await waitFor(() => {
      if (markdownPending(root)) throw new Error("lazy AgentMessage still pending");
    });
  }
  await act(async () => {
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  });
}

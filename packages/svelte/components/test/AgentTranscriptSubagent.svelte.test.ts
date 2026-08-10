import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import AgentTranscript from "../src/AgentTranscript.svelte";
import type { TranscriptItem } from "../src/types.ts";

const group: TranscriptItem = {
  kind: "subagent-group",
  id: "group:1",
  subagent: {
    id: "child:scout",
    label: "Scout",
    status: "running",
    activityLine: "Searching the parser crate",
  },
  detailLines: [],
};

describe("AgentTranscript subagent groups", () => {
  it("forwards the group click-through with the child id", async () => {
    const onOpenChild = vi.fn();
    const { getByRole } = render(AgentTranscript, {
      props: { items: [group], onOpenChild },
    });

    await fireEvent.click(getByRole("button", { name: "Open child work" }));

    expect(onOpenChild).toHaveBeenCalledOnce();
    expect(onOpenChild).toHaveBeenCalledWith("child:scout");
  });

  it("renders groups without a click-through affordance when no handler is set", () => {
    const { queryByRole } = render(AgentTranscript, {
      props: { items: [group] },
    });

    expect(queryByRole("button", { name: "Open child work" })).toBeNull();
  });
});

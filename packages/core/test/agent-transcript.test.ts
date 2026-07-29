import { describe, expect, test } from "bun:test";

import {
  changedFilesTotals,
  groupTranscriptItems,
  isPinnedToBottom,
  toolRunHiddenCount,
  toolRunLeadCall,
  toolRunStatus,
  transcriptWindow,
  type TranscriptItem,
  type TranscriptToolCall,
  type TranscriptToolRun,
} from "../src/agent-transcript";

const call = (
  id: string,
  status: TranscriptToolCall["status"] = "success",
): TranscriptToolCall => ({ kind: "tool-call", id, label: "Ran command", status });

const message = (id: string): TranscriptItem => ({
  kind: "message",
  id,
  role: "assistant",
  markdown: "text",
});

describe("groupTranscriptItems", () => {
  test("collapses adjacent tool calls into one run", () => {
    const blocks = groupTranscriptItems([call("a"), call("b"), call("c")]);

    expect(blocks).toHaveLength(1);
    expect((blocks[0] as TranscriptToolRun).calls.map((c) => c.id)).toEqual(["a", "b", "c"]);
  });

  test("a non-tool item ends the run", () => {
    const blocks = groupTranscriptItems([
      call("a"),
      call("b"),
      message("m"),
      call("c"),
    ]);

    expect(blocks.map((b) => b.kind)).toEqual(["tool-run", "message", "tool-run"]);
    expect((blocks[2] as TranscriptToolRun).calls.map((c) => c.id)).toEqual(["c"]);
  });

  test("a changed-files card splits a run rather than being absorbed", () => {
    // The transcript should say those commands happened either side of an edit,
    // not as one stretch of work.
    const blocks = groupTranscriptItems([
      call("a"),
      { kind: "changed-files", id: "diff", files: [] },
      call("b"),
    ]);

    expect(blocks.map((b) => b.kind)).toEqual(["tool-run", "changed-files", "tool-run"]);
  });

  test("a run's id is stable as calls are appended", () => {
    const first = groupTranscriptItems([call("a")])[0] as TranscriptToolRun;
    const later = groupTranscriptItems([call("a"), call("b")])[0] as TranscriptToolRun;

    expect(later.id).toBe(first.id);
  });

  test("an empty transcript produces no blocks", () => {
    expect(groupTranscriptItems([])).toEqual([]);
  });
});

describe("tool run summary", () => {
  const run = (...calls: TranscriptToolCall[]): TranscriptToolRun =>
    groupTranscriptItems(calls)[0] as TranscriptToolRun;

  test("the collapsed row shows the newest call, not the oldest", () => {
    expect(toolRunLeadCall(run(call("a"), call("b"), call("c"))).id).toBe("c");
  });

  test("hidden count covers everything but the lead call", () => {
    expect(toolRunHiddenCount(run(call("a"), call("b"), call("c")))).toBe(2);
    expect(toolRunHiddenCount(run(call("a")))).toBe(0);
  });

  test("one failure anywhere wins the run's status", () => {
    // The summary exists to tell you whether to open the run. One failed
    // command inside eight successful ones is exactly when you need to.
    expect(toolRunStatus(run(call("a"), call("b", "error"), call("c")))).toBe("error");
  });

  test("error outranks running", () => {
    expect(toolRunStatus(run(call("a", "running"), call("b", "error")))).toBe("error");
  });

  test("running wins only when nothing failed", () => {
    expect(toolRunStatus(run(call("a"), call("b", "running")))).toBe("running");
    expect(toolRunStatus(run(call("a"), call("b")))).toBe("success");
  });
});

describe("changedFilesTotals", () => {
  test("sums additions and deletions across files", () => {
    expect(
      changedFilesTotals([
        { path: "a.rs", additions: 361, deletions: 11 },
        { path: "b.md", additions: 15, deletions: 5 },
      ]),
    ).toEqual({ fileCount: 2, additions: 376, deletions: 16 });
  });
});

describe("transcriptWindow", () => {
  const uniform = (count: number, height: number) => Array.from({ length: count }, () => height);

  test("returns only the rows overlapping the viewport, plus overscan", () => {
    const win = transcriptWindow(uniform(100, 100), 100, 1000, 300, 0);

    expect(win.startIndex).toBe(10);
    expect(win.endIndex).toBe(13);
    expect(win.offsetY).toBe(1000);
    expect(win.totalHeight).toBe(10_000);
  });

  test("offsetY stays in step with startIndex once overscan is applied", () => {
    // The offset is what positions the rendered slice. If overscan moved the
    // index without moving the offset, every row would render three rows too
    // low — a spacer bug that looks like a scroll bug.
    const heights = [50, 150, 80, 200, 120, 90];
    const win = transcriptWindow(heights, 100, 300, 100, 2);
    const expectedOffset = heights.slice(0, win.startIndex).reduce((a, b) => a + b, 0);

    expect(win.offsetY).toBe(expectedOffset);
  });

  test("variable heights are respected, not averaged", () => {
    const win = transcriptWindow([1000, 20, 20, 20], 100, 0, 100, 0);

    expect(win.startIndex).toBe(0);
    expect(win.endIndex).toBe(1);
    expect(win.totalHeight).toBe(1060);
  });

  test("unmeasured rows fall back to the estimate", () => {
    const win = transcriptWindow([0, 0, 0, 0], 200, 0, 400, 0);

    expect(win.totalHeight).toBe(800);
    expect(win.endIndex).toBe(2);
  });

  test("scrolled past the end still yields a renderable slice", () => {
    const win = transcriptWindow(uniform(10, 100), 100, 100_000, 300, 3);

    expect(win.endIndex).toBeGreaterThanOrEqual(win.startIndex);
    expect(win.endIndex).toBeLessThanOrEqual(10);
  });

  test("an empty transcript windows to nothing", () => {
    expect(transcriptWindow([], 100, 0, 500)).toEqual({
      startIndex: 0,
      endIndex: 0,
      offsetY: 0,
      totalHeight: 0,
    });
  });
});

describe("isPinnedToBottom", () => {
  test("counts as pinned within the slack threshold", () => {
    expect(isPinnedToBottom(670, 1000, 300)).toBe(true);
    expect(isPinnedToBottom(700, 1000, 300)).toBe(true);
  });

  test("scrolled up is not pinned", () => {
    expect(isPinnedToBottom(200, 1000, 300)).toBe(false);
  });

  test("overscroll still counts as pinned", () => {
    expect(isPinnedToBottom(760, 1000, 300)).toBe(true);
  });
});

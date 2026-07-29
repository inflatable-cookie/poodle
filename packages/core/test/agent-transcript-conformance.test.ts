/**
 * Cross-runtime conformance: runs the shared vectors in
 * packages/contracts/headless/vectors/agent-transcript.json against the TS
 * implementation. The Rust mirror runs the same vectors
 * (packages/contracts/headless/tests/agent_transcript_conformance.rs).
 *
 * Grouping decides what the reader sees collapsed, so a divergence between web
 * and native would surface as the desktop build summarising a turn differently
 * from the web one, with nothing failing anywhere.
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

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
  type TranscriptToolRun,
} from "../src/agent-transcript";

const vectors = JSON.parse(
  readFileSync(
    join(import.meta.dir, "..", "..", "contracts", "headless", "vectors", "agent-transcript.json"),
    "utf8",
  ),
) as {
  grouping: Array<{ name: string; items: TranscriptItem[]; blocks: any[] }>;
  windowing: Array<{
    name: string;
    heights: number[];
    estimated: number;
    scrollTop: number;
    viewport: number;
    overscan: number;
    window: Record<string, number>;
  }>;
  pinned: Array<{
    name: string;
    scrollTop: number;
    scrollHeight: number;
    clientHeight: number;
    threshold: number;
    pinned: boolean;
  }>;
};

describe("transcript grouping vectors", () => {
  for (const testCase of vectors.grouping) {
    test(testCase.name, () => {
      const blocks = groupTranscriptItems(testCase.items);

      expect(blocks).toHaveLength(testCase.blocks.length);

      blocks.forEach((block, index) => {
        const want = testCase.blocks[index];

        expect(block.kind).toBe(want.kind);
        expect(block.id).toBe(want.id);

        if (block.kind === "tool-run") {
          const run = block as TranscriptToolRun;

          expect(run.calls.map((call) => call.id)).toEqual(want.callIds);
          expect(toolRunLeadCall(run).id).toBe(want.leadCallId);
          expect(toolRunHiddenCount(run)).toBe(want.hiddenCount);
          expect(toolRunStatus(run)).toBe(want.status);
        }

        if (block.kind === "changed-files") {
          expect(changedFilesTotals(block.files)).toEqual(want.totals);
        }
      });
    });
  }
});

describe("transcript windowing vectors", () => {
  for (const testCase of vectors.windowing) {
    test(testCase.name, () => {
      expect(
        transcriptWindow(
          testCase.heights,
          testCase.estimated,
          testCase.scrollTop,
          testCase.viewport,
          testCase.overscan,
        ),
      ).toEqual(testCase.window as any);
    });
  }
});

describe("bottom-anchoring vectors", () => {
  for (const testCase of vectors.pinned) {
    test(testCase.name, () => {
      expect(
        isPinnedToBottom(
          testCase.scrollTop,
          testCase.scrollHeight,
          testCase.clientHeight,
          testCase.threshold,
        ),
      ).toBe(testCase.pinned);
    });
  }
});

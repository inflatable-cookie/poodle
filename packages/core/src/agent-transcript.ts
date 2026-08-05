/**
 * Agent transcript machinery.
 * Contract: docs/contracts/components/agent-transcript.md.
 *
 * Pure logic for the output half of an agent conversation: the item model,
 * contiguous tool-run grouping, changed-file tree folding, and variable-height
 * scroll windowing. Rendering,
 * markdown parsing and scroll plumbing stay adapter-side.
 *
 * `AgentChatInput` owns the composer and explicitly scopes out the transcript;
 * this is the other side of that boundary.
 *
 * The Rust mirror is `poodle-headless::agent_transcript`. Both are driven by
 * `packages/contracts/headless/vectors/agent-transcript.json`, so grouping
 * cannot drift between the web targets and the natives.
 */

import type { AgentQuestionAnswer, AgentQuestionItem } from "./agent-question.ts";

export type TranscriptRole = "user" | "assistant";

/** How a tool call ended. `running` is the live case — no result yet. */
export type ToolCallStatus = "running" | "success" | "error";

export interface TranscriptMessage {
  kind: "message";
  id: string;
  role: TranscriptRole;
  /** Raw markdown. Parsed by the renderer, never pre-rendered by the host. */
  markdown: string;
  /** True while tokens are still arriving; drives the caret and disables collapse. */
  isStreaming?: boolean;
}

export interface TranscriptToolCall {
  kind: "tool-call";
  id: string;
  /** What kind of work this was — "Ran command", "File change", "Searched". */
  label: string;
  /** The argument line, shown dimmed and truncated to one line when collapsed. */
  detail?: string;
  status: ToolCallStatus;
  /** Icon name for the row's leading glyph; the renderer picks a default by label. */
  icon?: string;
  /** Full output, revealed when the row is expanded. */
  output?: string;
}

export type ChangedFileStatus = "added" | "modified" | "deleted" | "renamed";

export interface ChangedFile {
  path: string;
  additions: number;
  deletions: number;
  status?: ChangedFileStatus;
}

export interface TranscriptChangedFiles {
  kind: "changed-files";
  id: string;
  files: ChangedFile[];
}

/**
 * The record an answered question leaves behind.
 *
 * The pending question lives in the composer, because its free-text override is
 * the composer's editor. This is what it leaves in the conversation once
 * answered — read-only by construction, so there is never a second input on
 * screen. Without it the transcript would have a hole where a decision was
 * made.
 */
export interface TranscriptAnsweredQuestion {
  kind: "answered-question";
  id: string;
  question: AgentQuestionItem;
  answer: AgentQuestionAnswer;
}

/** The live footer — "Working for 1h 1m". Present only while the turn runs. */
export interface TranscriptActivity {
  kind: "activity";
  id: string;
  label: string;
}

export type TranscriptItem =
  | TranscriptMessage
  | TranscriptToolCall
  | TranscriptChangedFiles
  | TranscriptAnsweredQuestion
  | TranscriptActivity;

// ── Contiguous-run grouping ──

/**
 * A run of adjacent tool calls, presented as one collapsible unit.
 *
 * Collapsed, the run shows its *last* call and hides the rest behind
 * "+N previous tool calls" — the newest call is the one still telling you
 * something, and the older ones are history you can ask for. Expanded, the run
 * lists every call in chronological order, ending on the same call that was
 * visible while collapsed, so expanding never moves the row you were reading.
 */
export interface TranscriptToolRun {
  kind: "tool-run";
  /** Stable across appends: the id of the run's first call. */
  id: string;
  calls: TranscriptToolCall[];
}

export type TranscriptBlock =
  | TranscriptMessage
  | TranscriptToolRun
  | TranscriptChangedFiles
  | TranscriptAnsweredQuestion
  | TranscriptActivity;

/**
 * Collapse contiguous tool calls into runs, leaving every other item alone.
 *
 * Adjacency is the whole rule: anything that is not a tool call ends the run.
 * A changed-files card between two commands therefore splits them into two
 * runs, which is what the transcript should say — those commands happened
 * either side of an edit, not as one stretch of work.
 *
 * Grouping lives here rather than in the host because the events arrive as a
 * flat append-only stream, and every consumer would otherwise write this same
 * scan. It is a pure function of the list so a streaming run regroups correctly
 * as calls land, with no incremental state to get out of step.
 */
export function groupTranscriptItems(
  items: readonly TranscriptItem[],
): TranscriptBlock[] {
  const blocks: TranscriptBlock[] = [];
  let run: TranscriptToolRun | null = null;

  for (const item of items) {
    if (item.kind === "tool-call") {
      if (run) {
        run.calls.push(item);
      } else {
        run = { kind: "tool-run", id: item.id, calls: [item] };
        blocks.push(run);
      }
      continue;
    }

    run = null;
    blocks.push(item);
  }

  return blocks;
}

/** The call a collapsed run shows: the newest one. */
export function toolRunLeadCall(run: TranscriptToolRun): TranscriptToolCall {
  return run.calls[run.calls.length - 1];
}

/** How many calls "+N previous tool calls" is offering. */
export function toolRunHiddenCount(run: TranscriptToolRun): number {
  return Math.max(0, run.calls.length - 1);
}

/**
 * A run's status, for the collapsed summary's indicator.
 *
 * A single failure anywhere in the run wins over any number of successes: the
 * point of the summary is to tell you whether you need to open it, and one
 * failed command inside eight successful ones is exactly when you do. `running`
 * ranks below `error` for the same reason — a run that already broke is not
 * "in progress" in any sense the reader cares about.
 */
export function toolRunStatus(run: TranscriptToolRun): ToolCallStatus {
  if (run.calls.some((call) => call.status === "error")) return "error";
  if (run.calls.some((call) => call.status === "running")) return "running";
  return "success";
}

// ── Changed-files rollup ──

export interface ChangedFilesTotals {
  fileCount: number;
  additions: number;
  deletions: number;
}

export function changedFilesTotals(
  files: readonly ChangedFile[],
): ChangedFilesTotals {
  let additions = 0;
  let deletions = 0;

  for (const file of files) {
    additions += file.additions;
    deletions += file.deletions;
  }

  return { fileCount: files.length, additions, deletions };
}

// ── Variable-height scroll windowing ──

export interface TranscriptWindow {
  startIndex: number;
  endIndex: number;
  /** Pixel offset of `startIndex` from the top of the scrolled content. */
  offsetY: number;
  totalHeight: number;
}

/**
 * The window of blocks worth rendering, for variable-height rows.
 *
 * `treeVirtualWindow` divides by a uniform row height, which a transcript does
 * not have — a one-line message, a forty-row tool run and a file tree differ by
 * an order of magnitude. So this walks measured heights instead, falling back to
 * `estimatedHeight` for rows that have not been measured yet.
 *
 * Unmeasured rows make `totalHeight` an estimate, and it changes as rows are
 * measured. That is unavoidable for variable heights and is why the scrollbar
 * settles rather than being right immediately; the alternative is measuring
 * every row up front, which defeats the point of windowing.
 */
export function transcriptWindow(
  heights: readonly number[],
  estimatedHeight: number,
  scrollTop: number,
  viewportHeight: number,
  overscan = 3,
): TranscriptWindow {
  const heightAt = (index: number): number => {
    const measured = heights[index];
    return measured && measured > 0 ? measured : estimatedHeight;
  };

  const count = heights.length;
  const top = Math.max(0, scrollTop);
  const bottom = top + Math.max(0, viewportHeight);

  let startIndex = 0;
  let offsetY = 0;
  let cursor = 0;
  let endIndex = count;
  let seenBottom = false;

  for (let index = 0; index < count; index += 1) {
    const height = heightAt(index);
    const rowBottom = cursor + height;

    if (rowBottom <= top) {
      startIndex = index + 1;
      offsetY = rowBottom;
    } else if (cursor >= bottom && !seenBottom) {
      endIndex = index;
      seenBottom = true;
    }

    cursor = rowBottom;
  }

  // Overscan is applied after the scan so it cannot push the offset out of step
  // with the index it describes.
  for (let step = 0; step < overscan && startIndex > 0; step += 1) {
    startIndex -= 1;
    offsetY -= heightAt(startIndex);
  }

  endIndex = Math.min(count, endIndex + overscan);

  return {
    startIndex,
    endIndex: Math.max(startIndex, endIndex),
    offsetY: Math.max(0, offsetY),
    totalHeight: cursor,
  };
}

/**
 * Whether the viewport is close enough to the bottom to keep following new output.
 *
 * Anchoring is a latch, not a computation: once the reader scrolls up, the
 * transcript must stop dragging them back down, and it must resume only when
 * they return to the bottom themselves. The threshold exists because "at the
 * bottom" is never exact — subpixel scroll positions and a row growing by a
 * line both leave a few pixels of slack that should still count as following.
 */
export function isPinnedToBottom(
  scrollTop: number,
  scrollHeight: number,
  clientHeight: number,
  thresholdPx = 32,
): boolean {
  return scrollHeight - (scrollTop + clientHeight) <= thresholdPx;
}

// ── Changed-file tree ──

export interface ChangedFileNode {
  /** Full path from the root, used as the tree node's value. */
  path: string;
  /** What this row displays — a collapsed chain shows `crates/latex`. */
  label: string;
  isDirectory: boolean;
  additions: number;
  deletions: number;
  children: ChangedFileNode[];
}

/**
 * Fold flat paths into a directory tree with counts rolled up from descendants.
 *
 * Single-child directory chains collapse into one row. A path like
 * `cp-api/crates/latex/src/parser.rs` would otherwise cost four rows to say one
 * thing; the chain collapses to `crates/latex` exactly as long as no node in it
 * has a sibling. That is the difference between a tree you can read and an
 * indentation staircase.
 *
 * Mirrored in `poodle-headless::agent_transcript`; both are driven by the shared
 * vectors, so the natives fold identically.
 */
export function buildChangedFileTree(files: readonly ChangedFile[]): ChangedFileNode[] {
  const root: ChangedFileNode = {
    path: "",
    label: "",
    isDirectory: true,
    additions: 0,
    deletions: 0,
    children: [],
  };

  for (const file of files) {
    const segments = file.path.split("/").filter(Boolean);
    let node = root;

    segments.forEach((segment, index) => {
      const isLeaf = index === segments.length - 1;
      const path = segments.slice(0, index + 1).join("/");
      let next = node.children.find((child) => child.path === path);

      if (!next) {
        next = {
          path,
          label: segment,
          isDirectory: !isLeaf,
          additions: 0,
          deletions: 0,
          children: [],
        };
        node.children.push(next);
      }

      next.additions += file.additions;
      next.deletions += file.deletions;
      node = next;
    });
  }

  const collapse = (nodes: ChangedFileNode[]): ChangedFileNode[] =>
    nodes.map((node) => {
      let current = node;
      let label = node.label;

      // Only directories collapse, and only through single children — a
      // directory with two entries is a real fork and has to render as one.
      while (current.isDirectory && current.children.length === 1 && current.children[0].isDirectory) {
        current = current.children[0];
        label = `${label}/${current.label}`;
      }

      return { ...current, label, children: collapse(current.children) };
    });

  return collapse(root.children);
}

/** Top-level directories with their file counts, for the collapsed summary. */
export function changedFileScopes(
  files: readonly ChangedFile[],
): Array<{ name: string; fileCount: number }> {
  const counts = new Map<string, number>();

  for (const file of files) {
    const [head] = file.path.split("/").filter(Boolean);
    const name = head ?? file.path;
    counts.set(name, (counts.get(name) ?? 0) + 1);
  }

  return [...counts].map(([name, fileCount]) => ({ name, fileCount }));
}

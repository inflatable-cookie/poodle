/**
 * g15.047 — completeness and determinism checks over a batch of captures.
 *
 * The batch denominator is closed: 18 fixtures × 3 runtimes = 54 retained
 * captures. A missing, extra, or duplicated capture fails the run before any
 * pair comparison, and a repeat capture whose bytes differ from the retained
 * one fails the whole batch — determinism is a precondition, not a metric.
 */

import type { ButtonFixture } from "../fixtures/button-visual-inventory.ts";
import { RUNTIMES, type RuntimeName } from "./receipt.ts";

/**
 * A pre-capture preview transport failure: the navigation or marker wait
 * failed before any frame existed (page degradation, dead preview server).
 * This is the ONLY failure class the batch may recover from, once, on a fresh
 * page — infrastructure recovery, never evidence selection.
 */
export class PreviewTransportError extends Error {
  constructor(message: string, readonly cause?: unknown) {
    super(message);
    this.name = "PreviewTransportError";
  }
}

/**
 * A capture-integrity failure: repeat captures diverged, or a receipt does
 * not verify against its PNG. These are evidence failures, not
 * infrastructure — they must stop the batch immediately.
 */
export class CaptureIntegrityError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "CaptureIntegrityError";
  }
}

/**
 * The batch loop's recovery decision. Only a pre-capture transport failure is
 * recoverable; determinism, receipt, and comparison failures rethrow. Kept as
 * one exported predicate so the focused tests drive the exact production
 * decision.
 */
export function isRecoverableTransportError(error: unknown): boolean {
  return error instanceof PreviewTransportError;
}

export type CaptureId = { fixture: string; runtime: RuntimeName };

/**
 * Verify the present capture set is exactly the fixture roster cross the
 * runtime set: nothing missing, nothing extra, nothing duplicated. Returns
 * problem strings; an empty list means the set is complete and closed.
 */
export function captureSetProblems(fixtures: ButtonFixture[], present: CaptureId[]): string[] {
  const problems: string[] = [];
  const seen = new Map<string, number>();
  for (const entry of present) {
    const key = `${entry.fixture} [${entry.runtime}]`;
    seen.set(key, (seen.get(key) ?? 0) + 1);
  }
  for (const [key, count] of seen) {
    if (count > 1) problems.push(`duplicated capture: ${key} appears ${count} times`);
  }
  const expected = new Set<string>();
  for (const fixture of fixtures) {
    for (const runtime of RUNTIMES) {
      expected.add(`${fixture.name} [${runtime}]`);
    }
  }
  for (const key of expected) {
    if (!seen.has(key)) problems.push(`missing capture: ${key}`);
  }
  for (const key of seen.keys()) {
    if (!expected.has(key)) problems.push(`extra capture outside the accepted batch: ${key}`);
  }
  return problems;
}

/**
 * The repeat-capture rule: both captures of one fixture/runtime pair must be
 * byte-identical. Returns a problem string, or null when they match. The
 * caller never averages, retries away, or picks the nicer frame.
 */
export function repeatMismatchProblem(id: CaptureId, firstHash: string, repeatHash: string): string | null {
  if (firstHash === repeatHash) return null;
  return (
    `repeat captures differ for ${id.fixture} [${id.runtime}]: ${firstHash} vs ${repeatHash} — ` +
    "fixed input must render byte-identically; the batch stops rather than choosing a frame"
  );
}

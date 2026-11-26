/**
 * Cross-runtime update-status conformance: runs the shared vectors in
 * packages/contracts/headless/vectors/update.json against the TypeScript core.
 * The Rust mirror runs the same vectors (tests/update_conformance.rs).
 *
 * Covers the display copy, tones, actions and notices of `updateStatusView`,
 * and the message and retry rule of every `UpdateRejectionCode`, so a rename on
 * either side fails here or on the Rust side.
 */

import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import {
  updateDownloadLabel,
  updateRejectionMessage,
  updateRejectionRetry,
  updateStatusView,
  type UpdateRejectionCode,
  type UpdateStatusAction,
  type UpdateStatusInput,
} from "../src/update";

interface ViewVector {
  id: string;
  input: UpdateStatusInput;
  expect: {
    state: string;
    tone: string;
    title: string;
    body: string | null;
    actions: string[];
    notice: { tone: string; message: string; retry: string | null } | null;
  };
}

interface RejectionVector {
  code: UpdateRejectionCode;
  message: string;
  retry: string | null;
}

const vectors = JSON.parse(
  readFileSync(
    join(import.meta.dir, "..", "..", "contracts", "headless", "vectors", "update.json"),
    "utf8",
  ),
) as { views: ViewVector[]; rejections: RejectionVector[] };

const actionType = (action: UpdateStatusAction): string => action.type;

describe("update status view (shared vectors)", () => {
  for (const vector of vectors.views) {
    test(vector.id, () => {
      const view = updateStatusView(vector.input);
      expect(view.state).toBe(vector.expect.state as typeof view.state);
      expect(view.tone).toBe(vector.expect.tone as typeof view.tone);
      expect(view.title).toBe(vector.expect.title);
      expect(view.body).toBe(vector.expect.body);
      expect(view.actions.map(actionType)).toEqual(vector.expect.actions);
      if (vector.expect.notice === null) {
        expect(view.notice).toBeNull();
      } else {
        expect(view.notice).not.toBeNull();
        expect(view.notice?.tone).toBe(vector.expect.notice.tone as "neutral" | "danger");
        expect(view.notice?.message).toBe(vector.expect.notice.message);
        expect(view.notice?.retry ? actionType(view.notice.retry) : null).toBe(
          vector.expect.notice.retry,
        );
      }
    });
  }
});

describe("update rejections (shared vectors)", () => {
  for (const vector of vectors.rejections) {
    test(`${vector.code}: message and retry`, () => {
      expect(updateRejectionMessage(vector.code)).toBe(vector.message);
      const retry = updateRejectionRetry(vector.code);
      expect(retry ? actionType(retry) : null).toBe(vector.retry);
    });
  }

  test("every rejection code in the vector is one the core knows", () => {
    const codes: string[] = vectors.rejections.map((vector) => vector.code).sort();
    const expected: string[] = [
      "staleAuthority",
      "noOffer",
      "unavailable",
      "channelMismatch",
      "unreachable",
      "signatureRejected",
      "notWritable",
      "malformedArtifact",
      "installFailed",
    ].sort();
    expect(codes).toEqual(expected);
  });
});

describe("updateDownloadLabel", () => {
  test("clamps finite fractions to 0-1 and treats null or non-finite as indeterminate", () => {
    expect(updateDownloadLabel(null)).toBe("Downloading update");
    expect(updateDownloadLabel(Number.NaN)).toBe("Downloading update");
    expect(updateDownloadLabel(Number.POSITIVE_INFINITY)).toBe("Downloading update");
    expect(updateDownloadLabel(-0.5)).toBe("Downloading update, 0%");
    expect(updateDownloadLabel(0)).toBe("Downloading update, 0%");
    expect(updateDownloadLabel(0.42)).toBe("Downloading update, 42%");
    expect(updateDownloadLabel(1)).toBe("Downloading update, 100%");
    expect(updateDownloadLabel(1.5)).toBe("Downloading update, 100%");
  });
});

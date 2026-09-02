import { describe, expect, test } from "bun:test";

import { liveClockCount } from "../src/motion-policy.ts";
import { frameAt } from "../src/icons/geometry.ts";
import {
  abortIconGeometry,
  activateIconGeometry,
  candidateFixtureIds,
  completeIconGeometry,
  createIconGeometryRuntime,
  currentIconGeometryFrame,
  ICON_GEOMETRY_DURATION_MS,
  liveGeometryClockCount,
  plannedCandidateFixture,
  sampleIconGeometry,
  setIconGeometryPolicy,
  teardownIconGeometry,
} from "../src/icons/geometry-runtime.ts";

const PAIR_A = "chevron-left-to-chevron-right";
const PAIR_B = "circle-to-dot";
const REJECTED = "menu-to-ellipsis";

function ownerIntent(pairId: string, target: "from" | "to", initial = false) {
  return { owner: "fixture-owner", pairId, target, initial };
}

describe("icon geometry runtime", () => {
  test("candidate fixtures plan and paint exact authored endpoints", () => {
    expect(candidateFixtureIds()).toEqual([
      "arrow-down-to-arrow-up",
      "arrow-left-to-arrow-right",
      "chevron-left-to-chevron-right",
      "circle-to-dot",
      "ellipsis-to-ellipsis-vertical",
      "plus-to-x",
    ]);
    const planned = plannedCandidateFixture(PAIR_A);
    expect(planned).not.toBeNull();
    const runtime = createIconGeometryRuntime("full");
    const initial = activateIconGeometry(runtime, ownerIntent(PAIR_A, "from", true));
    expect(initial.schedule).toBe(false);
    expect(initial.paintEndpoint).toBe(true);
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(currentIconGeometryFrame(runtime)?.contours.map((contour) => contour.points)).toEqual(
      frameAt(planned!, 0).contours.map((contour) => contour.points),
    );

    const start = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    expect(start.accepted).toBe(true);
    expect(start.schedule).toBe(true);
    sampleIconGeometry(runtime, start.key, 1);
    completeIconGeometry(runtime, start.key);
    expect(currentIconGeometryFrame(runtime)?.contours.map((contour) => contour.points)).toEqual(
      frameAt(planned!, 1).contours.map((contour) => contour.points),
    );
  });

  test("A→B→A before completion rebases from the sampled frame", () => {
    const planned = plannedCandidateFixture(PAIR_A)!;
    const runtime = createIconGeometryRuntime("full");
    const forward = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    const mid = sampleIconGeometry(runtime, forward.key, 0.4);
    expect(mid?.contours[0]?.points).toEqual(frameAt(planned, 0.4).contours[0]?.points);

    const reverse = activateIconGeometry(runtime, ownerIntent(PAIR_A, "from"));
    expect(reverse.interruption).toBe("reverse");
    expect(reverse.schedule).toBe(true);
    expect(reverse.pairId).toBe(PAIR_A);
    expect(liveGeometryClockCount(runtime)).toBe(1);
    const resumed = sampleIconGeometry(runtime, reverse.key, 0);
    expect(resumed?.contours[0]?.points).toEqual(frameAt(planned, 0.4).contours[0]?.points);
    sampleIconGeometry(runtime, reverse.key, 1);
    completeIconGeometry(runtime, reverse.key);
    expect(currentIconGeometryFrame(runtime)?.contours[0]?.points).toEqual(
      frameAt(planned, 0).contours[0]?.points,
    );
  });

  test("A→B→C latest-state replacement cancels the old plan", () => {
    const runtime = createIconGeometryRuntime("full");
    const first = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    sampleIconGeometry(runtime, first.key, 0.3);
    const next = activateIconGeometry(runtime, ownerIntent(PAIR_B, "to"));
    expect(next.interruption).toBe("retarget");
    expect(next.accepted).toBe(true);
    expect(next.pairId).toBe(PAIR_B);
    expect(liveGeometryClockCount(runtime)).toBe(1);
    expect(runtime.pairId).toBe(PAIR_B);
    expect(currentIconGeometryFrame(runtime)?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_B)!, 0).contours[0]?.points,
    );
  });

  test("unrelated rejected pair id cancels and cannot recover meaning", () => {
    const runtime = createIconGeometryRuntime("full");
    activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    const rejected = activateIconGeometry(runtime, ownerIntent(REJECTED, "to"));
    expect(rejected.accepted).toBe(false);
    expect(rejected.pairId).toBeNull();
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(currentIconGeometryFrame(runtime)).toBeNull();
    expect(plannedCandidateFixture(REJECTED)).toBeNull();
  });

  test("repeated current target is inert", () => {
    const runtime = createIconGeometryRuntime("full");
    const first = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    const again = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    expect(again.interruption).toBe("inert");
    expect(again.schedule).toBe(false);
    expect(again.key).toBe(first.key);
    expect(liveGeometryClockCount(runtime)).toBe(1);
  });

  test("full → reduced → frozen snaps to the latest canonical endpoint", () => {
    const planned = plannedCandidateFixture(PAIR_A)!;
    const runtime = createIconGeometryRuntime("full");
    const forward = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    sampleIconGeometry(runtime, forward.key, 0.55);
    const reduced = setIconGeometryPolicy(runtime, "reduced");
    expect(reduced[0]?.liveClock).toBe(false);
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(currentIconGeometryFrame(runtime)?.contours[0]?.points).toEqual(
      frameAt(planned, 1).contours[0]?.points,
    );
    activateIconGeometry(runtime, ownerIntent(PAIR_A, "from"));
    expect(liveGeometryClockCount(runtime)).toBe(0);
    const frozen = setIconGeometryPolicy(runtime, "frozen");
    expect(frozen).toEqual([]);
    expect(currentIconGeometryFrame(runtime)?.contours[0]?.points).toEqual(
      frameAt(planned, 0).contours[0]?.points,
    );
  });

  test("SSR authored initial paints the endpoint with no clock", () => {
    const runtime = createIconGeometryRuntime("full");
    const decision = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to", true));
    expect(decision.schedule).toBe(false);
    expect(decision.paintEndpoint).toBe(true);
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(currentIconGeometryFrame(runtime)?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_A)!, 1).contours[0]?.points,
    );
  });

  test("abort settles the endpoint and teardown drops the handle", () => {
    const runtime = createIconGeometryRuntime("full");
    const forward = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    sampleIconGeometry(runtime, forward.key, 0.2);
    abortIconGeometry(runtime, forward.key);
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(currentIconGeometryFrame(runtime)?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_A)!, 1).contours[0]?.points,
    );
    teardownIconGeometry(runtime);
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(currentIconGeometryFrame(runtime)).toBeNull();
    expect(sampleIconGeometry(runtime, forward.key, 0.9)).toBeNull();
  });

  test("hot-path samples reuse the plan buffer after creation", () => {
    const runtime = createIconGeometryRuntime("full");
    const forward = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    const first = sampleIconGeometry(runtime, forward.key, 0.2);
    const contours = first?.contours;
    const firstPoints = first?.contours[0]?.points;
    expect(contours).toBe(runtime.frame.contours);
    const second = sampleIconGeometry(runtime, forward.key, 0.8);
    expect(second?.contours).toBe(contours);
    expect(second?.contours[0]?.points).toBe(firstPoints);
    expect(ICON_GEOMETRY_DURATION_MS).toBe(180);
    expect(liveClockCount({ policy: "full", clocks: [] })).toBe(0);
  });
});

import { describe, expect, test } from "bun:test";

import { liveClockCount } from "../src/motion-policy.ts";
import { frameAt, paintedContours, type GeometryFrameBuffer } from "../src/icons/geometry.ts";
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

function painted(frame: GeometryFrameBuffer | null | undefined) {
  if (!frame) return null;
  return paintedContours(frame);
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
    expect(painted(currentIconGeometryFrame(runtime))?.contours.map((contour) => contour.points)).toEqual(
      frameAt(planned!, 0).contours.map((contour) => contour.points),
    );

    const start = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    expect(start.accepted).toBe(true);
    expect(start.schedule).toBe(true);
    sampleIconGeometry(runtime, start.key, 1);
    completeIconGeometry(runtime, start.key);
    expect(painted(currentIconGeometryFrame(runtime))?.contours.map((contour) => contour.points)).toEqual(
      frameAt(planned!, 1).contours.map((contour) => contour.points),
    );
  });

  test("A→B→A before completion rebases from the sampled frame", () => {
    const planned = plannedCandidateFixture(PAIR_A)!;
    const runtime = createIconGeometryRuntime("full");
    const forward = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    const mid = sampleIconGeometry(runtime, forward.key, 0.4);
    expect(painted(mid)?.contours[0]?.points).toEqual(frameAt(planned, 0.4).contours[0]?.points);

    const reverse = activateIconGeometry(runtime, ownerIntent(PAIR_A, "from"));
    expect(reverse.interruption).toBe("reverse");
    expect(reverse.schedule).toBe(true);
    expect(reverse.pairId).toBe(PAIR_A);
    expect(liveGeometryClockCount(runtime)).toBe(1);
    const resumed = sampleIconGeometry(runtime, reverse.key, 0);
    expect(painted(resumed)?.contours[0]?.points).toEqual(frameAt(planned, 0.4).contours[0]?.points);
    sampleIconGeometry(runtime, reverse.key, 1);
    completeIconGeometry(runtime, reverse.key);
    expect(painted(currentIconGeometryFrame(runtime))?.contours[0]?.points).toEqual(
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
    expect(painted(currentIconGeometryFrame(runtime))?.contours[0]?.points).toEqual(
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
    expect(painted(currentIconGeometryFrame(runtime))?.contours[0]?.points).toEqual(
      frameAt(planned, 1).contours[0]?.points,
    );
    activateIconGeometry(runtime, ownerIntent(PAIR_A, "from"));
    expect(liveGeometryClockCount(runtime)).toBe(0);
    const frozen = setIconGeometryPolicy(runtime, "frozen");
    expect(frozen).toEqual([]);
    expect(painted(currentIconGeometryFrame(runtime))?.contours[0]?.points).toEqual(
      frameAt(planned, 0).contours[0]?.points,
    );
  });

  test("SSR authored initial paints the endpoint with no clock", () => {
    const runtime = createIconGeometryRuntime("full");
    const decision = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to", true));
    expect(decision.schedule).toBe(false);
    expect(decision.paintEndpoint).toBe(true);
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(painted(currentIconGeometryFrame(runtime))?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_A)!, 1).contours[0]?.points,
    );
  });

  test("abort settles the endpoint and teardown drops the handle", () => {
    const runtime = createIconGeometryRuntime("full");
    const forward = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
    sampleIconGeometry(runtime, forward.key, 0.2);
    abortIconGeometry(runtime, forward.key);
    expect(liveGeometryClockCount(runtime)).toBe(0);
    expect(painted(currentIconGeometryFrame(runtime))?.contours[0]?.points).toEqual(
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

  test("a second owner on one runtime retargets; separate runtimes stay independent", () => {
    const shared = createIconGeometryRuntime("full");
    const first = activateIconGeometry(shared, {
      owner: "owner-a",
      pairId: PAIR_A,
      target: "to",
    });
    const second = activateIconGeometry(shared, {
      owner: "owner-b",
      pairId: PAIR_B,
      target: "to",
    });
    expect(liveGeometryClockCount(shared)).toBe(1);
    expect(shared.pairId).toBe(PAIR_B);
    expect(sampleIconGeometry(shared, first.key, 0.5)).toBeNull();
    expect(painted(sampleIconGeometry(shared, second.key, 0.5))?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_B)!, 0.5).contours[0]?.points,
    );

    const runtimeA = createIconGeometryRuntime("full");
    const runtimeB = createIconGeometryRuntime("full");
    const keyA = activateIconGeometry(runtimeA, {
      owner: "owner-a",
      pairId: PAIR_A,
      target: "to",
    }).key;
    const keyB = activateIconGeometry(runtimeB, {
      owner: "owner-b",
      pairId: PAIR_B,
      target: "to",
    }).key;
    expect(painted(sampleIconGeometry(runtimeA, keyA, 0.5))?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_A)!, 0.5).contours[0]?.points,
    );
    expect(painted(sampleIconGeometry(runtimeB, keyB, 0.5))?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_B)!, 0.5).contours[0]?.points,
    );
    sampleIconGeometry(runtimeA, keyA, 0.9);
    expect(painted(sampleIconGeometry(runtimeB, keyB, 0.5))?.contours[0]?.points).toEqual(
      frameAt(plannedCandidateFixture(PAIR_B)!, 0.5).contours[0]?.points,
    );
  });

  test("interior samples allocate no Map and reuse point tuples", () => {
    const constructed = { map: 0 };
    const OriginalMap = globalThis.Map;
    globalThis.Map = class extends OriginalMap {
      constructor(...args: ConstructorParameters<typeof Map>) {
        super(...(args as []));
        constructed.map += 1;
      }
    } as typeof Map;
    try {
      const runtime = createIconGeometryRuntime("full");
      const forward = activateIconGeometry(runtime, ownerIntent(PAIR_A, "to"));
      const before = constructed.map;
      const first = sampleIconGeometry(runtime, forward.key, 0.2);
      const firstTuples = first?.contours[0]?.points.map((point) => point);
      for (let index = 0; index < 32; index += 1) {
        sampleIconGeometry(runtime, forward.key, 0.21 + index / 100);
      }
      const last = sampleIconGeometry(runtime, forward.key, 0.8);
      expect(constructed.map).toBe(before);
      expect(last?.contours[0]?.points).toBe(first?.contours[0]?.points);
      last?.contours[0]?.points.forEach((point, index) => {
        expect(point).toBe(firstTuples?.[index]);
      });
    } finally {
      globalThis.Map = OriginalMap;
    }
  });
});

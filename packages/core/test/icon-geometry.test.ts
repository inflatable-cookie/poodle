import { describe, expect, test } from "bun:test";

import vectors from "../src/icons/geometry-vectors.json";
import {
  frameAt,
  geometryToWire,
  normalizeIconGeometry,
  planIconGeometryPairWithEndpoints,
  reversePairPlan,
  tryNormalizeIconGeometry,
} from "../src/icons/geometry";
import type { IconGeometryInput } from "../src/icons/geometry";

type GeometryExpectation =
  | {
      status: "accepted";
      contourCount: number;
      closed: boolean[];
      segmentCounts: number[];
      canonicalPoints?: number[][][];
    }
  | { status: "rejected"; code: string };
type PairExpectation =
  | { status: "accepted"; reversed: boolean[]; offsets: number[] }
  | { status: "rejected"; code: string };
type GeometryVector = {
  id: string;
  left: IconGeometryInput;
  right?: IconGeometryInput;
  expect: {
    left: GeometryExpectation;
    right?: GeometryExpectation;
    pair?: PairExpectation;
  };
};
const vectorDocument = vectors as unknown as { vectors: GeometryVector[] };

function canonicalPoints(geometry: ReturnType<typeof normalizeIconGeometry>) {
  return geometry.canonical.contours.map((contour) => [
    contour.segments[0]!.start,
    ...contour.segments
      .filter((segment) => !segment.closing)
      .map((segment) => segment.end),
  ]);
}

describe("icon geometry vectors", () => {
  for (const vector of vectorDocument.vectors) {
    test(vector.id, () => {
      const left = tryNormalizeIconGeometry(vector.left);
      expect(left.ok).toBe(vector.expect.left.status === "accepted");
      if (vector.expect.left.status === "rejected") {
        if (left.ok) throw new Error("vector should reject");
        expect(left.error.code as string).toBe(vector.expect.left.code);
        return;
      }

      if (!left.ok) throw left.error;
      expect(left.value.topology.contourCount).toBe(vector.expect.left.contourCount);
      expect(left.value.topology.closed).toEqual(vector.expect.left.closed);
      expect(left.value.topology.segmentCounts).toEqual(vector.expect.left.segmentCounts);
      expect(left.value.sampled.contours.every((contour) => contour.points.length === 64)).toBe(true);
      if (vector.expect.left.canonicalPoints) {
        expect(canonicalPoints(left.value)).toEqual(
          vector.expect.left.canonicalPoints as unknown as ReturnType<typeof canonicalPoints>,
        );
      }

      if (!vector.right) return;
      const right = tryNormalizeIconGeometry(vector.right);
      const expectedRight = vector.expect.right;
      if (!expectedRight) throw new Error("missing right expectation");
      expect(right.ok).toBe(expectedRight.status === "accepted");
      if (expectedRight.status === "rejected") {
        if (right.ok) throw new Error("right vector should reject");
        expect(right.error.code as string).toBe(expectedRight.code);
        return;
      }
      if (!right.ok) throw right.error;
      expect(right.value.topology.contourCount).toBe(expectedRight.contourCount);
      expect(right.value.topology.closed).toEqual(expectedRight.closed);
      expect(right.value.topology.segmentCounts).toEqual(expectedRight.segmentCounts);

      const pair = vector.expect.pair;
      if (!pair) throw new Error("missing pair expectation");
      if (pair.status === "rejected") {
        expect(() => planIconGeometryPairWithEndpoints(left.value, right.value)).toThrow(pair.code);
        return;
      }

      const planned = planIconGeometryPairWithEndpoints(left.value, right.value);
      expect(planned.plan.contourMappings.map((mapping) => mapping.reversed)).toEqual(pair.reversed);
      expect(planned.plan.contourMappings.map((mapping) => mapping.offset)).toEqual(pair.offsets);
      expect(frameAt(planned, 0).contours.map((contour) => contour.points)).toEqual(canonicalPoints(left.value));
      expect(frameAt(planned, 1).contours.map((contour) => contour.points)).toEqual(canonicalPoints(right.value));
      expect(reversePairPlan(planned.plan).contourMappings).toHaveLength(pair.reversed.length);
    });
  }

  test("keeps the wire shape versioned and quantized", () => {
    const firstVector = vectorDocument.vectors[0];
    if (!firstVector) throw new Error("missing vector fixture");
    const geometry = normalizeIconGeometry(firstVector.left);
    const wire = geometryToWire(geometry) as { schemaVersion: number; canonical: { contours: unknown[] } };
    expect(wire.schemaVersion).toBe(1);
    expect(wire.canonical.contours).toHaveLength(1);
    expect(JSON.stringify(wire)).not.toContain("0.0000");
  });
});

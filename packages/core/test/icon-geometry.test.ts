import { describe, expect, test } from "bun:test";

import vectors from "../src/icons/geometry-vectors.json";
import {
  frameAt,
  geometryWireDigest,
  geometryToWire,
  normalizeIconGeometry,
  pairWireDigest,
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
      wireDigest?: string;
    }
  | { status: "rejected"; code: string };
type PairOracle = {
  leftDigest: string;
  rightDigest: string;
  pairDigest: string;
  mappings: {
    leftIndex: number;
    rightIndex: number;
    reversed: boolean;
    offset: number;
    costMicros: number;
  }[];
  costMicros: number;
};
type PairExpectation =
  | {
      status: "accepted";
      reversed?: boolean[];
      offsets?: number[];
      oracle?: PairOracle;
    }
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
      if (vector.expect.left.wireDigest) {
        expect(geometryWireDigest(left.value)).toBe(vector.expect.left.wireDigest);
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
      if (expectedRight.wireDigest) {
        expect(geometryWireDigest(right.value)).toBe(expectedRight.wireDigest);
      }

      const pair = vector.expect.pair;
      if (!pair) throw new Error("missing pair expectation");
      if (pair.status === "rejected") {
        expect(() => planIconGeometryPairWithEndpoints(left.value, right.value)).toThrow(pair.code);
        return;
      }

      const planned = planIconGeometryPairWithEndpoints(left.value, right.value);
      if (pair.reversed) {
        expect(planned.plan.contourMappings.map((mapping) => mapping.reversed)).toEqual(
          pair.reversed,
        );
      }
      if (pair.offsets) {
        expect(planned.plan.contourMappings.map((mapping) => mapping.offset)).toEqual(
          pair.offsets,
        );
      }
      for (const mapping of planned.plan.contourMappings) {
        expect(left.value.sampled.contours[mapping.leftIndex]!.closed).toBe(
          right.value.sampled.contours[mapping.rightIndex]!.closed,
        );
      }
      const oracle = pair.oracle;
      if (!oracle) throw new Error(`missing exact pair oracle for ${vector.id}`);
      expect(geometryWireDigest(left.value)).toBe(oracle.leftDigest);
      expect(geometryWireDigest(right.value)).toBe(oracle.rightDigest);
      expect(planned.plan.contourMappings).toEqual(oracle.mappings);
      expect(planned.plan.costMicros).toBe(oracle.costMicros);
      expect(pairWireDigest(left.value, right.value, planned.plan)).toBe(oracle.pairDigest);
      expect(frameAt(planned, 0).contours.map((contour) => contour.points)).toEqual(canonicalPoints(left.value));
      expect(frameAt(planned, 1).contours.map((contour) => contour.points)).toEqual(canonicalPoints(right.value));
      const reversePlan = reversePairPlan(planned.plan);
      expect(reversePlan.contourMappings).toHaveLength(planned.plan.contourMappings.length);
      const reversePair = { left: right.value, right: left.value, plan: reversePlan };
      expect(frameAt(reversePair, 0).contours.map((contour) => contour.points)).toEqual(
        canonicalPoints(right.value),
      );
      expect(frameAt(reversePair, 1).contours.map((contour) => contour.points)).toEqual(
        canonicalPoints(left.value),
      );
      for (const progress of [0.25, 0.5, 0.75]) {
        const forward = frameAt(planned, progress);
        const reverse = frameAt(reversePair, 1 - progress);
        for (const mapping of planned.plan.contourMappings) {
          const forwardPoints = forward.contours[mapping.leftIndex]!.points;
          const reversePoints = reverse.contours[mapping.rightIndex]!.points;
          for (let index = 0; index < forwardPoints.length; index += 1) {
            const reverseIndex = mapping.reversed
              ? (mapping.offset - index + reversePoints.length) % reversePoints.length
              : (mapping.offset + index) % reversePoints.length;
            expect(reversePoints[reverseIndex]).toEqual(forwardPoints[index]);
          }
        }
      }
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

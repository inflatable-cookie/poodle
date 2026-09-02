/**
 * Private icon-geometry plan/lifecycle over candidate fixtures.
 *
 * Architecture: docs/architecture/013-icon-geometry-substrate.md
 * Motion laws: docs/architecture/012-semantic-motion-policy.md
 * Card: docs/roadmaps/g16/050-icon-geometry-internal-runtime-substrate.md
 *
 * Not a public IconMorph. Candidate geometry is fixture input only.
 */

import {
  MOTION_DURATION_MS,
  motionKey,
  type MotionInterruption,
  type MotionPolicy,
} from "../motion-policy";
import {
  ICON_GEOMETRY_NORMALIZER_VERSION,
  ICON_GEOMETRY_SAMPLE_COUNT,
  ICON_GEOMETRY_SCHEMA_VERSION,
  writeFrameAt,
  type GeometryFrame,
  type GeometryFrameBuffer,
  type GeometrySegment,
  type NormalizedIconGeometry,
  type PlannedIconGeometryPair,
} from "./geometry";
import { ICON_GEOMETRY_REGISTRY } from "./morph-pairs.generated";

export const ICON_GEOMETRY_ROLE = "icon-geometry" as const;
export const ICON_GEOMETRY_CHANNEL = "glyph" as const;
export const ICON_GEOMETRY_DURATION_MS = MOTION_DURATION_MS.standard;

export type GeometryEndpoint = "from" | "to";

export type GeometryRuntimeIntent = {
  owner: string;
  pairId: string;
  target: GeometryEndpoint;
  initial?: boolean;
};

export type GeometryRuntimeDecision = {
  key: string;
  schedule: boolean;
  interruption: MotionInterruption;
  remnant: "endpoint" | "none";
  liveClock: boolean;
  paintEndpoint: boolean;
  accepted: boolean;
  pairId: string | null;
};

type GeometryClock = {
  key: string;
  pairId: string;
  target: GeometryEndpoint;
  progress: number;
  durationMs: number;
  originalDurationMs: number;
  axisFrom: number;
  axisTo: number;
};

export type IconGeometryRuntime = {
  policy: MotionPolicy;
  clocks: GeometryClock[];
  pairId: string | null;
  plan: PlannedIconGeometryPair | null;
  frame: GeometryFrameBuffer;
};

export function createIconGeometryRuntime(
  policy: MotionPolicy = "full",
): IconGeometryRuntime {
  return {
    policy,
    clocks: [],
    pairId: null,
    plan: null,
    frame: { contours: [] },
  };
}

export function liveGeometryClockCount(runtime: IconGeometryRuntime): number {
  return runtime.clocks.length;
}

export function candidateFixtureIds(): readonly string[] {
  return ICON_GEOMETRY_REGISTRY.pairs
    .filter((pair) => pair.status === "candidate")
    .map((pair) => pair.id);
}

export function plannedCandidateFixture(pairId: string): PlannedIconGeometryPair | null {
  const record = ICON_GEOMETRY_REGISTRY.pairs.find((pair) => pair.id === pairId);
  if (!record || record.status !== "candidate" || !record.geometryLeft || !record.geometryRight || !record.plan) {
    return null;
  }
  return {
    left: inflateGeneratedGeometry(record.geometryLeft),
    right: inflateGeneratedGeometry(record.geometryRight),
    plan: {
      contourMappings: record.plan.contourMappings,
      costMicros: record.plan.costMicros,
    },
  };
}

export function activateIconGeometry(
  runtime: IconGeometryRuntime,
  intent: GeometryRuntimeIntent,
): GeometryRuntimeDecision {
  const key = motionKey(intent.owner, ICON_GEOMETRY_ROLE, ICON_GEOMETRY_CHANNEL);
  const existing = runtime.clocks.find((clock) => clock.key === key);
  const plan = plannedCandidateFixture(intent.pairId);
  if (!plan) {
    if (existing) {
      removeClock(runtime, key);
    }
    runtime.pairId = null;
    runtime.plan = null;
    runtime.frame.contours.length = 0;
    return {
      key,
      schedule: false,
      interruption: existing ? "retarget" : "none",
      remnant: "endpoint",
      liveClock: false,
      paintEndpoint: true,
      accepted: false,
      pairId: null,
    };
  }

  if (existing && existing.pairId === intent.pairId && existing.target === intent.target) {
    return {
      key,
      schedule: false,
      interruption: "inert",
      remnant: "endpoint",
      liveClock: true,
      paintEndpoint: false,
      accepted: true,
      pairId: intent.pairId,
    };
  }

  if (existing && existing.pairId === intent.pairId) {
    const current = existing.axisFrom + (existing.axisTo - existing.axisFrom) * existing.progress;
    const axisTo = axisForTarget(intent.target);
    const durationMs = Math.round(Math.abs(axisTo - current) * existing.originalDurationMs);
    existing.target = intent.target;
    existing.progress = 0;
    existing.durationMs = durationMs;
    existing.axisFrom = current;
    existing.axisTo = axisTo;
    const schedule = durationMs > 0 && shouldSchedule(runtime.policy, intent);
    if (!schedule) {
      removeClock(runtime, key);
      writeCurrentFrame(runtime, plan, axisTo);
    }
    runtime.pairId = intent.pairId;
    runtime.plan = plan;
    return {
      key,
      schedule,
      interruption: "reverse",
      remnant: "endpoint",
      liveClock: schedule,
      paintEndpoint: !schedule,
      accepted: true,
      pairId: intent.pairId,
    };
  }

  if (existing) {
    removeClock(runtime, key);
  }

  runtime.pairId = intent.pairId;
  runtime.plan = plan;
  const axisTo = axisForTarget(intent.target);
  const schedule = shouldSchedule(runtime.policy, intent);
  if (schedule) {
    runtime.clocks.push({
      key,
      pairId: intent.pairId,
      target: intent.target,
      progress: 0,
      durationMs: ICON_GEOMETRY_DURATION_MS,
      originalDurationMs: ICON_GEOMETRY_DURATION_MS,
      axisFrom: 1 - axisTo,
      axisTo,
    });
    writeCurrentFrame(runtime, plan, 1 - axisTo);
  } else {
    writeCurrentFrame(runtime, plan, axisTo);
  }
  return {
    key,
    schedule,
    interruption: existing ? "retarget" : "none",
    remnant: "endpoint",
    liveClock: schedule,
    paintEndpoint: !schedule,
    accepted: true,
    pairId: intent.pairId,
  };
}

export function sampleIconGeometry(
  runtime: IconGeometryRuntime,
  key: string,
  progress: number,
): GeometryFrame | null {
  const clock = runtime.clocks.find((entry) => entry.key === key);
  if (!clock || !runtime.plan) {
    return null;
  }
  clock.progress = Math.min(1, Math.max(0, progress));
  const axis = clock.axisFrom + (clock.axisTo - clock.axisFrom) * clock.progress;
  writeFrameAt(runtime.plan, axis, runtime.frame);
  return runtime.frame;
}

export function currentIconGeometryFrame(runtime: IconGeometryRuntime): GeometryFrame | null {
  if (!runtime.plan || runtime.frame.contours.length === 0) {
    return null;
  }
  return runtime.frame;
}

export function completeIconGeometry(
  runtime: IconGeometryRuntime,
  key: string,
): GeometryRuntimeDecision {
  const clock = runtime.clocks.find((entry) => entry.key === key);
  if (!clock || !runtime.plan) {
    return {
      key,
      schedule: false,
      interruption: "none",
      remnant: "endpoint",
      liveClock: false,
      paintEndpoint: true,
      accepted: runtime.plan !== null,
      pairId: runtime.pairId,
    };
  }
  writeCurrentFrame(runtime, runtime.plan, clock.axisTo);
  removeClock(runtime, key);
  return {
    key,
    schedule: false,
    interruption: "none",
    remnant: "endpoint",
    liveClock: false,
    paintEndpoint: true,
    accepted: true,
    pairId: runtime.pairId,
  };
}

export function setIconGeometryPolicy(
  runtime: IconGeometryRuntime,
  policy: MotionPolicy,
): GeometryRuntimeDecision[] {
  runtime.policy = policy;
  const snap = policy !== "full";
  const decisions: GeometryRuntimeDecision[] = [];
  for (const clock of [...runtime.clocks]) {
    if (snap) {
      if (runtime.plan) {
        writeCurrentFrame(runtime, runtime.plan, clock.axisTo);
      }
      removeClock(runtime, clock.key);
    }
    decisions.push({
      key: clock.key,
      schedule: !snap,
      interruption: "none",
      remnant: "endpoint",
      liveClock: !snap,
      paintEndpoint: snap,
      accepted: true,
      pairId: clock.pairId,
    });
  }
  return decisions;
}

export function abortIconGeometry(
  runtime: IconGeometryRuntime,
  key?: string,
): GeometryRuntimeDecision[] {
  return cancelClocks(runtime, key, "endpoint");
}

export function teardownIconGeometry(
  runtime: IconGeometryRuntime,
  key?: string,
): GeometryRuntimeDecision[] {
  const decisions = cancelClocks(runtime, key, "none");
  if (!key || runtime.clocks.length === 0) {
    runtime.pairId = null;
    runtime.plan = null;
    runtime.frame.contours.length = 0;
  }
  return decisions;
}

function shouldSchedule(policy: MotionPolicy, intent: GeometryRuntimeIntent): boolean {
  if (policy !== "full" || intent.initial) {
    return false;
  }
  return true;
}

function axisForTarget(target: GeometryEndpoint): number {
  return target === "to" ? 1 : 0;
}

function writeCurrentFrame(
  runtime: IconGeometryRuntime,
  plan: PlannedIconGeometryPair,
  axis: number,
): void {
  writeFrameAt(plan, axis, runtime.frame);
}

function cancelClocks(
  runtime: IconGeometryRuntime,
  key: string | undefined,
  remnant: "endpoint" | "none",
): GeometryRuntimeDecision[] {
  const selected = key ? runtime.clocks.filter((clock) => clock.key === key) : [...runtime.clocks];
  for (const clock of selected) {
    if (remnant === "endpoint" && runtime.plan && clock.pairId === runtime.pairId) {
      writeCurrentFrame(runtime, runtime.plan, clock.axisTo);
    }
    removeClock(runtime, clock.key);
  }
  return selected.map((clock) => ({
    key: clock.key,
    schedule: false,
    interruption: "none" as const,
    remnant,
    liveClock: false,
    paintEndpoint: remnant === "endpoint",
    accepted: true,
    pairId: clock.pairId,
  }));
}

function removeClock(runtime: IconGeometryRuntime, key: string): void {
  runtime.clocks = runtime.clocks.filter((clock) => clock.key !== key);
}

type GeneratedContour = {
  readonly closed: boolean;
  readonly segments?: readonly (readonly [number, number, number, number, boolean])[];
  readonly points?: readonly (readonly [number, number])[];
};

type GeneratedGeometry = {
  readonly schemaVersion: 1;
  readonly normalizerVersion: "1.0.0";
  readonly canonical: { readonly contours: readonly GeneratedContour[] };
  readonly sampled: { readonly contours: readonly GeneratedContour[] };
};

function inflateGeneratedGeometry(geometry: GeneratedGeometry): NormalizedIconGeometry {
  const canonicalContours = geometry.canonical.contours.map((contour) => ({
    closed: contour.closed,
    segments: (contour.segments ?? []).map(
      (segment): GeometrySegment => ({
        kind: "line",
        start: [segment[0], segment[1]],
        end: [segment[2], segment[3]],
        closing: segment[4],
      }),
    ),
  }));
  const sampledContours = geometry.sampled.contours.map((contour) => ({
    closed: contour.closed,
    points: (contour.points ?? []).map((point) => [point[0], point[1]] as const),
  }));
  return {
    schemaVersion: ICON_GEOMETRY_SCHEMA_VERSION,
    normalizerVersion: ICON_GEOMETRY_NORMALIZER_VERSION,
    canonical: {
      viewBox: [0, 0, 24, 24],
      contours: canonicalContours,
    },
    sampled: {
      sampleCount: ICON_GEOMETRY_SAMPLE_COUNT,
      contours: sampledContours,
    },
    topology: {
      contourCount: canonicalContours.length,
      closed: canonicalContours.map((contour) => contour.closed),
      segmentCounts: canonicalContours.map((contour) => contour.segments.length),
      sampleCount: ICON_GEOMETRY_SAMPLE_COUNT,
    },
    elementTypes: [],
  };
}

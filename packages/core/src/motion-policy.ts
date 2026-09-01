/**
 * Shared host motion policy and framework-free lifecycle laws.
 *
 * Architecture: docs/architecture/012-semantic-motion-policy.md
 * Contract: docs/contracts/components/motion-policy-provider.md
 * Rust mirror: packages/contracts/headless/src/motion_policy.rs
 *
 * Hosts resolve system preference at their edge. Components never look up
 * media queries, OS settings, or backend capture clocks.
 */

export type MotionPolicy = "full" | "reduced" | "frozen";

/** Inherited CSS/DOM hook. Carries only the effective policy. */
export const MOTION_POLICY_DATA_ATTR = "data-poodle-motion-policy";

export const MOTION_DURATION_MS = {
  fast: 120,
  standard: 180,
  slow: 260,
  skeletonPulse: 1600,
} as const;

export const MOTION_ROLE = {
  disclosureHeight: "disclosure-height",
  disclosureIndicator: "disclosure-indicator",
  toastEnter: "toast-enter",
  toastExit: "toast-exit",
  tabsUnderline: "tabs-underline",
  discreteState: "discrete-state",
  loadingLoop: "loading-loop",
} as const;

export type MotionProperty =
  | "opacity"
  | "rotate"
  | "translateX"
  | "translateY"
  | "scaleX"
  | "scaleY"
  | "height";

export type MotionInterruption = "none" | "inert" | "reverse" | "retarget";
export type MotionRemnant = "endpoint" | "none";
export type GpuiApproximation = "none" | "opacity-stand-in" | "static-endpoint";

const POLICY_RANK: Record<MotionPolicy, number> = {
  full: 0,
  reduced: 1,
  frozen: 2,
};

export function resolveMotionPreference(
  preference: MotionPolicy | null | undefined,
): MotionPolicy {
  return preference ?? "full";
}

/** Restriction-only nesting: a descendant may freeze or reduce, never re-enable. */
export function restrictMotionPolicy(
  ancestor: MotionPolicy | null | undefined,
  requested: MotionPolicy | null | undefined,
): MotionPolicy {
  const parent = resolveMotionPreference(ancestor);
  const child = resolveMotionPreference(requested);
  return POLICY_RANK[child] > POLICY_RANK[parent] ? child : parent;
}

export function motionKey(owner: string, role: string, channel: string): string {
  return `${owner}\u001f${role}\u001f${channel}`;
}

export function roleAllowsReducedOpacity(role: string): boolean {
  return (
    role === MOTION_ROLE.toastEnter ||
    role === MOTION_ROLE.toastExit ||
    role === MOTION_ROLE.discreteState
  );
}

export function isLayoutMotionProperty(property: MotionProperty): boolean {
  return property === "height";
}

/** Loading loops may start only in full, after the first committed frame. */
export function shouldRunMotionLoop(
  policy: MotionPolicy,
  animated: boolean,
  firstFrameCommitted: boolean,
): boolean {
  return animated && policy === "full" && firstFrameCommitted;
}

export function filterMotionProperties(
  policy: MotionPolicy,
  requested: readonly MotionProperty[],
  options: { loop: boolean; reducedOpacity: boolean },
): MotionProperty[] {
  if (policy === "frozen") {
    return [];
  }
  if (policy === "full") {
    return requested.slice();
  }
  if (options.loop || !options.reducedOpacity) {
    return [];
  }
  return requested.filter((property) => property === "opacity");
}

export function gpuiMotionPlan(properties: readonly MotionProperty[]): {
  applied: MotionProperty[];
  dropped: MotionProperty[];
  approximation: GpuiApproximation;
} {
  const applied: MotionProperty[] = [];
  const dropped: MotionProperty[] = [];
  for (const property of properties) {
    if (property === "opacity" || property === "rotate") {
      applied.push(property);
    } else {
      dropped.push(property);
    }
  }
  if (dropped.length === 0) {
    return { applied, dropped, approximation: "none" };
  }
  if (dropped.some(isLayoutMotionProperty)) {
    return { applied, dropped, approximation: "static-endpoint" };
  }
  const approximation = applied.includes("opacity") ? "opacity-stand-in" : "static-endpoint";
  return { applied, dropped, approximation };
}

export interface MotionIntent {
  owner: string;
  role: string;
  channel: string;
  target: string;
  properties: MotionProperty[];
  durationMs: number;
  loop?: boolean;
  initial?: boolean;
  firstFrameCommitted?: boolean;
  forcedStatic?: boolean;
  reversible?: boolean;
  reducedOpacity?: boolean;
}

export interface MotionDecision {
  key: string;
  schedule: boolean;
  properties: MotionProperty[];
  durationMs: number;
  interruption: MotionInterruption;
  remnant: MotionRemnant;
  liveClock: boolean;
  paintEndpoint: boolean;
}

export interface MotionClock {
  key: string;
  target: string;
  progress: number;
  properties: MotionProperty[];
  durationMs: number;
  originalDurationMs: number;
  axisFrom: number;
  axisTo: number;
  loop: boolean;
  reversible: boolean;
  reducedOpacity: boolean;
}

export interface MotionTrace {
  policy: MotionPolicy;
  clocks: MotionClock[];
}

export function createMotionTrace(policy: MotionPolicy = "full"): MotionTrace {
  return { policy, clocks: [] };
}

export function liveClockCount(trace: MotionTrace): number {
  return trace.clocks.length;
}

function shouldSchedule(
  policy: MotionPolicy,
  intent: MotionIntent,
  properties: readonly MotionProperty[],
): boolean {
  if (intent.forcedStatic || properties.length === 0 || policy === "frozen") {
    return false;
  }
  if (intent.loop) {
    return policy === "full" && intent.firstFrameCommitted === true;
  }
  if (intent.initial) {
    return false;
  }
  return true;
}

function reducedOpacityFor(intent: MotionIntent): boolean {
  return intent.reducedOpacity ?? roleAllowsReducedOpacity(intent.role);
}

function axisForTarget(target: string): number | undefined {
  if (target === "open") {
    return 1;
  }
  if (target === "closed") {
    return 0;
  }
  return undefined;
}

export function activateMotion(trace: MotionTrace, intent: MotionIntent): MotionDecision {
  const key = motionKey(intent.owner, intent.role, intent.channel);
  const reducedOpacity = reducedOpacityFor(intent);
  const properties = filterMotionProperties(trace.policy, intent.properties, {
    loop: intent.loop === true,
    reducedOpacity,
  });
  const existing = trace.clocks.find((clock) => clock.key === key);

  if (existing && existing.target === intent.target) {
    return {
      key,
      schedule: false,
      properties: existing.properties,
      durationMs: existing.durationMs,
      interruption: "inert",
      remnant: "endpoint",
      liveClock: true,
      paintEndpoint: false,
    };
  }

  if (existing && existing.reversible && intent.reversible) {
    const current = existing.axisFrom + (existing.axisTo - existing.axisFrom) * existing.progress;
    const axisTo = axisForTarget(intent.target) ?? (existing.axisTo === 1 ? 0 : 1);
    const durationMs = Math.round(Math.abs(axisTo - current) * existing.originalDurationMs);
    existing.target = intent.target;
    existing.progress = 0;
    existing.durationMs = durationMs;
    existing.axisFrom = current;
    existing.axisTo = axisTo;
    existing.properties = properties;
    const schedule = durationMs > 0 && shouldSchedule(trace.policy, { ...intent, initial: false }, properties);
    if (!schedule) {
      removeClock(trace, key);
    }
    return {
      key,
      schedule,
      properties,
      durationMs,
      interruption: "reverse",
      remnant: "endpoint",
      liveClock: schedule,
      paintEndpoint: !schedule,
    };
  }

  if (existing && !existing.reversible) {
    existing.target = intent.target;
    existing.progress = 0;
    existing.properties = properties;
    const schedule = shouldSchedule(trace.policy, { ...intent, initial: false }, properties);
    if (!schedule) {
      removeClock(trace, key);
    }
    return {
      key,
      schedule,
      properties,
      durationMs: existing.durationMs,
      interruption: "retarget",
      remnant: "endpoint",
      liveClock: schedule,
      paintEndpoint: !schedule,
    };
  }

  const schedule = shouldSchedule(trace.policy, intent, properties);
  if (schedule) {
    const axisTo = axisForTarget(intent.target) ?? 1;
    trace.clocks.push({
      key,
      target: intent.target,
      progress: 0,
      properties,
      durationMs: intent.durationMs,
      originalDurationMs: intent.durationMs,
      axisFrom: 1 - axisTo,
      axisTo,
      loop: intent.loop === true,
      reversible: intent.reversible === true,
      reducedOpacity,
    });
  }
  return {
    key,
    schedule,
    properties,
    durationMs: intent.durationMs,
    interruption: "none",
    remnant: "endpoint",
    liveClock: schedule,
    paintEndpoint: !schedule,
  };
}

export function sampleMotion(trace: MotionTrace, key: string, progress: number): void {
  const clock = trace.clocks.find((entry) => entry.key === key);
  if (!clock) {
    return;
  }
  clock.progress = Math.min(1, Math.max(0, progress));
}

export function completeMotion(trace: MotionTrace, key: string): MotionDecision {
  const clock = trace.clocks.find((entry) => entry.key === key);
  if (!clock) {
    return {
      key,
      schedule: false,
      properties: [],
      durationMs: 0,
      interruption: "none",
      remnant: "endpoint",
      liveClock: false,
      paintEndpoint: true,
    };
  }
  if (!clock.loop) {
    removeClock(trace, key);
  } else {
    clock.progress = 0;
  }
  return {
    key,
    schedule: false,
    properties: clock.properties,
    durationMs: clock.durationMs,
    interruption: "none",
    remnant: "endpoint",
    liveClock: clock.loop,
    paintEndpoint: true,
  };
}

export function setMotionTracePolicy(trace: MotionTrace, policy: MotionPolicy): MotionDecision[] {
  trace.policy = policy;
  const decisions: MotionDecision[] = [];
  for (const clock of [...trace.clocks]) {
    const properties = filterMotionProperties(trace.policy, clock.properties, {
      loop: clock.loop,
      reducedOpacity: clock.reducedOpacity,
    });
    if (trace.policy === "frozen" || properties.length === 0 || (trace.policy === "reduced" && clock.loop)) {
      removeClock(trace, clock.key);
      decisions.push({
        key: clock.key,
        schedule: false,
        properties,
        durationMs: clock.durationMs,
        interruption: "none",
        remnant: "endpoint",
        liveClock: false,
        paintEndpoint: true,
      });
      continue;
    }
    clock.properties = properties;
    const continueClock = trace.policy !== "reduced" || (properties.length === 1 && properties[0] === "opacity");
    if (!continueClock) {
      removeClock(trace, clock.key);
    }
    decisions.push({
      key: clock.key,
      schedule: continueClock,
      properties,
      durationMs: clock.durationMs,
      interruption: "none",
      remnant: "endpoint",
      liveClock: continueClock,
      paintEndpoint: !continueClock,
    });
  }
  return decisions;
}

export function abortMotion(trace: MotionTrace, key?: string): MotionDecision[] {
  return cancelClocks(trace, key, "endpoint");
}

export function unmountMotion(trace: MotionTrace, key?: string): MotionDecision[] {
  return cancelClocks(trace, key, "none");
}

function cancelClocks(
  trace: MotionTrace,
  key: string | undefined,
  remnant: MotionRemnant,
): MotionDecision[] {
  const selected = key ? trace.clocks.filter((clock) => clock.key === key) : [...trace.clocks];
  for (const clock of selected) {
    removeClock(trace, clock.key);
  }
  return selected.map((clock) => ({
    key: clock.key,
    schedule: false,
    properties: clock.properties,
    durationMs: clock.durationMs,
    interruption: "none" as const,
    remnant,
    liveClock: false,
    paintEndpoint: remnant === "endpoint",
  }));
}

function removeClock(trace: MotionTrace, key: string): void {
  trace.clocks = trace.clocks.filter((clock) => clock.key !== key);
}

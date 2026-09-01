import { describe, expect, test } from "bun:test";

import {
  abortMotion,
  activateMotion,
  createMotionTrace,
  filterMotionProperties,
  gpuiMotionPlan,
  liveClockCount,
  MOTION_DURATION_MS,
  MOTION_ROLE,
  motionKey,
  resolveMotionPreference,
  restrictMotionPolicy,
  sampleMotion,
  setMotionTracePolicy,
  unmountMotion,
  type MotionIntent,
  type MotionProperty,
} from "../src/motion-policy.ts";

function oneShot(target: string, properties: MotionProperty[]): MotionIntent {
  return {
    owner: "owner-a",
    role: MOTION_ROLE.disclosureHeight,
    channel: "panel",
    target,
    properties,
    durationMs: MOTION_DURATION_MS.standard,
    reversible: true,
  };
}

describe("motion policy laws", () => {
  test("missing preference resolves to full", () => {
    expect(resolveMotionPreference(null)).toBe("full");
    expect(resolveMotionPreference(undefined)).toBe("full");
    expect(restrictMotionPolicy(null, undefined)).toBe("full");
  });

  test("nesting is restriction-only", () => {
    expect(restrictMotionPolicy("reduced", "full")).toBe("reduced");
    expect(restrictMotionPolicy("frozen", "reduced")).toBe("frozen");
    expect(restrictMotionPolicy("full", "reduced")).toBe("reduced");
  });

  test("keys are semantic and stable across rebuilds", () => {
    const first = motionKey("item-1", MOTION_ROLE.disclosureHeight, "panel");
    const rebuilt = motionKey("item-1", MOTION_ROLE.disclosureHeight, "panel");
    const sibling = motionKey("item-2", MOTION_ROLE.disclosureHeight, "panel");
    const channel = motionKey("item-1", MOTION_ROLE.disclosureIndicator, "glyph");
    expect(first).toBe(rebuilt);
    expect(first).not.toBe(sibling);
    expect(first).not.toBe(channel);
  });

  test("authored initial state paints the endpoint", () => {
    const trace = createMotionTrace("full");
    const decision = activateMotion(trace, {
      ...oneShot("open", ["height", "rotate"]),
      initial: true,
    });
    expect(decision.schedule).toBe(false);
    expect(decision.paintEndpoint).toBe(true);
    expect(liveClockCount(trace)).toBe(0);
  });

  test("loading loop waits for the first committed frame", () => {
    const trace = createMotionTrace("full");
    const intent: MotionIntent = {
      owner: "skeleton",
      role: MOTION_ROLE.loadingLoop,
      channel: "pulse",
      target: "loading",
      properties: ["opacity"],
      durationMs: MOTION_DURATION_MS.skeletonPulse,
      loop: true,
    };
    const before = activateMotion(trace, intent);
    expect(before.schedule).toBe(false);
    expect(liveClockCount(trace)).toBe(0);
    const after = activateMotion(trace, { ...intent, firstFrameCommitted: true });
    expect(after.schedule).toBe(true);
    expect(liveClockCount(trace)).toBe(1);
  });

  test("repeated target is inert and reversal uses remaining progress", () => {
    const trace = createMotionTrace("full");
    const open = activateMotion(trace, oneShot("open", ["height"]));
    expect(open.schedule).toBe(true);
    sampleMotion(trace, open.key, 0.4);
    const repeat = activateMotion(trace, oneShot("open", ["height"]));
    expect(repeat.interruption).toBe("inert");
    expect(liveClockCount(trace)).toBe(1);
    expect(trace.clocks[0]?.progress).toBe(0.4);

    const close = activateMotion(trace, oneShot("closed", ["height"]));
    expect(close.interruption).toBe("reverse");
    expect(close.durationMs).toBe(72);
    expect(liveClockCount(trace)).toBe(1);
    expect(trace.clocks[0]?.target).toBe("closed");
    expect(trace.clocks[0]?.progress).toBe(0);

    sampleMotion(trace, close.key, 0.5);
    const reopen = activateMotion(trace, oneShot("open", ["height"]));
    expect(reopen.interruption).toBe("reverse");
    expect(reopen.durationMs).toBe(144);
    expect(trace.clocks[0]?.target).toBe("open");
  });

  test("multi-target retarget does not queue", () => {
    const trace = createMotionTrace("full");
    const base: MotionIntent = {
      owner: "tabs",
      role: MOTION_ROLE.tabsUnderline,
      channel: "indicator",
      target: "a",
      properties: ["translateX"],
      durationMs: MOTION_DURATION_MS.standard,
    };
    activateMotion(trace, base);
    activateMotion(trace, { ...base, target: "b" });
    const third = activateMotion(trace, { ...base, target: "c" });
    expect(third.interruption).toBe("retarget");
    expect(liveClockCount(trace)).toBe(1);
    expect(trace.clocks[0]?.target).toBe("c");
  });

  test("reduced keeps only allowed opacity and drops layout", () => {
    expect(filterMotionProperties("reduced", ["height", "rotate"], { loop: false, reducedOpacity: false })).toEqual(
      [],
    );
    expect(
      filterMotionProperties("reduced", ["opacity", "translateY"], { loop: false, reducedOpacity: true }),
    ).toEqual(["opacity"]);
    expect(filterMotionProperties("reduced", ["opacity"], { loop: true, reducedOpacity: true })).toEqual([]);
  });

  test("tightening full to reduced to frozen is honest", () => {
    const trace = createMotionTrace("full");
    activateMotion(trace, {
      owner: "toast-1",
      role: MOTION_ROLE.toastEnter,
      channel: "item",
      target: "enter",
      properties: ["opacity", "translateY"],
      durationMs: MOTION_DURATION_MS.standard,
      reducedOpacity: true,
    });
    activateMotion(trace, {
      owner: "spinner",
      role: MOTION_ROLE.loadingLoop,
      channel: "ring",
      target: "spin",
      properties: ["rotate"],
      durationMs: 800,
      loop: true,
      firstFrameCommitted: true,
    });
    expect(liveClockCount(trace)).toBe(2);

    const reduced = setMotionTracePolicy(trace, "reduced");
    expect(trace.policy).toBe("reduced");
    expect(liveClockCount(trace)).toBe(1);
    expect(trace.clocks[0]?.properties).toEqual(["opacity"]);
    expect(reduced.some((decision) => !decision.liveClock)).toBe(true);

    const frozen = setMotionTracePolicy(trace, "frozen");
    expect(trace.policy).toBe("frozen");
    expect(liveClockCount(trace)).toBe(0);
    expect(frozen.every((decision) => !decision.liveClock && decision.paintEndpoint)).toBe(true);

    setMotionTracePolicy(trace, "full");
    expect(trace.policy).toBe("full");
  });

  test("abort keeps the endpoint and unmount drops the remnant", () => {
    const trace = createMotionTrace("full");
    const decision = activateMotion(trace, oneShot("open", ["height"]));
    const aborted = abortMotion(trace, decision.key);
    expect(aborted[0]?.remnant).toBe("endpoint");
    expect(liveClockCount(trace)).toBe(0);

    activateMotion(trace, oneShot("open", ["height"]));
    const unmounted = unmountMotion(trace);
    expect(unmounted[0]?.remnant).toBe("none");
    expect(liveClockCount(trace)).toBe(0);
  });

  test("height is the only layout exception and GPUI names gaps", () => {
    const height = gpuiMotionPlan(["height"]);
    expect(height.approximation).toBe("static-endpoint");
    expect(height.applied).toEqual([]);
    const toast = gpuiMotionPlan(["opacity", "translateY"]);
    expect(toast.approximation).toBe("opacity-stand-in");
    expect(toast.applied).toEqual(["opacity"]);
    const spin = gpuiMotionPlan(["rotate"]);
    expect(spin.approximation).toBe("none");
  });

  test("child full cannot re-enable reduced", () => {
    const trace = createMotionTrace("reduced");
    const decision = activateMotion(trace, oneShot("open", ["height"]));
    expect(decision.schedule).toBe(false);
    expect(decision.paintEndpoint).toBe(true);
    expect(liveClockCount(trace)).toBe(0);
  });
});

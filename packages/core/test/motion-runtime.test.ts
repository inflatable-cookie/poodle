import { afterEach, describe, expect, test } from "bun:test";

import {
  cancelWebMotion,
  createMotionTrace,
  liveClockCount,
  liveWebMotionCount,
  MOTION_DURATION_MS,
  MOTION_ROLE,
  motionKey,
  playClippedHeight,
  playWebAnimation,
} from "../src/index.ts";

afterEach(() => {
  cancelWebMotion();
});

type Hold = {
  cancel: () => void;
  finish: () => void;
};

function fakeElement(onCreate: (hold: Hold) => void): HTMLElement {
  const element = {
    style: {} as CSSStyleDeclaration,
    scrollHeight: 80,
    animate() {
      let settled = false;
      let resolve!: (value: unknown) => void;
      let reject!: (reason?: unknown) => void;
      const finished = new Promise((res, fail) => {
        resolve = res;
        reject = fail;
      });
      const hold: Hold = {
        cancel() {
          if (settled) {
            return;
          }
          settled = true;
          reject(new DOMException("The user aborted a request.", "AbortError"));
        },
        finish() {
          if (settled) {
            return;
          }
          settled = true;
          resolve(undefined);
        },
      };
      onCreate(hold);
      return { cancel: () => hold.cancel(), finished };
    },
  };
  return element as unknown as HTMLElement;
}

describe("web motion runtime", () => {
  test("replacing a key does not let the cancelled promise drop the new handle", async () => {
    const holds: Hold[] = [];
    const element = fakeElement((hold) => holds.push(hold));
    const intent = {
      owner: "owner",
      role: MOTION_ROLE.toastEnter,
      channel: "item",
      target: "enter",
      properties: ["opacity"] as const,
      durationMs: MOTION_DURATION_MS.standard,
      reducedOpacity: true,
    };
    playWebAnimation(createMotionTrace("full"), { ...intent }, element, [{ opacity: 0 }, { opacity: 1 }]);
    expect(liveWebMotionCount()).toBe(1);
    playWebAnimation(
      createMotionTrace("full"),
      { ...intent, target: "enter-again" },
      element,
      [{ opacity: 1 }, { opacity: 0 }],
    );
    expect(liveWebMotionCount()).toBe(1);
    await Promise.resolve();
    await Promise.resolve();
    expect(liveWebMotionCount()).toBe(1);
    holds.at(-1)?.finish();
    await Promise.resolve();
    await Promise.resolve();
    expect(liveWebMotionCount()).toBe(0);
  });

  test("clipped height retains the same owner clock across calls", () => {
    const holds: Hold[] = [];
    const element = fakeElement((hold) => holds.push(hold));
    const open = playClippedHeight(element, {
      owner: "panel",
      open: true,
      policy: "full",
      initial: false,
    });
    expect(open.schedule).toBe(true);
    expect(liveWebMotionCount()).toBe(1);
    const again = playClippedHeight(element, {
      owner: "panel",
      open: true,
      policy: "full",
      initial: false,
    });
    expect(again.interruption).toBe("inert");
    expect(liveWebMotionCount()).toBe(1);
  });

  test("natural clipped-height completion removes the exact live handle", async () => {
    const holds: Hold[] = [];
    const element = fakeElement((hold) => holds.push(hold));
    let completion: string | undefined;
    const decision = playClippedHeight(element, {
      owner: "panel",
      open: true,
      policy: "full",
      initial: false,
      onComplete: (status) => {
        completion = status;
      },
    });

    expect(decision.schedule).toBe(true);
    expect(liveWebMotionCount()).toBe(1);
    holds[0]?.finish();
    await Promise.resolve();
    await Promise.resolve();
    expect(completion).toBe("finish");
    expect(liveWebMotionCount()).toBe(0);
    expect(element.style.height).toBe("");
  });

  test("unsupported WAAPI paints the endpoint without retaining a clock", () => {
    const element = { style: {} as CSSStyleDeclaration } as unknown as HTMLElement;
    const trace = createMotionTrace("full");
    const decision = playWebAnimation(
      trace,
      {
        owner: "owner",
        role: MOTION_ROLE.toastEnter,
        channel: "item",
        target: "enter",
        properties: ["opacity"],
        durationMs: MOTION_DURATION_MS.standard,
        reducedOpacity: true,
      },
      element,
      [{ opacity: 0 }, { opacity: 1 }],
    );
    expect(decision.schedule).toBe(false);
    expect(decision.paintEndpoint).toBe(true);
    expect(liveClockCount(trace)).toBe(0);
    expect(liveWebMotionCount()).toBe(0);
  });

  test("cancelling a handle cannot erase a synchronous replacement", () => {
    const holds: Hold[] = [];
    const element = fakeElement((hold) => holds.push(hold));
    const intent = {
      owner: "owner",
      role: MOTION_ROLE.toastEnter,
      channel: "item",
      target: "enter",
      properties: ["opacity"] as const,
      durationMs: MOTION_DURATION_MS.standard,
      reducedOpacity: true,
    };
    const replacementTrace = createMotionTrace("full");
    let replaced = false;
    playWebAnimation(
      createMotionTrace("full"),
      intent,
      element,
      [{ opacity: 0 }, { opacity: 1 }],
      "ease-out",
      (status) => {
        if (status === "cancel" && !replaced) {
          replaced = true;
          playWebAnimation(
            replacementTrace,
            { ...intent, target: "replacement" },
            element,
            [{ opacity: 1 }, { opacity: 0 }],
          );
        }
      },
    );

    cancelWebMotion(motionKey(intent.owner, intent.role, intent.channel));
    expect(liveWebMotionCount()).toBe(1);
    holds.at(-1)?.finish();
  });
});

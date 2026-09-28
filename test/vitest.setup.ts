import { afterEach, beforeEach, vi } from "vitest";

// happy-dom implements requestAnimationFrame but not cancelAnimationFrame.
// Own both sides of the pair so motion-ready callbacks are teardown-safe and
// test state cannot accumulate across roster sweeps.
const pendingFrames = new Map<number, ReturnType<typeof setTimeout>>();
let nextFrameId = 1;
globalThis.requestAnimationFrame = ((cb: FrameRequestCallback) => {
  const id = nextFrameId++;
  pendingFrames.set(
    id,
    setTimeout(() => {
      pendingFrames.delete(id);
      cb(Date.now());
    }, 0),
  );
  return id;
}) as typeof requestAnimationFrame;
globalThis.cancelAnimationFrame = ((id: number) => {
  const timer = pendingFrames.get(id);
  if (timer !== undefined) {
    clearTimeout(timer);
    pendingFrames.delete(id);
  }
}) as typeof cancelAnimationFrame;

// happy-dom lacks the Web Animations API, which disclosure motion
// (Collapsible/Accordion panels) and Svelte 5 transitions (Drawer) call
// through `element.animate`. One shared stub here — component suites must
// not carry their own copies. The fake reports a finished animation and
// fires `onfinish` on the next microtask, after the caller has attached it,
// which drives intros/outros to completion; `finished` resolves so the
// motion runtime's completion path runs too. Guarded for the node project,
// which shares this setup file but has no Element.
if (typeof Element !== "undefined" && !("animate" in Element.prototype)) {
  (Element.prototype as unknown as { animate: () => unknown }).animate = () => {
    const animation = {
      onfinish: null as (() => void) | null,
      cancel: () => {},
      playState: "finished",
      currentTime: 0,
      effect: null,
      finished: Promise.resolve(),
    };
    queueMicrotask(() => animation.onfinish?.());
    return animation;
  };
}

// Smoke-test guard: any console.error during a render fails the test. Catches
// React key warnings, invalid DOM nesting, Svelte binding errors, etc. — the
// silent breakage a plain "did it mount" assertion would miss.
let captured: unknown[][] = [];

beforeEach(() => {
  captured = [];
  vi.spyOn(console, "error").mockImplementation((...args) => {
    captured.push(args);
  });
});

afterEach(() => {
  for (const timer of pendingFrames.values()) {
    clearTimeout(timer);
  }
  pendingFrames.clear();
  const spy = console.error as unknown as { mockRestore?: () => void };
  spy.mockRestore?.();
  if (captured.length > 0) {
    const detail = captured.map((a) => a.map(String).join(" ")).join("\n");
    throw new Error(`console.error called during test:\n${detail}`);
  }
});

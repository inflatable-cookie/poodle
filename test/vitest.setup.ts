import { afterEach, beforeEach, vi } from "vitest";

// happy-dom implements requestAnimationFrame but not cancelAnimationFrame.
// Motion-ready first-frame callbacks then accumulate across roster sweeps and
// OOM the worker. Own both sides of the pair in tests.
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

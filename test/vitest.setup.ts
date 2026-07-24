import { afterEach, beforeEach, vi } from "vitest";

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
  const spy = console.error as unknown as { mockRestore?: () => void };
  spy.mockRestore?.();
  if (captured.length > 0) {
    const detail = captured.map((a) => a.map(String).join(" ")).join("\n");
    throw new Error(`console.error called during test:\n${detail}`);
  }
});

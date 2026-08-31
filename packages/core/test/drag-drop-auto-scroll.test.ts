import { describe, expect, test } from "bun:test";

import {
  canScrollVertical,
  resolveAutoScroll,
  type AutoScrollCandidate,
  type AutoScrollMetrics,
} from "../src/dom/drag-drop-auto-scroll";

function metrics(overrides: Partial<AutoScrollMetrics> = {}): AutoScrollMetrics {
  return {
    scrollTop: 80,
    scrollLeft: 0,
    scrollHeight: 400,
    scrollWidth: 200,
    clientHeight: 100,
    clientWidth: 200,
    rect: { top: 0, right: 200, bottom: 100, left: 0 },
    ...overrides,
    rect: overrides.rect ?? { top: 0, right: 200, bottom: 100, left: 0 },
  };
}

function candidate(id: string, depth: number, overrides: Partial<AutoScrollMetrics> = {}): AutoScrollCandidate {
  return { id, depth, metrics: metrics(overrides) };
}

describe("resolveAutoScroll", () => {
  test("accelerates toward the nearer edge of one container", () => {
    const far = resolveAutoScroll([candidate("outer", 0)], { x: 100, y: 20 }, 100);
    const near = resolveAutoScroll([candidate("outer", 0)], { x: 100, y: 4 }, 100);
    expect(far?.id).toBe("outer");
    expect(near?.id).toBe("outer");
    expect(far?.dy ?? 0).toBeLessThan(0);
    expect(near?.dy ?? 0).toBeLessThan(far?.dy ?? 0);
  });

  test("prefers the deeper container while it can still scroll", () => {
    const inner = candidate("inner", 2, {
      rect: { top: 10, right: 180, bottom: 90, left: 20 },
      clientHeight: 80,
      scrollHeight: 300,
      scrollTop: 40,
    });
    const outer = candidate("outer", 1);
    const picked = resolveAutoScroll([outer, inner], { x: 100, y: 18 }, 50);
    expect(picked?.id).toBe("inner");
  });

  test("falls through to the outer container when the inner is exhausted", () => {
    const inner = candidate("inner", 2, {
      rect: { top: 10, right: 180, bottom: 90, left: 20 },
      clientHeight: 80,
      scrollHeight: 80,
      scrollTop: 0,
    });
    const outer = candidate("outer", 1);
    const picked = resolveAutoScroll([outer, inner], { x: 100, y: 18 }, 50);
    expect(picked?.id).toBe("outer");
    expect(picked?.dy ?? 0).toBeLessThan(0);
  });

  test("uses the inner edge direction even when that edge is mid-outer", () => {
    const inner = candidate("inner", 2, {
      rect: { top: 80, right: 180, bottom: 160, left: 20 },
      clientHeight: 80,
      scrollHeight: 80,
      scrollTop: 0,
    });
    const outer = candidate("outer", 1, {
      rect: { top: 0, right: 200, bottom: 200, left: 0 },
      clientHeight: 200,
      scrollHeight: 400,
      scrollTop: 80,
    });
    const picked = resolveAutoScroll([outer, inner], { x: 100, y: 86 }, 50);
    expect(picked?.id).toBe("outer");
    expect(picked?.dy ?? 0).toBeLessThan(0);
  });

  test("stops when the pointer leaves every container", () => {
    expect(resolveAutoScroll([candidate("outer", 0)], { x: 100, y: 400 }, 50)).toBeNull();
  });

  test("reports no delta when the container cannot scroll further", () => {
    const exhausted = candidate("outer", 0, { scrollTop: 0, scrollHeight: 100, clientHeight: 100 });
    expect(canScrollVertical(exhausted.metrics, 1)).toBe(false);
    expect(resolveAutoScroll([exhausted], { x: 100, y: 4 }, 50)).toBeNull();
  });
});

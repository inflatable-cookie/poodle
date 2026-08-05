import { describe, expect, test } from "bun:test";

import {
  intersectClip,
  isAnchorClipped,
  isPointAnchorClipped,
  viewportClipRect,
  type ClipRect,
} from "../src/dom/anchor.ts";

const rect = (top: number, right: number, bottom: number, left: number): ClipRect => ({
  top,
  right,
  bottom,
  left,
});

describe("viewportClipRect", () => {
  test("spans the whole viewport", () => {
    expect(viewportClipRect({ width: 1280, height: 900 })).toEqual(rect(0, 1280, 900, 0));
  });
});

describe("intersectClip", () => {
  test("keeps the tighter edge on every side", () => {
    expect(intersectClip(rect(0, 1000, 800, 0), rect(100, 600, 400, 50))).toEqual(
      rect(100, 600, 400, 50),
    );
  });

  test("disjoint boxes produce an inverted (empty) rect", () => {
    const result = intersectClip(rect(0, 100, 50, 0), rect(200, 100, 300, 0));
    expect(result.bottom).toBeLessThan(result.top);
  });
});

describe("isAnchorClipped", () => {
  const scroller = rect(120, 800, 600, 0);

  test("an anchor inside its clipper is visible", () => {
    expect(isAnchorClipped(rect(200, 300, 230, 100), scroller)).toBe(false);
  });

  test("an anchor partly inside stays visible", () => {
    // Scrolled so its top half is above the scroller's top edge.
    expect(isAnchorClipped(rect(100, 300, 140, 100), scroller)).toBe(false);
  });

  test("an anchor scrolled past the top edge is clipped", () => {
    expect(isAnchorClipped(rect(60, 300, 110, 100), scroller)).toBe(true);
  });

  test("an anchor scrolled past the bottom edge is clipped", () => {
    expect(isAnchorClipped(rect(620, 300, 650, 100), scroller)).toBe(true);
  });

  test("a zero-height sliver at the edge counts as clipped", () => {
    // Exactly flush with the top edge: nothing left to aim at.
    expect(isAnchorClipped(rect(90, 300, 120, 100), scroller)).toBe(true);
  });

  test("horizontal clipping counts too", () => {
    expect(isAnchorClipped(rect(200, -10, 230, -120), scroller)).toBe(true);
  });
});

describe("isPointAnchorClipped", () => {
  const clip = rect(100, 300, 400, 50);

  test("keeps a zero-size point visible inside its clip box", () => {
    expect(isPointAnchorClipped(rect(200, 200, 200, 200), clip)).toBe(false);
  });

  test("clips points outside or touching the clip box", () => {
    expect(isPointAnchorClipped(rect(40, 200, 40, 200), clip)).toBe(true);
    expect(isPointAnchorClipped(rect(200, 50, 200, 50), clip)).toBe(true);
    expect(isPointAnchorClipped(rect(400, 200, 400, 200), clip)).toBe(true);
  });
});

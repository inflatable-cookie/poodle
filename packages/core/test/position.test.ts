import { describe, expect, test } from "bun:test";

import { resolveOverlayPosition, type RectLike } from "../src/position.ts";

const viewport = { width: 1000, height: 800 };

function rect(left: number, top: number, width: number, height: number): RectLike {
  return { left, top, width, height, right: left + width, bottom: top + height };
}

const anchor = rect(450, 380, 100, 40); // roughly centered
const overlay = rect(0, 0, 200, 100);

describe("resolveOverlayPosition", () => {
  test("bottom-start places below the anchor, aligned to its left edge", () => {
    const position = resolveOverlayPosition(anchor, overlay, "bottom-start", viewport, 6);

    expect(position.placement).toBe("bottom-start");
    expect(position.top).toBe(anchor.bottom + 6);
    expect(position.left).toBe(anchor.left);
  });

  test("top centers horizontally above the anchor", () => {
    const position = resolveOverlayPosition(anchor, overlay, "top", viewport, 6);

    expect(position.placement).toBe("top");
    expect(position.top).toBe(anchor.top - overlay.height - 6);
    expect(position.left).toBe(anchor.left + (anchor.width - overlay.width) / 2);
  });

  test("flips to the opposite side when the preferred side overflows", () => {
    const nearTop = rect(450, 10, 100, 40);
    const position = resolveOverlayPosition(nearTop, overlay, "top", viewport, 6);

    expect(position.placement.startsWith("bottom")).toBe(true);
    expect(position.top).toBe(nearTop.bottom + 6);
  });

  test("clamps into the viewport with 8px padding when nothing fits cleanly", () => {
    const tinyViewport = { width: 150, height: 120 };
    const bigOverlay = rect(0, 0, 300, 300);
    const position = resolveOverlayPosition(rect(10, 10, 20, 20), bigOverlay, "bottom", tinyViewport, 6);

    expect(position.left).toBe(8);
    expect(position.top).toBe(8);
  });

  test("prefers a candidate with zero overflow and zero anchor overlap", () => {
    const nearRight = rect(920, 380, 60, 40);
    const position = resolveOverlayPosition(nearRight, overlay, "right", viewport, 6);

    // right side would overflow; resolver must pick a clean alternative
    expect(position.left + overlay.width).toBeLessThanOrEqual(viewport.width - 8);
    expect(position.placement.startsWith("right")).toBe(false);
  });
});

import { describe, expect, test } from "bun:test";

import {
  SLIDER_FAMILY_BLOCK_HIT_PX,
  SLIDER_FAMILY_BLOCK_MARKER_INSET_PX,
  SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX,
  SLIDER_FAMILY_BLOCK_MARKER_THICKNESS_PX,
  SLIDER_FAMILY_TEXT_DOCK_GAP_PX,
  SLIDER_FAMILY_TEXT_END_INSET_PX,
  sliderFamilyBlockMarkerPercent,
  sliderFamilyCapsuleSpan,
  sliderFamilyValueDockedToMarker,
  sliderFamilyValueNorm,
} from "../src/slider-family.ts";

const rect = { left: 10, top: 20, width: 200, height: 40 };

describe("slider-family foundation", () => {
  test("the shared metrics are the accepted family constants", () => {
    expect(SLIDER_FAMILY_BLOCK_HIT_PX).toBe(44);
    expect(SLIDER_FAMILY_BLOCK_MARKER_INSET_PX).toBe(4);
    expect(SLIDER_FAMILY_BLOCK_MARKER_THICKNESS_PX).toBe(4);
    expect(SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX).toBe(6);
    expect(SLIDER_FAMILY_TEXT_DOCK_GAP_PX).toBe(4);
    expect(SLIDER_FAMILY_TEXT_END_INSET_PX).toBe(12);
  });

  test("capsule span measures the value axis, not the cross axis", () => {
    expect(sliderFamilyCapsuleSpan(rect, "horizontal")).toBe(200);
    expect(sliderFamilyCapsuleSpan(rect, "vertical")).toBe(40);
  });

  test("pointer mapping is one law for both families and both axes", () => {
    // Horizontal ltr: physical x maps straight through.
    expect(sliderFamilyValueNorm({ rect, orientation: "horizontal", direction: "ltr", clientX: 110, clientY: 40 })).toBeCloseTo(0.5, 6);
    // Horizontal rtl mirrors.
    expect(sliderFamilyValueNorm({ rect, orientation: "horizontal", direction: "rtl", clientX: 110, clientY: 40 })).toBeCloseTo(0.5, 6);
    expect(sliderFamilyValueNorm({ rect, orientation: "horizontal", direction: "rtl", clientX: 10, clientY: 40 })).toBeCloseTo(1, 6);
    // Vertical is bottom-referenced and never mirrors.
    expect(sliderFamilyValueNorm({ rect, orientation: "vertical", direction: "rtl", clientX: 0, clientY: 20 })).toBeCloseTo(1, 6);
    expect(sliderFamilyValueNorm({ rect, orientation: "vertical", direction: "ltr", clientX: 0, clientY: 60 })).toBeCloseTo(0, 6);
  });

  test("pointer mapping clamps outside the control", () => {
    expect(sliderFamilyValueNorm({ rect, orientation: "horizontal", direction: "ltr", clientX: -50, clientY: 40 })).toBe(0);
    expect(sliderFamilyValueNorm({ rect, orientation: "horizontal", direction: "ltr", clientX: 900, clientY: 40 })).toBe(1);
  });

  test("the docking law fires only inside the reserved end box", () => {
    // 12px end inset + 4px breathing room around a 30px advance.
    expect(sliderFamilyValueDockedToMarker({ valueNorm: 0.95, span: 200, advance: 30 })).toBe(true);
    expect(sliderFamilyValueDockedToMarker({ valueNorm: 0.5, span: 200, advance: 30 })).toBe(false);
    // Zero-width hosts never dock.
    expect(sliderFamilyValueDockedToMarker({ valueNorm: 1, span: 0, advance: 30 })).toBe(false);
  });

  test("the marker is clamped inside the capsule at both extrema", () => {
    const span = 200;
    const offsetPercent = (SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX / span) * 100;
    // Positive interior (fill extends toward larger values) insets forward.
    expect(sliderFamilyBlockMarkerPercent({ valuePercent: 50, spanPx: span, interior: 1 })).toBeCloseTo(50 + offsetPercent, 6);
    expect(sliderFamilyBlockMarkerPercent({ valuePercent: 0, spanPx: span, interior: 1 })).toBeCloseTo(offsetPercent, 6);
    expect(sliderFamilyBlockMarkerPercent({ valuePercent: 100, spanPx: span, interior: 1 })).toBeCloseTo(100 - offsetPercent, 6);
    // Negative interior (fill extends toward smaller values, e.g. the range
    // upper edge) insets backward, then clamps.
    expect(sliderFamilyBlockMarkerPercent({ valuePercent: 0, spanPx: span, interior: -1 })).toBeCloseTo(offsetPercent, 6);
  });

  test("the marker never leaves the capsule even for out-of-range input", () => {
    const span = 200;
    expect(sliderFamilyBlockMarkerPercent({ valuePercent: -40, spanPx: span, interior: 1 })).toBeGreaterThanOrEqual(0);
    expect(sliderFamilyBlockMarkerPercent({ valuePercent: 240, spanPx: span, interior: -1 })).toBeLessThanOrEqual(100);
  });
});

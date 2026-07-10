import { describe, expect, test } from "bun:test";

import { disclosureTransition } from "../src/disclosure";
import { buildVisiblePages, canRequestPage } from "../src/pagination";

describe("disclosureTransition", () => {
  test("toggle flips and emits; disabled inert; SET_OPEN silent", () => {
    const opened = disclosureTransition({ open: false, disabled: false }, { type: "TOGGLE" });
    expect(opened.context.open).toBe(true);
    expect(opened.effects).toEqual([{ type: "emitOpenChange", open: true }]);

    const closed = disclosureTransition(opened.context, { type: "TOGGLE" });
    expect(closed.context.open).toBe(false);
    expect(closed.effects).toEqual([{ type: "emitOpenChange", open: false }]);

    expect(disclosureTransition({ open: false, disabled: true }, { type: "TOGGLE" }).effects).toEqual([]);
    expect(disclosureTransition({ open: false, disabled: false }, { type: "SET_OPEN", open: true }).effects).toEqual([]);
  });
});

describe("buildVisiblePages", () => {
  test("windows around the current page with boundary pages and ellipses", () => {
    expect(buildVisiblePages(5, 10, 1)).toEqual([1, "ellipsis", 4, 5, 6, "ellipsis", 10]);
    expect(buildVisiblePages(1, 3, 1)).toEqual([1, 2, 3]);
    expect(buildVisiblePages(1, 1, 1)).toEqual([1]);
  });

  test("no ellipsis for gaps of exactly one page", () => {
    expect(buildVisiblePages(3, 5, 1)).toEqual([1, 2, 3, 4, 5]);
  });
});

describe("canRequestPage", () => {
  test("bounds, integrality, and no-op navigation", () => {
    expect(canRequestPage(2, 1, 5)).toBe(true);
    expect(canRequestPage(0, 1, 5)).toBe(false);
    expect(canRequestPage(6, 1, 5)).toBe(false);
    expect(canRequestPage(1, 1, 5)).toBe(false);
    expect(canRequestPage(2.5, 1, 5)).toBe(false);
  });
});

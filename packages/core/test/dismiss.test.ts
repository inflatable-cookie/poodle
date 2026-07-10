import { describe, expect, test } from "bun:test";

import { resolveDismiss, type DismissLayer } from "../src/dom/dismiss";

function layer(overrides: Partial<DismissLayer> = {}): DismissLayer {
  return {
    contains: () => false,
    onDismiss: () => {},
    dismissOnOutsideInteract: true,
    ...overrides,
  };
}

const node = {} as Node;

describe("resolveDismiss", () => {
  test("empty stack resolves nothing", () => {
    expect(resolveDismiss([], "escape", null)).toBeNull();
  });

  test("escape targets the innermost layer only", () => {
    const outer = layer();
    const inner = layer();

    expect(resolveDismiss([outer, inner], "escape", null)).toBe(inner);
  });

  test("outside interaction inside the top layer does not dismiss", () => {
    const top = layer({ contains: () => true });

    expect(resolveDismiss([top], "outside", node)).toBeNull();
  });

  test("outside interaction outside the top layer dismisses it", () => {
    const outer = layer();
    const inner = layer();

    expect(resolveDismiss([outer, inner], "outside", node)).toBe(inner);
  });

  test("dismissOnOutsideInteract=false blocks outside dismissal but not escape", () => {
    const pinned = layer({ dismissOnOutsideInteract: false });

    expect(resolveDismiss([pinned], "outside", node)).toBeNull();
    expect(resolveDismiss([pinned], "escape", null)).toBe(pinned);
  });
});

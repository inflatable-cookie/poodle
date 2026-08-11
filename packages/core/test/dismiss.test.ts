import { describe, expect, test } from "bun:test";

import { resolveDismiss, type DismissLayer } from "../src/dom/dismiss.ts";

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
    expect(resolveDismiss([], "escape", null)).toEqual([]);
  });

  test("escape targets the innermost layer only", () => {
    const outer = layer();
    const inner = layer();

    expect(resolveDismiss([outer, inner], "escape", null)).toEqual([inner]);
  });

  test("outside interaction inside the top layer does not dismiss it", () => {
    const top = layer({ contains: () => true });

    expect(resolveDismiss([top], "outside", node)).toEqual([]);
  });

  test("outside interaction outside the top layer dismisses it", () => {
    const inner = layer();

    expect(resolveDismiss([inner], "outside", node)).toEqual([inner]);
  });

  test("dismissOnOutsideInteract=false blocks outside dismissal but not escape", () => {
    const pinned = layer({ dismissOnOutsideInteract: false });

    expect(resolveDismiss([pinned], "outside", node)).toEqual([]);
    expect(resolveDismiss([pinned], "escape", null)).toEqual([pinned]);
  });

  // The NavigationMenu specimen page put several peer overlays on screen at
  // once. Dismissing only the innermost made them queue: each click closed a
  // different overlay than the one aimed at, cascading up the page.
  test("outside interaction dismisses every peer it fell outside of", () => {
    const first = layer();
    const second = layer();
    const third = layer();

    expect(resolveDismiss([first, second, third], "outside", node)).toEqual([
      third,
      second,
      first,
    ]);
  });

  test("outside dismissals are ordered innermost first", () => {
    const outer = layer();
    const inner = layer();

    expect(resolveDismiss([outer, inner], "outside", node)).toEqual([inner, outer]);
  });

  test("a layer containing the target is spared while its peers dismiss", () => {
    const peer = layer();
    const clicked = layer({ contains: () => true });
    const other = layer();

    expect(resolveDismiss([peer, clicked, other], "outside", node)).toEqual([other, peer]);
  });

  test("a pinned layer is spared while its peers dismiss", () => {
    const pinned = layer({ dismissOnOutsideInteract: false });
    const dismissable = layer();

    expect(resolveDismiss([pinned, dismissable], "outside", node)).toEqual([dismissable]);
  });

  test("escape still unwinds one layer at a time when overlays nest", () => {
    const dialog = layer();
    const menu = layer();

    // Esc closes the menu; the dialog stays until the next Esc.
    expect(resolveDismiss([dialog, menu], "escape", null)).toEqual([menu]);
    expect(resolveDismiss([dialog], "escape", null)).toEqual([dialog]);
  });
});

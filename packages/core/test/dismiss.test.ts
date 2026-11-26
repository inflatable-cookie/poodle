import { afterEach, describe, expect, test } from "bun:test";
import { Window } from "happy-dom";

import { registerDismissLayer, resolveDismiss, type DismissLayer } from "../src/dom/dismiss.ts";

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

  test("a child layer's portalled surface spares the child and its ancestor, but not the ancestor's peer", () => {
    const peer = layer();
    const popover = layer();
    // Select portals its listbox to body; only the listbox contains the target.
    const select = layer({ contains: () => true, parent: popover });

    expect(resolveDismiss([peer, popover, select], "outside", node)).toEqual([peer]);
  });

  test("ancestry survives portalling — the parent is the layer on top at registration, not a DOM ancestor", () => {
    const popover = layer();
    // The select's portalled surface is not a DOM descendant of the popover,
    // so DOM-based ancestry could not spare the popover; registration can.
    const select = layer({ contains: () => true });

    const unregisterPopover = registerDismissLayer(popover);
    const unregisterSelect = registerDismissLayer(select);

    expect(select.parent).toBe(popover);
    expect(resolveDismiss([popover, select], "outside", node)).toEqual([]);

    unregisterSelect();
    unregisterPopover();
  });

  test("three levels deep: clicking the innermost spares all three", () => {
    const outer = layer();
    const middle = layer({ parent: outer });
    const inner = layer({ contains: () => true, parent: middle });

    expect(resolveDismiss([outer, middle, inner], "outside", node)).toEqual([]);
  });

  test("a true outside click still dismisses the whole chain in one interaction", () => {
    const outer = layer();
    const middle = layer({ parent: outer });
    const inner = layer({ parent: middle });

    expect(resolveDismiss([outer, middle, inner], "outside", node)).toEqual([
      inner,
      middle,
      outer,
    ]);
  });
});

describe("registerDismissLayer keydown guards (happy-dom)", () => {
  const originalDocument = globalThis.document;
  let win: Window | null = null;
  let unregister: (() => void) | null = null;

  afterEach(() => {
    unregister?.();
    unregister = null;
    win?.close();
    win = null;
    if (originalDocument === undefined) {
      delete (globalThis as { document?: Document }).document;
    } else {
      globalThis.document = originalDocument;
    }
  });

  function setupDom(): Window {
    win = new Window();
    globalThis.document = win.document as unknown as Document;
    return win;
  }

  test("ignores Escape during IME composition (isComposing and keyCode 229)", () => {
    const w = setupDom();
    let dismissed = 0;
    unregister = registerDismissLayer(layer({ onDismiss: () => { dismissed += 1; } }));

    const composingEvent = new w.KeyboardEvent("keydown", {
      key: "Escape",
      isComposing: true,
      cancelable: true,
    });
    w.document.dispatchEvent(composingEvent);
    expect(dismissed).toBe(0);
    expect(composingEvent.defaultPrevented).toBe(false);

    const legacyImeEvent = new w.KeyboardEvent("keydown", {
      key: "Escape",
      keyCode: 229,
      cancelable: true,
    });
    w.document.dispatchEvent(legacyImeEvent);
    expect(dismissed).toBe(0);
    expect(legacyImeEvent.defaultPrevented).toBe(false);
  });

  test("ignores Escape that is already defaultPrevented", () => {
    const w = setupDom();
    let dismissed = 0;
    unregister = registerDismissLayer(layer({ onDismiss: () => { dismissed += 1; } }));

    const preventedEvent = new w.KeyboardEvent("keydown", {
      key: "Escape",
      cancelable: true,
    });
    preventedEvent.preventDefault();
    w.document.dispatchEvent(preventedEvent);
    expect(dismissed).toBe(0);
  });

  test("dismisses the top layer on a normal Escape", () => {
    const w = setupDom();
    const calls: string[] = [];
    unregister = registerDismissLayer(layer({ onDismiss: (reason) => { calls.push(reason); } }));

    const event = new w.KeyboardEvent("keydown", {
      key: "Escape",
      cancelable: true,
    });
    w.document.dispatchEvent(event);
    expect(calls).toEqual(["escape"]);
    expect(event.defaultPrevented).toBe(true);
  });
});

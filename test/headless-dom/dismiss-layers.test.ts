/**
 * Dismissable-layer stack (g14.005): registered-layer parenthood and order
 * derive from real layer containment, not registration order. A nested
 * overlay registers inside its host's layer no matter which framework effect
 * ran first or where the portalled surfaces were attached.
 */

import { describe, expect, it } from "vitest";

import { registerDismissLayer, resolveDismiss, type DismissLayer } from "@inflatable-cookie/poodle-core/dom/dismiss";

interface LayerHost {
  root: HTMLElement;
  surface: HTMLElement;
  layer: DismissLayer;
}

/** A popover-like layer: containment covers the root AND the portalled
 * surface (which lives elsewhere in the document). */
function popoverLayer(label: string, mountIn: HTMLElement): LayerHost {
  const root = document.createElement("div");
  root.textContent = label;
  const surface = document.createElement("div");
  surface.textContent = `${label} surface`;
  root.appendChild(surface);
  mountIn.appendChild(root);
  // The surface portals out of the root to the body — siblings with every
  // other portalled surface, never nested by DOM position.
  document.body.appendChild(surface);
  const layer: DismissLayer = {
    contains: (target) =>
      root.contains(target as Node) || surface.contains(target as Node),
    onDismiss: () => {},
    dismissOnOutsideInteract: true,
    hostElement: root,
  };
  return { root, surface, layer };
}

function cleanup(...hosts: LayerHost[]): void {
  for (const host of hosts) {
    host.root.remove();
    host.surface.remove();
  }
}

describe("registerDismissLayer ancestry", () => {
  it("registered peers get no parent and dismiss independently", () => {
    const stage = document.createElement("div");
    document.body.appendChild(stage);
    const a = popoverLayer("A", stage);
    const b = popoverLayer("B", stage);

    const unregisterA = registerDismissLayer(a.layer);
    const unregisterB = registerDismissLayer(b.layer);

    // The second peer must NOT inherit the first as its parent: a press
    // inside A dismisses only B, never A.
    expect(b.layer.parent).toBeNull();
    expect(resolveDismiss([a.layer, b.layer], "outside", a.surface)).toEqual([b.layer]);
    expect(resolveDismiss([a.layer, b.layer], "escape", null)).toEqual([b.layer]);

    unregisterB();
    unregisterA();
    cleanup(a, b);
    stage.remove();
  });

  it("a layer opened around an earlier one inserts below it (innermost first)", () => {
    const stage = document.createElement("div");
    document.body.appendChild(stage);
    // The inner popover's root lives INSIDE the outer's surface; its own
    // surface portals to body as a sibling of the outer's.
    const outer = popoverLayer("Outer", stage);
    const inner = popoverLayer("Inner", outer.surface);

    // Registration order is the framework's: the inner (child) effect runs
    // first, the outer second.
    const unregisterInner = registerDismissLayer(inner.layer);
    const unregisterOuter = registerDismissLayer(outer.layer);

    // The outer wrapped the inner: it sits below it and the inner's parent
    // points back at it — the innermost layer is the one that closes first.
    expect(outer.layer.parent).toBeNull();
    expect(inner.layer.parent).toBe(outer.layer);
    expect(resolveDismiss([outer.layer, inner.layer], "escape", null)).toEqual([inner.layer]);

    unregisterOuter();
    unregisterInner();
    cleanup(outer, inner);
    stage.remove();
  });

  it("reversed portal order does not change ancestry — containment is real, not DOM position", () => {
    const stage = document.createElement("div");
    document.body.appendChild(stage);
    const outer = popoverLayer("Outer", stage);
    // The inner's surface attaches to body BEFORE the outer's surface does
    // (both end up as body siblings); only the roots keep the nesting.
    document.body.appendChild(outer.surface);
    document.body.removeChild(outer.surface);
    document.body.appendChild(outer.surface);
    const inner = popoverLayer("Inner", outer.surface);

    const unregisterInner = registerDismissLayer(inner.layer);
    const unregisterOuter = registerDismissLayer(outer.layer);

    expect(inner.layer.parent).toBe(outer.layer);
    expect(resolveDismiss([outer.layer, inner.layer], "escape", null)).toEqual([inner.layer]);
    // A press inside the outer's surface but outside the inner's spares the
    // outer (its layer contains the target) and dismisses the inner.
    const outsideInner = document.createElement("div");
    outer.surface.appendChild(outsideInner);
    expect(resolveDismiss([outer.layer, inner.layer], "outside", outsideInner)).toEqual([
      inner.layer,
    ]);

    unregisterOuter();
    unregisterInner();
    cleanup(outer, inner);
    stage.remove();
  });
});

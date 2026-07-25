/**
 * Dismissable-layer stack.
 *
 * Overlays register while open; document-level Escape and outside-pointerdown
 * dismiss the innermost (most recently registered) layer only, so nested
 * overlays close one at a time instead of all at once. Listeners are attached
 * while at least one layer is registered and removed when the stack empties.
 *
 * The stack logic is pure and unit-tested via `resolveDismiss`; the document
 * wiring below is the thin DOM binding.
 */

export interface DismissLayer {
  /** True when `target` is inside the layer (trigger + surface). */
  contains: (target: Node) => boolean;
  /** Dismiss request; the layer decides what closing means. */
  onDismiss: (reason: "escape" | "outside") => void;
  /** Guard for outside-interaction dismissal (escape always dismisses). */
  dismissOnOutsideInteract: boolean;
}

/**
 * Which layer, if any, should dismiss for an interaction. Pure so the
 * innermost-first policy is testable without a DOM.
 */
export function resolveDismiss(
  layers: readonly DismissLayer[],
  reason: "escape" | "outside",
  target: Node | null,
): DismissLayer | null {
  const top = layers[layers.length - 1];

  if (!top) {
    return null;
  }

  if (reason === "escape") {
    return top;
  }

  if (!top.dismissOnOutsideInteract) {
    return null;
  }

  if (target && top.contains(target)) {
    return null;
  }

  return top;
}

/**
 * `contains` for a layer whose parts are not one subtree. A portalled surface
 * is not a descendant of its trigger, so an outside-interaction check has to
 * ask both — otherwise the first click inside the surface dismisses the layer.
 */
export function layerContains(
  target: Node,
  ...elements: Array<Element | null | undefined>
): boolean {
  return elements.some((element) => element?.contains(target) ?? false);
}

const stack: DismissLayer[] = [];
let listenersAttached = false;

function handlePointerDown(event: MouseEvent): void {
  resolveDismiss(stack, "outside", event.target as Node | null)?.onDismiss("outside");
}

function handleKeydown(event: KeyboardEvent): void {
  if (event.key !== "Escape") {
    return;
  }

  const layer = resolveDismiss(stack, "escape", null);

  if (layer) {
    event.preventDefault();
    layer.onDismiss("escape");
  }
}

function syncListeners(): void {
  if (typeof document === "undefined") {
    return;
  }

  if (stack.length > 0 && !listenersAttached) {
    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);
    listenersAttached = true;
  } else if (stack.length === 0 && listenersAttached) {
    document.removeEventListener("mousedown", handlePointerDown);
    document.removeEventListener("keydown", handleKeydown);
    listenersAttached = false;
  }
}

/** Register an open overlay. Returns an unregister function; call it on close and on unmount. */
export function registerDismissLayer(layer: DismissLayer): () => void {
  stack.push(layer);
  syncListeners();

  return () => {
    const index = stack.indexOf(layer);

    if (index >= 0) {
      stack.splice(index, 1);
    }

    syncListeners();
  };
}

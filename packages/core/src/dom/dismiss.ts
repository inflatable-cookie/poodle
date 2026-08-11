/**
 * Dismissable-layer stack.
 *
 * Overlays register while open; document-level Escape and outside-pointerdown
 * dismiss from this stack. Listeners are attached while at least one layer is
 * registered and removed when the stack empties.
 *
 * The two reasons deliberately differ:
 *
 * - **Escape** dismisses the innermost layer only, so nested overlays unwind
 *   one keypress at a time — Esc closes the menu, Esc again closes the dialog
 *   it sits in.
 * - **Outside interaction** dismisses *every* layer the interaction fell
 *   outside of. Peer overlays are indistinguishable from nested ones here (a
 *   layer knows whether it contains the target, not whether another layer
 *   contains it), and dismissing only the innermost made sibling overlays
 *   queue: with N open peers, N clicks were needed to get past them, each one
 *   closing a different overlay than the one the user aimed at. Closing
 *   everything the click was outside of is both what a user expects and the
 *   only behaviour that does not depend on registration order.
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
 * Which layers should dismiss for an interaction, innermost first. Pure so the
 * policy is testable without a DOM.
 *
 * Escape yields at most the innermost layer. An outside interaction yields
 * every layer that opted into outside dismissal and does not contain the
 * target.
 */
export function resolveDismiss(
  layers: readonly DismissLayer[],
  reason: "escape" | "outside",
  target: Node | null,
): DismissLayer[] {
  const top = layers[layers.length - 1];

  if (!top) {
    return [];
  }

  if (reason === "escape") {
    return [top];
  }

  // Innermost first, so a layer that closes something beneath it has already
  // run by the time the outer layer is dismissed.
  return layers
    .slice()
    .reverse()
    .filter(
      (layer) =>
        layer.dismissOnOutsideInteract && !(target !== null && layer.contains(target)),
    );
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
  // Snapshot before dispatching: each onDismiss unregisters its own layer and
  // mutates `stack` while we are iterating it.
  for (const layer of resolveDismiss(stack, "outside", event.target as Node | null)) {
    layer.onDismiss("outside");
  }
}

function handleKeydown(event: KeyboardEvent): void {
  if (event.key !== "Escape") {
    return;
  }

  const [layer] = resolveDismiss(stack, "escape", null);

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

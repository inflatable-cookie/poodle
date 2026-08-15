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
 *   outside of, except a layer that contains the target and any ancestor of
 *   such a layer. The stack is not flat: a layer's parent is the innermost
 *   layer already registered whose containment covers this layer's host — the
 *   layer this one opened inside (host-aware layers; see
 *   `registerDismissLayer`). A click inside a nested layer therefore spares
 *   the whole chain back to the host, while true peers — layers with no
 *   parent link to the hit layer — still all dismiss. Closing everything the
 *   click was outside of is what the peer-dismissal change was for; ancestry
 *   just stops a nested layer from reading as a peer of its own host.
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
  /**
   * The layer this one opened inside: the innermost layer already registered
   * whose `contains` covers this layer's `hostElement` (host-aware layers),
   * or the registration top for layers without a host element. Recorded by
   * `registerDismissLayer`; optional because callers construct layers before
   * registration and the pure tests exercise `resolveDismiss` directly.
   */
  parent?: DismissLayer | null;
  /**
   * The layer's component root, when it has one. Parenthood and stack order
   * derive from real layer containment, not registration order: the layer's
   * `contains` covers both the root and any portalled surface, so a nested
   * overlay's host is contained by its host's layer whatever the framework
   * effect order and wherever the surfaces were portalled.
   */
  hostElement?: Element | null;
}

/**
 * Which layers should dismiss for an interaction, innermost first. Pure so the
 * policy is testable without a DOM.
 *
 * Escape yields at most the innermost layer. An outside interaction yields
 * every layer that opted into outside dismissal and neither contains the
 * target nor is an ancestor of a layer that does.
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

  // Layers spared by containment: every layer that contains the target plus
  // every ancestor of such a layer, walking the parent chain recorded at
  // registration. Peers — layers with no parent link to the hit layer — are
  // not in the set and still dismiss, so one click closes every unrelated
  // overlay.
  const spared = target === null ? null : sparedByAncestry(layers, target);

  // Innermost first, so a layer that closes something beneath it has already
  // run by the time the outer layer is dismissed.
  return layers
    .slice()
    .reverse()
    .filter(
      (layer) => layer.dismissOnOutsideInteract && !(spared !== null && spared.has(layer)),
    );
}

/**
 * The set of layers an outside interaction must spare: every layer that
 * contains the target, plus every ancestor of those layers. Ancestry follows
 * the `parent` chain derived from layer containment — a portalled surface is
 * not a DOM descendant of its host, so the relationship lives in the layers'
 * containment, not the DOM tree.
 */
function sparedByAncestry(
  layers: readonly DismissLayer[],
  target: Node,
): Set<DismissLayer> {
  const spared = new Set<DismissLayer>();

  for (const layer of layers) {
    if (!layer.contains(target)) {
      continue;
    }

    for (
      let current: DismissLayer | null | undefined = layer;
      current && !spared.has(current);
      current = current.parent
    ) {
      spared.add(current);
    }
  }

  return spared;
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

/**
 * Register an open overlay. Returns an unregister function; call it on close
 * and on unmount.
 *
 * Parenthood is derived from real layer containment, not registration order:
 * a layer's parent is the innermost layer already registered whose
 * `contains` covers the new layer's `hostElement` — the layer this one
 * opened inside. Peers (no layer contains the host) get no parent; layers
 * without a host element keep the registration-top default, since only the
 * host-aware popover-like layers can be located by containment.
 *
 * When the new layer's own containment covers an already-registered layer's
 * host, this layer opened AROUND it, so it inserts BELOW the contained layer
 * (innermost closes first) whatever the framework's effect order.
 */
export function registerDismissLayer(layer: DismissLayer): () => void {
  const host = layer.hostElement ?? null;
  let parent: DismissLayer | null = null;
  if (host) {
    for (let index = stack.length - 1; index >= 0; index -= 1) {
      if (stack[index].contains(host as Node)) {
        parent = stack[index];
        break;
      }
    }
  } else {
    parent = stack[stack.length - 1] ?? null;
  }
  layer.parent = parent;

  const insertAt = host
    ? stack.findIndex(
        (existing) =>
          existing.hostElement &&
          existing.hostElement !== host &&
          layer.contains(existing.hostElement as Node),
      )
    : -1;
  if (insertAt >= 0) {
    stack.splice(insertAt, 0, layer);
    for (let index = insertAt + 1; index < stack.length; index += 1) {
      const existing = stack[index];
      if (existing.hostElement && layer.contains(existing.hostElement as Node)) {
        existing.parent = layer;
      }
    }
  } else {
    stack.push(layer);
  }
  syncListeners();

  return () => {
    const index = stack.indexOf(layer);

    if (index >= 0) {
      stack.splice(index, 1);
    }

    syncListeners();
  };
}

/** The number of open dismissable layers — the web side's layer order /
 * overlay-state observation channel (component-observation.v1). */
export function dismissLayerStackLength(): number {
  return stack.length;
}

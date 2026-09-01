import { getContext, setContext } from "svelte";
import type { Action } from "svelte/action";
import { readable, type Readable } from "svelte/store";

import type {
  DragDropController,
  DragDropSnapshot,
  DragSourceRegistration,
  DropTargetRegistration,
  KeyboardDropCommand,
  KeyboardDropTargetHandle,
  KeyboardDropTargetRegistration,
} from "@inflatable-cookie/poodle-core";

const POODLE_DRAG_DROP = Symbol("poodle-drag-drop");

export interface DragDropContextValue {
  controller: DragDropController;
}

export function setDragDrop(value: DragDropContextValue): void {
  setContext(POODLE_DRAG_DROP, value);
}

/**
 * The registration actions for one controller.
 *
 * Split out of `useDragDrop` so a component that owns its controller — a
 * DockRegion with no ambient provider — registers through exactly the same
 * code path as one that joined somebody else's.
 *
 * A `null` registration registers nothing at all, and un-registers what was
 * there. That is the same rule the renderer-neutral `attach_source` /
 * `attach_target` follow, and for the same reason: a registered-and-disabled
 * source is still reachable by keyboard traversal and still nameable in an
 * announcement, so a component that cannot drag must not appear to.
 */
export function dragSourceAction(
  controller: DragDropController,
): Action<HTMLElement, DragSourceRegistration | null> {
  return (node, registration) => {
    let current = registration ?? null;
    let handle = current ? controller.registerSource(node, current) : null;
    return {
      update(next) {
        const value = next ?? null;
        if (value === null) {
          handle?.unregister();
          handle = null;
        } else if (handle === null || current === null || value.sourceId !== current.sourceId) {
          handle?.unregister();
          handle = controller.registerSource(node, value);
        } else {
          handle.update(value);
        }
        current = value;
      },
      destroy() {
        handle?.unregister();
      },
    };
  };
}

export function dropTargetAction(
  controller: DragDropController,
): Action<HTMLElement, DropTargetRegistration | null> {
  return (node, registration) => {
    let current = registration ?? null;
    let handle = current ? controller.registerTarget(node, current) : null;
    return {
      update(next) {
        const value = next ?? null;
        if (value === null) {
          handle?.unregister();
          handle = null;
        } else if (handle === null || current === null || value.targetId !== current.targetId) {
          handle?.unregister();
          handle = controller.registerTarget(node, value);
        } else {
          handle.update(value);
        }
        current = value;
      },
      destroy() {
        handle?.unregister();
      },
    };
  };
}

function presentationKey(snapshot: DragDropSnapshot): string {
  return [
    snapshot.phase,
    snapshot.sourceId ?? "",
    snapshot.targetId ?? "",
    snapshot.targetPosture ?? "",
    snapshot.announcement ?? "",
    snapshot.preview?.label ?? "",
    snapshot.preview ? "1" : "0",
  ].join("|");
}

/** Presentation-only read: skips pointer-only moves so drop-target trees stay quiet. */
export function dragDropSnapshotStore(controller: DragDropController): Readable<DragDropSnapshot> {
  return readable(controller.getSnapshot(), (set) => {
    let last = presentationKey(controller.getSnapshot());
    return controller.subscribe(() => {
      const next = controller.getSnapshot();
      const key = presentationKey(next);
      if (key === last) return;
      last = key;
      set(next);
    });
  });
}

/** Public snapshot, including consecutive pointer/preview coordinates. */
export function dragDropLiveSnapshotStore(controller: DragDropController): Readable<DragDropSnapshot> {
  return readable(controller.getSnapshot(), (set) => controller.subscribe(() => set(controller.getSnapshot())));
}

/**
 * The nearest drag-drop context, or `undefined`.
 *
 * Internal to the component package and deliberately not re-exported: a
 * component that *joins* an ambient provider when one exists needs to ask
 * without throwing, but a consumer reaching for the controller should still
 * get the loud `useDragDrop` error rather than a silent null.
 */
export function tryDragDrop(): DragDropContextValue | undefined {
  return getContext<DragDropContextValue | undefined>(POODLE_DRAG_DROP);
}

export function useDragDrop(): {
  snapshot: Readable<DragDropSnapshot>;
  cancel: () => void;
  requestKeyboardDrop: (command: KeyboardDropCommand) => boolean;
  dragSource: Action<HTMLElement, DragSourceRegistration | null>;
  dropTarget: Action<HTMLElement, DropTargetRegistration | null>;
  keyboardDropTarget: (registration: KeyboardDropTargetRegistration) => KeyboardDropTargetHandle;
} {
  const ctx = getContext<DragDropContextValue | undefined>(POODLE_DRAG_DROP);
  if (!ctx) {
    throw new Error("useDragDrop must be used inside DragDropProvider");
  }

  const snapshot = dragDropLiveSnapshotStore(ctx.controller);

  const dragSource = dragSourceAction(ctx.controller);
  const dropTarget = dropTargetAction(ctx.controller);

  const keyboardDropTarget = (registration: KeyboardDropTargetRegistration): KeyboardDropTargetHandle =>
    ctx.controller.registerKeyboardTarget(registration);

  return {
    snapshot,
    cancel: () => ctx.controller.cancel(),
    requestKeyboardDrop: (command) => ctx.controller.requestKeyboardDrop(command),
    dragSource,
    dropTarget,
    keyboardDropTarget,
  };
}

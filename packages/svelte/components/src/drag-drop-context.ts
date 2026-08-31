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
 */
export function dragSourceAction(
  controller: DragDropController,
): Action<HTMLElement, DragSourceRegistration> {
  return (node, registration) => {
    let current = registration;
    let handle = controller.registerSource(node, current);
    return {
      update(next) {
        if (next.sourceId !== current.sourceId) {
          handle.unregister();
          handle = controller.registerSource(node, next);
        } else {
          handle.update(next);
        }
        current = next;
      },
      destroy() {
        handle.unregister();
      },
    };
  };
}

export function dropTargetAction(
  controller: DragDropController,
): Action<HTMLElement, DropTargetRegistration> {
  return (node, registration) => {
    let current = registration;
    let handle = controller.registerTarget(node, current);
    return {
      update(next) {
        if (next.targetId !== current.targetId) {
          handle.unregister();
          handle = controller.registerTarget(node, next);
        } else {
          handle.update(next);
        }
        current = next;
      },
      destroy() {
        handle.unregister();
      },
    };
  };
}

/** An immutable presentation read of one controller, as a store. */
export function dragDropSnapshotStore(controller: DragDropController): Readable<DragDropSnapshot> {
  return readable(controller.getSnapshot(), (set) =>
    controller.subscribe(() => set(controller.getSnapshot())),
  );
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
  dragSource: Action<HTMLElement, DragSourceRegistration>;
  dropTarget: Action<HTMLElement, DropTargetRegistration>;
  keyboardDropTarget: (registration: KeyboardDropTargetRegistration) => KeyboardDropTargetHandle;
} {
  const ctx = getContext<DragDropContextValue | undefined>(POODLE_DRAG_DROP);
  if (!ctx) {
    throw new Error("useDragDrop must be used inside DragDropProvider");
  }

  const snapshot = dragDropSnapshotStore(ctx.controller);

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

import { getContext, setContext } from "svelte";
import type { Action } from "svelte/action";
import { readable, type Readable } from "svelte/store";

import type {
  DragDropController,
  DragDropSnapshot,
  DragSourceRegistration,
  DropTargetRegistration,
} from "@inflatable-cookie/poodle-core";

const POODLE_DRAG_DROP = Symbol("poodle-drag-drop");

export interface DragDropContextValue {
  controller: DragDropController;
}

export function setDragDrop(value: DragDropContextValue): void {
  setContext(POODLE_DRAG_DROP, value);
}

export function useDragDrop(): {
  snapshot: Readable<DragDropSnapshot>;
  cancel: () => void;
  dragSource: Action<HTMLElement, DragSourceRegistration>;
  dropTarget: Action<HTMLElement, DropTargetRegistration>;
} {
  const ctx = getContext<DragDropContextValue | undefined>(POODLE_DRAG_DROP);
  if (!ctx) {
    throw new Error("useDragDrop must be used inside DragDropProvider");
  }

  const snapshot = readable(ctx.controller.getSnapshot(), (set) =>
    ctx.controller.subscribe(() => set(ctx.controller.getSnapshot())),
  );

  const dragSource: Action<HTMLElement, DragSourceRegistration> = (node, registration) => {
    let current = registration;
    let handle = ctx.controller.registerSource(node, current);
    return {
      update(next) {
        if (next.sourceId !== current.sourceId) {
          handle.unregister();
          handle = ctx.controller.registerSource(node, next);
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

  const dropTarget: Action<HTMLElement, DropTargetRegistration> = (node, registration) => {
    let current = registration;
    let handle = ctx.controller.registerTarget(node, current);
    return {
      update(next) {
        if (next.targetId !== current.targetId) {
          handle.unregister();
          handle = ctx.controller.registerTarget(node, next);
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

  return {
    snapshot,
    cancel: () => ctx.controller.cancel(),
    dragSource,
    dropTarget,
  };
}

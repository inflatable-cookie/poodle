import type { ReactNode } from "react";
import type {
  DragDropCommitResult,
  DropIntent,
  DropPosition,
} from "@inflatable-cookie/poodle-core";

import { useDragSource, useDropTarget } from "../drag-drop";
import type { EditorBlock } from "../types";

/**
 * One block shell.
 *
 * It exists as its own component because the drag substrate's registration
 * hooks cannot run inside a list loop; the Svelte pair registers through the
 * controller's actions and needs no equivalent part.
 */
export interface BlockEditorBlockProps {
  block: EditorBlock;
  index: number;
  active: boolean;
  /** Whether this editor can reorder at all. `false` registers nothing. */
  canDrag: boolean;
  subjectKind: string;
  sourceId: string;
  targetId: string;
  /** Index of a subject id in the live block list, or `-1` when not ours. */
  indexOfBlock: (id: string) => number;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
  onActivate: () => void;
  children: ReactNode;
}

export function BlockEditorBlock({
  block,
  index,
  active,
  canDrag,
  subjectKind,
  sourceId,
  targetId,
  indexOfBlock,
  onDrop,
  onActivate,
  children,
}: BlockEditorBlockProps) {
  const label = `${block.type} block`;

  const { getSourceProps, dragging } = useDragSource({
    sourceId,
    subject: { kind: subjectKind, id: block.id },
    allowedOperations: ["move"],
    label,
    disabled: !canDrag,
    // The grip is the handle; the block body stays an ordinary editing
    // surface, so a press in the textarea or a toolbar control never starts a
    // drag.
    handle: ".poodle-block-editor__drag-grip",
  });

  const { getTargetProps, accepted } = useDropTarget({
    targetId,
    acceptedKinds: [subjectKind],
    disabled: !canDrag,
    label,
    // One band per block: a block travelling down lands after its target and
    // one travelling up lands before it, so the dropped block ends up *at* the
    // block it was dropped on.
    resolvePosition: ({ subject }): DropPosition =>
      indexOfBlock(subject.id) < index ? "after" : "before",
    canDrop: (intent, subject) => {
      if (indexOfBlock(subject.id) < 0) {
        return { accepted: false, reason: "not this editor" };
      }
      return subject.id === block.id
        ? { accepted: false, reason: "same block" }
        : { accepted: true, intent };
    },
    onDrop,
  });

  // Registering nothing is not the same as registering and disabling: a
  // registered source is still keyboard-reachable and still nameable in an
  // announcement. The hooks still run — their order is fixed — but no element
  // reaches them, so no registration exists.
  const sourceProps = canDrag ? getSourceProps() : {};
  const targetProps = canDrag ? getTargetProps() : {};

  return (
    <div
      {...targetProps}
      {...sourceProps}
      ref={
        canDrag
          ? (node) => {
              sourceProps.ref?.(node);
              targetProps.ref?.(node);
            }
          : undefined
      }
      className={[
        "poodle-block-editor__block",
        active ? "poodle-active" : "",
        accepted ? "poodle-drag-over" : "",
        dragging ? "poodle-dragging" : "",
      ]
        .filter(Boolean)
        .join(" ")}
      data-type={block.type}
      onFocus={onActivate}
      role="group"
      aria-label={label}
    >
      {children}
    </div>
  );
}

import type { KeyboardEvent, ReactNode } from "react";
import type {
  DragDropCommitResult,
  DropIntent,
  DropPosition,
  ModelCatalogueItem,
} from "@inflatable-cookie/poodle-core";

import { Icon } from "../Icon";
import { useDragSource, useDropTarget } from "../drag-drop";

/**
 * One shown model row and its reorder handle.
 *
 * It exists as its own component because the drag substrate's registration
 * hooks cannot run inside a list loop; the Svelte pair registers through the
 * controller's actions and needs no equivalent part.
 */
export interface ModelCatalogueRowProps {
  item: ModelCatalogueItem;
  index: number;
  total: number;
  grabbed: boolean;
  locked: boolean;
  isDragEnabled: boolean;
  subjectKind: string;
  sourceId: string;
  targetId: string;
  /** Index of a subject id in the live shown list, or `-1` when not ours. */
  indexOfShown: (id: string) => number;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
  onHandleKeyDown: (event: KeyboardEvent<HTMLButtonElement>, index: number) => void;
  onToggleGrab: () => void;
  children: ReactNode;
}

export function ModelCatalogueRow({
  item,
  index,
  total,
  grabbed,
  locked,
  isDragEnabled,
  subjectKind,
  sourceId,
  targetId,
  indexOfShown,
  onDrop,
  onHandleKeyDown,
  onToggleGrab,
  children,
}: ModelCatalogueRowProps) {
  const { getSourceProps } = useDragSource({
    sourceId,
    subject: { kind: subjectKind, id: item.id },
    allowedOperations: ["move"],
    label: item.label,
    disabled: !isDragEnabled || locked || item.isDisabled,
  });

  const { getTargetProps, accepted } = useDropTarget({
    targetId,
    acceptedKinds: [subjectKind],
    disabled: !isDragEnabled || locked,
    label: item.label,
    // One band per row: a model travelling down lands after its target and one
    // travelling up lands before it, so the dropped model ends up *at* the row
    // it was dropped on — the same result the native renderer emits.
    resolvePosition: ({ subject }): DropPosition =>
      indexOfShown(subject.id) < index ? "after" : "before",
    canDrop: (intent, subject) => {
      if (indexOfShown(subject.id) < 0) {
        return { accepted: false, reason: "not this catalogue" };
      }
      return subject.id === item.id
        ? { accepted: false, reason: "same model" }
        : { accepted: true, intent };
    },
    onDrop,
  });

  return (
    <li
      {...getTargetProps()}
      className="poodle-model-catalogue-editor__row"
      data-model-catalogue-id={item.id}
      data-grabbed={grabbed ? "true" : "false"}
      data-drop-target={accepted ? "true" : "false"}
    >
      <button
        {...getSourceProps({
          onKeyDown: (event) => onHandleKeyDown(event as KeyboardEvent<HTMLButtonElement>, index),
        })}
        type="button"
        className="poodle-icon-button"
        data-variant="ghost"
        data-size-role="chrome"
        data-reorder-handle=""
        aria-pressed={grabbed}
        aria-label={`${item.label}, position ${index + 1} of ${total}`}
        disabled={locked || item.isDisabled}
        onClick={onToggleGrab}
      >
        <span className="poodle-icon-button__glyph" aria-hidden="true">
          <Icon name="grip-vertical" />
        </span>
      </button>
      {children}
    </li>
  );
}

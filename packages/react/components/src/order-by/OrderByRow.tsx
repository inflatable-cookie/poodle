import type {
  DragDropCommitResult,
  DropIntent,
  DropPosition,
} from "@inflatable-cookie/poodle-core";

import { IconButton } from "../IconButton";
import { useDragSource, useDropTarget } from "../drag-drop";
import type { OrderByField } from "../types";

/**
 * One sort clause.
 *
 * It exists as its own component because the drag substrate's registration
 * hooks cannot run inside a list loop; the Svelte pair registers through the
 * controller's actions and needs no equivalent part.
 */
export interface OrderByRowProps {
  item: OrderByField;
  index: number;
  label: string;
  disabled: boolean;
  subjectKind: string;
  sourceId: string;
  targetId: string;
  /** Index of a subject id in the live ordering, or `-1` when it is not ours. */
  indexOfKey: (key: string) => number;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
  onMove: (index: number, offset: -1 | 1) => void;
  onToggleDirection: (index: number) => void;
  onRemove: (index: number) => void;
  total: number;
}

export function OrderByRow({
  item,
  index,
  label,
  disabled,
  subjectKind,
  sourceId,
  targetId,
  indexOfKey,
  onDrop,
  onMove,
  onToggleDirection,
  onRemove,
  total,
}: OrderByRowProps) {
  const { getSourceProps, dragging } = useDragSource({
    sourceId,
    subject: { kind: subjectKind, id: item.key },
    allowedOperations: ["move"],
    label,
    disabled,
  });

  const { getTargetProps, accepted } = useDropTarget({
    targetId,
    acceptedKinds: [subjectKind],
    disabled,
    label,
    // The whole row is one band: a field travelling down lands after its
    // target and one travelling up lands before it, so the dropped field ends
    // up *at* the row it was dropped on.
    resolvePosition: ({ subject }): DropPosition =>
      indexOfKey(subject.id) < index ? "after" : "before",
    canDrop: (intent, subject) => {
      if (indexOfKey(subject.id) < 0) {
        return { accepted: false, reason: "not this sort builder" };
      }
      return subject.id === item.key
        ? { accepted: false, reason: "same field" }
        : { accepted: true, intent };
    },
    onDrop,
  });

  return (
    <div
      {...getTargetProps()}
      className={[
        "poodle-order-by__item",
        dragging ? "poodle-order-by__item--dragging" : "",
        accepted ? "poodle-order-by__item--drop-target" : "",
      ]
        .filter(Boolean)
        .join(" ")}
      role="listitem"
    >
      <button
        {...getSourceProps({
          onKeyDown: (event) => {
            if (event.altKey && event.key === "ArrowUp" && index > 0) {
              event.preventDefault();
              onMove(index, -1);
            }
            if (event.altKey && event.key === "ArrowDown" && index < total - 1) {
              event.preventDefault();
              onMove(index, 1);
            }
          },
        })}
        type="button"
        className="poodle-order-by__drag-handle"
        disabled={disabled}
        aria-label={`Reorder ${label}. Drag or use Alt plus arrow keys.`}
      >
        ⠿
      </button>
      <span className="poodle-order-by__item-label">{label}</span>
      <IconButton
        icon={item.direction === "asc" ? "arrow-up" : "arrow-down"}
        ariaLabel={`${label}: ${item.direction === "asc" ? "ascending" : "descending"}. Click to toggle.`}
        tooltip={item.direction === "asc" ? "Asc" : "Desc"}
        size="xs"
        variant="ghost"
        disabled={disabled}
        onClick={() => onToggleDirection(index)}
      />
      <IconButton
        icon="x"
        ariaLabel={`Remove ${label}`}
        tooltip="Remove"
        size="xs"
        variant="ghost"
        disabled={disabled}
        onClick={() => onRemove(index)}
      />
    </div>
  );
}

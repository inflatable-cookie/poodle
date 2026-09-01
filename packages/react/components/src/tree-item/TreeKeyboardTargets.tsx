import {
  isTreeBranch,
  type DragDropCommitResult,
  type DragSubject,
  type DropEligibility,
  type DropIntent,
} from "@inflatable-cookie/poodle-core";

import { useKeyboardDropTarget } from "../drag-drop";
import type { TreeNode } from "../types";

interface Row {
  node: TreeNode;
  depth: number;
  parent: string | null;
}

function TreeKeyboardTarget({
  row,
  index,
  disabled,
  canDrop,
  onDrop,
}: {
  row: Row;
  index: number;
  disabled: boolean;
  canDrop: (intent: DropIntent, subject: DragSubject) => boolean | DropEligibility;
  onDrop: (intent: DropIntent) => DragDropCommitResult | Promise<DragDropCommitResult>;
}) {
  useKeyboardDropTarget({
    targetId: row.node.value,
    acceptedKinds: ["poodle.tree"],
    disabled,
    label: row.node.label,
    order: index,
    resolvePosition: (input) => {
      if (input.direction === "previous" || input.direction === "first") return "before";
      if (!isTreeBranch(row.node)) return "after";
      return input.direction === "last" ? "after" : "inside";
    },
    canDrop,
    onDrop,
  });
  return null;
}

export function TreeKeyboardTargets({
  rows,
  reorderable,
  editingValue,
  canDrop,
  onDrop,
}: {
  rows: Row[];
  reorderable: boolean;
  editingValue: string | null;
  canDrop: (intent: DropIntent, subject: DragSubject) => boolean | DropEligibility;
  onDrop: (intent: DropIntent) => DragDropCommitResult | Promise<DragDropCommitResult>;
}) {
  if (!reorderable) return null;
  return (
    <>
      {rows.map((row, index) => (
        <TreeKeyboardTarget
          key={row.node.value}
          row={row}
          index={index}
          disabled={Boolean(row.node.isDisabled) || editingValue === row.node.value}
          canDrop={canDrop}
          onDrop={onDrop}
        />
      ))}
    </>
  );
}

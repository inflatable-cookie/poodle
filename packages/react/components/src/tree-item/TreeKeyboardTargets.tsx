import { isTreeBranch, treeCanAcceptDrop, type DragDropCommitResult, type DropIntent } from "@inflatable-cookie/poodle-core";

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
  nodes,
  disabled,
  onDrop,
}: {
  row: Row;
  index: number;
  nodes: TreeNode[];
  disabled: boolean;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
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
    canDrop: (intent, subject) =>
      treeCanAcceptDrop(nodes, subject.id, intent.targetId)
        ? { accepted: true, intent }
        : { accepted: false, reason: subject.id === intent.targetId ? "self" : "subtree" },
    onDrop,
  });
  return null;
}

export function TreeKeyboardTargets({
  rows,
  nodes,
  reorderable,
  editingValue,
  onDrop,
}: {
  rows: Row[];
  nodes: TreeNode[];
  reorderable: boolean;
  editingValue: string | null;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
}) {
  if (!reorderable) return null;
  return (
    <>
      {rows.map((row, index) => (
        <TreeKeyboardTarget
          key={row.node.value}
          row={row}
          index={index}
          nodes={nodes}
          disabled={Boolean(row.node.isDisabled) || editingValue === row.node.value}
          onDrop={onDrop}
        />
      ))}
    </>
  );
}

<script lang="ts">
  import {
    isTreeBranch,
    treeCanAcceptDrop,
    type DragDropCommitResult,
    type DropIntent,
    type KeyboardDropTargetHandle,
    type TreeNodeLike,
  } from "@inflatable-cookie/poodle-core";

  import { useDragDrop } from "../drag-drop";
  import type { TreeNode } from "../types";

  interface Row {
    node: TreeNode;
    depth: number;
    parent: string | null;
  }

  interface Props {
    rows: Row[];
    nodes: TreeNode[];
    reorderable: boolean;
    editingValue: string | null;
    onDrop: (intent: DropIntent) => DragDropCommitResult;
  }

  let { rows, nodes, reorderable, editingValue, onDrop }: Props = $props();

  const { keyboardDropTarget } = useDragDrop();

  $effect(() => {
    if (!reorderable) return;
    const handles: KeyboardDropTargetHandle[] = rows.map((row, index) =>
      keyboardDropTarget({
        targetId: row.node.value,
        acceptedKinds: ["poodle.tree"],
        disabled: Boolean(row.node.isDisabled) || editingValue === row.node.value,
        label: row.node.label,
        order: index,
        resolvePosition: (input) => {
          if (input.direction === "previous" || input.direction === "first") return "before";
          if (!isTreeBranch(row.node as TreeNodeLike)) return "after";
          return input.direction === "last" ? "after" : "inside";
        },
        canDrop: (intent, subject) =>
          treeCanAcceptDrop(nodes, subject.id, intent.targetId)
            ? { accepted: true, intent }
            : { accepted: false, reason: subject.id === intent.targetId ? "self" : "subtree" },
        onDrop,
      }),
    );
    return () => {
      for (const handle of handles) handle.unregister();
    };
  });
</script>

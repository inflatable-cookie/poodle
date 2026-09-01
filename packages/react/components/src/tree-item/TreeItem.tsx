import { useRef, type KeyboardEvent, type MouseEvent, type ReactNode } from "react";
import {
  isTreeBranch,
  treeCanAcceptDrop,
  treeResolveDropPosition,
  type DragDropCommitResult,
  type DragSession,
  type DragTerminalOutcome,
  type DropIntent,
} from "@inflatable-cookie/poodle-core";

import { useDragSource, useDropTarget } from "../drag-drop";
import type { TreeNode } from "../types";

export interface TreeItemProps {
  node: TreeNode;
  nodes: TreeNode[];
  depth: number;
  parent: string | null;
  branch: boolean;
  open: boolean;
  selected: boolean;
  muted: boolean;
  focused: boolean;
  reorderable: boolean;
  editing: boolean;
  showGroup: boolean;
  row: ReactNode;
  group?: ReactNode;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
  onDragStart: (session: DragSession) => void;
  onDragEnd: (outcome: DragTerminalOutcome) => void;
  onClick: (event: MouseEvent) => void;
  onDoubleClick: (event: MouseEvent) => void;
  onContextMenu: (event: MouseEvent) => void;
  onKeyDown: (event: KeyboardEvent) => void;
}

export function TreeItem({
  node,
  nodes,
  depth,
  branch,
  open,
  selected,
  muted,
  focused,
  reorderable,
  editing,
  showGroup,
  row,
  group,
  onDrop,
  onDragStart,
  onDragEnd,
  onClick,
  onDoubleClick,
  onContextMenu,
  onKeyDown,
}: TreeItemProps) {
  const canDrag = reorderable && !node.isDisabled && !editing;
  const rowRef = useRef<HTMLDivElement | null>(null);
  const { getSourceProps } = useDragSource({
    sourceId: node.value,
    subject: { kind: "poodle.tree", id: node.value },
    allowedOperations: ["move"],
    label: node.label,
    disabled: !canDrag,
    handle: ".poodle-tree__row",
    onDragStart,
    onDragEnd,
  });
  const { getTargetProps } = useDropTarget({
    targetId: node.value,
    acceptedKinds: ["poodle.tree"],
    disabled: !canDrag,
    label: node.label,
    resolvePosition: (input) => {
      const rect = rowRef.current?.getBoundingClientRect() ?? input.rect;
      return treeResolveDropPosition({
        nodes,
        from: input.subject.id,
        to: node.value,
        y: input.y,
        rect,
        targetIsBranch: isTreeBranch(node),
      });
    },
    canDrop: (intent, subject) =>
      treeCanAcceptDrop(nodes, subject.id, intent.targetId)
        ? { accepted: true, intent }
        : { accepted: false, reason: subject.id === intent.targetId ? "self" : "subtree" },
    onDrop,
  });

  const itemProps = getSourceProps(
    getTargetProps({
      className: "poodle-tree__item",
      onClick,
      onDoubleClick,
      onContextMenu,
      onKeyDown,
    }),
  );

  return (
    <div
      {...itemProps}
      role="treeitem"
      data-value={node.value}
      data-branch={branch ? "true" : undefined}
      data-selected={selected ? "true" : undefined}
      data-muted={muted ? "true" : undefined}
      tabIndex={focused ? 0 : -1}
      aria-level={depth + 1}
      aria-selected={selected}
      aria-expanded={branch ? open : undefined}
      aria-disabled={node.isDisabled ? true : undefined}
    >
      <div className="poodle-tree__row" ref={rowRef}>
        {row}
      </div>
      {showGroup ? (
        <div className="poodle-tree__group" role="group">
          {group}
        </div>
      ) : null}
    </div>
  );
}

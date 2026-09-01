import { useLayoutEffect, useRef, type KeyboardEvent, type MouseEvent, type ReactNode } from "react";
import {
  treeDropEligibility,
  treeResolveOutlineDrop,
  readTreeDropMetrics,
  type DragDropCommitResult,
  type TreeOutlineRow,
  type DragSession,
  type DragTerminalOutcome,
  type DropIntent,
} from "@inflatable-cookie/poodle-core";

import { useDragSource, useDropTarget, useOptionalDragDrop } from "../drag-drop";
import type { TreeNode } from "../types";

export interface TreeItemProps {
  node: TreeNode;
  nodes: TreeNode[];
  outlineRows: TreeOutlineRow[];
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
  outlineRows,
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
  const itemRef = useRef<HTMLDivElement | null>(null);
  const outlineRowsRef = useRef(outlineRows);
  outlineRowsRef.current = outlineRows;
  const dragDrop = useOptionalDragDrop();
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
      const metrics = rowRef.current
        ? readTreeDropMetrics(rowRef.current)
        : { indentPx: 16, gutterPx: 24 };
      const placement = treeResolveOutlineDrop({
        rows: outlineRows,
        from: input.subject.id,
        to: node.value,
        x: input.x,
        y: input.y,
        rect,
        ...metrics,
      });
      if (!placement) return null;
      return {
        position: placement.indicator,
        destination: { targetId: placement.to, position: placement.position },
      };
    },
    canDrop: (intent, subject) => treeDropEligibility(nodes, subject.id, intent),
    onDrop,
  });

  useLayoutEffect(() => {
    const controller = dragDrop?.controller;
    if (!controller) return;
    const apply = (): void => {
      const item = itemRef.current;
      const row = rowRef.current;
      if (!item) return;
      const snap = controller.getSnapshot();
      if (snap.targetId !== node.value || snap.targetPosture !== "accepted" || !snap.pointer || !row) {
        item.style.removeProperty("--poodle-tree-drop-depth");
        return;
      }
      const depth = treeResolveOutlineDrop({
        rows: outlineRowsRef.current,
        from: snap.session?.subject.id ?? "",
        to: node.value,
        x: snap.pointer.x,
        y: snap.pointer.y,
        rect: row.getBoundingClientRect(),
        ...readTreeDropMetrics(row),
      })?.depth;
      if (depth == null) item.style.removeProperty("--poodle-tree-drop-depth");
      else item.style.setProperty("--poodle-tree-drop-depth", String(depth));
    };
    apply();
    return controller.subscribe(apply);
  }, [dragDrop?.controller, node.value]);

  const itemProps = getSourceProps(
    getTargetProps({
      className: "poodle-tree__item",
      ref: itemRef,
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

<script lang="ts">
  import {
    treeCanAcceptDrop,
    treeResolveOutlineDrop,
    type DragDropCommitResult,
    type TreeOutlineRow,
    type DragSession,
    type DragSourceRegistration,
    type DragTerminalOutcome,
    type DropIntent,
    type DropTargetRegistration,
  } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { useDragDrop } from "../drag-drop";
  import type { TreeNode } from "../types";

  interface Props {
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
    row: Snippet;
    group?: Snippet;
    showGroup?: boolean;
    onDrop: (intent: DropIntent) => DragDropCommitResult;
    onDragStart: (session: DragSession) => void;
    onDragEnd: (outcome: DragTerminalOutcome) => void;
    onClick: (event: MouseEvent) => void;
    onDblClick: (event: MouseEvent) => void;
    onContextMenu: (event: MouseEvent) => void;
    onKeyDown: (event: KeyboardEvent) => void;
  }

  let {
    node,
    nodes,
    outlineRows,
    depth,
    parent: _parent,
    branch,
    open,
    selected,
    muted,
    focused,
    reorderable,
    editing,
    row,
    group,
    showGroup = false,
    onDrop,
    onDragStart,
    onDragEnd,
    onClick,
    onDblClick,
    onContextMenu,
    onKeyDown,
  }: Props = $props();

  const { dragSource, dropTarget, snapshot } = useDragDrop();
  const canDrag = $derived(reorderable && !node.isDisabled && !editing);
  let rowEl: HTMLElement | undefined;

  function outlineDrop(x: number, y: number, rect: DOMRectReadOnly) {
    return treeResolveOutlineDrop({
      rows: outlineRows,
      from: $snapshot.session?.subject.id ?? "",
      to: node.value,
      x,
      y,
      rect,
    });
  }

  const dropDepth = $derived.by(() => {
    if ($snapshot.targetId !== node.value || $snapshot.targetPosture !== "accepted") return null;
    const pointer = $snapshot.pointer;
    const rect = rowEl?.getBoundingClientRect();
    if (!pointer || !rect) return null;
    return outlineDrop(pointer.x, pointer.y, rect)?.depth ?? null;
  });

  const sourceRegistration = $derived<DragSourceRegistration>({
    sourceId: node.value,
    subject: { kind: "poodle.tree", id: node.value },
    allowedOperations: ["move"],
    label: node.label,
    disabled: !canDrag,
    handle: ".poodle-tree__row",
    onDragStart,
    onDragEnd,
  });

  const targetRegistration = $derived<DropTargetRegistration>({
    targetId: node.value,
    acceptedKinds: ["poodle.tree"],
    disabled: !canDrag,
    label: node.label,
    resolvePosition: (input) => {
      const rect = rowEl?.getBoundingClientRect() ?? input.rect;
      return (
        treeResolveOutlineDrop({
          rows: outlineRows,
          from: input.subject.id,
          to: node.value,
          x: input.x,
          y: input.y,
          rect,
        })?.position ?? null
      );
    },
    canDrop: (intent, subject) =>
      treeCanAcceptDrop(nodes, subject.id, intent.targetId)
        ? { accepted: true, intent }
        : { accepted: false, reason: subject.id === intent.targetId ? "self" : "subtree" },
    onDrop,
  });
</script>

<div
  class="poodle-tree__item"
  role="treeitem"
  data-value={node.value}
  data-branch={branch ? "true" : undefined}
  data-selected={selected ? "true" : undefined}
  data-muted={muted ? "true" : undefined}
  style={dropDepth === null ? undefined : `--poodle-tree-drop-depth:${dropDepth}`}
  tabindex={focused ? 0 : -1}
  aria-level={depth + 1}
  aria-selected={selected}
  aria-expanded={branch ? open : undefined}
  aria-disabled={node.isDisabled ? "true" : undefined}
  onclick={onClick}
  ondblclick={onDblClick}
  oncontextmenu={onContextMenu}
  onkeydown={onKeyDown}
  use:dragSource={sourceRegistration}
  use:dropTarget={targetRegistration}
>
  <div class="poodle-tree__row" bind:this={rowEl}>
    {@render row()}
  </div>
  {#if showGroup && group}
    <div class="poodle-tree__group" role="group">
      {@render group()}
    </div>
  {/if}
</div>

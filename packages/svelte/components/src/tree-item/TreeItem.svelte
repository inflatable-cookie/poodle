<script lang="ts">
  import {
    isTreeBranch,
    resolveNestedDropPosition,
    treeCanAcceptDrop,
    type DragDropCommitResult,
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

  const { dragSource, dropTarget } = useDragDrop();
  const canDrag = $derived(reorderable && !node.isDisabled && !editing);
  let rowEl: HTMLElement | undefined;

  const sourceRegistration = $derived<DragSourceRegistration>({
    sourceId: node.value,
    subject: { kind: "poodle.tree", id: node.value },
    allowedOperations: ["move"],
    label: node.label,
    disabled: !canDrag,
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
      return resolveNestedDropPosition({
        y: input.y,
        rect,
        kind: isTreeBranch(node) ? "container" : "item",
      });
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
  tabindex={focused ? 0 : -1}
  aria-level={depth + 1}
  aria-selected={selected}
  aria-expanded={branch ? open : undefined}
  aria-disabled={node.isDisabled ? "true" : undefined}
  onclick={onClick}
  ondblclick={onDblClick}
  oncontextmenu={onContextMenu}
  onkeydown={onKeyDown}
  use:dropTarget={targetRegistration}
>
  <div class="poodle-tree__row" bind:this={rowEl} use:dragSource={sourceRegistration}>
    {@render row()}
  </div>
  {#if showGroup && group}
    <div class="poodle-tree__group" role="group">
      {@render group()}
    </div>
  {/if}
</div>

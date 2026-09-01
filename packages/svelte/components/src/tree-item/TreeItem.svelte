<script lang="ts">
  import {
    treeDropEligibility,
    treeResolveOutlineDrop,
    readTreeDropMetrics,
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
  import { tryDragDrop } from "../drag-drop-context";
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

  const { dragSource, dropTarget } = useDragDrop();
  const controller = tryDragDrop()?.controller;
  const canDrag = $derived(reorderable && !node.isDisabled && !editing);
  let rowEl: HTMLElement | undefined;
  let itemEl: HTMLElement | undefined;

  function applyDropDepth(): void {
    if (!itemEl) return;
    const snap = controller?.getSnapshot();
    const rect = rowEl?.getBoundingClientRect();
    if (
      !snap ||
      snap.targetId !== node.value ||
      snap.targetPosture !== "accepted" ||
      !snap.pointer ||
      !rect
    ) {
      itemEl.style.removeProperty("--poodle-tree-drop-depth");
      return;
    }
    const metrics = rowEl ? readTreeDropMetrics(rowEl) : { indentPx: 16, gutterPx: 24 };
    const depth = treeResolveOutlineDrop({
      rows: outlineRows,
      from: snap.session?.subject.id ?? "",
      to: node.value,
      x: snap.pointer.x,
      y: snap.pointer.y,
      rect,
      ...metrics,
    })?.depth;
    if (depth == null) itemEl.style.removeProperty("--poodle-tree-drop-depth");
    else itemEl.style.setProperty("--poodle-tree-drop-depth", String(depth));
  }

  $effect(() => {
    void itemEl;
    void rowEl;
    if (!controller) return;
    applyDropDepth();
    return controller.subscribe(applyDropDepth);
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
      const metrics = rowEl ? readTreeDropMetrics(rowEl) : { indentPx: 16, gutterPx: 24 };
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
</script>

<div
  bind:this={itemEl}
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

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/tree.css";
  import {
    findTreeNode,
    flattenVisibleTreeRows,
    isTreeBranch,
    treeCheckState,
    treeKeydownIntent,
    treeRangeSelection,
    treeSiblingReorderTarget,
    treeToggleCheck,
    treeVirtualWindow,
  } from "@inflatable-cookie/poodle-core";
  import { untrack } from "svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Checkbox } from "./Checkbox.svelte";
  import { default as Spinner } from "./Spinner.svelte";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    TreeNode,
  } from "./types";

  interface Props {
    nodes?: TreeNode[];
    selectedValues?: string[];
    expandedValues?: string[] | null;
    defaultExpandedValues?: string[];
    checkedValues?: string[];
    loadingValues?: string[];
    editingValue?: string | null;
    ariaLabel?: string | null;
    showGuides?: boolean;
    /**
     * Reclaim the twisty gutter when nothing in the tree can expand.
     *
     * Leaves render a twisty-sized spacer so their labels align with branch
     * labels. In a tree that is genuinely flat that spacer aligns them with
     * nothing, leaving an empty column down the left. Opt in and it collapses —
     * and comes back the moment any node becomes a branch, so the alignment is
     * never wrong, only absent when it buys nothing. See tree.md §7.
     */
    collapseTwistyWhenFlat?: boolean;
    showIcons?: boolean;
    showCheckboxes?: boolean;
    reorderable?: boolean;
    virtualized?: boolean;
    virtualHeight?: number;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onSelectionChange?: ((values: string[]) => void) | undefined;
    onExpandedChange?: ((values: string[]) => void) | undefined;
    onCheckedChange?: ((values: string[]) => void) | undefined;
    onLoadChildren?: ((value: string) => void) | undefined;
    onRenameCommit?: ((value: string, text: string) => void) | undefined;
    onRenameCancel?: (() => void) | undefined;
    onContextMenu?: ((value: string, x: number, y: number) => void) | undefined;
    onReorder?: ((from: string, to: string, position: DropPosition) => void) | undefined;
    onActivate?: ((value: string) => void) | undefined;
  }

  type DropPosition = "before" | "after" | "inside";

  let {
    nodes = [],
    selectedValues = $bindable<string[]>([]),
    expandedValues = $bindable<string[] | null>(null),
    defaultExpandedValues = [],
    checkedValues = $bindable<string[]>([]),
    loadingValues = [],
    editingValue = $bindable<string | null>(null),
    ariaLabel = null,
    showGuides = true,
    collapseTwistyWhenFlat = false,
    showIcons = true,
    showCheckboxes = false,
    reorderable = false,
    virtualized = false,
    virtualHeight = 320,
    size = null,
    sizeRole = "chrome",
    density = null,
    onSelectionChange = undefined,
    onExpandedChange = undefined,
    onCheckedChange = undefined,
    onLoadChildren = undefined,
    onRenameCommit = undefined,
    onRenameCancel = undefined,
    onContextMenu = undefined,
    onReorder = undefined,
    onActivate = undefined,
  }: Props = $props();

  type Row = { node: TreeNode; depth: number; parent: string | null };

  let rootEl: HTMLDivElement | null = $state(null);

  // Expansion is controlled when `expandedValues` is non-null, otherwise the
  // component tracks it internally seeded from `defaultExpandedValues`.
  let internalExpanded = $state<string[]>(untrack(() => [...defaultExpandedValues]));
  const isControlledExpansion = $derived(expandedValues !== null);
  const expanded = $derived(isControlledExpansion ? (expandedValues ?? []) : internalExpanded);

  function isExpanded(value: string): boolean {
    return expanded.includes(value);
  }
  function isSelected(value: string): boolean {
    return selectedValues.includes(value);
  }
  /** Whether any node in the whole tree can expand. */
  function hasAnyBranch(list: TreeNode[]): boolean {
    return list.some((node) => isTreeBranch(node) || hasAnyBranch(node.children ?? []));
  }
  const isFlat = $derived(collapseTwistyWhenFlat && !hasAnyBranch(nodes));

  function isBranch(node: TreeNode): boolean {
    return isTreeBranch(node);
  }
  function isChecked(value: string): boolean {
    return checkedValues.includes(value);
  }
  function isLoading(value: string): boolean {
    return loadingValues.includes(value);
  }

  function findNode(value: string): TreeNode | null {
    return findTreeNode(nodes, value);
  }

  // ── Checkbox cascade ───────────────────────────────────────────
  // Checkable atoms under a node: itself when it has no children (leaf or
  // empty/lazy branch), otherwise every leaf descendant.
  type Tri = "checked" | "unchecked" | "mixed";
  function checkState(node: TreeNode): Tri {
    return treeCheckState(node, checkedValues);
  }
  function setChecked(next: string[]): void {
    checkedValues = next;
    onCheckedChange?.(next);
  }
  function toggleCheck(node: TreeNode): void {
    setChecked(treeToggleCheck(node, checkedValues));
  }

  // Row height in px per size, for virtual-scroll windowing.
  const ROW_PX: Record<string, number> = { xs: 22, sm: 24, md: 28, lg: 32, xl: 36 };
  const rowHeightPx = $derived(ROW_PX[size ?? "md"] ?? 28);

  // ── Inline rename ──────────────────────────────────────────────
  function isEditing(value: string): boolean {
    return editingValue === value;
  }
  let renameDraft = $state("");
  let renameInputEl: HTMLInputElement | null = $state(null);
  let lastEditing: string | null = null;
  // Seed the draft from the node's label whenever rename starts (works for both
  // F2 and externally-controlled `editingValue`, e.g. a context-menu action),
  // then focus + select the input.
  $effect(() => {
    if (editingValue !== lastEditing) {
      lastEditing = editingValue;
      if (editingValue) renameDraft = findNode(editingValue)?.label ?? "";
    }
    if (editingValue && renameInputEl) {
      renameInputEl.focus();
      renameInputEl.select();
    }
  });
  function startRename(node: TreeNode): void {
    if (node.isDisabled) return;
    editingValue = node.value;
  }
  function commitRename(node: TreeNode): void {
    if (editingValue !== node.value) return;
    const text = renameDraft;
    editingValue = null;
    onRenameCommit?.(node.value, text);
  }
  function cancelRename(): void {
    if (editingValue === null) return;
    editingValue = null;
    onRenameCancel?.();
  }
  function renameKeydown(node: TreeNode, event: KeyboardEvent): void {
    event.stopPropagation();
    if (event.key === "Enter") {
      event.preventDefault();
      commitRename(node);
    } else if (event.key === "Escape") {
      event.preventDefault();
      cancelRename();
    }
  }

  // ── Context menu ───────────────────────────────────────────────
  function handleContextMenu(node: TreeNode, event: MouseEvent): void {
    if (!onContextMenu) return;
    event.preventDefault();
    event.stopPropagation();
    focusedValue = node.value;
    onContextMenu(node.value, event.clientX, event.clientY);
  }

  // ── Drag-and-drop reorder ──────────────────────────────────────
  let dragValue = $state<string | null>(null);
  let dropTarget = $state<string | null>(null);
  let dropPosition = $state<DropPosition>("after");

  // The sibling array that contains `value` (its parent's children, or roots).
  function siblingListOf(value: string, list: TreeNode[] = nodes): TreeNode[] | null {
    if (list.some((n) => n.value === value)) return list;
    for (const n of list) {
      if (n.children) {
        const found = siblingListOf(value, n.children);
        if (found) return found;
      }
    }
    return null;
  }

  function clearDrag(): void {
    dragValue = null;
    dropTarget = null;
  }
  function handleDragStart(node: TreeNode, event: DragEvent): void {
    if (!reorderable || node.isDisabled) return;
    dragValue = node.value;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", node.value);
    }
  }
  function handleDragOver(node: TreeNode, event: DragEvent): void {
    if (!reorderable || !dragValue || dragValue === node.value) return;
    event.preventDefault(); // mark as a valid drop target
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    const ratio = rect.height > 0 ? (event.clientY - rect.top) / rect.height : 0.5;
    let pos: DropPosition;
    if (isBranch(node)) {
      pos = ratio < 0.25 ? "before" : ratio > 0.75 ? "after" : "inside";
    } else {
      pos = ratio < 0.5 ? "before" : "after";
    }
    dropTarget = node.value;
    dropPosition = pos;
  }
  function handleDrop(node: TreeNode, event: DragEvent): void {
    if (!reorderable || !dragValue) return;
    event.preventDefault();
    const from = dragValue;
    const pos = dropPosition;
    clearDrag();
    if (from !== node.value) onReorder?.(from, node.value, pos);
  }
  // Alt+Up/Down moves the focused node among its siblings.
  function moveSibling(node: TreeNode, dir: 1 | -1): void {
    const sibs = siblingListOf(node.value);
    if (!sibs) return;
    const move = treeSiblingReorderTarget(sibs, node.value, dir);
    if (!move) return;
    onReorder?.(node.value, move.target, move.position);
  }

  // Flattened visible rows drive keyboard navigation and range selection.
  const visibleRows = $derived(flattenVisibleTreeRows(nodes, expanded) as Row[]);

  let focusedValue = $state<string | null>(null);
  // Roving tabindex: exactly one visible row is tabbable.
  const effectiveFocus = $derived(
    focusedValue && visibleRows.some((r) => r.node.value === focusedValue)
      ? focusedValue
      : selectedValues.find((v) => visibleRows.some((r) => r.node.value === v)) ??
        visibleRows[0]?.node.value ??
        null,
  );

  // Anchor for shift-range selection.
  let anchorValue: string | null = null;

  // ── Virtual scroll windowing (opt-in, flat render) ─────────────
  let scrollTop = $state(0);
  const virtualWindow = $derived(treeVirtualWindow(visibleRows.length, rowHeightPx, scrollTop, virtualHeight));
  const totalHeight = $derived(virtualWindow.totalHeight);
  const windowRows = $derived(visibleRows.slice(virtualWindow.startIndex, virtualWindow.endIndex));
  const offsetY = $derived(virtualWindow.offsetY);
  function handleScroll(event: Event): void {
    scrollTop = (event.currentTarget as HTMLElement).scrollTop;
  }

  // ── Expansion ──────────────────────────────────────────────────
  function setExpanded(next: string[]): void {
    if (isControlledExpansion) {
      expandedValues = next;
    } else {
      internalExpanded = next;
    }
    onExpandedChange?.(next);
  }
  function toggleExpanded(value: string): void {
    if (isExpanded(value)) collapse(value);
    else expand(value);
  }
  function expand(value: string): void {
    if (isExpanded(value)) return;
    setExpanded([...expanded, value]);
    // Lazy: expanding an empty branch requests its children.
    const node = findNode(value);
    if (node && isBranch(node) && !node.children?.length && !isLoading(value)) {
      onLoadChildren?.(value);
    }
  }
  function collapse(value: string): void {
    if (isExpanded(value)) setExpanded(expanded.filter((v) => v !== value));
  }

  // ── Selection ──────────────────────────────────────────────────
  function setSelection(next: string[]): void {
    selectedValues = next;
    onSelectionChange?.(next);
  }
  function selectOnly(value: string): void {
    anchorValue = value;
    setSelection([value]);
  }
  function toggleSelection(value: string): void {
    anchorValue = value;
    setSelection(
      isSelected(value) ? selectedValues.filter((v) => v !== value) : [...selectedValues, value],
    );
  }
  function selectRange(toValue: string): void {
    const range = treeRangeSelection(visibleRows, anchorValue, toValue);
    if (range === null) {
      selectOnly(toValue);
      return;
    }
    setSelection(range);
  }
  function extendSelection(toValue: string): void {
    if (anchorValue === null) anchorValue = effectiveFocus;
    selectRange(toValue);
  }

  // ── Pointer ────────────────────────────────────────────────────
  function handleRowClick(node: TreeNode, event: MouseEvent): void {
    // Stop the click bubbling to ancestor treeitems.
    event.stopPropagation();
    // Checkbox clicks toggle the check (cascade), not the row selection.
    if ((event.target as HTMLElement | null)?.closest(".poodle-tree__checkbox")) return;
    if (node.isDisabled) return;
    focusedValue = node.value;
    if (event.shiftKey) extendSelection(node.value);
    else if (event.metaKey || event.ctrlKey) toggleSelection(node.value);
    else selectOnly(node.value);
  }
  function handleTwistyClick(node: TreeNode, event: MouseEvent): void {
    event.stopPropagation();
    if (node.isDisabled) return;
    toggleExpanded(node.value);
  }
  function handleRowDblClick(node: TreeNode, event: MouseEvent): void {
    event.stopPropagation();
    if (node.isDisabled) return;
    onActivate?.(node.value);
  }

  // ── Keyboard (WAI-ARIA tree pattern) ───────────────────────────
  function focusRow(value: string | null): void {
    if (!value) return;
    focusedValue = value;
    const el = rootEl?.querySelector<HTMLElement>(`[data-value="${CSS.escape(value)}"]`);
    el?.focus();
  }

  function handleKeydown(row: Row, event: KeyboardEvent): void {
    // Keydown targets the focused treeitem; stop it reaching ancestors.
    event.stopPropagation();

    const intent = treeKeydownIntent(
      visibleRows,
      row.node.value,
      event.key,
      { altKey: event.altKey, shiftKey: event.shiftKey },
      { reorderable, expandedValues: expanded },
    );

    if (!intent) {
      return;
    }

    event.preventDefault();

    switch (intent.type) {
      case "focus": {
        if (intent.extendSelection && intent.value) extendSelection(intent.value);
        focusRow(intent.value);
        break;
      }
      case "expand":
        expand(intent.value);
        break;
      case "collapse":
        collapse(intent.value);
        break;
      case "focusParent":
        focusRow(intent.parent);
        break;
      case "moveSibling":
        moveSibling(row.node, intent.direction);
        break;
      case "activate":
        selectOnly(row.node.value);
        onActivate?.(row.node.value);
        break;
      case "toggleSelection":
        toggleSelection(row.node.value);
        break;
      case "startRename":
        startRename(row.node);
        break;
    }
  }
</script>

<div
  bind:this={rootEl}
  class="poodle-tree"
  role="tree"
  aria-multiselectable="true"
  aria-label={ariaLabel ?? undefined}
  data-size={size ?? undefined}
  data-density={density ?? undefined}
  data-size-role={sizeRole}
  data-virtualized={virtualized ? "true" : undefined}
  data-flat={isFlat ? "true" : undefined}
  style={virtualized ? `height:${virtualHeight}px;overflow-y:auto;` : undefined}
  onscroll={virtualized ? handleScroll : undefined}
>
  {#if virtualized}
    <div class="poodle-tree__viewport" style={`height:${totalHeight}px;`}>
      <div class="poodle-tree__window" style={`transform:translateY(${offsetY}px);`}>
        {#each windowRows as row (row.node.value)}
          {@render flatItem(row.node, row.depth, row.parent)}
        {/each}
      </div>
    </div>
  {:else}
    {#each nodes as node (node.value)}
      {@render renderNode(node, 0, null)}
    {/each}
  {/if}
</div>

{#snippet rowMarkup(node: TreeNode, depth: number, branch: boolean, open: boolean)}
  <div class="poodle-tree__row">
    {#each Array.from({ length: depth }) as _, i (i)}
      <span
        class="poodle-tree__indent"
        data-guide={showGuides ? "true" : undefined}
        aria-hidden="true"
      ></span>
    {/each}
    <span
      class="poodle-tree__twisty"
      data-expanded={open ? "true" : undefined}
      onclick={(event) => handleTwistyClick(node, event)}
      aria-hidden="true"
    >
      {#if branch}
        <Icon name="chevron-right" />
      {/if}
    </span>
    {#if showCheckboxes}
      {@const cs = checkState(node)}
      <span class="poodle-tree__checkbox">
        <Checkbox
          checked={cs === "checked"}
          mixed={cs === "mixed"}
          disabled={node.isDisabled}
          label={null}
          size={size ?? "xs"}
          onCheckedChange={() => toggleCheck(node)}
        />
      </span>
    {/if}
    {#if showIcons}
      <span class="poodle-tree__icon" aria-hidden="true">
        {#if node.icon}
          <Icon name={node.icon} />
        {/if}
      </span>
    {/if}
    {#if isEditing(node.value)}
      <!-- svelte-ignore a11y_autofocus -->
      <input
        bind:this={renameInputEl}
        class="poodle-tree__rename"
        bind:value={renameDraft}
        onkeydown={(event) => renameKeydown(node, event)}
        onblur={() => commitRename(node)}
        onclick={(event) => event.stopPropagation()}
        aria-label={`Rename ${node.label}`}
      />
    {:else}
      <span class="poodle-tree__label">{node.label}</span>
      {#if node.endLabel}
        <span class="poodle-tree__end-label">{node.endLabel}</span>
      {/if}
    {/if}
  </div>
{/snippet}

{#snippet renderNode(node: TreeNode, depth: number, parent: string | null)}
  {@const branch = isBranch(node)}
  {@const open = branch && isExpanded(node.value)}
  {@const selected = isSelected(node.value)}
  <div
    class="poodle-tree__item"
    role="treeitem"
    data-value={node.value}
    data-branch={branch ? "true" : undefined}
    data-selected={selected ? "true" : undefined}
    data-muted={node.isMuted ? "true" : undefined}
    data-drop={dropTarget === node.value ? dropPosition : undefined}
    draggable={reorderable && !node.isDisabled && !isEditing(node.value)}
    tabindex={effectiveFocus === node.value ? 0 : -1}
    aria-level={depth + 1}
    aria-selected={selected}
    aria-expanded={branch ? open : undefined}
    aria-disabled={node.isDisabled ? "true" : undefined}
    onclick={(event) => handleRowClick(node, event)}
    ondblclick={(event) => handleRowDblClick(node, event)}
    oncontextmenu={(event) => handleContextMenu(node, event)}
    ondragstart={(event) => handleDragStart(node, event)}
    ondragover={(event) => handleDragOver(node, event)}
    ondrop={(event) => handleDrop(node, event)}
    ondragend={clearDrag}
    onkeydown={(event) => handleKeydown({ node, depth, parent }, event)}
  >
    {@render rowMarkup(node, depth, branch, open)}
    {#if branch && open}
      {#if node.children?.length}
        <div class="poodle-tree__group" role="group">
          {#each node.children as child (child.value)}
            {@render renderNode(child, depth + 1, node.value)}
          {/each}
        </div>
      {:else if isLoading(node.value)}
        <div class="poodle-tree__group" role="group">
          {@render loadingRow(depth + 1)}
        </div>
      {/if}
    {/if}
  </div>
{/snippet}

{#snippet flatItem(node: TreeNode, depth: number, parent: string | null)}
  {@const branch = isBranch(node)}
  {@const open = branch && isExpanded(node.value)}
  {@const selected = isSelected(node.value)}
  <div
    class="poodle-tree__item"
    role="treeitem"
    data-value={node.value}
    data-branch={branch ? "true" : undefined}
    data-selected={selected ? "true" : undefined}
    data-muted={node.isMuted ? "true" : undefined}
    data-drop={dropTarget === node.value ? dropPosition : undefined}
    draggable={reorderable && !node.isDisabled && !isEditing(node.value)}
    tabindex={effectiveFocus === node.value ? 0 : -1}
    aria-level={depth + 1}
    aria-selected={selected}
    aria-expanded={branch ? open : undefined}
    aria-disabled={node.isDisabled ? "true" : undefined}
    onclick={(event) => handleRowClick(node, event)}
    ondblclick={(event) => handleRowDblClick(node, event)}
    oncontextmenu={(event) => handleContextMenu(node, event)}
    ondragstart={(event) => handleDragStart(node, event)}
    ondragover={(event) => handleDragOver(node, event)}
    ondrop={(event) => handleDrop(node, event)}
    ondragend={clearDrag}
    onkeydown={(event) => handleKeydown({ node, depth, parent }, event)}
  >
    {@render rowMarkup(node, depth, branch, open)}
  </div>
{/snippet}

{#snippet loadingRow(depth: number)}
  <div
    class="poodle-tree__item"
    role="treeitem"
    aria-level={depth + 1}
    aria-selected="false"
    aria-disabled="true"
  >
    <div class="poodle-tree__row poodle-tree__row--loading">
      {#each Array.from({ length: depth }) as _, i (i)}
        <span class="poodle-tree__indent" aria-hidden="true"></span>
      {/each}
      <span class="poodle-tree__twisty" aria-hidden="true"></span>
      <span class="poodle-tree__spinner"><Spinner size="xs" ariaLabel="Loading" /></span>
      <span class="poodle-tree__label poodle-tree__label--muted">Loading…</span>
    </div>
  </div>
{/snippet}

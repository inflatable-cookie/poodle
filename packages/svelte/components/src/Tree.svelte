<script lang="ts">
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
  function isBranch(node: TreeNode): boolean {
    return Boolean(node.isBranch) || (node.children?.length ?? 0) > 0;
  }
  function isChecked(value: string): boolean {
    return checkedValues.includes(value);
  }
  function isLoading(value: string): boolean {
    return loadingValues.includes(value);
  }

  function findNode(value: string, list: TreeNode[] = nodes): TreeNode | null {
    for (const node of list) {
      if (node.value === value) return node;
      const found = node.children ? findNode(value, node.children) : null;
      if (found) return found;
    }
    return null;
  }

  // ── Checkbox cascade ───────────────────────────────────────────
  // Checkable atoms under a node: itself when it has no children (leaf or
  // empty/lazy branch), otherwise every leaf descendant.
  function checkableUnder(node: TreeNode): string[] {
    if (!node.children?.length) return [node.value];
    return node.children.flatMap(checkableUnder);
  }
  type Tri = "checked" | "unchecked" | "mixed";
  function checkState(node: TreeNode): Tri {
    const leaves = checkableUnder(node);
    const n = leaves.filter(isChecked).length;
    return n === 0 ? "unchecked" : n === leaves.length ? "checked" : "mixed";
  }
  function setChecked(next: string[]): void {
    checkedValues = next;
    onCheckedChange?.(next);
  }
  function toggleCheck(node: TreeNode): void {
    const leaves = checkableUnder(node);
    const allOn = leaves.every(isChecked);
    setChecked(
      allOn
        ? checkedValues.filter((v) => !leaves.includes(v))
        : [...new Set([...checkedValues, ...leaves])],
    );
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
    const i = sibs.findIndex((n) => n.value === node.value);
    const j = i + dir;
    if (j < 0 || j >= sibs.length) return;
    onReorder?.(node.value, sibs[j].value, dir < 0 ? "before" : "after");
  }

  // Flattened visible rows drive keyboard navigation and range selection.
  const visibleRows = $derived.by(() => {
    const rows: Row[] = [];
    const walk = (list: TreeNode[], depth: number, parent: string | null): void => {
      for (const node of list) {
        rows.push({ node, depth, parent });
        if (isBranch(node) && isExpanded(node.value) && node.children?.length) {
          walk(node.children, depth + 1, node.value);
        }
      }
    };
    walk(nodes, 0, null);
    return rows;
  });

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
  const overscan = 6;
  const totalHeight = $derived(visibleRows.length * rowHeightPx);
  const startIdx = $derived(Math.max(0, Math.floor(scrollTop / rowHeightPx) - overscan));
  const endIdx = $derived(
    Math.min(visibleRows.length, Math.ceil((scrollTop + virtualHeight) / rowHeightPx) + overscan),
  );
  const windowRows = $derived(visibleRows.slice(startIdx, endIdx));
  const offsetY = $derived(startIdx * rowHeightPx);
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
    const order = visibleRows.map((r) => r.node.value);
    const a = order.indexOf(anchorValue ?? toValue);
    const b = order.indexOf(toValue);
    if (a === -1 || b === -1) {
      selectOnly(toValue);
      return;
    }
    const [lo, hi] = a <= b ? [a, b] : [b, a];
    const range = order.slice(lo, hi + 1).filter((v) => {
      const row = visibleRows.find((r) => r.node.value === v);
      return row !== undefined && !row.node.isDisabled;
    });
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
    const order = visibleRows;
    const idx = order.findIndex((r) => r.node.value === row.node.value);
    const node = row.node;
    switch (event.key) {
      case "ArrowDown": {
        event.preventDefault();
        if (event.altKey && reorderable) {
          moveSibling(node, 1);
          break;
        }
        const next = order[idx + 1]?.node.value ?? null;
        if (event.shiftKey && next && !order[idx + 1].node.isDisabled) extendSelection(next);
        focusRow(next);
        break;
      }
      case "ArrowUp": {
        event.preventDefault();
        if (event.altKey && reorderable) {
          moveSibling(node, -1);
          break;
        }
        const prev = order[idx - 1]?.node.value ?? null;
        if (event.shiftKey && prev && !order[idx - 1].node.isDisabled) extendSelection(prev);
        focusRow(prev);
        break;
      }
      case "ArrowRight": {
        event.preventDefault();
        if (isBranch(node)) {
          if (!isExpanded(node.value)) expand(node.value);
          else focusRow(order[idx + 1]?.node.value ?? null);
        }
        break;
      }
      case "ArrowLeft": {
        event.preventDefault();
        if (isBranch(node) && isExpanded(node.value)) collapse(node.value);
        else focusRow(row.parent);
        break;
      }
      case "Home": {
        event.preventDefault();
        focusRow(order[0]?.node.value ?? null);
        break;
      }
      case "End": {
        event.preventDefault();
        focusRow(order[order.length - 1]?.node.value ?? null);
        break;
      }
      case "Enter": {
        event.preventDefault();
        if (!node.isDisabled) {
          selectOnly(node.value);
          onActivate?.(node.value);
        }
        break;
      }
      case " ": {
        event.preventDefault();
        if (!node.isDisabled) toggleSelection(node.value);
        break;
      }
      case "F2": {
        event.preventDefault();
        startRename(node);
        break;
      }
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

<style>
  .poodle-tree {
    --poodle-tree-row-height: 1.75rem;
    --poodle-tree-row-font: var(--poodle-typography-label-size);
    --poodle-tree-twisty-size: calc(var(--poodle-tree-row-font) * 1.5);
    --poodle-tree-chevron-size: calc(var(--poodle-tree-row-font) * 0.85);
    --poodle-tree-indent: 1rem;
    --poodle-tree-row-gap: 0.25rem;
    --poodle-tree-row-pad-inline: 0.375rem;
    display: flex;
    flex-direction: column;
    min-width: 0;
    padding: var(--poodle-space-panel-y) 0.25rem;
  }

  /* ── Size variants (row height + font; never touched by density) ── */
  .poodle-tree[data-size="xs"] {
    --poodle-tree-row-height: 1.375rem;
    --poodle-tree-row-font: 0.6875rem;
  }
  .poodle-tree[data-size="sm"] {
    --poodle-tree-row-height: 1.5rem;
    --poodle-tree-row-font: 0.75rem;
  }
  .poodle-tree[data-size="md"] {
    --poodle-tree-row-height: 1.75rem;
    --poodle-tree-row-font: 0.8125rem;
  }
  .poodle-tree[data-size="lg"] {
    --poodle-tree-row-height: 2rem;
    --poodle-tree-row-font: 0.875rem;
  }
  .poodle-tree[data-size="xl"] {
    --poodle-tree-row-height: 2.25rem;
    --poodle-tree-row-font: 0.9375rem;
  }

  /* ── Density variants (horizontal rhythm only) ── */
  .poodle-tree[data-density="compact"] {
    --poodle-tree-indent: 0.75rem;
    --poodle-tree-row-gap: 0.1875rem;
    --poodle-tree-row-pad-inline: 0.25rem;
  }
  .poodle-tree[data-density="default"] {
    --poodle-tree-indent: 1rem;
    --poodle-tree-row-gap: 0.25rem;
    --poodle-tree-row-pad-inline: 0.375rem;
  }
  .poodle-tree[data-density="comfortable"] {
    --poodle-tree-indent: 1.25rem;
    --poodle-tree-row-gap: 0.375rem;
    --poodle-tree-row-pad-inline: 0.5rem;
  }

  .poodle-tree__item {
    display: flex;
    flex-direction: column;
    min-width: 0;
    outline: none;
  }

  .poodle-tree__row {
    display: flex;
    align-items: center;
    gap: var(--poodle-tree-row-gap);
    width: 100%;
    min-width: 0;
    min-height: var(--poodle-tree-row-height);
    padding-inline: var(--poodle-tree-row-pad-inline);
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-tree-row-font);
    font-weight: 500;
    line-height: 1.3;
    text-align: left;
    cursor: pointer;
    transition:
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-tree__indent {
    flex: 0 0 auto;
    align-self: stretch;
    width: var(--poodle-tree-indent);
    border-left: 0.0625rem solid transparent;
  }
  .poodle-tree__indent[data-guide="true"] {
    border-left-color: color-mix(in srgb, var(--poodle-color-border-subtle) 54%, transparent);
  }

  .poodle-tree__twisty {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-tree-twisty-size);
    font-size: var(--poodle-tree-chevron-size);
    color: var(--poodle-color-text-secondary);
    transition: transform var(--poodle-motion-duration-interaction)
      var(--poodle-motion-easing-standard);
  }
  .poodle-tree__twisty[data-expanded="true"] {
    transform: rotate(90deg);
  }

  .poodle-tree__icon {
    flex: 0 0 auto;
    display: inline-flex;
    align-items: center;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-tree__checkbox {
    flex: 0 0 auto;
    display: inline-flex;
    align-items: center;
  }

  .poodle-tree__spinner {
    flex: 0 0 auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-tree-twisty-size);
    color: var(--poodle-color-text-secondary);
  }

  .poodle-tree__label--muted {
    color: var(--poodle-color-text-secondary);
    font-style: italic;
  }

  .poodle-tree__rename {
    flex: 1 1 auto;
    min-width: 0;
    margin: 0;
    padding: 0 0.25rem;
    border: 0.0625rem solid var(--poodle-color-accent-base);
    border-radius: 0.1875rem;
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: inherit;
    font-size: inherit;
    line-height: 1.3;
    outline: none;
  }
  .poodle-tree__rename:focus-visible {
    box-shadow: 0 0 0 0.125rem
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 50%, transparent);
  }

  .poodle-tree[data-virtualized="true"] {
    display: block;
  }
  .poodle-tree__viewport {
    position: relative;
    width: 100%;
  }
  .poodle-tree__window {
    display: flex;
    flex-direction: column;
    will-change: transform;
  }

  .poodle-tree__label {
    flex: 1 1 auto;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .poodle-tree__row:hover {
    color: var(--poodle-color-text-primary);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 60%, transparent);
  }

  .poodle-tree__item[data-selected="true"] > .poodle-tree__row {
    color: var(--poodle-color-text-primary);
    font-weight: 600;
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
    box-shadow: inset 0 0 0 0.0625rem
      color-mix(in srgb, var(--poodle-color-accent-base) 20%, transparent);
  }

  .poodle-tree__item:focus-visible > .poodle-tree__row {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .poodle-tree__item[aria-disabled="true"] > .poodle-tree__row {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  /* ── Drag-and-drop reorder indicator ── */
  .poodle-tree__item > .poodle-tree__row {
    position: relative;
  }
  .poodle-tree__item[data-drop="before"] > .poodle-tree__row::before,
  .poodle-tree__item[data-drop="after"] > .poodle-tree__row::after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    height: 0.125rem;
    background: var(--poodle-color-accent-base);
    pointer-events: none;
  }
  .poodle-tree__item[data-drop="before"] > .poodle-tree__row::before {
    top: -0.0625rem;
  }
  .poodle-tree__item[data-drop="after"] > .poodle-tree__row::after {
    bottom: -0.0625rem;
  }
  .poodle-tree__item[data-drop="inside"] > .poodle-tree__row {
    box-shadow: inset 0 0 0 0.0625rem var(--poodle-color-accent-base);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }
</style>

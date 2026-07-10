/**
 * Tree behavior machinery.
 * Contract: docs/contracts/components/tree.md (pending — see the g11.004
 * classification sweep), "Behavior Machine" section.
 *
 * Pure logic for the tree: visible-row flattening (drives keyboard order,
 * range selection, and virtualization), tri-state checkbox cascade,
 * keyboard-intent resolution, shift-range selection, sibling-reorder
 * targets, and virtual-scroll windowing. Rename drafts, drag DOM plumbing,
 * and focus calls stay adapter-side.
 */

export interface TreeNodeLike {
  value: string;
  children?: TreeNodeLike[];
  isBranch?: boolean;
  isDisabled?: boolean;
}

export interface TreeRow<T extends TreeNodeLike = TreeNodeLike> {
  node: T;
  depth: number;
  parent: string | null;
}

export function isTreeBranch(node: TreeNodeLike): boolean {
  return Boolean(node.isBranch) || (node.children?.length ?? 0) > 0;
}

/** Depth-first flatten of the rows currently visible given the expansion set. */
export function flattenVisibleTreeRows<T extends TreeNodeLike>(
  nodes: readonly T[],
  expandedValues: readonly string[],
): TreeRow<T>[] {
  const rows: TreeRow<T>[] = [];

  const walk = (list: readonly T[], depth: number, parent: string | null): void => {
    for (const node of list) {
      rows.push({ node, depth, parent });

      if (isTreeBranch(node) && expandedValues.includes(node.value) && node.children?.length) {
        walk(node.children as T[], depth + 1, node.value);
      }
    }
  };

  walk(nodes, 0, null);

  return rows;
}

export function findTreeNode<T extends TreeNodeLike>(nodes: readonly T[], value: string): T | null {
  for (const node of nodes) {
    if (node.value === value) {
      return node;
    }

    const found = node.children ? findTreeNode(node.children as T[], value) : null;

    if (found) {
      return found;
    }
  }

  return null;
}

// ── Tri-state checkbox cascade ──

/** Checkable atoms under a node: itself when childless, else every leaf descendant. */
export function treeCheckableUnder(node: TreeNodeLike): string[] {
  if (!node.children?.length) {
    return [node.value];
  }

  return node.children.flatMap(treeCheckableUnder);
}

export type TreeCheckState = "checked" | "unchecked" | "mixed";

export function treeCheckState(node: TreeNodeLike, checkedValues: readonly string[]): TreeCheckState {
  const leaves = treeCheckableUnder(node);
  const checked = leaves.filter((value) => checkedValues.includes(value)).length;

  return checked === 0 ? "unchecked" : checked === leaves.length ? "checked" : "mixed";
}

/** Cascade toggle: all-on clears the subtree, otherwise checks it fully. */
export function treeToggleCheck(node: TreeNodeLike, checkedValues: readonly string[]): string[] {
  const leaves = treeCheckableUnder(node);
  const allOn = leaves.every((value) => checkedValues.includes(value));

  return allOn
    ? checkedValues.filter((value) => !leaves.includes(value))
    : [...new Set([...checkedValues, ...leaves])];
}

// ── Shift-range selection over the visible order ──

export function treeRangeSelection<T extends TreeNodeLike>(
  rows: readonly TreeRow<T>[],
  anchorValue: string | null,
  toValue: string,
): string[] | null {
  const order = rows.map((row) => row.node.value);
  const a = order.indexOf(anchorValue ?? toValue);
  const b = order.indexOf(toValue);

  if (a === -1 || b === -1) {
    return null;
  }

  const [lo, hi] = a <= b ? [a, b] : [b, a];

  return order
    .slice(lo, hi + 1)
    .filter((value) => !rows.find((row) => row.node.value === value)?.node.isDisabled);
}

// ── Sibling reorder target ──

export function treeSiblingReorderTarget<T extends TreeNodeLike>(
  siblings: readonly T[],
  value: string,
  direction: 1 | -1,
): { target: string; position: "before" | "after" } | null {
  const index = siblings.findIndex((node) => node.value === value);
  const nextIndex = index + direction;

  if (index < 0 || nextIndex < 0 || nextIndex >= siblings.length) {
    return null;
  }

  const target = siblings[nextIndex];

  if (target === undefined) {
    return null;
  }

  return { target: target.value, position: direction < 0 ? "before" : "after" };
}

// ── Keyboard intents ──

export type TreeKeyIntent =
  | { type: "focus"; value: string | null; extendSelection: boolean }
  | { type: "expand"; value: string }
  | { type: "collapse"; value: string }
  | { type: "focusParent"; parent: string | null }
  | { type: "moveSibling"; direction: 1 | -1 }
  | { type: "activate" }
  | { type: "toggleSelection" }
  | { type: "startRename" };

/**
 * Resolve a treeitem keydown to an intent the adapter executes. Mirrors the
 * pre-machine Svelte behavior exactly, including Alt+Arrow reorder and
 * Shift+Arrow range extension over enabled rows.
 */
export function treeKeydownIntent<T extends TreeNodeLike>(
  rows: readonly TreeRow<T>[],
  currentValue: string,
  key: string,
  modifiers: { altKey: boolean; shiftKey: boolean },
  options: { reorderable: boolean; expandedValues: readonly string[] },
): TreeKeyIntent | null {
  const index = rows.findIndex((row) => row.node.value === currentValue);
  const row = rows[index];

  if (!row) {
    return null;
  }

  const node = row.node;

  switch (key) {
    case "ArrowDown": {
      if (modifiers.altKey && options.reorderable) {
        return { type: "moveSibling", direction: 1 };
      }

      const next = rows[index + 1];

      return {
        type: "focus",
        value: next?.node.value ?? null,
        extendSelection: modifiers.shiftKey && next !== undefined && !next.node.isDisabled,
      };
    }
    case "ArrowUp": {
      if (modifiers.altKey && options.reorderable) {
        return { type: "moveSibling", direction: -1 };
      }

      const prev = rows[index - 1];

      return {
        type: "focus",
        value: prev?.node.value ?? null,
        extendSelection: modifiers.shiftKey && prev !== undefined && !prev.node.isDisabled,
      };
    }
    case "ArrowRight": {
      if (!isTreeBranch(node)) {
        return null;
      }

      if (!options.expandedValues.includes(node.value)) {
        return { type: "expand", value: node.value };
      }

      return { type: "focus", value: rows[index + 1]?.node.value ?? null, extendSelection: false };
    }
    case "ArrowLeft": {
      if (isTreeBranch(node) && options.expandedValues.includes(node.value)) {
        return { type: "collapse", value: node.value };
      }

      return { type: "focusParent", parent: row.parent };
    }
    case "Home":
      return { type: "focus", value: rows[0]?.node.value ?? null, extendSelection: false };
    case "End":
      return { type: "focus", value: rows[rows.length - 1]?.node.value ?? null, extendSelection: false };
    case "Enter":
      return node.isDisabled ? null : { type: "activate" };
    case " ":
      return node.isDisabled ? null : { type: "toggleSelection" };
    case "F2":
      return { type: "startRename" };
    default:
      return null;
  }
}

// ── Virtual-scroll windowing ──

export interface TreeVirtualWindow {
  startIndex: number;
  endIndex: number;
  offsetY: number;
  totalHeight: number;
}

export function treeVirtualWindow(
  rowCount: number,
  rowHeightPx: number,
  scrollTop: number,
  viewportHeightPx: number,
  overscan = 6,
): TreeVirtualWindow {
  const startIndex = Math.max(0, Math.floor(scrollTop / rowHeightPx) - overscan);
  const endIndex = Math.min(rowCount, Math.ceil((scrollTop + viewportHeightPx) / rowHeightPx) + overscan);

  return {
    startIndex,
    endIndex,
    offsetY: startIndex * rowHeightPx,
    totalHeight: rowCount * rowHeightPx,
  };
}

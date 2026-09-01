/**
 * Adapter-owned nested drop geometry.
 *
 * Architecture 011: measured geometry stays out of the semantic session.
 * Spec 069: a target maps a point + cached rect onto one before/inside/after
 * intent (or null). Leaves have no inside zone.
 */

import { flattenVisibleTreeRows, isTreeBranch, type TreeNodeLike } from "../tree";

export type NestedDropKind = "item" | "container";

export interface NestedDropRect {
  readonly top: number;
  readonly height: number;
}

export interface NestedDropPositionInput {
  readonly y: number;
  readonly rect: NestedDropRect;
  readonly kind: NestedDropKind;
  /** Fraction of height reserved for before and after on a container. Default 0.25. */
  readonly edgeRatio?: number;
}

/**
 * Map a pointer Y into one semantic position against a measured row.
 *
 * A container uses a middle inside band; an item is only before/after.
 * Degenerate or empty rectangles yield null so they cannot steal a hit.
 */
export function resolveNestedDropPosition(input: NestedDropPositionInput): "before" | "inside" | "after" | null {
  const height = input.rect.height;
  if (!(height > 0) || !Number.isFinite(input.y) || !Number.isFinite(input.rect.top)) {
    return null;
  }

  const ratio = (input.y - input.rect.top) / height;
  if (ratio < 0 || ratio > 1) {
    return null;
  }

  if (input.kind === "item") {
    return ratio < 0.5 ? "before" : "after";
  }

  const edge = clampEdgeRatio(input.edgeRatio);
  if (ratio < edge) return "before";
  if (ratio > 1 - edge) return "after";
  return "inside";
}

function clampEdgeRatio(value: number | undefined): number {
  if (value === undefined || !Number.isFinite(value)) return 0.25;
  if (value <= 0) return 0;
  if (value >= 0.5) return 0.5;
  return value;
}

export interface TreeOutlineRow {
  readonly value: string;
  readonly depth: number;
  readonly parent: string | null;
  readonly branch: boolean;
}

export function treeOutlineRows<T extends TreeNodeLike>(
  rows: readonly { node: T; depth: number; parent: string | null }[],
): TreeOutlineRow[] {
  return rows.map((row) => ({
    value: row.node.value,
    depth: row.depth,
    parent: row.parent,
    branch: isTreeBranch(row.node),
  }));
}

export interface TreeOutlineDrop {
  readonly to: string;
  readonly position: "before" | "after" | "inside";
  /** Visible depth the drop line should paint at. */
  readonly depth: number;
}

/**
 * Pointer placement for a tree drop, including indent from X.
 *
 * Vertical band picks before / inside / after on the hovered row. An `after`
 * on the last visible descendant of an open parent then offers every ancestor
 * that ends at this gap. Those levels are equal bands across the row: left
 * un-nests, right stays inside (or nests into a collapsed folder). Same-parent
 * leaves still land *at* the hovered row. A child dropped on its parent has
 * no `inside` (that would no-op).
 */
export function treeResolveOutlineDrop(input: {
  rows: readonly TreeOutlineRow[];
  from: string;
  to: string;
  y: number;
  rect: NestedDropRect & { readonly left?: number; readonly width?: number };
  x?: number;
}): TreeOutlineDrop | null {
  const hoveredIndex = input.rows.findIndex((row) => row.value === input.to);
  const fromIndex = input.rows.findIndex((row) => row.value === input.from);
  if (hoveredIndex < 0) return null;

  const hovered = input.rows[hoveredIndex]!;
  const fromRow = fromIndex >= 0 ? input.rows[fromIndex] : null;
  const band = verticalBand(input, hovered, fromRow);
  if (!band) return null;

  if (band === "inside") {
    return { to: hovered.value, position: "inside", depth: hovered.depth + 1 };
  }

  if (band === "before") {
    return { to: hovered.value, position: "before", depth: hovered.depth };
  }

  if (
    hovered.branch &&
    fromRow?.parent !== hovered.value &&
    lastVisibleUnder(input.rows, hoveredIndex) > hoveredIndex
  ) {
    // After-band on an expanded folder row sits above its children; that is
    // inside the folder, not after the whole subtree. A child already in the
    // folder uses the half-band to un-nest instead.
    return { to: hovered.value, position: "inside", depth: hovered.depth + 1 };
  }

  const afterSlots = afterCandidates(input.rows, hoveredIndex).slice().reverse();
  const nestInside =
    hovered.branch && fromRow?.parent !== hovered.value
      ? ({
          to: hovered.value,
          position: "inside" as const,
          depth: hovered.depth + 1,
        } satisfies TreeOutlineDrop)
      : null;
  const slots = nestInside ? [...afterSlots, nestInside] : afterSlots;
  const deepestAfter = afterSlots[afterSlots.length - 1]!;
  if (input.x === undefined || !Number.isFinite(input.x)) {
    return deepestAfter;
  }
  return pickHorizontalSlot(slots, input.x, input.rect.left ?? 0, input.rect.width ?? 0, deepestAfter);
}

function verticalBand(
  input: {
    rows: readonly TreeOutlineRow[];
    from: string;
    to: string;
    y: number;
    rect: NestedDropRect;
  },
  hovered: TreeOutlineRow,
  fromRow: TreeOutlineRow | null,
): "before" | "after" | "inside" | null {
  if (fromRow?.parent === input.to && hovered.branch) {
    return resolveNestedDropPosition({ y: input.y, rect: input.rect, kind: "item" });
  }

  if (fromRow && fromRow.parent === hovered.parent && !hovered.branch) {
    const fromOrder = input.rows.findIndex((row) => row.value === fromRow.value);
    const toOrder = input.rows.findIndex((row) => row.value === hovered.value);
    if (fromOrder < 0 || toOrder < 0 || fromOrder === toOrder) return null;
    return fromOrder < toOrder ? "after" : "before";
  }

  return resolveNestedDropPosition({
    y: input.y,
    rect: input.rect,
    kind: hovered.branch ? "container" : "item",
  });
}

function lastVisibleUnder(rows: readonly TreeOutlineRow[], ancestorIndex: number): number {
  const depth = rows[ancestorIndex]!.depth;
  let last = ancestorIndex;
  for (let index = ancestorIndex + 1; index < rows.length; index += 1) {
    if (rows[index]!.depth <= depth) break;
    last = index;
  }
  return last;
}

function afterCandidates(rows: readonly TreeOutlineRow[], hoveredIndex: number): TreeOutlineDrop[] {
  const hovered = rows[hoveredIndex]!;
  const candidates: TreeOutlineDrop[] = [
    { to: hovered.value, position: "after", depth: hovered.depth },
  ];
  let parent = hovered.parent;
  while (parent) {
    const ancestorIndex = rows.findIndex((row) => row.value === parent);
    if (ancestorIndex < 0) break;
    if (lastVisibleUnder(rows, ancestorIndex) !== hoveredIndex) break;
    const ancestor = rows[ancestorIndex]!;
    candidates.push({ to: ancestor.value, position: "after", depth: ancestor.depth });
    parent = ancestor.parent;
  }
  return candidates;
}

function pickHorizontalSlot<T>(
  slots: readonly T[],
  x: number,
  left: number,
  width: number,
  fallback: T,
): T {
  if (slots.length === 0 || !(width > 0)) return fallback;
  if (slots.length === 1) return slots[0] ?? fallback;
  const t = Math.min(1, Math.max(0, (x - left) / width));
  const index = Math.min(slots.length - 1, Math.floor(t * slots.length));
  return slots[index] ?? fallback;
}

function allBranchValues<T extends TreeNodeLike>(nodes: readonly T[]): string[] {
  const values: string[] = [];
  const walk = (list: readonly T[]): void => {
    for (const node of list) {
      if (isTreeBranch(node)) {
        values.push(node.value);
        if (node.children?.length) walk(node.children as T[]);
      }
    }
  };
  walk(nodes);
  return values;
}

/**
 * Pointer placement for a tree drop.
 *
 * Same-parent leaves land *at* the hovered row (arrival-band), matching
 * OrderBy / Tabs. A child dropped on its parent does not offer `inside` —
 * that would no-op — so the row splits in half and the lower half un-nests
 * the child to sit after the parent. Other branch targets keep the nested
 * before / inside / after bands. Pass `x` to walk the last-descendant
 * ancestor chain; omit it to keep the deepest (hovered) after-slot.
 */
export function treeResolveDropPosition<T extends TreeNodeLike>(input: {
  nodes: readonly T[];
  from: string;
  to: string;
  y: number;
  rect: NestedDropRect & { readonly left?: number; readonly width?: number };
  targetIsBranch?: boolean;
  x?: number;
}): "before" | "inside" | "after" | null {
  const rows = flattenVisibleTreeRows(input.nodes, allBranchValues(input.nodes)).map((row) => ({
    value: row.node.value,
    depth: row.depth,
    parent: row.parent,
    branch: input.targetIsBranch !== undefined && row.node.value === input.to
      ? input.targetIsBranch
      : isTreeBranch(row.node),
  }));
  return (
    treeResolveOutlineDrop({
      rows,
      from: input.from,
      to: input.to,
      y: input.y,
      rect: input.rect,
      x: input.x,
    })?.position ?? null
  );
}

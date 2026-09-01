/**
 * Adapter-owned nested drop geometry.
 *
 * Architecture 011: measured geometry stays out of the semantic session.
 * Spec 069: a target maps a point + cached rect onto one before/inside/after
 * intent (or null). Leaves have no inside zone.
 */

import { dropCommitDestination, type DropIntent } from "../drag-drop";
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

/** Indicator depth for an accepted intent's commit destination, not a second pointer pass. */
export function treeAcceptedDropDepth(
  rows: readonly TreeOutlineRow[],
  intent: DropIntent,
): number | null {
  const dest = dropCommitDestination(intent);
  const destRow = rows.find((row) => row.value === dest.targetId);
  if (!destRow) return null;
  if (dest.position === "inside") return destRow.depth + 1;
  if (dest.position === "before" || dest.position === "after") return destRow.depth;
  return null;
}

export interface TreeOutlineDrop {
  readonly to: string;
  readonly position: "before" | "after" | "inside";
  /** Visible depth the drop line should paint at. */
  readonly depth: number;
  /**
   * Where to draw the line on the *hovered* row. The gap above a row is the
   * same as the gap below the previous row; the commit `position` may be
   * `after` a previous node while the indicator stays `before` on this one.
   */
  readonly indicator: "before" | "after" | "inside";
}

/**
 * Pointer placement for a tree drop, including indent from X.
 *
 * Vertical band picks the gap above or below the hovered row. The gap below
 * the last visible descendant of an open parent offers every ancestor that
 * ends there. That same gap is the dragged next sibling's whole row and the
 * last descendant's whole row, so Y between those two rows does not change
 * depth. If that last descendant is the dragged row (nothing follows it),
 * its own row is still that gap. The gap above an open folder is only before
 * the folder: it does not indent. Nest from the folder row or the gap below.
 * `inside` appends as last child; the gap between an open folder header and
 * its first child is before that child. Depth steps are two indent columns so
 * a root filename stays at root until the pointer moves into the nested slot.
 * Same-parent leaves still land *at* the hovered row. Hovering a folder
 * (including the dragged node's parent or next sibling folder) uses an
 * `inside` band that appends as last child.
 */
export function treeResolveOutlineDrop(input: {
  rows: readonly TreeOutlineRow[];
  from: string;
  to: string;
  y: number;
  rect: NestedDropRect & { readonly left?: number; readonly width?: number };
  x?: number;
  indentPx?: number;
  gutterPx?: number;
}): TreeOutlineDrop | null {
  const hoveredIndex = input.rows.findIndex((row) => row.value === input.to);
  const fromIndex = input.rows.findIndex((row) => row.value === input.from);
  if (hoveredIndex < 0) return null;

  const hovered = input.rows[hoveredIndex]!;
  const fromRow = fromIndex >= 0 ? input.rows[fromIndex] : null;
  const band = verticalBand(input, hovered, fromRow, hoveredIndex, fromIndex);
  if (!band) return null;

  if (band === "inside") {
    return {
      to: hovered.value,
      position: "inside",
      depth: hovered.depth + 1,
      indicator: "inside",
    };
  }

  if (band === "before") {
    if (isExpandedFolder(input.rows, hoveredIndex) && fromRow?.parent !== hovered.value) {
      return {
        to: hovered.value,
        position: "before",
        depth: hovered.depth,
        indicator: "before",
      };
    }
    const landAtHovered =
      hoveredIndex === 0 ||
      (fromRow !== null &&
        fromRow.value !== hovered.value &&
        fromRow.parent === hovered.parent &&
        !hovered.branch);
    if (landAtHovered) {
      return {
        to: hovered.value,
        position: "before",
        depth: hovered.depth,
        indicator: "before",
      };
    }
    return afterGap(input.rows, hoveredIndex - 1, fromRow, input, "before");
  }

  const nextIndex = hoveredIndex + 1;
  if (
    fromRow?.value === hovered.value &&
    isExpandedFolder(input.rows, nextIndex) &&
    fromRow.parent !== input.rows[nextIndex]!.value
  ) {
    const folder = input.rows[nextIndex]!;
    return {
      to: folder.value,
      position: "before",
      depth: folder.depth,
      indicator: "after",
    };
  }

  if (
    hovered.branch &&
    fromRow?.parent !== hovered.value &&
    lastVisibleUnder(input.rows, hoveredIndex) > hoveredIndex
  ) {
    // After-band on an expanded folder row sits above its children. `inside`
    // appends, so this gap is before the first child. A child already in the
    // folder uses the half-band to un-nest instead.
    const first = input.rows[hoveredIndex + 1];
    if (first && first.depth > hovered.depth) {
      return {
        to: first.value,
        position: "before",
        depth: first.depth,
        indicator: "after",
      };
    }
  }

  return afterGap(input.rows, hoveredIndex, fromRow, input, "after");
}

function afterGap(
  rows: readonly TreeOutlineRow[],
  gapIndex: number,
  fromRow: TreeOutlineRow | null,
  input: {
    x?: number;
    indentPx?: number;
    gutterPx?: number;
    rect: NestedDropRect & { readonly left?: number; readonly width?: number };
  },
  indicator: "before" | "after",
): TreeOutlineDrop | null {
  const gapRow = rows[gapIndex];
  if (!gapRow) return null;
  // After an expanded folder row is the first-child gap (before the first
  // child — `inside` appends). A child already in the folder un-nests after
  // the folder from that lower half.
  if (gapRow.branch && lastVisibleUnder(rows, gapIndex) > gapIndex) {
    if (fromRow?.parent === gapRow.value) {
      return {
        to: gapRow.value,
        position: "after",
        depth: gapRow.depth,
        indicator,
      };
    }
    const first = rows[gapIndex + 1];
    if (first && first.depth > gapRow.depth) {
      return {
        to: first.value,
        position: "before",
        depth: first.depth,
        indicator,
      };
    }
  }
  const afterSlots = afterCandidates(rows, gapIndex).map((slot) => ({ ...slot, indicator }));
  const nestInside =
    gapRow.branch && fromRow?.parent !== gapRow.value
      ? ({
          to: gapRow.value,
          position: "inside" as const,
          depth: gapRow.depth + 1,
          indicator,
        } satisfies TreeOutlineDrop)
      : null;
  const slots = nestInside ? [...afterSlots.slice().reverse(), nestInside] : afterSlots.slice().reverse();
  const deepestAfter = afterSlots[0] ? { ...afterSlots[0], indicator } : null;
  if (!deepestAfter) return null;
  if (input.x === undefined || !Number.isFinite(input.x)) {
    return deepestAfter;
  }
  return pickStepSlot(
    slots,
    input.x,
    input.rect.left ?? 0,
    input.indentPx ?? 0,
    input.gutterPx ?? 0,
    slots[0] ?? deepestAfter,
  );
}

function isExpandedFolder(rows: readonly TreeOutlineRow[], index: number): boolean {
  const row = rows[index];
  return Boolean(row?.branch && lastVisibleUnder(rows, index) > index);
}

function gapOffersIndent(
  rows: readonly TreeOutlineRow[],
  gapIndex: number,
  fromRow: TreeOutlineRow,
): boolean {
  if (afterCandidates(rows, gapIndex).length > 1) return true;
  const gapRow = rows[gapIndex]!;
  return gapRow.branch && lastVisibleUnder(rows, gapIndex) === gapIndex && fromRow.parent !== gapRow.value;
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
  hoveredIndex: number,
  fromIndex: number,
): "before" | "after" | "inside" | null {
  // Closing gap of an open parent: the dragged next sibling (whole row) and
  // the last descendant (whole row) share one X-mapped candidate set. When the
  // last descendant is the dragged row, nothing follows it — still that gap.
  if (fromRow && fromIndex === hoveredIndex && hoveredIndex > 0 && gapOffersIndent(input.rows, hoveredIndex - 1, fromRow)) {
    return "before";
  }
  if (fromRow && fromIndex === hoveredIndex && gapOffersIndent(input.rows, hoveredIndex, fromRow)) {
    return "after";
  }
  if (fromRow && fromIndex === hoveredIndex + 1 && gapOffersIndent(input.rows, hoveredIndex, fromRow)) {
    return "after";
  }
  // Entry gap: the dragged row sitting immediately above an expanded folder
  // is before-folder only. The folder row itself still has an inside band so
  // a drop on the folder appends.
  if (
    fromRow &&
    fromIndex === hoveredIndex &&
    isExpandedFolder(input.rows, hoveredIndex + 1) &&
    fromRow.parent !== input.rows[hoveredIndex + 1]!.value
  ) {
    return "after";
  }

  if (fromRow && fromRow.parent === hovered.parent && !hovered.branch) {
    if (fromIndex < 0 || fromIndex === hoveredIndex) return null;
    return fromIndex < hoveredIndex ? "after" : "before";
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
    { to: hovered.value, position: "after", depth: hovered.depth, indicator: "after" },
  ];
  let parent = hovered.parent;
  while (parent) {
    const ancestorIndex = rows.findIndex((row) => row.value === parent);
    if (ancestorIndex < 0) break;
    if (lastVisibleUnder(rows, ancestorIndex) !== hoveredIndex) break;
    const ancestor = rows[ancestorIndex]!;
    candidates.push({
      to: ancestor.value,
      position: "after",
      depth: ancestor.depth,
      indicator: "after",
    });
    parent = ancestor.parent;
  }
  return candidates;
}

function pickStepSlot<T extends { depth: number }>(
  slots: readonly T[],
  x: number,
  left: number,
  indentPx: number,
  gutterPx: number,
  fallback: T,
): T {
  if (slots.length === 0) return fallback;
  if (slots.length === 1) return slots[0] ?? fallback;
  // Two indent columns per level so the filename of a root row stays at root,
  // and a modest move right reaches the nested slot without crossing half the row.
  const step = Math.max(indentPx > 0 ? indentPx * 2 : 32, 32);
  const index = Math.max(
    0,
    Math.min(slots.length - 1, Math.floor((x - left - gutterPx) / step)),
  );
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
 * OrderBy / Tabs. A child dropped on its parent keeps before / inside /
 * after: inside appends as last child, the lower edge un-nests after the
 * parent. Other branch targets keep the nested before / inside / after
 * bands. Pass `x` to walk the last-descendant
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
  indentPx?: number;
  gutterPx?: number;
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
      indentPx: input.indentPx,
      gutterPx: input.gutterPx,
    })?.position ?? null
  );
}

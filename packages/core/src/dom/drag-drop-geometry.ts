/**
 * Adapter-owned nested drop geometry.
 *
 * Architecture 011: measured geometry stays out of the semantic session.
 * Spec 069: a target maps a point + cached rect onto one before/inside/after
 * intent (or null). Leaves have no inside zone.
 */

import { isTreeBranch, treeLocate, type TreeNodeLike } from "../tree";

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

/**
 * Pointer placement for a tree drop.
 *
 * Same-parent leaves land *at* the hovered row (arrival-band), matching
 * OrderBy / Tabs. A child dropped on its parent does not offer `inside` —
 * that would no-op — so the row splits in half and the lower half un-nests
 * the child to sit after the parent. Other branch targets keep the nested
 * before / inside / after bands.
 */
export function treeResolveDropPosition<T extends TreeNodeLike>(input: {
  nodes: readonly T[];
  from: string;
  to: string;
  y: number;
  rect: NestedDropRect;
  targetIsBranch?: boolean;
}): "before" | "inside" | "after" | null {
  const fromLoc = treeLocate(input.nodes, input.from);
  const toLoc = treeLocate(input.nodes, input.to);
  if (!fromLoc || !toLoc) return null;

  const target = toLoc.siblings[toLoc.index];
  const targetIsBranch = input.targetIsBranch ?? (target !== undefined && isTreeBranch(target));

  if (fromLoc.parent === input.to && targetIsBranch) {
    return resolveNestedDropPosition({ y: input.y, rect: input.rect, kind: "item" });
  }

  if (fromLoc.parent === toLoc.parent && !targetIsBranch) {
    return fromLoc.index < toLoc.index ? "after" : "before";
  }

  return resolveNestedDropPosition({
    y: input.y,
    rect: input.rect,
    kind: targetIsBranch ? "container" : "item",
  });
}

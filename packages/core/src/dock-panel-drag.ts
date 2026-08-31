/**
 * The semantic subject a dragged dock panel carries.
 *
 * Contract: `docs/contracts/components/dock-region.md` §8.
 * Spec: `docs/specs/069-dependable-drag-and-drop-substrate.md`.
 *
 * A panel move is an ordinary drag session, so the panel's identity has to
 * travel in the one field the substrate carries: `DragSubject.id`. That field
 * is opaque to Poodle's kernel and resolved through consumer state, which is
 * exactly what a dock panel needs — the receiving region has to know which
 * panel, from which edge, and from which zone, and it has to know all three
 * during *hover*, not only at drop.
 *
 * This is what replaces the old module-global `dockPanelDragSession`. That
 * side channel existed only because HTML5 hides the `DataTransfer` body during
 * `dragover`, so a hovered region could not learn which panel was in flight
 * from the event. On the shared substrate the subject is simply part of the
 * session, and a document-global registry has nothing left to do.
 *
 * Lives here rather than in a framework package because both web targets must
 * encode it identically: a region written by one and hovered by the other has
 * to read the same subject.
 */

export interface DockPanelDragSubject {
  panelId: string;
  sourceEdge: string;
  /** The source region's `dragZoneId`, falling back to its edge. */
  sourceZone: string;
}

/**
 * The one subject kind every dock region accepts.
 *
 * Deliberately not scoped per instance, unlike a reorder surface's kind: a
 * panel is *meant* to cross between regions, and two regions can only resolve
 * each other's targets when they agree on the kind. Which regions can actually
 * see each other is decided by which controller they registered with, not by
 * the kind.
 */
export const DOCK_PANEL_SUBJECT_KIND = "poodle.dock-panel";

/**
 * Encode a panel's identity into a subject id.
 *
 * JSON rather than a delimited string: a panel id, an edge, or a zone id is
 * consumer-supplied text, and any separator character chosen here would one
 * day appear inside one of them.
 */
export function encodeDockPanelSubject(subject: DockPanelDragSubject): string {
  return JSON.stringify({
    panelId: subject.panelId,
    sourceEdge: subject.sourceEdge,
    sourceZone: subject.sourceZone,
  });
}

/** Decode a subject id, or `null` when it is not one of ours. */
export function decodeDockPanelSubject(id: string): DockPanelDragSubject | null {
  let parsed: unknown;
  try {
    parsed = JSON.parse(id);
  } catch {
    return null;
  }
  if (typeof parsed !== "object" || parsed === null) return null;
  const candidate = parsed as Partial<DockPanelDragSubject>;
  if (
    typeof candidate.panelId !== "string" ||
    typeof candidate.sourceEdge !== "string" ||
    typeof candidate.sourceZone !== "string"
  ) {
    return null;
  }
  return {
    panelId: candidate.panelId,
    sourceEdge: candidate.sourceEdge,
    sourceZone: candidate.sourceZone,
  };
}

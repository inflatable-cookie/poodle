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
 * The marker that says an id is one of ours.
 *
 * Checked before decoding so a consumer value that merely happens to contain
 * separators is refused rather than parsed into a plausible-looking panel.
 */
const PREFIX = "poodle-panel:";

/**
 * Encode a panel's identity into a subject id.
 *
 * Percent-encoded fields joined by `|`, not JSON: this value becomes part of
 * generated DOM ids, so it has to survive an attribute without braces and
 * quotes in it. Encoding each field keeps the separator unambiguous even when
 * a consumer's panel id or zone contains one.
 */
export function encodeDockPanelSubject(subject: DockPanelDragSubject): string {
  return [
    PREFIX + encodeURIComponent(subject.sourceZone),
    encodeURIComponent(subject.sourceEdge),
    encodeURIComponent(subject.panelId),
  ].join("|");
}

/** Decode a subject id, or `null` when it is not one of ours. */
export function decodeDockPanelSubject(id: string): DockPanelDragSubject | null {
  if (!id.startsWith(PREFIX)) return null;
  const parts = id.slice(PREFIX.length).split("|");
  if (parts.length !== 3) return null;
  const [zone, edge, panel] = parts as [string, string, string];
  try {
    return {
      panelId: decodeURIComponent(panel),
      sourceEdge: decodeURIComponent(edge),
      sourceZone: decodeURIComponent(zone),
    };
  } catch {
    return null;
  }
}

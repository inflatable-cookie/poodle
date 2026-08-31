import {
  decodeDockPanelSubject,
  encodeDockPanelSubject,
  DOCK_PANEL_SUBJECT_KIND,
  type CrossWindowDragSourceBridge,
  type DragDropCommitResult,
} from "@inflatable-cookie/poodle-core";
import type { ReactNode } from "react";

import { useDragSource, useDropTarget } from "../drag-drop";
import type { DockEdge, PanelDragData, PanelTabItem } from "../types";

/**
 * One stacked panel: a drag source, and a target for the insert position.
 *
 * Static mode is where DockRegion owns the panels themselves, so this is the
 * local move the substrate carries end to end. A drop from this region's own
 * zone is a reorder; anything else is a transfer.
 *
 * It is a component because the substrate is consumed through hooks and a hook
 * count cannot depend on a list length.
 */
export interface DockStackItemProps {
  item: PanelTabItem;
  index: number;
  edge: DockEdge;
  dropZoneId: string;
  items: PanelTabItem[];
  canAcceptPanel: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
  crossWindowDragSource?: CrossWindowDragSourceBridge;
  liveSubjectId: () => string;
  onReorder?: (items: string[]) => void;
  onPanelDrop?: (payload: { panel: PanelDragData; targetEdge: DockEdge }) => void;
  children: ReactNode;
}

export function DockStackItem({
  item,
  index,
  edge,
  dropZoneId,
  items,
  canAcceptPanel,
  crossWindowDragSource,
  liveSubjectId,
  onReorder,
  onPanelDrop,
  children,
}: DockStackItemProps) {
  const label = item.label ?? `Panel ${index + 1}`;

  const { getSourceProps, dragging } = useDragSource({
    sourceId: `${dropZoneId}:${item.value}`,
    subject: {
      kind: DOCK_PANEL_SUBJECT_KIND,
      id: encodeDockPanelSubject({
        panelId: item.value,
        sourceEdge: edge,
        sourceZone: dropZoneId,
      }),
    },
    allowedOperations: ["move"],
    label,
    crossWindowSourceBridge: crossWindowDragSource,
  });

  const { getTargetProps, accepted } = useDropTarget({
    targetId: `${dropZoneId}:slot:${item.value}`,
    acceptedKinds: [DOCK_PANEL_SUBJECT_KIND],
    label,
    resolvePosition: () => "inside",
    canDrop: (intent, subject) => {
      const panel = decodeDockPanelSubject(subject.id);
      if (!panel) return { accepted: false, reason: "not a panel" };
      if (panel.sourceZone === dropZoneId && panel.panelId === item.value) {
        return { accepted: false, reason: "same panel" };
      }
      if (canAcceptPanel !== null && !canAcceptPanel(panel.panelId, panel.sourceEdge as DockEdge)) {
        return { accepted: false, reason: "refused by host" };
      }
      return { accepted: true, intent };
    },
    onDrop: (): DragDropCommitResult => {
      const panel = decodeDockPanelSubject(liveSubjectId());
      if (!panel) return { status: "rejected", reason: "not a panel" };

      if (panel.sourceZone === dropZoneId) {
        const order = items.map((entry) => entry.value);
        const from = order.indexOf(panel.panelId);
        if (from < 0) return { status: "rejected", reason: "unknown panel" };
        const [moved] = order.splice(from, 1);
        order.splice(index, 0, moved);
        onReorder?.(order);
        return { status: "committed" };
      }

      onPanelDrop?.({
        panel: {
          panelId: panel.panelId,
          sourceEdge: panel.sourceEdge as DockEdge,
          sourceZone: panel.sourceZone,
        },
        targetEdge: edge,
      });
      return { status: "committed" };
    },
  });

  // Nested rather than merged by hand: each getter composes the ref and
  // handlers it was handed, so one element can be both a source and a target
  // without either overwriting the other.
  return (
    <div
      {...getTargetProps(
        getSourceProps({
          className: "poodle-dock-region__stack-item",
          role: "group",
          "aria-label": label,
        } as never),
      )}
      data-drop-target={accepted || undefined}
      data-drag-source={dragging || undefined}
    >
      {children}
    </div>
  );
}

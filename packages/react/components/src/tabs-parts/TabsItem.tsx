import type {
  CrossWindowDragSourceBridge,
  DragDropCommitResult,
  DropIntent,
  DropPosition,
} from "@inflatable-cookie/poodle-core";
import type { KeyboardEvent as ReactKeyboardEvent, ReactNode } from "react";

import { Icon } from "../Icon";
import { useDragSource, useDropTarget } from "../drag-drop";
import type { ControlSize, TabItem } from "../types";

/**
 * One interactive tab.
 *
 * It exists as its own component because the drag substrate is consumed
 * through hooks, and hooks cannot run inside the component that renders the
 * provider they read. The measure list has no drag behaviour and stays inline
 * in Tabs.
 */
export interface TabsItemProps {
  item: TabItem;
  tabsId: string;
  subjectKind: string;
  selected: boolean;
  focused: boolean;
  hasPanel: boolean;
  isVertical: boolean;
  reorderable: boolean;
  iconSize: ControlSize;
  crossWindowSourceBridge?: CrossWindowDragSourceBridge;
  /** Whether a subject id belongs to this strip at all. */
  ownsValue: (value: string) => boolean;
  sourceId: string;
  targetId: string;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
  onElement: (element: HTMLButtonElement | null) => void;
  onSelect: () => void;
  onClose: () => void;
  onKeyDown: (event: ReactKeyboardEvent) => void;
  onFocus: () => void;
  onBlur: () => void;
  onEnter: () => void;
  onLeave: () => void;
  content: ReactNode;
  tooltip: ReactNode;
}

export function TabsItem({
  item,
  tabsId,
  subjectKind,
  selected,
  focused,
  hasPanel,
  isVertical,
  reorderable,
  iconSize,
  crossWindowSourceBridge,
  ownsValue,
  sourceId,
  targetId,
  onDrop,
  onElement,
  onSelect,
  onClose,
  onKeyDown,
  onFocus,
  onBlur,
  onEnter,
  onLeave,
  content,
  tooltip,
}: TabsItemProps) {
  /** A disabled tab cannot be picked up. It is still a place to put one. */
  const canDrag = reorderable && item.disabled !== true;

  const { getSourceProps, dragging } = useDragSource({
    sourceId,
    subject: { kind: subjectKind, id: item.value },
    allowedOperations: ["move"],
    label: item.label,
    disabled: !canDrag,
    crossWindowSourceBridge,
  });

  /**
   * Contract: the band rule reads the fraction of this tab's own bounds along
   * the strip axis. The origin-facing half is `before`, the trailing half is
   * `after`, so dragging over a sibling and back toward origin is a no-op
   * rather than a swap.
   */
  const { getTargetProps, accepted } = useDropTarget({
    targetId,
    acceptedKinds: [subjectKind],
    disabled: !reorderable,
    label: item.label,
    resolvePosition: ({ x, y, rect }): DropPosition =>
      isVertical
        ? y < rect.top + rect.height / 2
          ? "before"
          : "after"
        : x < rect.left + rect.width / 2
          ? "before"
          : "after",
    canDrop: (intent, subject) => {
      // A shared family means another surface's subject can reach this target.
      // Refusing it *here*, during eligibility, is what lets arbitration
      // discard this tab and hand the drop to an eligible ancestor composite.
      // Claiming it and rejecting at commit would swallow the drop instead.
      if (!ownsValue(subject.id)) {
        return { accepted: false, reason: "not this tab set" };
      }
      return subject.id === item.value
        ? { accepted: false, reason: "same tab" }
        : { accepted: true, intent };
    },
    onDrop,
  });

  return (
    <div
      {...getTargetProps({
        className: "poodle-tabs__item",
        role: "presentation",
        onMouseEnter: onEnter,
        onMouseLeave: onLeave,
      })}
      data-selected={selected}
      data-reorderable={canDrag || undefined}
      data-drag-source={dragging || undefined}
      data-drop-target={accepted || undefined}
    >
      <button
        {...getSourceProps({
          className: "poodle-tabs__tab",
          onFocus,
          onBlur,
          onClick: onSelect,
          onKeyDown,
          ref: onElement as never,
        })}
        type="button"
        disabled={item.disabled === true}
        id={`poodle-tab-${tabsId}-${item.value}`}
        data-value={item.value}
        role="tab"
        tabIndex={focused ? 0 : -1}
        aria-selected={selected ? "true" : "false"}
        aria-controls={hasPanel ? `poodle-tabpanel-${tabsId}-${item.value}` : undefined}
      >
        {content}
      </button>

      {item.closable ? (
        <button
          type="button"
          className="poodle-tabs__close"
          aria-label={`Close ${item.label}`}
          onClick={(event) => {
            event.stopPropagation();
            onClose();
          }}
        >
          <Icon name="x" size={iconSize} />
        </button>
      ) : null}

      {tooltip}
    </div>
  );
}

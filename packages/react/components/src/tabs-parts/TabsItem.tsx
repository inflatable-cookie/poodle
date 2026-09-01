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
  index: number;
  tabsId: string;
  subjectKind: string;
  selected: boolean;
  focused: boolean;
  hasPanel: boolean;
  reorderable: boolean;
  iconSize: ControlSize;
  crossWindowSourceBridge?: CrossWindowDragSourceBridge;
  /** Index of the value being dragged, so the whole-tab band can resolve. */
  indexOfValue: (value: string) => number;
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
  index,
  tabsId,
  subjectKind,
  selected,
  focused,
  hasPanel,
  reorderable,
  iconSize,
  crossWindowSourceBridge,
  indexOfValue,
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
   * The drop indicator paints the whole tab, so the band must too: a drop on
   * this sibling lands *at* this sibling. Which half the pointer is over does
   * not change that public result. Coming from the left, "at" is after; coming
   * from the right, it is before.
   */
  const { getTargetProps, accepted } = useDropTarget({
    targetId,
    acceptedKinds: [subjectKind],
    disabled: !reorderable,
    label: item.label,
    resolvePosition: ({ subject }): DropPosition =>
      indexOfValue(subject.id) < index ? "after" : "before",
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

import type { DragDropCommitResult, DropIntent } from "@inflatable-cookie/poodle-core";

import { useKeyboardDropTarget } from "../drag-drop";
import type { TabItem } from "../types";

/**
 * The ordered logical target registry Alt+Arrow moves through.
 *
 * Tabs keeps its established one-keystroke reorder, so it never enters pickup
 * mode — Space and Enter stay selection keys. The move still runs as an
 * ordinary keyboard session through `requestKeyboardDrop`, which is what keeps
 * announcements, revalidation, and the terminal identical to a pointer drop.
 *
 * One registration per tab, each its own component, because a hook count
 * cannot depend on a list length.
 */
export interface TabsKeyboardTargetsProps {
  items: TabItem[];
  reorderable: boolean;
  subjectKind: string;
  targetIdOf: (value: string) => string;
  ownsValue: (value: string) => boolean;
  isPinnedValue: (value: string) => boolean;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
}
function TabKeyboardTarget({
  item,
  index,
  reorderable,
  subjectKind,
  targetIdOf,
  ownsValue,
  isPinnedValue,
  onDrop,
}: {
  item: TabItem;
  index: number;
  reorderable: boolean;
  subjectKind: string;
  targetIdOf: (value: string) => string;
  ownsValue: (value: string) => boolean;
  isPinnedValue: (value: string) => boolean;
  onDrop: (intent: DropIntent) => DragDropCommitResult;
}) {
  useKeyboardDropTarget({
    targetId: targetIdOf(item.value),
    acceptedKinds: [subjectKind],
    disabled: !reorderable,
    label: item.label,
    order: index,
    resolvePosition: (input) =>
      input.direction === "previous" || input.direction === "first" ? "before" : "after",
    canDrop: (intent, subject) => {
      if (!ownsValue(subject.id)) {
        return { accepted: false, reason: "not this tab set" };
      }
      if (subject.id === item.value) {
        return { accepted: false, reason: "same tab" };
      }
      if (isPinnedValue(subject.id) || isPinnedValue(item.value)) {
        return { accepted: false, reason: "pinned partition" };
      }
      return { accepted: true, intent };
    },
    onDrop,
  });
  return null;
}

export function TabsKeyboardTargets({
  items,
  reorderable,
  subjectKind,
  targetIdOf,
  ownsValue,
  isPinnedValue,
  onDrop,
}: TabsKeyboardTargetsProps) {
  return (
    <>
      {items.map((item, index) => (
        <TabKeyboardTarget
          key={item.value}
          item={item}
          index={index}
          reorderable={reorderable}
          subjectKind={subjectKind}
          targetIdOf={targetIdOf}
          ownsValue={ownsValue}
          isPinnedValue={isPinnedValue}
          onDrop={onDrop}
        />
      ))}
    </>
  );
}

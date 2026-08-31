<script lang="ts">
  import type {
    DragDropCommitResult,
    DropIntent,
    KeyboardDropTargetHandle,
  } from "@inflatable-cookie/poodle-core";

  import { useDragDrop } from "../drag-drop";
  import type { TabItem } from "../types";

  /**
   * The ordered logical target registry Alt+Arrow moves through.
   *
   * Tabs keeps its established one-keystroke reorder, so it never enters
   * pickup mode — Space and Enter stay selection keys. The move still runs as
   * an ordinary keyboard session through `requestKeyboardDrop`, which is what
   * keeps announcements, revalidation, and the terminal identical to a
   * pointer drop.
   */
  interface Props {
    items: TabItem[];
    reorderable: boolean;
    subjectKind: string;
    targetIdOf: (value: string) => string;
    ownsValue: (value: string) => boolean;
    onDrop: (intent: DropIntent) => DragDropCommitResult;
  }

  let { items, reorderable, subjectKind, targetIdOf, ownsValue, onDrop }: Props = $props();

  const { keyboardDropTarget } = useDragDrop();

  $effect(() => {
    const handles: KeyboardDropTargetHandle[] = items.map((item, index) =>
      keyboardDropTarget({
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
          return subject.id === item.value
            ? { accepted: false, reason: "same tab" }
            : { accepted: true, intent };
        },
        onDrop,
      }),
    );
    return () => {
      for (const handle of handles) handle.unregister();
    };
  });
</script>

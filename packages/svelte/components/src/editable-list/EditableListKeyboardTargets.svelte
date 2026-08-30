<script lang="ts" generics="T extends { id: string; label?: string }">
  import type { DragDropCommitResult, DropIntent, KeyboardDropTargetHandle } from "@inflatable-cookie/poodle-core";

  import { useDragDrop } from "../drag-drop";

  interface Props {
    items: T[];
    reorderable: boolean;
    isUnavailable: boolean;
    onDrop: (intent: DropIntent) => DragDropCommitResult;
  }

  let { items, reorderable, isUnavailable, onDrop }: Props = $props();

  const { keyboardDropTarget } = useDragDrop();
  const canDrag = $derived(reorderable && !isUnavailable);

  $effect(() => {
    const handles: KeyboardDropTargetHandle[] = items.map((item, index) =>
      keyboardDropTarget({
        targetId: item.id,
        acceptedKinds: ["poodle.editable-list"],
        disabled: !canDrag,
        label: item.label ?? item.id,
        order: index,
        resolvePosition: (input) =>
          input.direction === "previous" || input.direction === "first" ? "before" : "after",
        canDrop: (intent, subject) =>
          subject.id === intent.targetId ? { accepted: false, reason: "self" } : { accepted: true, intent },
        onDrop,
      }),
    );
    return () => {
      for (const handle of handles) handle.unregister();
    };
  });
</script>

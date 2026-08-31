<script lang="ts">
  import type { DragDropCommitResult, DropIntent } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { useDragDrop } from "../src/drag-drop-context";

  /**
   * A minimal composite target, standing in for what DockRegion is.
   *
   * It accepts the shared family and wraps a Tabs strip, so a foreign subject
   * that the strip's own targets refuse has somewhere eligible to land.
   */
  interface Props {
    kind: string;
    onDropped: (intent: DropIntent) => void;
    children: Snippet;
  }

  let { kind, onDropped, children }: Props = $props();

  const { dropTarget } = useDragDrop();
</script>

<div
  data-testid="composite"
  use:dropTarget={{
    targetId: "composite",
    acceptedKinds: [kind],
    label: "Composite",
    priority: -1,
    resolvePosition: () => "inside",
    canDrop: (intent) => ({ accepted: true, intent }),
    onDrop: (intent): DragDropCommitResult => {
      onDropped(intent);
      return { status: "committed" };
    },
  }}
>
  {@render children()}
</div>

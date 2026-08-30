<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/drag-drop.css";
  import {
    createDragDropController,
    type DragAnnouncementEvent,
    type DragDropController,
    type DragDropSnapshot,
    type DragPreviewSnapshot,
  } from "@inflatable-cookie/poodle-core";
  import { onMount, untrack, type Snippet } from "svelte";

  import { setDragDrop } from "./drag-drop-context";

  interface Props {
    controller?: DragDropController;
    describeAnnouncement?: (event: DragAnnouncementEvent) => string | null;
    preview?: Snippet<[DragPreviewSnapshot]>;
    children?: Snippet;
  }

  let { controller, describeAnnouncement, preview, children }: Props = $props();

  const owned = untrack(() => controller === undefined);
  const ctrl = untrack(() => controller ?? createDragDropController({ describeAnnouncement }));
  let root: HTMLDivElement | undefined;
  let snapshot: DragDropSnapshot = $state(ctrl.getSnapshot());

  setDragDrop({ controller: ctrl });

  onMount(() => {
    if (!root) return;
    const unsub = ctrl.subscribe(() => {
      snapshot = ctrl.getSnapshot();
    });
    const disconnect = ctrl.connect(root);
    return () => {
      unsub();
      disconnect();
      if (owned) ctrl.destroy();
    };
  });

  const previewStyle = $derived(
    snapshot.preview
      ? `left: ${snapshot.preview.x}px; top: ${snapshot.preview.y}px`
      : "",
  );
</script>

<div bind:this={root} class="poodle-drag-drop-provider">
  {@render children?.()}
  <div class="poodle-drag-overlay" aria-hidden="true">
    {#if snapshot.preview}
      <div class="poodle-drag-preview" style={previewStyle}>
        {#if preview}
          {@render preview(snapshot.preview)}
        {:else}
          {snapshot.preview.label}
        {/if}
      </div>
    {/if}
  </div>
  <div class="poodle-drag-live-region" aria-live="polite" aria-atomic="true">
    {snapshot.announcement ?? ""}
  </div>
</div>

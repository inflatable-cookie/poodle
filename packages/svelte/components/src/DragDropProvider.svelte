<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/drag-drop.css";
  import {
    createDragDropController,
    type DragAnnouncementEvent,
    type CrossWindowDragTargetBridge,
    type DragDropController,
    type DragDropSnapshot,
    type DragPreviewSnapshot,
    type InboundFileHostBridge,
  } from "@inflatable-cookie/poodle-core";
  import { onMount, untrack, type Snippet } from "svelte";

  import { setDragDrop } from "./drag-drop-context";

  interface Props {
    controller?: DragDropController;
    describeAnnouncement?: (event: DragAnnouncementEvent) => string | null;
    /**
     * Incoming cross-window host projection, commit, and accessible target
     * picking for this document. Ignored when an explicit `controller` is
     * supplied, because that controller already owns its own bridge.
     */
    crossWindowTargetBridge?: CrossWindowDragTargetBridge;
    /**
     * Inbound external files for this document. Ignored when an explicit
     * `controller` is supplied, because that controller already owns its own
     * bridge, and exclusive by construction: the bridge names the one
     * transport that owns inbound files here.
     */
    inboundFileBridge?: InboundFileHostBridge;
    preview?: Snippet<[DragPreviewSnapshot]>;
    children?: Snippet;
  }

  let {
    controller,
    describeAnnouncement,
    crossWindowTargetBridge,
    inboundFileBridge,
    preview,
    children,
  }: Props = $props();

  const owned = untrack(() => controller === undefined);
  const ctrl = untrack(
    () =>
      controller ??
      createDragDropController({
        describeAnnouncement,
        crossWindowTargetBridge,
        inboundFileBridge,
      }),
  );
  let root: HTMLDivElement | undefined;
  let previewEl: HTMLDivElement | undefined = $state();
  let snapshot: DragDropSnapshot = $state(ctrl.getSnapshot());
  let previewPose = $state(ctrl.getSnapshot().preview);
  let presentation = presentationKey(ctrl.getSnapshot());
  let lastPreviewPose = previewPoseKey(ctrl.getSnapshot().preview);

  setDragDrop({ controller: ctrl });

  function presentationKey(next: DragDropSnapshot): string {
    return [
      next.phase,
      next.sourceId ?? "",
      next.targetId ?? "",
      next.targetPosture ?? "",
      next.announcement ?? "",
      next.preview?.label ?? "",
      next.preview ? "1" : "0",
    ].join("|");
  }

  function previewPoseKey(preview: DragPreviewSnapshot | null): string {
    if (!preview) return "";
    return `${preview.x}|${preview.y}|${preview.label}|${preview.sourceId}`;
  }

  function applyPreview(next: DragDropSnapshot): void {
    if (!previewEl || !next.preview) return;
    previewEl.style.transform = `translate3d(${next.preview.x}px, ${next.preview.y}px, 0)`;
  }

  onMount(() => {
    if (!root) return;
    const unsub = ctrl.subscribe(() => {
      const next = ctrl.getSnapshot();
      applyPreview(next);
      const pose = previewPoseKey(next.preview);
      if (pose !== lastPreviewPose) {
        lastPreviewPose = pose;
        previewPose = next.preview ? { ...next.preview } : null;
      }
      const key = presentationKey(next);
      if (key === presentation) return;
      presentation = key;
      snapshot = next;
    });
    const disconnect = ctrl.connect(root);
    return () => {
      unsub();
      disconnect();
      if (owned) ctrl.destroy();
    };
  });

  const previewStyle = $derived(
    previewPose ? `transform: translate3d(${previewPose.x}px, ${previewPose.y}px, 0)` : "",
  );
</script>

<div bind:this={root} class="poodle-drag-drop-provider">
  {@render children?.()}
  <div class="poodle-drag-overlay" aria-hidden="true">
    {#if previewPose}
      <div bind:this={previewEl} class="poodle-drag-preview" style={previewStyle}>
        {#if preview}
          {@render preview(previewPose)}
        {:else}
          {previewPose.label}
        {/if}
      </div>
    {/if}
  </div>
  <div class="poodle-drag-live-region" aria-live="polite" aria-atomic="true">
    {snapshot.announcement ?? ""}
  </div>
</div>

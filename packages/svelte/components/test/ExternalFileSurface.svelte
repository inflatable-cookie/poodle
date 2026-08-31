<script lang="ts">
  /**
   * A curated external-file surface: one drop zone for files arriving from
   * outside the application, one row that can be dragged out to it.
   *
   * This is what a consumer actually writes. It shows the states worth
   * showing — a zone that is being offered files, one refusing them with a
   * reason, and an export moving through preparing, armed, dragging, and its
   * ending — and nothing here has any idea what a path, a `File`, or a shell
   * is. Exhaustive transport and validation cases live in the controller
   * tests; this is the useful shape.
   */
  import {
    INBOUND_FILE_SUBJECT_KIND,
    type DragExportBridge,
    type DropCommitContext,
    type DropIntent,
    type InboundFileConstraints,
  } from "@inflatable-cookie/poodle-core";

  import { useDragDrop } from "../src/drag-drop";

  interface Props {
    exportBridge?: DragExportBridge;
    constraints?: InboundFileConstraints;
  }

  let { exportBridge, constraints = { accept: "audio/*", maxFiles: 4 } }: Props = $props();

  const { dragSource, dropTarget, snapshot } = useDragDrop();

  let accepted: string[] = $state([]);

  // Display names only. The receipts the host issued are opaque, and the
  // surface never asks for anything more than something to render.
  const offered = $derived(
    ($snapshot.inboundFiles?.files ?? [])
      .map((file) => file.name)
      .filter((name): name is string => name !== null),
  );
  const exportState = $derived($snapshot.fileExport?.state ?? "idle");
  const exportName = $derived($snapshot.fileExport?.displayName ?? "");

  function zoneRegistration() {
    return {
      targetId: "library",
      acceptedKinds: [INBOUND_FILE_SUBJECT_KIND],
      label: "Sample library",
      inboundFiles: constraints,
      resolvePosition: () => "inside" as const,
      canDrop: (intent: DropIntent) => ({ accepted: true as const, intent }),
      onDrop: (_intent: DropIntent, context: DropCommitContext) => {
        accepted = (context.inboundFiles?.files ?? [])
          .map((file) => file.name)
          .filter((name): name is string => name !== null);
        return { status: "committed" as const };
      },
    };
  }

  function clipRegistration() {
    return {
      sourceId: "clip-1",
      subject: { kind: "clip", id: "clip-1" },
      allowedOperations: ["copy"] as const,
      label: "Intro clip",
      fileExportBridge: exportBridge,
    };
  }
</script>

<section
  class="poodle-drag-drop-list"
  data-testid="library"
  use:dropTarget={zoneRegistration()}
>
  <h3>Sample library</h3>
  {#if $snapshot.targetPosture === "accepted"}
    <p data-testid="library-state">Drop {offered.length || "files"} here</p>
  {:else if $snapshot.targetPosture === "rejected"}
    <p data-testid="library-state">Cannot take these files: {$snapshot.rejectedReason}</p>
  {:else}
    <p data-testid="library-state">Drop audio files here</p>
  {/if}
  <ul data-testid="library-files">
    {#each accepted as name (name)}
      <li>{name}</li>
    {/each}
  </ul>
</section>

<div
  class="poodle-drag-drop-item"
  data-testid="clip"
  use:dragSource={clipRegistration()}
>
  Intro clip
  <span data-testid="export-state">{exportState}</span>
  {#if exportName}
    <span data-testid="export-name">{exportName}</span>
  {/if}
</div>

<div data-testid="announcement">{$snapshot.announcement ?? ""}</div>

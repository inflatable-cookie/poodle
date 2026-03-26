<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Button, Spinner, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  import type { MediaUploadWorkflowStep } from "./types";

  export let uploadStep: MediaUploadWorkflowStep = "checking";
  export let duplicateLabel: string | null = null;
  export let uploadProgress = 0;
  export let uploadError: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    uploadAnyway: void;
    selectDuplicate: void;
    clearUpload: void;
    selectUploaded: void;
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
</script>

{#if uploadStep === "checking"}
  <div class="media-upload-status-panel" data-size={resolvedSize} data-density={resolvedDensity}>
    <Spinner variant="grid" tone="accent" />
    <p>Checking for duplicates...</p>
  </div>
{:else if uploadStep === "duplicate"}
  <div class="media-upload-status-panel media-upload-status-panel--warning" data-size={resolvedSize} data-density={resolvedDensity}>
    <p>This file already exists.</p>
    {#if duplicateLabel}
      <strong>{duplicateLabel}</strong>
    {/if}
    <div class="media-upload-status-panel__actions">
      <Button variant="secondary" on:click={() => dispatch("uploadAnyway")}>Upload as new</Button>
      <Button variant="primary" on:click={() => dispatch("selectDuplicate")}>Use existing</Button>
    </div>
  </div>
{:else if uploadStep === "uploading"}
  <div class="media-upload-status-panel" data-size={resolvedSize} data-density={resolvedDensity}>
    <div class="media-upload-status-panel__progress">
      <div class="media-upload-status-panel__progress-fill" style="width: {uploadProgress}%"></div>
    </div>
    <p>Uploading... {uploadProgress.toFixed(0)}%</p>
  </div>
{:else if uploadStep === "finalising"}
  <div class="media-upload-status-panel" data-size={resolvedSize} data-density={resolvedDensity}>
    <Spinner variant="grid" tone="accent" />
    <p>Finalising...</p>
  </div>
{:else if uploadStep === "complete"}
  <div class="media-upload-status-panel media-upload-status-panel--success" data-size={resolvedSize} data-density={resolvedDensity}>
    <p>Upload complete.</p>
    <div class="media-upload-status-panel__actions">
      <Button variant="secondary" on:click={() => dispatch("clearUpload")}>Upload another</Button>
      <Button variant="primary" on:click={() => dispatch("selectUploaded")}>Use this media</Button>
    </div>
  </div>
{:else if uploadStep === "error"}
  <div class="media-upload-status-panel media-upload-status-panel--danger" data-size={resolvedSize} data-density={resolvedDensity}>
    <p>{uploadError || "Upload failed"}</p>
    <Button variant="secondary" on:click={() => dispatch("clearUpload")}>Try again</Button>
  </div>
{/if}

<style>
  .media-upload-status-panel {
    display: grid;
    justify-items: center;
    gap: var(--poodle-space-stack-sm);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    text-align: center;
  }

  .media-upload-status-panel p,
  .media-upload-status-panel strong {
    margin: 0;
  }

  .media-upload-status-panel p {
    color: var(--poodle-color-text-secondary);
  }

  .media-upload-status-panel--warning {
    color: var(--poodle-color-warning-base);
  }

  .media-upload-status-panel--success {
    color: var(--poodle-color-success-base);
  }

  .media-upload-status-panel--danger {
    color: var(--poodle-color-danger-base);
  }

  .media-upload-status-panel__actions {
    display: flex;
    justify-content: center;
    gap: var(--poodle-space-inline-sm);
    flex-wrap: wrap;
  }

  .media-upload-status-panel__progress {
    width: min(14rem, 100%);
    height: 0.375rem;
    border-radius: 999px;
    overflow: hidden;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 82%, transparent);
  }

  .media-upload-status-panel__progress-fill {
    height: 100%;
    background: var(--poodle-color-accent-default);
    transition: width 0.1s ease-out;
  }

  /* ── Size variants ──────────────────────────────────────────── */

  .media-upload-status-panel[data-size="xs"] {
    font-size: 0.6875rem;
  }

  .media-upload-status-panel[data-size="xs"] .media-upload-status-panel__progress {
    height: 0.25rem;
    width: min(11rem, 100%);
  }

  .media-upload-status-panel[data-size="sm"] {
    font-size: 0.75rem;
  }

  .media-upload-status-panel[data-size="sm"] .media-upload-status-panel__progress {
    height: 0.3125rem;
    width: min(12.5rem, 100%);
  }

  .media-upload-status-panel[data-size="lg"] {
    font-size: 0.875rem;
  }

  .media-upload-status-panel[data-size="lg"] .media-upload-status-panel__progress {
    height: 0.4375rem;
    width: min(15.5rem, 100%);
  }

  .media-upload-status-panel[data-size="xl"] {
    font-size: 0.9375rem;
  }

  .media-upload-status-panel[data-size="xl"] .media-upload-status-panel__progress {
    height: 0.5rem;
    width: min(17rem, 100%);
  }

  /* Density variants */
  .media-upload-status-panel[data-density="compact"] { padding: 0.5rem; }
  .media-upload-status-panel[data-density="comfortable"] { padding: 1rem; }
</style>

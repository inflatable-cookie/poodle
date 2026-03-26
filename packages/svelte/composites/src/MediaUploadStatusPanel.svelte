<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Button, Spinner } from "@poodle/svelte-primitives";

  import type { MediaUploadWorkflowStep } from "./types";

  export let uploadStep: MediaUploadWorkflowStep = "checking";
  export let duplicateLabel: string | null = null;
  export let uploadProgress = 0;
  export let uploadError: string | null = null;

  const dispatch = createEventDispatcher<{
    uploadAnyway: void;
    selectDuplicate: void;
    clearUpload: void;
    selectUploaded: void;
  }>();
</script>

{#if uploadStep === "checking"}
  <div class="media-upload-status-panel">
    <Spinner variant="grid" size="md" tone="accent" />
    <p>Checking for duplicates...</p>
  </div>
{:else if uploadStep === "duplicate"}
  <div class="media-upload-status-panel media-upload-status-panel--warning">
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
  <div class="media-upload-status-panel">
    <div class="media-upload-status-panel__progress">
      <div class="media-upload-status-panel__progress-fill" style="width: {uploadProgress}%"></div>
    </div>
    <p>Uploading... {uploadProgress.toFixed(0)}%</p>
  </div>
{:else if uploadStep === "finalising"}
  <div class="media-upload-status-panel">
    <Spinner variant="grid" size="md" tone="accent" />
    <p>Finalising...</p>
  </div>
{:else if uploadStep === "complete"}
  <div class="media-upload-status-panel media-upload-status-panel--success">
    <p>Upload complete.</p>
    <div class="media-upload-status-panel__actions">
      <Button variant="secondary" on:click={() => dispatch("clearUpload")}>Upload another</Button>
      <Button variant="primary" on:click={() => dispatch("selectUploaded")}>Use this media</Button>
    </div>
  </div>
{:else if uploadStep === "error"}
  <div class="media-upload-status-panel media-upload-status-panel--danger">
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
</style>

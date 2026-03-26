<script lang="ts">
  import { Eyebrow } from "@poodle/svelte-primitives";
  import { MediaUploadStatusPanel } from "@poodle/svelte-composites";
  import type { MediaUploadWorkflowStep } from "@poodle/svelte-composites";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let uploadStep: MediaUploadWorkflowStep = "checking";
  let lastAction = "None";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Upload status panel</Eyebrow>
    <div class="specimen__actions">
      <button type="button" on:click={() => (uploadStep = "checking")}>Checking</button>
      <button type="button" on:click={() => (uploadStep = "duplicate")}>Duplicate</button>
      <button type="button" on:click={() => (uploadStep = "uploading")}>Uploading</button>
      <button type="button" on:click={() => (uploadStep = "finalising")}>Finalising</button>
      <button type="button" on:click={() => (uploadStep = "complete")}>Complete</button>
      <button type="button" on:click={() => (uploadStep = "error")}>Error</button>
    </div>
    <MediaUploadStatusPanel
      {uploadStep}
      duplicateLabel="hero-banner.jpg"
      uploadProgress={62}
      uploadError="Upload failed because the file could not be finalised."
      on:uploadAnyway={() => (lastAction = "Upload anyway")}
      on:selectDuplicate={() => (lastAction = "Select duplicate")}
      on:clearUpload={() => (lastAction = "Clear upload")}
      on:selectUploaded={() => (lastAction = "Select uploaded")}
    />
    <p>Last action: <strong>{lastAction}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <MediaUploadStatusPanel
          uploadStep="uploading"
          uploadProgress={62}
          {size}
        />
      {/each}
    </div>
  </div>
</div>

<style>
  .specimen,
  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .specimen__actions button {
    padding: 0.375rem 0.625rem;
  }

  p {
    margin: 0;
  }
</style>

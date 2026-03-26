<script lang="ts">
  import { DEFAULT_COMPRESSION, FileUpload, Eyebrow } from "@poodle/svelte-primitives";
  import type { FileUploadItem } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let imageFiles: FileUploadItem[] = [];
  let docFiles: FileUploadItem[] = [];
  let compressedFiles: FileUploadItem[] = [];
  let errorMsg = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Image upload with preview</Eyebrow>
    <FileUpload
      accept="image/*"
      multiple
      maxFiles={5}
      maxSize={5 * 1024 * 1024}
      bind:files={imageFiles}
      on:error={(e) => (errorMsg = e.detail.message)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Document upload (single file)</Eyebrow>
    <FileUpload
      accept=".pdf,.doc,.docx,.txt"
      maxSize={10 * 1024 * 1024}
      showPreview={false}
      bind:files={docFiles}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Compressed image upload with custom validation</Eyebrow>
    <FileUpload
      accept="image/*"
      multiple
      compress
      compressionOptions={{ ...DEFAULT_COMPRESSION, maxWidth: 1200, maxHeight: 800, quality: 0.8 }}
      bind:files={compressedFiles}
      validate={(file) => (file.name.includes(" ") ? "Filename cannot contain spaces" : null)}
      on:error={(e) => (errorMsg = e.detail.message)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <FileUpload accept="image/*" {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <FileUpload disabled />
  </div>

  {#if errorMsg}
    <div class="specimen__group">
      <Eyebrow>Last error</Eyebrow>
      <p class="error">{errorMsg}</p>
    </div>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .error {
    margin: 0;
    color: var(--poodle-color-text-danger, #ef4444);
    font-size: 0.8125rem;
  }
</style>

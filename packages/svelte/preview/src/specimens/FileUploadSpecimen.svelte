<script lang="ts">
  import { DEFAULT_COMPRESSION, FileUpload } from "@poodle/svelte";
  import type { FileUploadItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let imageFiles: FileUploadItem[] = [];
  let docFiles: FileUploadItem[] = [];
  let compressedFiles: FileUploadItem[] = [];
  let errorMsg = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Image upload with preview">
    <FileUpload
      accept="image/*"
      multiple
      maxFiles={5}
      maxSize={5 * 1024 * 1024}
      bind:files={imageFiles}
      on:error={(e) => (errorMsg = e.detail.message)}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Document upload (single file)">
    <FileUpload
      accept=".pdf,.doc,.docx,.txt"
      maxSize={10 * 1024 * 1024}
      showPreview={false}
      bind:files={docFiles}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Compressed image upload with custom validation">
    <FileUpload
      accept="image/*"
      multiple
      compress
      compressionOptions={{ ...DEFAULT_COMPRESSION, maxWidth: 1200, maxHeight: 800, quality: 0.8 }}
      bind:files={compressedFiles}
      validate={(file) => (file.name.includes(" ") ? "Filename cannot contain spaces" : null)}
      on:error={(e) => (errorMsg = e.detail.message)}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <FileUpload disabled />
  </SpecimenGroup>

  {#if errorMsg}
    <SpecimenGroup label="Last error">
      <p class="error">{errorMsg}</p>
    </SpecimenGroup>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .error {
    margin: 0;
    color: var(--poodle-color-text-danger, #ef4444);
    font-size: 0.8125rem;
  }
</style>

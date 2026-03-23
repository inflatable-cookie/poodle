<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";

  import type { FileUploadItem } from "./types";

  export let accept: string | null = null;
  export let maxSize: number = 10 * 1024 * 1024;
  export let multiple = false;
  export let maxFiles = 10;
  export let showPreview = true;
  export let disabled = false;
  export let files: FileUploadItem[] = [];

  const dispatch = createEventDispatcher<{
    change: { files: FileUploadItem[] };
    error: { file: File; message: string };
    remove: { item: FileUploadItem };
  }>();

  let inputElement: HTMLInputElement | null = null;
  let dragActive = false;
  let nextId = 1;

  function generateId(): string {
    return `file-${nextId++}`;
  }

  function validateFile(file: File): string | null {
    if (file.size > maxSize) {
      const sizeMB = (maxSize / (1024 * 1024)).toFixed(1);
      return `File exceeds maximum size of ${sizeMB} MB`;
    }

    if (accept) {
      const acceptedTypes = accept.split(",").map((t) => t.trim());
      const matches = acceptedTypes.some((type) => {
        if (type.startsWith(".")) {
          return file.name.toLowerCase().endsWith(type.toLowerCase());
        }

        if (type.endsWith("/*")) {
          return file.type.startsWith(type.replace("/*", "/"));
        }

        return file.type === type;
      });

      if (!matches) {
        return `File type "${file.type || "unknown"}" is not accepted`;
      }
    }

    return null;
  }

  function createPreviewUrl(file: File): string | null {
    if (!showPreview || !file.type.startsWith("image/")) {
      return null;
    }

    return URL.createObjectURL(file);
  }

  function addFiles(newFiles: FileList | File[]): void {
    const fileArray = Array.from(newFiles);

    for (const file of fileArray) {
      if (!multiple && files.length >= 1) {
        break;
      }

      if (multiple && files.length >= maxFiles) {
        dispatch("error", { file, message: `Maximum of ${maxFiles} files allowed` });
        break;
      }

      const error = validateFile(file);

      if (error) {
        dispatch("error", { file, message: error });
        continue;
      }

      const item: FileUploadItem = {
        file,
        id: generateId(),
        progress: 0,
        status: "pending",
        previewUrl: createPreviewUrl(file),
      };

      files = [...files, item];
    }

    dispatch("change", { files });
  }

  function removeFile(id: string): void {
    const item = files.find((f) => f.id === id);

    if (item) {
      if (item.previewUrl) {
        URL.revokeObjectURL(item.previewUrl);
      }

      files = files.filter((f) => f.id !== id);
      dispatch("remove", { item });
      dispatch("change", { files });
    }
  }

  export function updateProgress(id: string, progress: number): void {
    files = files.map((f) =>
      f.id === id
        ? { ...f, progress: Math.min(100, Math.max(0, progress)), status: progress >= 100 ? "complete" : "uploading" as const }
        : f,
    );
  }

  export function setError(id: string, message: string): void {
    files = files.map((f) =>
      f.id === id ? { ...f, status: "error" as const, error: message } : f,
    );
  }

  export function clear(): void {
    for (const f of files) {
      if (f.previewUrl) {
        URL.revokeObjectURL(f.previewUrl);
      }
    }

    files = [];
    dispatch("change", { files });
  }

  function handleDrop(event: DragEvent): void {
    event.preventDefault();
    dragActive = false;

    if (disabled || !event.dataTransfer?.files.length) {
      return;
    }

    addFiles(event.dataTransfer.files);
  }

  function handleDragOver(event: DragEvent): void {
    event.preventDefault();

    if (!disabled) {
      dragActive = true;
    }
  }

  function handleDragLeave(): void {
    dragActive = false;
  }

  function handleInputChange(event: Event): void {
    const target = event.target as HTMLInputElement;

    if (target.files?.length) {
      addFiles(target.files);
      target.value = "";
    }
  }

  function handleClick(): void {
    if (!disabled) {
      inputElement?.click();
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if ((event.key === "Enter" || event.key === " ") && !disabled) {
      event.preventDefault();
      inputElement?.click();
    }
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  onDestroy(() => {
    for (const f of files) {
      if (f.previewUrl) {
        URL.revokeObjectURL(f.previewUrl);
      }
    }
  });
</script>

<div class="file-upload" class:file-upload--disabled={disabled}>
  <div
    class="file-upload__dropzone"
    class:file-upload__dropzone--active={dragActive}
    role="button"
    tabindex={disabled ? -1 : 0}
    aria-label={multiple ? "Drop files here or click to browse" : "Drop a file here or click to browse"}
    on:drop={handleDrop}
    on:dragover={handleDragOver}
    on:dragleave={handleDragLeave}
    on:click={handleClick}
    on:keydown={handleKeydown}
  >
    <input
      bind:this={inputElement}
      type="file"
      accept={accept}
      {multiple}
      {disabled}
      class="file-upload__input"
      on:change={handleInputChange}
      tabindex="-1"
    />
    <div class="file-upload__dropzone-content">
      <svg class="file-upload__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M12 16V4m0 0L8 8m4-4l4 4" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M20 16v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <p class="file-upload__label">
        {#if dragActive}
          Drop to upload
        {:else}
          Drop files here or <span class="file-upload__browse">browse</span>
        {/if}
      </p>
      {#if accept || maxSize}
        <p class="file-upload__hint">
          {#if accept}{accept}{/if}
          {#if accept && maxSize} · {/if}
          {#if maxSize}Max {formatSize(maxSize)}{/if}
        </p>
      {/if}
    </div>
  </div>

  {#if files.length > 0}
    <ul class="file-upload__list" role="list">
      {#each files as item (item.id)}
        <li class="file-upload__item" class:file-upload__item--error={item.status === "error"}>
          {#if item.previewUrl}
            <img class="file-upload__preview" src={item.previewUrl} alt="" />
          {:else}
            <div class="file-upload__file-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M14 2v6h6" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
            </div>
          {/if}

          <div class="file-upload__meta">
            <span class="file-upload__name">{item.file.name}</span>
            <span class="file-upload__size">
              {formatSize(item.file.size)}
              {#if item.status === "error" && item.error}
                · <span class="file-upload__error-text">{item.error}</span>
              {:else if item.status === "uploading"}
                · {item.progress}%
              {:else if item.status === "complete"}
                · Complete
              {/if}
            </span>
          </div>

          {#if item.status === "uploading"}
            <div class="file-upload__progress">
              <div class="file-upload__progress-bar" style="width: {item.progress}%"></div>
            </div>
          {/if}

          <button
            type="button"
            class="file-upload__remove"
            aria-label="Remove {item.file.name}"
            on:click|stopPropagation={() => removeFile(item.id)}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" />
            </svg>
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .file-upload {
    display: flex;
    flex-direction: column;
    gap: var(--poodle-space-stack-sm, 0.5rem);
  }

  .file-upload--disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .file-upload__dropzone {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 8rem;
    padding: var(--poodle-space-panel-y, 1.5rem) var(--poodle-space-panel-x, 1.5rem);
    border: 0.125rem dashed var(--poodle-color-border-default, #444);
    border-radius: var(--poodle-radius-surface, 0.5rem);
    background: transparent;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
  }

  .file-upload__dropzone:hover,
  .file-upload__dropzone:focus-visible {
    border-color: var(--poodle-color-border-focus, #888);
    background: color-mix(in srgb, var(--poodle-color-background-panel, #1a1a1a) 50%, transparent);
    outline: none;
  }

  .file-upload__dropzone--active {
    border-color: var(--poodle-color-accent-default, #6366f1);
    background: color-mix(in srgb, var(--poodle-color-accent-default, #6366f1) 8%, transparent);
  }

  .file-upload__input {
    position: absolute;
    width: 0;
    height: 0;
    overflow: hidden;
    opacity: 0;
  }

  .file-upload__dropzone-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.375rem;
    text-align: center;
  }

  .file-upload__icon {
    width: 2rem;
    height: 2rem;
    color: var(--poodle-color-text-secondary, #999);
  }

  .file-upload__label {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary, #999);
  }

  .file-upload__browse {
    color: var(--poodle-color-accent-default, #6366f1);
    text-decoration: underline;
  }

  .file-upload__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-tertiary, #666);
  }

  .file-upload__list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .file-upload__item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem 0.5rem;
    border-radius: var(--poodle-radius-control, 0.375rem);
    background: var(--poodle-color-background-panel, #1a1a1a);
  }

  .file-upload__item--error {
    background: color-mix(in srgb, var(--poodle-color-text-danger, #ef4444) 10%, var(--poodle-color-background-panel, #1a1a1a));
  }

  .file-upload__preview {
    width: 2rem;
    height: 2rem;
    border-radius: 0.25rem;
    object-fit: cover;
  }

  .file-upload__file-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    flex-shrink: 0;
  }

  .file-upload__file-icon svg {
    width: 1.25rem;
    height: 1.25rem;
    color: var(--poodle-color-text-tertiary, #666);
  }

  .file-upload__meta {
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
    flex: 1;
    min-width: 0;
  }

  .file-upload__name {
    font-size: 0.8125rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-upload__size {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary, #999);
  }

  .file-upload__error-text {
    color: var(--poodle-color-text-danger, #ef4444);
  }

  .file-upload__progress {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 0.125rem;
    border-radius: 0 0 var(--poodle-radius-control, 0.375rem) var(--poodle-radius-control, 0.375rem);
    background: var(--poodle-color-border-default, #444);
    overflow: hidden;
  }

  .file-upload__progress-bar {
    height: 100%;
    background: var(--poodle-color-accent-default, #6366f1);
    transition: width 0.2s;
  }

  .file-upload__remove {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: 0;
    border-radius: 0.25rem;
    background: transparent;
    color: var(--poodle-color-text-secondary, #999);
    cursor: pointer;
  }

  .file-upload__remove:hover {
    background: var(--poodle-color-background-elevated, #2a2a2a);
    color: var(--poodle-color-text-default, #eee);
  }

  .file-upload__remove svg {
    width: 0.875rem;
    height: 0.875rem;
  }
</style>

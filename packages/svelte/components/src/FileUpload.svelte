<script lang="ts">
  import "@poodle/styles/file-upload.css";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";

  import type { ControlDensity, ControlSize, FileUploadItem, SemanticControlSizeRole } from "./types.ts";

  import {
    DEFAULT_COMPRESSION,
    compressImage,
    formatFileSize,
    generateFileUploadId,
    validateUploadFile,
    type FileUploadValidationError,
    type ImageCompressionOptions,
  } from "./file-upload.ts";

  let {
    size = null,
    sizeRole = "control",
    density = null,
    accept = null,
    maxSize = 10 * 1024 * 1024,
    multiple = false,
    maxFiles = 10,
    showPreview = true,
    disabled = false,
    files = $bindable([]),
    validate = undefined,
    compress = false,
    compressionOptions = DEFAULT_COMPRESSION,
    onChange = undefined,
    onUpload = undefined,
    onError = undefined,
    onRemove = undefined,
  }: {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    accept?: string | null;
    maxSize?: number;
    multiple?: boolean;
    maxFiles?: number;
    showPreview?: boolean;
    disabled?: boolean;
    files?: FileUploadItem[];
    validate?: ((file: File) => string | null) | undefined;
    compress?: boolean;
    compressionOptions?: ImageCompressionOptions;
    onChange?: ((files: FileUploadItem[]) => void) | undefined;
    onUpload?: ((files: File[]) => void) | undefined;
    onError?: ((event: FileUploadValidationError) => void) | undefined;
    onRemove?: ((item: FileUploadItem) => void) | undefined;
  } = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  let inputElement: HTMLInputElement | null = null;
  let dragActive = $state(false);

  function createPreviewUrl(file: File): string | null {
    if (!showPreview || !file.type.startsWith("image/")) {
      return null;
    }

    return URL.createObjectURL(file);
  }

  async function addFiles(newFiles: FileList | File[]): Promise<void> {
    const fileArray = Array.from(newFiles);
    const filesToUpload: File[] = [];

    for (const file of fileArray) {
      if (!multiple && files.length >= 1) {
        break;
      }

      if (multiple && files.length >= maxFiles) {
        const message = `Maximum of ${maxFiles} files allowed`;
        onError?.({ file, message });
        break;
      }

      const error = validateUploadFile({
        file,
        maxSize,
        accept: accept ?? "*",
        validate,
      });

      if (error) {
        onError?.({ file, message: error });
        continue;
      }

      let processedFile = file;
      let originalFile: File | undefined;

      if (compress && file.type.startsWith("image/")) {
        const compressed = await compressImage(file, compressionOptions);
        if (compressed !== file) {
          originalFile = file;
          processedFile = compressed;
        }
      }

      const item: FileUploadItem = {
        file: processedFile,
        id: generateFileUploadId(),
        progress: 0,
        status: "pending",
        previewUrl: createPreviewUrl(processedFile),
        originalFile,
      };

      files = [...files, item];
      filesToUpload.push(processedFile);
    }

    onChange?.(files);

    if (filesToUpload.length > 0) {
      onUpload?.(filesToUpload);
    }
  }

  function removeFile(id: string): void {
    const item = files.find((f) => f.id === id);

    if (!item) {
      return;
    }

    if (item.previewUrl) {
      URL.revokeObjectURL(item.previewUrl);
    }

    files = files.filter((f) => f.id !== id);
    onRemove?.(item);
    onChange?.(files);
  }

  export function updateProgress(id: string, progress: number): void {
    files = files.map((f) =>
      f.id === id
        ? {
            ...f,
            progress: Math.min(100, Math.max(0, progress)),
            status: progress >= 100 ? "complete" : ("uploading" as const),
          }
        : f,
    );
    onChange?.(files);
  }

  export function setError(id: string, message: string): void {
    files = files.map((f) =>
      f.id === id ? { ...f, status: "error" as const, error: message } : f,
    );
    onChange?.(files);
  }

  export function clear(): void {
    for (const f of files) {
      if (f.previewUrl) {
        URL.revokeObjectURL(f.previewUrl);
      }
    }

    files = [];
    onChange?.(files);
  }

  function handleDrop(event: DragEvent): void {
    event.preventDefault();
    dragActive = false;

    if (disabled || !event.dataTransfer?.files.length) {
      return;
    }

    void addFiles(event.dataTransfer.files);
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
      void addFiles(target.files);
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

  $effect(() => {
    return () => {
    for (const f of files) {
      if (f.previewUrl) {
        URL.revokeObjectURL(f.previewUrl);
      }
    }
    };
  });
</script>

<div class="poodle-file-upload" class:poodle-file-upload--disabled={disabled} data-size={resolvedSize} data-density={resolvedDensity}>
  <!-- The focusable dropzone is a composite around the native file input, not
       a second control; Enter and Space delegate activation to that input. -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="poodle-file-upload__dropzone"
    class:poodle-file-upload__dropzone--active={dragActive}
    role="group"
    tabindex={disabled ? -1 : 0}
    aria-disabled={disabled}
    aria-label="File upload dropzone"
    ondrop={handleDrop}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    onclick={handleClick}
    onkeydown={handleKeydown}
  >
    <input
      bind:this={inputElement}
      type="file"
      accept={accept}
      {multiple}
      {disabled}
      class="poodle-file-upload__input"
      onchange={handleInputChange}
      aria-label={multiple ? "Drop files here or click to browse" : "Drop a file here or click to browse"}
    />
    <div class="poodle-file-upload__dropzone-content">
      <svg class="poodle-file-upload__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M12 16V4m0 0L8 8m4-4l4 4" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M20 16v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <p class="poodle-file-upload__label">
        {#if dragActive}
          Drop to upload
        {:else}
          Drop files here or <span class="poodle-file-upload__browse">browse</span>
        {/if}
      </p>
      {#if accept || maxSize}
        <p class="poodle-file-upload__hint">
          {#if accept}{accept}{/if}
          {#if accept && maxSize} · {/if}
          {#if maxSize}Max {formatFileSize(maxSize)}{/if}
        </p>
      {/if}
    </div>
  </div>

  {#if files.length > 0}
    <ul class="poodle-file-upload__list" role="list">
      {#each files as item (item.id)}
        <li class="poodle-file-upload__item" class:poodle-file-upload__item--error={item.status === "error"}>
          {#if item.previewUrl}
            <img class="poodle-file-upload__preview" src={item.previewUrl} alt="" />
          {:else}
            <div class="poodle-file-upload__file-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M14 2v6h6" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
            </div>
          {/if}

          <div class="poodle-file-upload__meta">
            <span class="poodle-file-upload__name">{item.file.name}</span>
            <span class="poodle-file-upload__size">
              {formatFileSize(item.file.size)}
              {#if item.status === "error" && item.error}
                · <span class="poodle-file-upload__error-text">{item.error}</span>
              {:else if item.status === "uploading"}
                · {item.progress}%
              {:else if item.status === "complete"}
                · Complete
              {/if}
            </span>
          </div>

          {#if item.status === "uploading"}
            <div class="poodle-file-upload__progress">
              <div class="poodle-file-upload__progress-bar" style="width: {item.progress}%"></div>
            </div>
          {/if}

          <button
            type="button"
            class="poodle-file-upload__remove"
            aria-label="Remove {item.file.name}"
            onclick={(event) => {
              event.stopPropagation();
              removeFile(item.id);
            }}
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

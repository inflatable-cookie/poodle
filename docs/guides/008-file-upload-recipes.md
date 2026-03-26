# 008 File Upload Recipes

Use Poodle `FileUpload` for generic file selection, preview, client-side
validation, and optional raster-image compression. Keep duplicate checks,
upload queues, signed-upload handshakes, and backend progress orchestration
owned by the host app.

## Default Boundary

- `FileUpload` owns local file intake and validation.
- Apps own network upload lifecycle and duplicate detection.
- Higher-order media-library workflows should not be pushed into `FileUpload`.

## Generic Upload Intake

```svelte
<script lang="ts">
  import { FileUpload, type FileUploadItem } from "@poodle/svelte-primitives";

  let files: FileUploadItem[] = [];

  function handleUpload(event: CustomEvent<{ files: File[] }>) {
    const selectedFiles = event.detail.files;
    // start app-owned upload orchestration here
  }
</script>

<FileUpload
  accept="image/*,.pdf"
  multiple
  bind:files
  on:upload={handleUpload}
/>
```

## Compression And Validation

```svelte
<script lang="ts">
  import { DEFAULT_COMPRESSION, FileUpload } from "@poodle/svelte-primitives";
</script>

<FileUpload
  accept="image/*"
  compress
  compressionOptions={{ ...DEFAULT_COMPRESSION, maxWidth: 1200, maxHeight: 800, quality: 0.8 }}
  validate={(file) => (file.name.includes(" ") ? "Filename cannot contain spaces" : null)}
/>
```

## App-Owned Upload Queue

- derive queue state in the route or form
- use `on:upload` or `onUpload` to start queue work
- use `bind:files` for current file state
- use component methods like `updateProgress` and `setError` only if the app is
  managing per-item upload state inside the same file list

## Keep Out Of Poodle

- duplicate-check APIs
- media-record creation callbacks
- signed-upload handshakes
- upload finalisation workflows
- library browse pagination

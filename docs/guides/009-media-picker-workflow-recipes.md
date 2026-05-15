# 009 Media Picker Workflow Recipes

Use Poodle `MediaPicker` when the host app already has a local item array and
only needs a lightweight browse/search/upload-tab selector. Do not force
callback-driven media-library orchestration into the Poodle composite.

## Default Boundary

- Poodle `MediaPicker` owns local search and tab posture.
- Poodle `MediaBrowsePanel` and `MediaUploadStatusPanel` can be reused by
  heavier workflow shells.
- Apps own item loading, pagination, duplicate checks, and upload orchestration.
- If the picker itself must coordinate backend callbacks and upload lifecycle,
  keep that as app/shared workflow code rather than inflating the Poodle
  contract.

## Use Poodle MediaPicker

```svelte
<script lang="ts">
  import { MediaPicker, type MediaPickerItem } from "@poodle/svelte";

  let open = false;
  let items: MediaPickerItem[] = [];
</script>

<MediaPicker
  open={open}
  onOpenChange={(nextOpen) => (open = nextOpen)}
  {items}
  onSelect={(item) => {
    // app-owned selection handling
  }}
  onUpload={(files) => {
    // app-owned upload queue handling
  }}
/>
```

## Keep Out Of Poodle MediaPicker

- paginated browse callbacks
- duplicate-detection APIs
- create/initiate/finalise upload callbacks
- media-record creation policy

## Retained Workflow Shells

If a shared repo still needs a callback-driven media-library picker, keep it as
the thinnest possible workflow shell over:

- Poodle `MediaBrowsePanel`
- Poodle `MediaUploadStatusPanel`
- Poodle `FileUpload`
- app/shared media commands
- app/shared upload-flow state

Do not treat the lightweight Poodle `MediaPicker` as a one-for-one replacement
until those callback-driven behaviors become generic enough to standardize.

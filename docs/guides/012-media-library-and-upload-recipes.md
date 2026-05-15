# Media Library And Upload Recipes

Reusable media-library UI composition rules for Poodle-based admin apps.

## Purpose

Use this guide when you need to build media browse, preview, picker, and upload
surfaces without recreating a second media workflow kit above Poodle.

## Default Posture

- use Poodle `MediaPicker` for lightweight local selection
- use Poodle `MediaBrowsePanel` and `MediaUploadStatusPanel` for heavier
  callback-driven media-library shells
- use Poodle `MediaThumbnail` and `MediaPreview` for display posture
- keep upload orchestration, duplicate detection, and media-record policy in
  host code or retained runtime helpers

## Lightweight Selector

Use `MediaPicker` when the host already owns the local items and upload queue.

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
    // host-owned selection handling
  }}
  onUpload={(files) => {
    // host-owned queue handling
  }}
/>
```

## Browse And Upload Shell

When the flow needs paginated browse and upload status together, compose
directly over Poodle media composites instead of inventing a new generic media
wrapper.

```svelte
<script lang="ts">
  import {
    MediaBrowsePanel,
    MediaUploadStatusPanel,
    type MediaBrowseItem
  } from "@poodle/svelte";

  let items: MediaBrowseItem[] = [];
  let uploads = [];
</script>

<div class="media-library-shell">
  <MediaBrowsePanel
    title="Media"
    items={items}
    state="ready"
    query={query}
    onSelect={(item) => {
      // host-owned selection
    }}
  />

  <MediaUploadStatusPanel
    title="Uploads"
    uploads={uploads}
    onUploadAnyway={(upload) => {
      // host-owned retry
    }}
  />
</div>
```

## Display Posture

Use `MediaThumbnail` and `MediaPreview` directly for read-only media framing.

```svelte
<script lang="ts">
  import { MediaPreview, MediaThumbnail } from "@poodle/svelte";
</script>

<MediaThumbnail src={media.thumbnailUrl} alt={media.title} aspectRatio="landscape" />
<MediaPreview kind={media.kind} src={media.url} title={media.title} />
```

## Media Detail Pattern

Use this when the app needs a full media detail page with metadata, version
history, preview, and usage tabs.

```svelte
<PageHeader title={media.title} backHref="/media" backLabel="Back to media">
  {#snippet actions()}
    <MediaActionsMenu {media} />
  {/snippet}
</PageHeader>

<MetaBar ariaLabel="Media metadata">
  <MetaItem label="ID">
    <Code inline source={media.id} showCopyButton />
  </MetaItem>
  <Pill tone="neutral" appearance="badge" size="lg">{media.kindLabel}</Pill>
  <Pill tone="neutral" appearance="badge" size="lg">{media.visibilityLabel}</Pill>
</MetaBar>

<Tabs
  value={activeTab}
  items={[
    { value: "details", label: "Details" },
    { value: "preview", label: "Preview" },
    { value: "usage", label: "Usage", count: media.usageCount }
  ]}
  variant="card"
  size="sm"
  historyKey="tab"
  ariaLabel="Media sections"
/>

<Card>
  <DetailSection title="File Details" columns={2} separated={false}>
    <DetailItem presentation="surface" label="Original Filename" value={media.originalFilename} />
  </DetailSection>
</Card>
```

Rules for this pattern:

- keep one top-level `PageHeader` and one `MetaBar` above the tabs
- use `Tabs variant="card"` with `historyKey="tab"` for stable navigation
- render the details tab as `Card` + `DetailSection` + `DetailItem`, not as a
  second nested page shell
- use `InlineListSection` for compact versions and usage sections under their
  tabs
- keep media lifecycle actions, version activation, and destructive policy in
  host code even when the visible shell is shared

## What Stays Out

- duplicate-detection APIs
- create/initiate/finalize upload commands
- media-record creation and versioning policy
- media-usage tracking
- app-specific destructive wording and permission checks

Those remain host-owned unless a genuinely generic workflow seam emerges across
multiple apps.

## Decision

- keep media UI composition Poodle-first
- keep media orchestration host-owned
- only add Poodle capability when multiple apps need the same generic media
  interaction, not when one app wants a convenience wrapper

## Related Contracts

- [MediaPicker](../contracts/components/media-picker.md)
- [MediaBrowsePanel](../contracts/components/media-browse-panel.md)
- [MediaUploadStatusPanel](../contracts/components/media-upload-status-panel.md)
- [MediaPreview](../contracts/components/media-preview.md)
- [MediaThumbnail](../contracts/components/media-thumbnail.md)

## Next Task

Add the next media recipe only when a genuinely generic media workflow seam is
proven across multiple apps, instead of recreating app-specific library logic
inside Poodle.

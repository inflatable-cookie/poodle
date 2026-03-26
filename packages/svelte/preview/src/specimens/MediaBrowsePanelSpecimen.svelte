<script lang="ts">
  import { Eyebrow, UiPresentationProvider } from "@poodle/svelte-primitives";
  import { MediaBrowsePanel, type MediaPickerItem } from "@poodle/svelte-composites";

  let loading = false;
  let error: string | null = null;
  let items: MediaPickerItem[] = [
    { id: "1", label: "Hero banner", kind: "image", meta: "Image" },
    { id: "2", label: "Launch trailer", kind: "video", meta: "Video" },
    { id: "3", label: "Podcast intro", kind: "audio", meta: "Audio" },
    { id: "4", label: "Quarterly report", kind: "document", meta: "Document" }
  ];
  let lastAction = "None";

  function reset(): void {
    loading = false;
    error = null;
    items = [
      { id: "1", label: "Hero banner", kind: "image", meta: "Image" },
      { id: "2", label: "Launch trailer", kind: "video", meta: "Video" },
      { id: "3", label: "Podcast intro", kind: "audio", meta: "Audio" },
      { id: "4", label: "Quarterly report", kind: "document", meta: "Document" }
    ];
    lastAction = "Reset";
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Browse panel</Eyebrow>
    <div class="specimen__actions">
      <button type="button" on:click={() => { loading = true; error = null; lastAction = "Loading"; }}>Loading</button>
      <button type="button" on:click={() => { loading = false; error = "Failed to load media"; lastAction = "Error"; }}>Error</button>
      <button type="button" on:click={() => { loading = false; error = null; items = []; lastAction = "Empty"; }}>Empty</button>
      <button type="button" on:click={reset}>Reset</button>
    </div>
    <MediaBrowsePanel
      {loading}
      {error}
      {items}
      hasMore={items.length > 0}
      on:select={(event) => (lastAction = `Selected ${event.detail.item.label}`)}
      on:loadMore={() => (lastAction = "Load more")}
    />
    <p>Last action: <strong>{lastAction}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Semantic presentation</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <MediaBrowsePanel {items} hasMore />
        <MediaBrowsePanel {items} hasMore sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </div>
</div>

<style>
  .specimen,
  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .specimen__stack {
    display: grid;
    gap: 0.75rem;
  }

  .specimen__actions button {
    padding: 0.375rem 0.625rem;
  }

  p {
    margin: 0;
  }
</style>

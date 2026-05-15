<script lang="ts">
  import { UiPresentationProvider } from "@poodle/svelte";
  import { MediaBrowsePanel, type MediaPickerItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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

<div class="poodle-specimen">
  <SpecimenGroup label="Browse panel">
    <div class="poodle-specimen__actions">
      <button type="button" onclick={() => { loading = true; error = null; lastAction = "Loading"; }}>Loading</button>
      <button type="button" onclick={() => { loading = false; error = "Failed to load media"; lastAction = "Error"; }}>Error</button>
      <button type="button" onclick={() => { loading = false; error = null; items = []; lastAction = "Empty"; }}>Empty</button>
      <button type="button" onclick={reset}>Reset</button>
    </div>
    <MediaBrowsePanel
      {loading}
      {error}
      {items}
      hasMore={items.length > 0}
      onSelect={(item) => (lastAction = `Selected ${item.label}`)}
      onLoadMore={() => (lastAction = "Load more")}
    />
    <p>Last action: <strong>{lastAction}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Semantic presentation">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="poodle-specimen__stack">
        <MediaBrowsePanel {items} hasMore />
        <MediaBrowsePanel {items} hasMore sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen,
  .poodle-specimen__actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .poodle-specimen__stack {
    display: grid;
    gap: 0.75rem;
  }

  .poodle-specimen__actions button {
    padding: 0.375rem 0.625rem;
  }

  p {
    margin: 0;
  }
</style>

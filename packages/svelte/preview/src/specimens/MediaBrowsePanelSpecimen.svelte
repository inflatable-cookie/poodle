<script lang="ts">
  import { Button, MediaBrowsePanel, type MediaPickerItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let loading = $state(false);
  let error = $state<string | null>(null);
  let items = $state<MediaPickerItem[]>([
    { id: "1", label: "Hero banner", kind: "image", meta: "Image" },
    { id: "2", label: "Launch trailer", kind: "video", meta: "Video" },
    { id: "3", label: "Podcast intro", kind: "audio", meta: "Audio" },
    { id: "4", label: "Quarterly report", kind: "document", meta: "Document" }
  ]);
  let lastAction = $state("None");

  const baseItems: MediaPickerItem[] = [
    { id: "1", label: "Hero banner", kind: "image", meta: "Image" },
    { id: "2", label: "Launch trailer", kind: "video", meta: "Video" },
    { id: "3", label: "Podcast intro", kind: "audio", meta: "Audio" },
    { id: "4", label: "Quarterly report", kind: "document", meta: "Document" }
  ];

  function reset(): void {
    loading = false;
    error = null;
    items = [...baseItems];
    lastAction = "Reset";
  }
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Browse panel" bare>
        <div class="poodle-specimen__actions">
          <Button variant="secondary" sizeRole="chrome" onClick={() => { loading = true; error = null; lastAction = "Loading"; }}>
            Loading
          </Button>
          <Button variant="secondary" sizeRole="chrome" onClick={() => { loading = false; error = "Failed to load media"; lastAction = "Error"; }}>
            Error
          </Button>
          <Button variant="secondary" sizeRole="chrome" onClick={() => { loading = false; error = null; items = []; lastAction = "Empty"; }}>
            Empty
          </Button>
          <Button variant="secondary" sizeRole="chrome" onClick={reset}>
            Reset
          </Button>
        </div>
        <div class="poodle-media-browse-panel-specimen__variant">
          <MediaBrowsePanel
            {loading}
            {error}
            {items}
            hasMore={items.length > 0}
            onSelect={(item) => (lastAction = `Selected ${item.label}`)}
            onLoadMore={() => (lastAction = "Load more")}
          />
        </div>
        <p>Last action: <strong>{lastAction}</strong></p>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-media-browse-panel-specimen__variant">
      <MediaBrowsePanel
        items={baseItems}
        hasMore
        {size}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-media-browse-panel-specimen__variant">
      <MediaBrowsePanel
        items={baseItems}
        hasMore
        {density}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .poodle-media-browse-panel-specimen__variant {
    width: min(100%, 40rem);
  }
  p {
    margin: 0;
  }
</style>

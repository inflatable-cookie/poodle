<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Button, Callout } from "@poodle/svelte-primitives";

  import MediaThumbnail from "./MediaThumbnail.svelte";
  import type { MediaKind, MediaPickerItem } from "./types";

  export let loading = false;
  export let error: string | null = null;
  export let items: MediaPickerItem[] = [];
  export let hasMore = false;
  export let emptyMessage = "No media found";
  export let loadMoreLabel = "Load more";

  const dispatch = createEventDispatcher<{
    select: { item: MediaPickerItem };
    loadMore: void;
  }>();

  function toMediaKind(kind?: MediaKind): MediaKind {
    return kind ?? "image";
  }
</script>

<div class="media-browse-panel">
  {#if loading && items.length === 0}
    <div class="media-browse-panel__state">
      <p>Loading media...</p>
    </div>
  {:else if error}
    <Callout tone="danger" message={error} announceMode="polite" />
  {:else if items.length === 0}
    <div class="media-browse-panel__state">
      <p>{emptyMessage}</p>
    </div>
  {:else}
    <div class="media-browse-panel__grid">
      {#each items as item (item.id)}
        <button
          type="button"
          class="media-browse-panel__item"
          on:click={() => dispatch("select", { item })}
        >
          <MediaThumbnail
            kind={toMediaKind(item.kind)}
            presentation="compact"
            aspectRatio="square"
            ariaLabel={item.label}
          >
            {#if item.thumbnailUrl}
              <img
                src={item.thumbnailUrl}
                alt={item.label}
                class="media-browse-panel__image"
              />
            {/if}
          </MediaThumbnail>
          <span class="media-browse-panel__label">{item.label}</span>
          {#if item.meta}
            <span class="media-browse-panel__meta">{item.meta}</span>
          {/if}
        </button>
      {/each}
    </div>

    {#if hasMore}
      <div class="media-browse-panel__actions">
        <Button variant="secondary" on:click={() => dispatch("loadMore")} disabled={loading}>
          {loading ? "Loading..." : loadMoreLabel}
        </Button>
      </div>
    {/if}
  {/if}
</div>

<style>
  .media-browse-panel {
    min-height: 18rem;
  }

  .media-browse-panel__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(11rem, 1fr));
    gap: var(--poodle-space-stack-sm);
    margin-top: var(--poodle-space-stack-sm);
  }

  .media-browse-panel__item {
    display: grid;
    gap: 0.375rem;
    padding: 0.75rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
    color: inherit;
    text-align: left;
    cursor: pointer;
  }

  .media-browse-panel__item:hover,
  .media-browse-panel__item:focus-visible {
    border-color: var(--poodle-color-border-focus);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 90%, transparent);
    outline: none;
  }

  .media-browse-panel__image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .media-browse-panel__label {
    font-size: 0.875rem;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .media-browse-panel__meta,
  .media-browse-panel__state p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .media-browse-panel__actions {
    display: flex;
    justify-content: center;
    margin-top: var(--poodle-space-stack-sm);
  }

  .media-browse-panel__state {
    display: grid;
    place-items: center;
    min-height: 18rem;
    text-align: center;
  }
</style>

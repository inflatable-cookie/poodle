<script lang="ts">
  import { default as Button } from "./Button.svelte";
  import { default as Callout } from "./Callout.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  import { default as MediaThumbnail } from "./MediaThumbnail.svelte";
  import type { MediaKind, MediaPickerItem } from "./types";

  interface Props {
    loading?: boolean;
    error?: string | null;
    items?: MediaPickerItem[];
    hasMore?: boolean;
    emptyMessage?: string;
    loadMoreLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onSelect?: ((item: MediaPickerItem) => void) | undefined;
    onLoadMore?: (() => void) | undefined;
  }

  let {
    loading = false,
    error = null,
    items = [],
    hasMore = false,
    emptyMessage = "No media found",
    loadMoreLabel = "Load more",
    size = null,
    sizeRole = "control",
    density = null,
    onSelect = undefined,
    onLoadMore = undefined,
  }: Props = $props();
  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  function toMediaKind(kind?: MediaKind): MediaKind {
    return kind ?? "image";
  }
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div class="poodle-media-browse-panel" data-size={resolvedSize} data-density={resolvedDensity}>
    {#if loading && items.length === 0}
      <div class="poodle-media-browse-panel__state">
        <p>Loading media...</p>
      </div>
    {:else if error}
      <Callout tone="danger" message={error} announceMode="polite" />
    {:else if items.length === 0}
      <div class="poodle-media-browse-panel__state">
        <p>{emptyMessage}</p>
      </div>
    {:else}
      <div class="poodle-media-browse-panel__grid">
        {#each items as item (item.id)}
          <button
            type="button"
            class="poodle-media-browse-panel__item"
            onclick={() => onSelect?.(item)}
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
                  class="poodle-media-browse-panel__image"
                />
              {/if}
            </MediaThumbnail>
            <span class="poodle-media-browse-panel__label">{item.label}</span>
            {#if item.meta}
              <span class="poodle-media-browse-panel__meta">{item.meta}</span>
            {/if}
          </button>
        {/each}
      </div>

      {#if hasMore}
        <div class="poodle-media-browse-panel__actions">
          <Button variant="secondary" size={resolvedSize} onClick={() => onLoadMore?.()} disabled={loading}>
            {loading ? "Loading..." : loadMoreLabel}
          </Button>
        </div>
      {/if}
    {/if}
  </div>
</UiPresentationProvider>

<style>
  .poodle-media-browse-panel {
    --poodle-media-browse-grid-gap: var(--poodle-space-stack-sm);
    --poodle-media-browse-item-gap: 0.375rem;
    --poodle-media-browse-item-pad: 0.75rem;
    --poodle-media-browse-grid-min: 11rem;
    min-height: 18rem;
  }

  .poodle-media-browse-panel[data-size="xs"] {
    --poodle-media-browse-grid-min: 8.5rem;
  }

  .poodle-media-browse-panel[data-size="sm"] {
    --poodle-media-browse-grid-min: 10rem;
  }

  .poodle-media-browse-panel[data-size="lg"] {
    --poodle-media-browse-grid-min: 12rem;
  }

  .poodle-media-browse-panel[data-size="xl"] {
    --poodle-media-browse-grid-min: 13rem;
  }

  .poodle-media-browse-panel[data-density="compact"] {
    --poodle-media-browse-grid-gap: 0.375rem;
    --poodle-media-browse-item-gap: 0.25rem;
    --poodle-media-browse-item-pad: 0.5rem;
  }

  .poodle-media-browse-panel[data-density="comfortable"] {
    --poodle-media-browse-grid-gap: 0.75rem;
    --poodle-media-browse-item-gap: 0.5rem;
    --poodle-media-browse-item-pad: 0.875rem;
  }

  .poodle-media-browse-panel__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--poodle-media-browse-grid-min), 1fr));
    gap: var(--poodle-media-browse-grid-gap);
    margin-top: var(--poodle-media-browse-grid-gap);
  }

  .poodle-media-browse-panel__item {
    display: grid;
    gap: var(--poodle-media-browse-item-gap);
    padding: var(--poodle-media-browse-item-pad);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
    color: inherit;
    text-align: left;
    cursor: pointer;
  }

  .poodle-media-browse-panel__item:hover,
  .poodle-media-browse-panel__item:focus-visible {
    border-color: var(--poodle-color-border-focus);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 90%, transparent);
    outline: none;
  }

  .poodle-media-browse-panel__image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .poodle-media-browse-panel__label {
    font-size: var(--poodle-typography-body-size);
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-media-browse-panel__meta,
  .poodle-media-browse-panel__state p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .poodle-media-browse-panel__actions {
    display: flex;
    justify-content: center;
    margin-top: var(--poodle-media-browse-grid-gap);
  }

  .poodle-media-browse-panel__state {
    display: grid;
    place-items: center;
    min-height: 18rem;
    text-align: center;
  }
</style>

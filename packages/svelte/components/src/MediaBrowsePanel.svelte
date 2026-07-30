<script lang="ts">
  import "@poodle/styles/media-browse-panel.css";
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

  function toMediaKind(kind: MediaKind | undefined): MediaKind {
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
            <!--
              Two branches rather than one with an `{#if}` inside: MediaThumbnail
              falls back to the kind placeholder icon only when no slot content
              is supplied at all, and a slot that is present but renders nothing
              suppressed that placeholder.
            -->
            {#if item.thumbnailUrl}
              <MediaThumbnail
                kind={toMediaKind(item.kind)}
                presentation="compact"
                aspectRatio="square"
                ariaLabel={item.label}
              >
                <img
                  src={item.thumbnailUrl}
                  alt={item.label}
                  class="poodle-media-browse-panel__image"
                />
              </MediaThumbnail>
            {:else}
              <MediaThumbnail
                kind={toMediaKind(item.kind)}
                presentation="compact"
                aspectRatio="square"
                ariaLabel={item.label}
              />
            {/if}
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

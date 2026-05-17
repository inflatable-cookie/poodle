<script lang="ts">
  import type { Snippet } from "svelte";
  import { default as Card } from "./Card.svelte";
  import { default as MediaThumbnail } from "./MediaThumbnail.svelte";

  import type { AspectRatio, CardVariant, MediaKind, MediaState } from "./types";

  interface Props {
    title: string;
    description?: string | null;
    eyebrow?: string | null;
    caption?: string | null;
    meta?: string[];
    badge?: string | null;
    thumbnailMeta?: string | null;
    kind?: MediaKind;
    state?: MediaState;
    aspectRatio?: AspectRatio;
    variant?: CardVariant;
    ariaLabel?: string | null;
    stateTitle?: string | null;
    stateMessage?: string | null;
    mediaContent?: Snippet;
    children?: Snippet;
  }

  let {
    title,
    description = null,
    eyebrow = null,
    caption = null,
    meta = [],
    badge = null,
    thumbnailMeta = null,
    kind = "image",
    state = "ready",
    aspectRatio = "landscape",
    variant = "default",
    ariaLabel = null,
    stateTitle = null,
    stateMessage = null,
    mediaContent,
    children,
  }: Props = $props();
</script>

<Card {variant} media={true} ariaLabel={ariaLabel ?? title}>
  {#snippet mediaContent()}
    <MediaThumbnail
      {kind}
      {state}
      {aspectRatio}
      title={null}
      badge={badge}
      meta={null}
      ariaLabel={title}
      stateTitle={stateTitle}
      stateMessage={stateMessage}
    >
      {@render mediaContent?.()}
    </MediaThumbnail>
  {/snippet}

  {#snippet header()}
    <div class="poodle-media-preview__header">
      <div class="poodle-media-preview__heading">
        {#if eyebrow}
          <p class="poodle-media-preview__eyebrow">{eyebrow}</p>
        {/if}
        <h3>{title}</h3>
        {#if description}
          <p class="poodle-media-preview__description">{description}</p>
        {/if}
      </div>

      {#if thumbnailMeta || meta.length > 0}
        <ul class="poodle-media-preview__meta" aria-label="preview metadata">
          {#if thumbnailMeta}
            <li>{thumbnailMeta}</li>
          {/if}
          {#each meta as item}
            <li>{item}</li>
          {/each}
        </ul>
      {/if}
    </div>
  {/snippet}

  <div class="poodle-media-preview__body">
    {#if caption}
      <p class="poodle-media-preview__caption">{caption}</p>
    {/if}
    {@render children?.()}
  </div>
</Card>

<style>
  .poodle-media-preview__header,
  .poodle-media-preview__heading,
  .poodle-media-preview__body {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-media-preview__eyebrow,
  .poodle-media-preview__description,
  .poodle-media-preview__caption {
    margin: 0;
  }

  .poodle-media-preview__eyebrow {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .poodle-media-preview__header h3 {
    margin: 0;
    font-size: 1.25rem;
    line-height: 1.2;
  }

  .poodle-media-preview__header {
    gap: var(--poodle-space-stack-md);
  }

  .poodle-media-preview__description,
  .poodle-media-preview__caption,
  .poodle-media-preview__meta {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-media-preview__meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .poodle-media-preview__meta li {
    padding: 0.375rem 0.625rem;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 70%, transparent);
  }

  :global([data-theme="light"]) .poodle-media-preview__meta li {
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent);
  }
</style>

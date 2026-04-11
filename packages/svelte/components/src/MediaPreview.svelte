<script lang="ts">
  import Card from "./Card.svelte";
  import MediaThumbnail from "./MediaThumbnail.svelte";

  import type { AspectRatio, CardVariant, MediaKind, MediaState } from "./types";

  export let title: string;
  export let description: string | null = null;
  export let eyebrow: string | null = null;
  export let caption: string | null = null;
  export let meta: string[] = [];
  export let badge: string | null = null;
  export let thumbnailMeta: string | null = null;
  export let kind: MediaKind = "image";
  export let state: MediaState = "ready";
  export let aspectRatio: AspectRatio = "landscape";
  export let variant: CardVariant = "default";
  export let ariaLabel: string | null = null;
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;
</script>

<Card {variant} media={true} ariaLabel={ariaLabel ?? title}>
  <div slot="media">
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
      <slot name="media" />
    </MediaThumbnail>
  </div>

  <div slot="header" class="media-preview__header">
    <div class="media-preview__heading">
      {#if eyebrow}
        <p class="media-preview__eyebrow">{eyebrow}</p>
      {/if}
      <h3>{title}</h3>
      {#if description}
        <p class="media-preview__description">{description}</p>
      {/if}
    </div>

    {#if thumbnailMeta || meta.length > 0}
      <ul class="media-preview__meta" aria-label="preview metadata">
        {#if thumbnailMeta}
          <li>{thumbnailMeta}</li>
        {/if}
        {#each meta as item}
          <li>{item}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="media-preview__body">
    {#if caption}
      <p class="media-preview__caption">{caption}</p>
    {/if}
    <slot />
  </div>

</Card>

<style>
  .media-preview__header,
  .media-preview__heading,
  .media-preview__body {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .media-preview__eyebrow,
  .media-preview__description,
  .media-preview__caption {
    margin: 0;
  }

  .media-preview__eyebrow {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .media-preview__header h3 {
    margin: 0;
    font-size: 1.25rem;
    line-height: 1.2;
  }

  .media-preview__header {
    gap: var(--poodle-space-stack-md);
  }

  .media-preview__description,
  .media-preview__caption,
  .media-preview__meta {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .media-preview__meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .media-preview__meta li {
    padding: 0.375rem 0.625rem;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 70%, transparent);
  }

  :global([data-theme="light"]) .media-preview__meta li {
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent);
  }
</style>

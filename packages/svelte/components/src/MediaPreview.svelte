<script lang="ts">
  import type { Snippet } from "svelte";
  import { default as Card } from "./Card.svelte";
  import { default as MediaThumbnail } from "./MediaThumbnail.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { AspectRatio, CardVariant, ControlDensity, ControlSize, MediaKind, MediaState, SemanticControlSizeRole } from "./types";

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
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
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
    size = null,
    sizeRole = "control",
    density = null,
    mediaContent,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div class="poodle-media-preview" data-size={resolvedSize} data-density={resolvedDensity}>
    <Card {variant} media={true} ariaLabel={ariaLabel ?? title} density={resolvedDensity}>
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
  </div>
</UiPresentationProvider>

<style>
  .poodle-media-preview {
    --poodle-media-preview-header-gap: var(--poodle-space-stack-md);
    --poodle-media-preview-section-gap: var(--poodle-space-stack-sm);
    --poodle-media-preview-eyebrow-size: 0.6875rem;
    --poodle-media-preview-title-size: 1.125rem;
    --poodle-media-preview-title-line-height: 1.2;
    --poodle-media-preview-body-size: 0.8125rem;
    --poodle-media-preview-meta-padding-y: 0.375rem;
    --poodle-media-preview-meta-padding-x: 0.625rem;
  }

  .poodle-media-preview__header,
  .poodle-media-preview__heading,
  .poodle-media-preview__body {
    display: grid;
    gap: var(--poodle-media-preview-section-gap);
  }

  .poodle-media-preview__eyebrow,
  .poodle-media-preview__description,
  .poodle-media-preview__caption {
    margin: 0;
  }

  .poodle-media-preview__eyebrow {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-media-preview-eyebrow-size);
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .poodle-media-preview__header h3 {
    margin: 0;
    font-size: var(--poodle-media-preview-title-size);
    line-height: var(--poodle-media-preview-title-line-height);
  }

  .poodle-media-preview__header {
    gap: var(--poodle-media-preview-header-gap);
  }

  .poodle-media-preview__description,
  .poodle-media-preview__caption,
  .poodle-media-preview__meta {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-media-preview-body-size);
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
    padding: var(--poodle-media-preview-meta-padding-y) var(--poodle-media-preview-meta-padding-x);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 70%, transparent);
  }

  .poodle-media-preview[data-size="xs"] {
    --poodle-media-preview-eyebrow-size: 0.625rem;
    --poodle-media-preview-title-size: 0.9375rem;
    --poodle-media-preview-body-size: 0.75rem;
    --poodle-media-preview-meta-padding-y: 0.25rem;
    --poodle-media-preview-meta-padding-x: 0.5rem;
  }

  .poodle-media-preview[data-size="sm"] {
    --poodle-media-preview-eyebrow-size: 0.65625rem;
    --poodle-media-preview-title-size: 1rem;
    --poodle-media-preview-body-size: 0.78125rem;
    --poodle-media-preview-meta-padding-y: 0.3125rem;
    --poodle-media-preview-meta-padding-x: 0.5625rem;
  }

  .poodle-media-preview[data-size="md"] {
    --poodle-media-preview-eyebrow-size: 0.6875rem;
    --poodle-media-preview-title-size: 1.125rem;
    --poodle-media-preview-body-size: 0.8125rem;
    --poodle-media-preview-meta-padding-y: 0.375rem;
    --poodle-media-preview-meta-padding-x: 0.625rem;
  }

  .poodle-media-preview[data-size="lg"] {
    --poodle-media-preview-eyebrow-size: 0.71875rem;
    --poodle-media-preview-title-size: 1.1875rem;
    --poodle-media-preview-body-size: 0.875rem;
    --poodle-media-preview-meta-padding-y: 0.4375rem;
    --poodle-media-preview-meta-padding-x: 0.6875rem;
  }

  .poodle-media-preview[data-size="xl"] {
    --poodle-media-preview-eyebrow-size: 0.75rem;
    --poodle-media-preview-title-size: 1.25rem;
    --poodle-media-preview-body-size: 0.9375rem;
    --poodle-media-preview-meta-padding-y: 0.5rem;
    --poodle-media-preview-meta-padding-x: 0.75rem;
  }

  .poodle-media-preview[data-density="compact"] {
    --poodle-media-preview-header-gap: 0.625rem;
    --poodle-media-preview-section-gap: 0.375rem;
  }

  .poodle-media-preview[data-density="comfortable"] {
    --poodle-media-preview-header-gap: 1rem;
    --poodle-media-preview-section-gap: 0.625rem;
  }

  :global([data-theme="light"]) .poodle-media-preview__meta li {
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent);
  }
</style>

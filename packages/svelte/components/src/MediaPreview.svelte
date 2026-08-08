<script lang="ts">
  import "@inflatable-cookie/poodle-styles/media-preview.css";
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

  // Aliased because the template declares a `mediaContent` snippet for Card,
  // which shadows this prop. Passed straight through as MediaThumbnail's
  // `children` so that an absent snippet stays undefined — wrapping it in a
  // snippet that renders nothing would suppress MediaThumbnail's empty-state
  // placeholder, which the contract requires.
  const mediaSnippet = $derived(mediaContent);
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
          children={mediaSnippet}
        />
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


<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { default as Spinner } from "./Spinner.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type { AspectRatio, MediaKind, MediaState } from "./types";

  interface Props {
    kind?: MediaKind;
    state?: MediaState;
    aspectRatio?: AspectRatio;
    title?: string | null;
    badge?: string | null;
    meta?: string | null;
    ariaLabel?: string | null;
    stateTitle?: string | null;
    stateMessage?: string | null;
    presentation?: "default" | "compact";
    fit?: "cover" | "contain";
    frameWidth?: "fill" | "xl" | number | string | null;
    frameMinHeight?: number | string | null;
    frameMaxHeight?: number | string | null;
    children?: Snippet;
  }

  let {
    kind = "image",
    state = "ready",
    aspectRatio = "landscape",
    title = null,
    badge = null,
    meta = null,
    ariaLabel = null,
    stateTitle = null,
    stateMessage = null,
    presentation = "default",
    fit = "cover",
    frameWidth = "fill",
    frameMinHeight = null,
    frameMaxHeight = null,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedKind = $derived(kind === "pdf" || kind === "other" ? "document" : kind);
  const rootStyle = $derived(
    frameWidth === null || frameWidth === undefined || frameWidth === "fill"
      ? "inline-size: 100%;"
      : frameWidth === "xl"
        ? "inline-size: min(100%, 24rem);"
        : `inline-size: ${typeof frameWidth === "number" ? `${frameWidth}px` : frameWidth};`,
  );
  const frameStyle = $derived(
    [
      frameMinHeight === null || frameMinHeight === undefined
        ? null
        : `min-block-size: ${typeof frameMinHeight === "number" ? `${frameMinHeight}px` : frameMinHeight};`,
      frameMaxHeight === null || frameMaxHeight === undefined
        ? null
        : `max-block-size: ${typeof frameMaxHeight === "number" ? `${frameMaxHeight}px` : frameMaxHeight};`,
    ]
      .filter(Boolean)
      .join(" "),
  );
  const resolvedStateTitle = $derived(
    stateTitle ??
      (state === "loading"
        ? "Loading preview"
        : state === "error"
          ? "Preview unavailable"
          : "No preview"),
  );
  const fallbackIcon = $derived(
    resolvedKind === "audio"
      ? "music"
      : resolvedKind === "video"
        ? "play"
        : resolvedKind === "document"
          ? "file-text"
          : resolvedKind === "embed"
            ? "external-link"
            : "image",
  );
  const resolvedVisualSize = $derived(resolveSemanticControlSize($uiPresentation.sizeScale, "control"));
  const resolvedSupportingSize = $derived(resolveSupportingVisualSize(resolvedVisualSize));
  const resolvedSpinnerSize = $derived(
    presentation === "compact"
      ? resolveSupportingVisualSize(resolvedSupportingSize)
      : resolvedSupportingSize,
  );
</script>

<figure
  class="poodle-media-thumbnail"
  data-kind={resolvedKind}
  data-state={state}
  data-aspect-ratio={aspectRatio}
  data-presentation={presentation}
  data-fit={fit}
  aria-label={ariaLabel ?? title ?? undefined}
  aria-busy={state === "loading"}
  style={rootStyle}
>
  <div class="poodle-media-thumbnail__frame" style={frameStyle}>
    {#if state === "ready"}
      {#if children}
        {@render children()}
      {:else}
        <div class="poodle-media-thumbnail__placeholder" aria-hidden="true">
          <Icon name={fallbackIcon} size={resolvedVisualSize} />
        </div>
      {/if}

      {#if resolvedKind === "audio" || resolvedKind === "video"}
        <span class="poodle-media-thumbnail__play" aria-hidden="true">
          <Icon name={resolvedKind === "audio" ? "music" : "play"} size={resolvedSupportingSize} />
        </span>
      {/if}
    {:else}
      <div class="poodle-media-thumbnail__state">
        {#if state === "loading"}
          <span class="poodle-media-thumbnail__spinner" aria-hidden="true">
            <Spinner variant="grid" size={resolvedSpinnerSize} tone="accent" />
          </span>
        {/if}
        <strong>{resolvedStateTitle}</strong>
        {#if stateMessage && presentation !== "compact"}
          <p>{stateMessage}</p>
        {/if}
      </div>
    {/if}

    {#if badge}
      <span class="poodle-media-thumbnail__badge">{badge}</span>
    {/if}
  </div>

  {#if presentation !== "compact" && (title || meta)}
    <figcaption class="poodle-media-thumbnail__caption">
      {#if title}
        <strong>{title}</strong>
      {/if}
      {#if meta}
        <span>{meta}</span>
      {/if}
    </figcaption>
  {/if}
</figure>

<style>
  .poodle-media-thumbnail {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    margin: 0;
  }

  .poodle-media-thumbnail[data-presentation="compact"] {
    gap: 0;
  }

  .poodle-media-thumbnail__frame {
    position: relative;
    overflow: hidden;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: calc(var(--poodle-radius-surface) - 0.125rem);
    background:
      radial-gradient(circle at top left, color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent), transparent 38%),
      color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
  }

  .poodle-media-thumbnail__frame :global(img),
  .poodle-media-thumbnail__frame :global(svg) {
    display: block;
    width: 100%;
    height: 100%;
  }

  .poodle-media-thumbnail__frame :global(img) {
    object-fit: var(--poodle-media-thumbnail-object-fit, cover);
  }

  .poodle-media-thumbnail__frame :global(svg) {
    object-fit: contain;
  }

  .poodle-media-thumbnail[data-aspect-ratio="square"] .poodle-media-thumbnail__frame {
    aspect-ratio: 1 / 1;
  }

  .poodle-media-thumbnail[data-aspect-ratio="landscape"] .poodle-media-thumbnail__frame {
    aspect-ratio: 16 / 10;
  }

  .poodle-media-thumbnail[data-aspect-ratio="portrait"] .poodle-media-thumbnail__frame {
    aspect-ratio: 3 / 4;
  }

  .poodle-media-thumbnail[data-aspect-ratio="video"] .poodle-media-thumbnail__frame {
    aspect-ratio: 16 / 9;
  }

  .poodle-media-thumbnail[data-aspect-ratio="auto"] .poodle-media-thumbnail__frame {
    aspect-ratio: auto;
  }

  .poodle-media-thumbnail[data-fit="contain"] {
    --poodle-media-thumbnail-object-fit: contain;
  }

  .poodle-media-thumbnail__placeholder,
  .poodle-media-thumbnail__state {
    display: grid;
    width: 100%;
    height: 100%;
  }

  .poodle-media-thumbnail__placeholder {
    place-items: center;
    width: 100%;
    height: 100%;
    padding: 0.5rem var(--poodle-space-panel-x);
  }

  .poodle-media-thumbnail__state {
    gap: var(--poodle-space-stack-sm);
    align-content: end;
    justify-items: start;
    padding: 0.5rem var(--poodle-space-panel-x);
    text-align: left;
    background:
      linear-gradient(180deg, transparent, color-mix(in srgb, var(--poodle-color-background-panel) 46%, transparent)),
      color-mix(in srgb, var(--poodle-color-background-surface) 78%, transparent);
  }

  .poodle-media-thumbnail__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .poodle-media-thumbnail__state strong,
  .poodle-media-thumbnail__state p {
    margin: 0;
  }

  .poodle-media-thumbnail[data-presentation="compact"] .poodle-media-thumbnail__state {
    align-content: center;
    justify-items: center;
    padding: 0.875rem;
    text-align: center;
  }

  .poodle-media-thumbnail[data-presentation="compact"] .poodle-media-thumbnail__state strong {
    font-size: 0.875rem;
    line-height: 1.35;
  }

  .poodle-media-thumbnail__state p,
  .poodle-media-thumbnail__caption span {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-media-thumbnail__caption {
    display: grid;
    gap: 0.125rem;
  }

  .poodle-media-thumbnail__caption strong {
    font-size: 0.875rem;
    line-height: 1.35;
  }

  .poodle-media-thumbnail__badge,
  .poodle-media-thumbnail__play {
    position: absolute;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--poodle-radius-control);
    backdrop-filter: blur(1rem);
  }

  .poodle-media-thumbnail__badge {
    top: 0.625rem;
    right: 0.625rem;
    min-height: 1.5rem;
    padding: 0 0.625rem;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 74%, transparent);
    color: var(--poodle-color-text-primary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .poodle-media-thumbnail[data-presentation="compact"] .poodle-media-thumbnail__badge {
    top: 0.5rem;
    right: 0.5rem;
  }

  .poodle-media-thumbnail__play {
    left: 0.625rem;
    bottom: 0.625rem;
    width: 2rem;
    height: 2rem;
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 78%, transparent);
    color: var(--poodle-color-text-primary);
    font-size: 0.9375rem;
  }
</style>

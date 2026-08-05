<script lang="ts">
  import "@poodle/styles/media-thumbnail.css";
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { default as Spinner } from "./Spinner.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation.ts";

  import type { AspectRatio, MediaKind, MediaState } from "./types.ts";

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


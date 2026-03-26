<script lang="ts">
  import { Icon, Spinner } from "@poodle/svelte-primitives";
  import { getUiPresentation, resolveSemanticControlSize, resolveSupportingVisualSize } from "@poodle/svelte-primitives";

  import type { AspectRatio, MediaKind, MediaState } from "./types";

  export let kind: MediaKind = "image";
  export let state: MediaState = "ready";
  export let aspectRatio: AspectRatio = "landscape";
  export let title: string | null = null;
  export let badge: string | null = null;
  export let meta: string | null = null;
  export let ariaLabel: string | null = null;
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;
  export let presentation: "default" | "compact" = "default";

  const uiPresentation = getUiPresentation();

  $: resolvedStateTitle =
    stateTitle ??
    (state === "loading"
      ? "Loading preview"
      : state === "error"
        ? "Preview unavailable"
        : "No preview");
  $: fallbackIcon =
    kind === "audio" ? "music" : kind === "video" ? "play" : kind === "document" ? "file-text" : kind === "embed" ? "external-link" : "image";
  $: resolvedVisualSize = resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", "control");
  $: resolvedSupportingSize = resolveSupportingVisualSize(resolvedVisualSize);
  $: resolvedSpinnerSize =
    presentation === "compact" ? resolveSupportingVisualSize(resolvedSupportingSize) : resolvedSupportingSize;
</script>

<figure
  class="media-thumbnail"
  data-kind={kind}
  data-state={state}
  data-aspect-ratio={aspectRatio}
  data-presentation={presentation}
  aria-label={ariaLabel ?? title ?? undefined}
  aria-busy={state === "loading"}
>
  <div class="media-thumbnail__frame">
    {#if state === "ready"}
      {#if $$slots.default}
        <slot />
      {:else}
        <div class="media-thumbnail__placeholder" aria-hidden="true">
          <Icon name={fallbackIcon} size={resolvedVisualSize} />
        </div>
      {/if}

      {#if kind === "audio" || kind === "video"}
        <span class="media-thumbnail__play" aria-hidden="true">
          <Icon name={kind === "audio" ? "music" : "play"} size={resolvedSupportingSize} />
        </span>
      {/if}
    {:else}
      <div class="media-thumbnail__state">
        {#if state === "loading"}
          <span class="media-thumbnail__spinner" aria-hidden="true">
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
      <span class="media-thumbnail__badge">{badge}</span>
    {/if}
  </div>

  {#if presentation !== "compact" && (title || meta)}
    <figcaption class="media-thumbnail__caption">
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
  .media-thumbnail {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    margin: 0;
  }

  .media-thumbnail[data-presentation="compact"] {
    gap: 0;
  }

  .media-thumbnail__frame {
    position: relative;
    overflow: hidden;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: calc(var(--poodle-radius-surface) - 0.125rem);
    background:
      radial-gradient(circle at top left, color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent), transparent 38%),
      color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
  }

  .media-thumbnail[data-aspect-ratio="square"] .media-thumbnail__frame {
    aspect-ratio: 1 / 1;
  }

  .media-thumbnail[data-aspect-ratio="landscape"] .media-thumbnail__frame {
    aspect-ratio: 16 / 10;
  }

  .media-thumbnail[data-aspect-ratio="portrait"] .media-thumbnail__frame {
    aspect-ratio: 3 / 4;
  }

  .media-thumbnail[data-aspect-ratio="video"] .media-thumbnail__frame {
    aspect-ratio: 16 / 9;
  }

  .media-thumbnail__placeholder,
  .media-thumbnail__state {
    display: grid;
    width: 100%;
    height: 100%;
  }

  .media-thumbnail__placeholder {
    place-items: center;
    width: 100%;
    height: 100%;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
  }

  .media-thumbnail__state {
    gap: var(--poodle-space-stack-sm);
    align-content: end;
    justify-items: start;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    text-align: left;
    background:
      linear-gradient(180deg, transparent, color-mix(in srgb, var(--poodle-color-background-panel) 46%, transparent)),
      color-mix(in srgb, var(--poodle-color-background-surface) 78%, transparent);
  }

  .media-thumbnail__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .media-thumbnail__state strong,
  .media-thumbnail__state p {
    margin: 0;
  }

  .media-thumbnail[data-presentation="compact"] .media-thumbnail__state {
    align-content: center;
    justify-items: center;
    padding: 0.875rem;
    text-align: center;
  }

  .media-thumbnail[data-presentation="compact"] .media-thumbnail__state strong {
    font-size: 0.875rem;
    line-height: 1.35;
  }

  .media-thumbnail__state p,
  .media-thumbnail__caption span {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .media-thumbnail__caption {
    display: grid;
    gap: 0.125rem;
  }

  .media-thumbnail__caption strong {
    font-size: 0.875rem;
    line-height: 1.35;
  }

  .media-thumbnail__badge,
  .media-thumbnail__play {
    position: absolute;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--poodle-radius-control);
    backdrop-filter: blur(1rem);
  }

  .media-thumbnail__badge {
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

  .media-thumbnail[data-presentation="compact"] .media-thumbnail__badge {
    top: 0.5rem;
    right: 0.5rem;
  }

  .media-thumbnail__play {
    left: 0.625rem;
    bottom: 0.625rem;
    width: 2rem;
    height: 2rem;
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 78%, transparent);
    color: var(--poodle-color-text-primary);
    font-size: 0.9375rem;
  }
</style>

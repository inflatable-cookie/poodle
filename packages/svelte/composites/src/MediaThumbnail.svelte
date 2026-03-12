<script lang="ts">
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

  $: resolvedStateTitle =
    stateTitle ??
    (state === "loading"
      ? "Loading preview"
      : state === "error"
        ? "Preview unavailable"
        : "No preview");
  $: fallbackGlyph =
    kind === "audio" ? "♫" : kind === "video" ? "▶" : kind === "document" ? "▣" : kind === "embed" ? "⇱" : "▥";
</script>

<figure
  class="media-thumbnail"
  data-kind={kind}
  data-state={state}
  data-aspect-ratio={aspectRatio}
  aria-label={ariaLabel ?? title ?? undefined}
  aria-busy={state === "loading"}
>
  <div class="media-thumbnail__frame">
    {#if state === "ready"}
      {#if $$slots.default}
        <slot />
      {:else}
        <div class="media-thumbnail__placeholder" aria-hidden="true">
          <span>{fallbackGlyph}</span>
        </div>
      {/if}

      {#if kind === "audio" || kind === "video"}
        <span class="media-thumbnail__play" aria-hidden="true">
          {kind === "audio" ? "♫" : "▶"}
        </span>
      {/if}
    {:else}
      <div class="media-thumbnail__state">
        <strong>{resolvedStateTitle}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      </div>
    {/if}

    {#if badge}
      <span class="media-thumbnail__badge">{badge}</span>
    {/if}
  </div>

  {#if title || meta}
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
    gap: var(--pug-space-stack-sm);
    margin: 0;
  }

  .media-thumbnail__frame {
    position: relative;
    overflow: hidden;
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: calc(var(--pug-radius-surface) - 2px);
    background:
      radial-gradient(circle at top left, color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent), transparent 38%),
      color-mix(in srgb, var(--pug-color-background-panel) 94%, transparent);
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
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
  }

  .media-thumbnail__placeholder span {
    font-size: 28px;
    color: var(--pug-color-text-secondary);
  }

  .media-thumbnail__state {
    gap: var(--pug-space-stack-sm);
    align-content: end;
    justify-items: start;
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    text-align: left;
    background:
      linear-gradient(180deg, transparent, color-mix(in srgb, var(--pug-color-background-panel) 46%, transparent)),
      color-mix(in srgb, var(--pug-color-background-surface) 78%, transparent);
  }

  .media-thumbnail__state strong,
  .media-thumbnail__state p {
    margin: 0;
  }

  .media-thumbnail__state p,
  .media-thumbnail__caption span {
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 1.5;
  }

  .media-thumbnail__caption {
    display: grid;
    gap: 2px;
  }

  .media-thumbnail__caption strong {
    font-size: 14px;
    line-height: 1.35;
  }

  .media-thumbnail__badge,
  .media-thumbnail__play {
    position: absolute;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--pug-radius-control);
    backdrop-filter: blur(16px);
  }

  .media-thumbnail__badge {
    top: 10px;
    right: 10px;
    min-height: 24px;
    padding: 0 10px;
    background: color-mix(in srgb, var(--pug-color-background-surface) 74%, transparent);
    color: var(--pug-color-text-primary);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .media-thumbnail__play {
    left: 10px;
    bottom: 10px;
    width: 32px;
    height: 32px;
    background: color-mix(in srgb, var(--pug-color-background-elevated) 78%, transparent);
    color: var(--pug-color-text-primary);
    font-size: 15px;
  }
</style>

<script lang="ts">
  import { Skeleton } from "@pug/svelte-primitives";

  import type { ParsedEmbed } from "./types";

  export let parsed: ParsedEmbed | null = null;
  export let aspectRatio: number | "auto" = 16 / 9;
  export let loading = false;
  export let error: string | null = null;
  export let emptyMessage = "No embed to preview";

  $: embedUrl = parsed ? getEmbedUrl(parsed) : null;
  $: isAudio = parsed?.provider === "audioboom";
  $: effectiveAspectRatio = isAudio ? "auto" : aspectRatio;

  function getEmbedUrl(embed: ParsedEmbed): string | null {
    switch (embed.provider) {
      case "youtube":
        return `https://www.youtube-nocookie.com/embed/${embed.id}`;
      case "vimeo":
        return `https://player.vimeo.com/video/${embed.id}`;
      default:
        return embed.originalUrl ?? null;
    }
  }
</script>

<div class="embed-preview">
  {#if loading}
    <div class="embed-preview__loading">
      <Skeleton shape="block" />
      <span class="embed-preview__loading-text">Loading preview...</span>
    </div>
  {:else if error}
    <div class="embed-preview__error">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <circle cx="12" cy="12" r="10" />
        <path d="M12 8v4m0 4h.01" stroke-linecap="round" />
      </svg>
      <span>{error}</span>
    </div>
  {:else if !parsed}
    <div class="embed-preview__empty">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="2" y="4" width="20" height="16" rx="2" />
        <path d="M10 9l5 3-5 3V9z" />
      </svg>
      <span>{emptyMessage}</span>
    </div>
  {:else if embedUrl}
    <div
      class="embed-preview__container"
      style={effectiveAspectRatio !== "auto" ? `aspect-ratio: ${effectiveAspectRatio}` : ""}
    >
      <iframe
        src={embedUrl}
        title="{parsed.provider} embed"
        frameborder="0"
        allowfullscreen
        loading="lazy"
        sandbox="allow-scripts allow-same-origin allow-popups"
        class="embed-preview__iframe"
      ></iframe>
    </div>
  {:else if parsed.originalEmbed}
    <div
      class="embed-preview__container"
      style={effectiveAspectRatio !== "auto" ? `aspect-ratio: ${effectiveAspectRatio}` : ""}
    >
      {@html parsed.originalEmbed}
    </div>
  {:else}
    <div class="embed-preview__fallback">
      <a href={parsed.originalUrl} target="_blank" rel="noopener noreferrer">
        {parsed.originalUrl ?? parsed.id}
      </a>
    </div>
  {/if}
</div>

<style>
  .embed-preview {
    border-radius: var(--pug-radius-surface, 0.5rem);
    overflow: hidden;
  }

  .embed-preview__container {
    position: relative;
    width: 100%;
    background: var(--pug-color-background-panel, #1a1a1a);
  }

  .embed-preview__iframe {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border: 0;
  }

  .embed-preview__container:not([style*="aspect-ratio"]) .embed-preview__iframe {
    position: static;
    height: 10rem;
  }

  .embed-preview__loading,
  .embed-preview__error,
  .embed-preview__empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    min-height: 8rem;
    padding: 1.5rem;
    background: var(--pug-color-background-panel, #1a1a1a);
    border-radius: var(--pug-radius-surface, 0.5rem);
  }

  .embed-preview__error svg,
  .embed-preview__empty svg {
    width: 2rem;
    height: 2rem;
    color: var(--pug-color-text-tertiary, #666);
  }

  .embed-preview__loading-text {
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary, #999);
  }

  .embed-preview__error span,
  .embed-preview__empty span {
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary, #999);
  }

  .embed-preview__error svg {
    color: var(--pug-color-text-danger, #ef4444);
  }

  .embed-preview__fallback {
    padding: 0.75rem 1rem;
    background: var(--pug-color-background-panel, #1a1a1a);
    border-radius: var(--pug-radius-surface, 0.5rem);
  }

  .embed-preview__fallback a {
    color: var(--pug-color-accent-default, #6366f1);
    font-size: 0.8125rem;
    word-break: break-all;
  }
</style>

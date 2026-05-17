<script lang="ts">
  import { default as Skeleton } from "./Skeleton.svelte";
  import { default as TextLink } from "./TextLink.svelte";

  import type { ParsedEmbed } from "./types";

  let {
    parsed = null,
    aspectRatio = 16 / 9,
    loading = false,
    error = null,
    emptyMessage = "No embed to preview",
  }: {
    parsed?: ParsedEmbed | null;
    aspectRatio?: number | "auto";
    loading?: boolean;
    error?: string | null;
    emptyMessage?: string;
  } = $props();

  const embedUrl = $derived(parsed ? getEmbedUrl(parsed) : null);
  const isAudio = $derived(parsed?.provider === "audioboom");
  const effectiveAspectRatio = $derived(isAudio ? "auto" : aspectRatio);

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

<div class="poodle-embed-preview">
  {#if loading}
    <div class="poodle-embed-preview__loading">
      <Skeleton shape="block" />
      <span class="poodle-embed-preview__loading-text">Loading preview...</span>
    </div>
  {:else if error}
    <div class="poodle-embed-preview__error">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <circle cx="12" cy="12" r="10" />
        <path d="M12 8v4m0 4h.01" stroke-linecap="round" />
      </svg>
      <span>{error}</span>
    </div>
  {:else if !parsed}
    <div class="poodle-embed-preview__empty">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="2" y="4" width="20" height="16" rx="2" />
        <path d="M10 9l5 3-5 3V9z" />
      </svg>
      <span>{emptyMessage}</span>
    </div>
  {:else if embedUrl}
    <div
      class="poodle-embed-preview__container"
      style={effectiveAspectRatio !== "auto" ? `aspect-ratio: ${effectiveAspectRatio}` : ""}
    >
      <iframe
        src={embedUrl}
        title="{parsed.provider} embed"
        frameborder="0"
        allowfullscreen
        loading="lazy"
        sandbox="allow-scripts allow-same-origin allow-popups"
        class="poodle-embed-preview__iframe"
      ></iframe>
    </div>
  {:else if parsed.originalEmbed}
    <div
      class="poodle-embed-preview__container"
      style={effectiveAspectRatio !== "auto" ? `aspect-ratio: ${effectiveAspectRatio}` : ""}
    >
      {@html parsed.originalEmbed}
    </div>
  {:else}
    <div class="poodle-embed-preview__fallback">
      <TextLink href={parsed.originalUrl} target="_blank" rel="noopener noreferrer">
        {parsed.originalUrl ?? parsed.id}
      </TextLink>
    </div>
  {/if}
</div>

<style>
  .poodle-embed-preview {
    border-radius: var(--poodle-radius-surface, 0.5rem);
    overflow: hidden;
  }

  .poodle-embed-preview__container {
    position: relative;
    width: 100%;
    background: var(--poodle-color-background-panel, #1a1a1a);
  }

  .poodle-embed-preview__iframe {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border: 0;
  }

  .poodle-embed-preview__container:not([style*="aspect-ratio"]) .poodle-embed-preview__iframe {
    position: static;
    height: 10rem;
  }

  .poodle-embed-preview__loading,
  .poodle-embed-preview__error,
  .poodle-embed-preview__empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    min-height: 8rem;
    padding: 1.5rem;
    background: var(--poodle-color-background-panel, #1a1a1a);
    border-radius: var(--poodle-radius-surface, 0.5rem);
  }

  .poodle-embed-preview__error svg,
  .poodle-embed-preview__empty svg {
    width: 2rem;
    height: 2rem;
    color: var(--poodle-color-text-tertiary, #666);
  }

  .poodle-embed-preview__loading-text {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary, #999);
  }

  .poodle-embed-preview__error span,
  .poodle-embed-preview__empty span {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary, #999);
  }

  .poodle-embed-preview__error svg {
    color: var(--poodle-color-text-danger, #ef4444);
  }

  .poodle-embed-preview__fallback {
    padding: 0.75rem 1rem;
    background: var(--poodle-color-background-panel, #1a1a1a);
    border-radius: var(--poodle-radius-surface, 0.5rem);
  }

  .poodle-embed-preview__fallback :global(.poodle-text-link) {
    font-size: 0.8125rem;
    word-break: break-all;
  }
</style>

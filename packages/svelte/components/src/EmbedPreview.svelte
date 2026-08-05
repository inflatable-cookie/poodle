<script lang="ts">
  import "@poodle/styles/embed-preview.css";
  import { default as Skeleton } from "./Skeleton.svelte";
  import { default as TextLink } from "./TextLink.svelte";

  import type { ParsedEmbed } from "./types.ts";

  let {
    parsed = null,
    trustedHtml = null,
    aspectRatio = 16 / 9,
    loading = false,
    error = null,
    emptyMessage = "No embed to preview",
  }: {
    parsed?: ParsedEmbed | null;
    trustedHtml?: string | null;
    aspectRatio?: number | "auto";
    loading?: boolean;
    error?: string | null;
    emptyMessage?: string;
  } = $props();

  const embedUrl = $derived(parsed ? getEmbedUrl(parsed) : null);
  const isAudio = $derived(parsed?.provider === "audioboom");
  const effectiveAspectRatio = $derived(isAudio ? "auto" : aspectRatio);
  const hasFixedAspectRatio = $derived(effectiveAspectRatio !== "auto");
  const containerStyle = $derived(hasFixedAspectRatio ? `aspect-ratio: ${effectiveAspectRatio}` : "");

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
  {:else if !parsed && !trustedHtml}
    <div class="poodle-embed-preview__empty">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="2" y="4" width="20" height="16" rx="2" />
        <path d="M10 9l5 3-5 3V9z" />
      </svg>
      <span>{emptyMessage}</span>
    </div>
  {:else if parsed && embedUrl}
    <div
      class="poodle-embed-preview__container"
      data-fixed-aspect={hasFixedAspectRatio}
      style={containerStyle}
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
  {:else if parsed?.originalEmbed}
    <div
      class="poodle-embed-preview__container"
      data-fixed-aspect={hasFixedAspectRatio}
      style={containerStyle}
    >
      {@html parsed.originalEmbed}
    </div>
  {:else if trustedHtml}
    <div
      class="poodle-embed-preview__container"
      data-fixed-aspect={hasFixedAspectRatio}
      style={containerStyle}
    >
      {@html trustedHtml}
    </div>
  {:else if parsed}
    <div class="poodle-embed-preview__fallback">
      <TextLink href={parsed.originalUrl} target="_blank" rel="noopener noreferrer">
        {parsed.originalUrl ?? parsed.id}
      </TextLink>
    </div>
  {/if}
</div>


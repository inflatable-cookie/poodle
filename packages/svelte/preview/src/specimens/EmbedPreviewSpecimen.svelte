<script lang="ts">
  import { EmbedPreview } from "@flint/svelte-composites";
  import { Eyebrow } from "@flint/svelte-primitives";
  import type { ParsedEmbed } from "@flint/svelte-composites";

  const youtubeParsed: ParsedEmbed = {
    provider: "youtube",
    id: "dQw4w9WgXcQ",
    originalUrl: "https://youtube.com/watch?v=dQw4w9WgXcQ",
  };

  const vimeoParsed: ParsedEmbed = {
    provider: "vimeo",
    id: "76979871",
    originalUrl: "https://vimeo.com/76979871",
  };
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>YouTube embed</Eyebrow>
    <EmbedPreview parsed={youtubeParsed} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Vimeo embed</Eyebrow>
    <EmbedPreview parsed={vimeoParsed} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading state</Eyebrow>
    <EmbedPreview loading />
  </div>

  <div class="specimen__group">
    <Eyebrow>Error state</Eyebrow>
    <EmbedPreview error="Failed to load embed. The URL may be invalid or the provider is unavailable." />
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty state</Eyebrow>
    <EmbedPreview emptyMessage="Paste a URL above to see a preview" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>

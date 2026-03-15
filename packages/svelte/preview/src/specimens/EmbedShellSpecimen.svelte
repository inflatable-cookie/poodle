<script lang="ts">
  import { EmbedShell } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ready (video aspect)</Eyebrow>
    <EmbedShell title="Product demo" provider="YouTube" aspectRatio="video">
      <div class="placeholder-embed">Video player would render here</div>
    </EmbedShell>
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading state</Eyebrow>
    <EmbedShell title="Loading embed" provider="Vimeo" state="loading" aspectRatio="video" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Error state</Eyebrow>
    <EmbedShell
      title="Broken embed"
      state="error"
      stateTitle="Failed to load"
      stateMessage="The embedded content could not be retrieved."
      aspectRatio="video"
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 28rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .placeholder-embed {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    background: var(--pug-color-bg-subtle);
    color: var(--pug-color-text-secondary);
    font-size: 0.875rem;
  }
</style>

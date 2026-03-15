<script lang="ts">
  import { MediaPreview } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Image preview</Eyebrow>
    <MediaPreview
      title="Hero banner"
      description="Main landing page banner image for the product launch."
      eyebrow="Image"
      meta={["1920 × 1080", "245 KB", "PNG"]}
      kind="image"
      aspectRatio="landscape"
    >
      <div slot="media" class="placeholder-media">Image placeholder</div>
    </MediaPreview>
  </div>

  <div class="specimen__group">
    <Eyebrow>Video preview</Eyebrow>
    <MediaPreview
      title="Onboarding walkthrough"
      eyebrow="Video"
      meta={["3:42", "48 MB"]}
      kind="video"
      aspectRatio="video"
    >
      <div slot="media" class="placeholder-media">Video placeholder</div>
    </MediaPreview>
  </div>

  <div class="specimen__group">
    <Eyebrow>Error state</Eyebrow>
    <MediaPreview
      title="Corrupted file"
      kind="document"
      state="error"
      stateTitle="Preview unavailable"
      stateMessage="This file cannot be previewed."
      aspectRatio="landscape"
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 24rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .placeholder-media {
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

<script lang="ts">
  import { MediaPreview } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Image preview">
    <MediaPreview
      title="Hero banner"
      description="Main landing page banner image for the product launch."
      eyebrow="Image"
      meta={["1920 × 1080", "245 KB", "PNG"]}
      kind="image"
      aspectRatio="landscape"
    >
      <div slot="media" class="poodle-placeholder-media">Image placeholder</div>
    </MediaPreview>
  </SpecimenGroup>

  <SpecimenGroup label="Video preview">
    <MediaPreview
      title="Onboarding walkthrough"
      eyebrow="Video"
      meta={["3:42", "48 MB"]}
      kind="video"
      aspectRatio="video"
    >
      <div slot="media" class="poodle-placeholder-media">Video placeholder</div>
    </MediaPreview>
  </SpecimenGroup>

  <SpecimenGroup label="Error state">
    <MediaPreview
      title="Corrupted file"
      kind="document"
      state="error"
      stateTitle="Preview unavailable"
      stateMessage="This file cannot be previewed."
      aspectRatio="landscape"
    />
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 24rem;
  }

  .poodle-placeholder-media {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    background: var(--poodle-color-bg-subtle);
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
  }
</style>

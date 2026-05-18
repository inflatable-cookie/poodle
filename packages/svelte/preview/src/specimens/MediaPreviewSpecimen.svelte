<script lang="ts">
  import { MediaPreview } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Image preview" bare>
        <div class="poodle-media-preview-specimen__variant">
          <MediaPreview
            title="Hero banner"
            description="Main landing page banner image for the product launch."
            eyebrow="Image"
            meta={["1920 × 1080", "245 KB", "PNG"]}
            kind="image"
            aspectRatio="landscape"
          >
            {#snippet mediaContent()}
              <div class="poodle-placeholder-media">Image placeholder</div>
            {/snippet}
          </MediaPreview>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Video preview" bare>
        <div class="poodle-media-preview-specimen__variant">
          <MediaPreview
            title="Onboarding walkthrough"
            eyebrow="Video"
            meta={["3:42", "48 MB"]}
            kind="video"
            aspectRatio="video"
          >
            {#snippet mediaContent()}
              <div class="poodle-placeholder-media">Video placeholder</div>
            {/snippet}
          </MediaPreview>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Error state" bare>
        <div class="poodle-media-preview-specimen__variant">
          <MediaPreview
            title="Corrupted file"
            kind="document"
            state="error"
            stateTitle="Preview unavailable"
            stateMessage="This file cannot be previewed."
            aspectRatio="landscape"
          />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-media-preview-specimen__variant">
      <MediaPreview
        title="Hero banner"
        description="Main landing page banner image for the product launch."
        eyebrow="Image"
        meta={["1920 × 1080", "245 KB", "PNG"]}
        kind="image"
        aspectRatio="landscape"
        {size}
      >
        {#snippet mediaContent()}
          <div class="poodle-placeholder-media">Image placeholder</div>
        {/snippet}
      </MediaPreview>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-media-preview-specimen__variant">
      <MediaPreview
        title="Hero banner"
        description="Main landing page banner image for the product launch."
        eyebrow="Image"
        meta={["1920 × 1080", "245 KB", "PNG"]}
        kind="image"
        aspectRatio="landscape"
        {density}
      >
        {#snippet mediaContent()}
          <div class="poodle-placeholder-media">Image placeholder</div>
        {/snippet}
      </MediaPreview>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 24rem;
  }

  .poodle-media-preview-specimen__variant {
    width: min(100%, 24rem);
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

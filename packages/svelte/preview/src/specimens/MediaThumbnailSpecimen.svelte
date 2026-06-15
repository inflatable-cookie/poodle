<script lang="ts">
  import { MediaThumbnail } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Image thumbnails">
    <div class="poodle-thumb-grid">
      <MediaThumbnail kind="image" title="Photo 1" badge="New" aspectRatio="square">
        <div class="poodle-placeholder-thumb">IMG</div>
      </MediaThumbnail>
      <MediaThumbnail kind="image" title="Photo 2" meta="2.4 MB" aspectRatio="square">
        <div class="poodle-placeholder-thumb">IMG</div>
      </MediaThumbnail>
      <MediaThumbnail kind="video" title="Clip" badge="HD" meta="1:24" aspectRatio="square">
        <div class="poodle-placeholder-thumb">VID</div>
      </MediaThumbnail>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Compact presentation">
    <div class="poodle-thumb-grid">
      <MediaThumbnail kind="document" title="Report.pdf" presentation="compact" aspectRatio="landscape">
        <div class="poodle-placeholder-thumb">PDF</div>
      </MediaThumbnail>
      <MediaThumbnail kind="audio" title="Interview.mp3" presentation="compact" aspectRatio="landscape">
        <div class="poodle-placeholder-thumb">MP3</div>
      </MediaThumbnail>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Loading state">
    <div class="poodle-thumb-single">
      <MediaThumbnail kind="image" state="loading" aspectRatio="square" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Contained image">
    <div class="poodle-thumb-wide">
      <MediaThumbnail
        kind="image"
        aspectRatio="auto"
        fit="contain"
        frameMinHeight="10rem"
        frameMaxHeight="14rem"
        title="Question diagram"
      >
        <div class="poodle-contained-image">Image</div>
      </MediaThumbnail>
    </div>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-thumb-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(7rem, 1fr));
    gap: 0.75rem;
  }

  .poodle-placeholder-thumb {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    background: var(--poodle-color-bg-subtle);
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 600;
  }

  .poodle-thumb-single {
    width: min(100%, 10rem);
  }

  .poodle-thumb-wide {
    width: min(100%, 28rem);
  }

  .poodle-contained-image {
    display: grid;
    place-items: center;
    width: 100%;
    min-height: 10rem;
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 600;
  }
</style>

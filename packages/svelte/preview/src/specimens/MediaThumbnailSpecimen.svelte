<script lang="ts">
  import { MediaThumbnail } from "@poodle/svelte-composites";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Image thumbnails">
    <div class="thumb-grid">
      <MediaThumbnail kind="image" title="Photo 1" badge="New" aspectRatio="square">
        <div class="placeholder-thumb">IMG</div>
      </MediaThumbnail>
      <MediaThumbnail kind="image" title="Photo 2" meta="2.4 MB" aspectRatio="square">
        <div class="placeholder-thumb">IMG</div>
      </MediaThumbnail>
      <MediaThumbnail kind="video" title="Clip" badge="HD" meta="1:24" aspectRatio="square">
        <div class="placeholder-thumb">VID</div>
      </MediaThumbnail>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Compact presentation">
    <div class="thumb-grid">
      <MediaThumbnail kind="document" title="Report.pdf" presentation="compact" aspectRatio="landscape">
        <div class="placeholder-thumb">PDF</div>
      </MediaThumbnail>
      <MediaThumbnail kind="audio" title="Interview.mp3" presentation="compact" aspectRatio="landscape">
        <div class="placeholder-thumb">MP3</div>
      </MediaThumbnail>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Loading state">
    <MediaThumbnail kind="image" state="loading" aspectRatio="square" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .thumb-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(7rem, 1fr));
    gap: 0.75rem;
  }

  .placeholder-thumb {
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
</style>

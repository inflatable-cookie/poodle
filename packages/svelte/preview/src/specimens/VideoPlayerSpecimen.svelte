<script lang="ts">
  import { VideoPlayer } from "@poodle/svelte-composites";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <SpecimenGroup label="Video player">
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      ariaLabel="Sample video"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <VideoPlayer
          src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
          {size}
          ariaLabel="Video player at {size}"
        />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Custom aspect ratio (4:3)">
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      aspectRatio={4 / 3}
      ariaLabel="4:3 aspect video"
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>

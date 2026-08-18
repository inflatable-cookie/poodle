<script lang="ts">
  import { VideoPlayer } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Video player">
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      ariaLabel="Sample video"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Custom aspect ratio (4:3)">
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      aspectRatio={4 / 3}
      ariaLabel="4:3 aspect video"
    />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      ariaLabel="Sample video"
      {size}
    />
  {/snippet}

  {#snippet densities(density)}
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      ariaLabel="Sample video"
      {density}
    />
  {/snippet}
</SpecimenLayout>

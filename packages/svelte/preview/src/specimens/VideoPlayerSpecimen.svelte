<script lang="ts">
  import { VideoPlayer } from "@flint/svelte-composites";
  import { Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Video player</Eyebrow>
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      ariaLabel="Sample video"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom aspect ratio (4:3)</Eyebrow>
    <VideoPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
      aspectRatio={4 / 3}
      ariaLabel="4:3 aspect video"
    />
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
    max-width: 36rem;
  }
</style>

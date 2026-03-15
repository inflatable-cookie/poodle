<script lang="ts">
  import { AudioPlayer } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic audio player</Eyebrow>
    <AudioPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"
      ariaLabel="T-Rex roar audio"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With speed control</Eyebrow>
    <AudioPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"
      ariaLabel="Audio with speed control"
      showSpeedControl
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
  }
</style>

<script lang="ts">
  import { AudioPlayer } from "@poodle/svelte-composites";
  import { Eyebrow, UiPresentationProvider } from "@poodle/svelte-primitives";
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

  <div class="specimen__group">
    <Eyebrow>Semantic presentation</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <AudioPlayer
          src="https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"
          ariaLabel="Compact audio player"
        />
        <AudioPlayer
          src="https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"
          ariaLabel="Prominent audio player"
          showSpeedControl
          sizeRole="prominent"
        />
      </div>
    </UiPresentationProvider>
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

  .specimen__stack {
    display: grid;
    gap: 0.75rem;
  }
</style>

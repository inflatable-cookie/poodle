<script lang="ts">
  import { AudioPlayer } from "@poodle/svelte";
  import { UiPresentationProvider } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Basic audio player">
    <AudioPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"
      ariaLabel="T-Rex roar audio"
    />
  </SpecimenGroup>

  <SpecimenGroup label="With speed control">
    <AudioPlayer
      src="https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"
      ariaLabel="Audio with speed control"
      showSpeedControl
    />
  </SpecimenGroup>

  <SpecimenGroup label="Semantic presentation">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="poodle-specimen__stack">
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
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__stack {
    display: grid;
    gap: 0.75rem;
  }
</style>

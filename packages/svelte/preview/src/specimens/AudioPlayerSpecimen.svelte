<script lang="ts">
  import { AudioPlayer } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const src = "https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3";
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Basic audio player" bare>
        <AudioPlayer
          src={src}
          ariaLabel="T-Rex roar audio"
        />
      </SpecimenGroup>

      <SpecimenGroup label="With speed control" bare>
        <AudioPlayer
          src={src}
          ariaLabel="Audio with speed control"
          showSpeedControl
        />
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-audio-player-specimen__variant">
      <AudioPlayer
        src={src}
        ariaLabel={`Audio player at ${size}`}
        showSpeedControl
        {size}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-audio-player-specimen__variant">
      <AudioPlayer
        src={src}
        ariaLabel={`Audio player at ${density} density`}
        showSpeedControl
        {density}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-audio-player-specimen__variant {
    width: min(100%, 36rem);
  }
</style>

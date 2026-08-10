<script lang="ts">
  import { createAudioMeterContext, audioMeterTransition } from "@inflatable-cookie/poodle-core";
  import { AudioMeter } from "@inflatable-cookie/poodle-svelte";
  const sample = { atMs: 16, peak: 0.72, meanSquare: 0.18, durationMs: 16 };
  let left = $state(audioMeterTransition(createAudioMeterContext({ mode: "ppm" }), { type: "PUSH_FRAME", frame: sample }).context);
  let right = $state(audioMeterTransition(createAudioMeterContext({ mode: "ppm" }), { type: "PUSH_FRAME", frame: { ...sample, peak: 0.48, meanSquare: 0.1 } }).context);
</script>
<div class="poodle-audio-meter-specimen">
  <AudioMeter bind:context={left} bind:rightContext={right} ariaLabel="Stereo master level" />
  <AudioMeter context={left} style="bar" orientation="horizontal" ariaLabel="Mono bus level" />
</div>
<style>.poodle-audio-meter-specimen { display: flex; align-items: end; gap: 2rem; }</style>

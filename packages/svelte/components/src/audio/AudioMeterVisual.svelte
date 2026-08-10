<script lang="ts">
  import type { AudioMeterVisualState } from "@inflatable-cookie/poodle-core";

  let {
    visualState,
    style = "bar",
    orientation = "vertical",
    segments = 20,
  }: {
    visualState: AudioMeterVisualState;
    style?: "bar" | "segments";
    orientation?: "horizontal" | "vertical";
    segments?: number;
  } = $props();

  const segmentList = $derived(Array.from({ length: Math.max(segments, 1) }, (_, index) => index));
</script>

<span
  class="poodle-audio-meter-visual"
  data-style={style}
  data-orientation={orientation}
  data-enabled={visualState.enabled}
  aria-hidden="true"
  style={`--poodle-audio-meter-value: ${visualState.ballisticValue}; --poodle-audio-meter-peak: ${visualState.peakHold ?? 0};`}
>
  <span class="poodle-audio-meter-visual__track">
    {#if style === "segments"}
      {#each segmentList as segment}
        <span
          class="poodle-audio-meter-visual__segment"
          data-active={(segment + 1) / segmentList.length <= visualState.ballisticValue}
          data-warning={segment / segmentList.length >= 0.75}
          data-clip={segment / segmentList.length >= 0.95}
        ></span>
      {/each}
    {:else}
      <span class="poodle-audio-meter-visual__bar"></span>
    {/if}
    {#if visualState.peakHold !== null}
      <span class="poodle-audio-meter-visual__peak"></span>
    {/if}
  </span>
  <span class="poodle-audio-meter-visual__clip" data-active={visualState.clip}></span>
</span>

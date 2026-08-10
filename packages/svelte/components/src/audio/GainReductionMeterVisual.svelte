<script lang="ts">
  import type { GainReductionMeterVisualState } from "@inflatable-cookie/poodle-core";

  let {
    visualState, style = "segments", orientation = "vertical", segments = 20,
  }: {
    visualState: GainReductionMeterVisualState;
    style?: "bar" | "segments";
    orientation?: "horizontal" | "vertical";
    segments?: number;
  } = $props();
  const segmentList = $derived(Array.from({ length: Math.max(segments, 1) }, (_, index) => index));
</script>

<span
  class="poodle-gain-reduction-meter-visual"
  data-style={style}
  data-orientation={orientation}
  data-enabled={visualState.enabled}
  aria-hidden="true"
  style={`--poodle-gain-reduction-value: ${visualState.ballisticValue};`}
>
  <span class="poodle-gain-reduction-meter-visual__track">
    {#if style === "segments"}
      {#each segmentList as segment}
        <span class="poodle-gain-reduction-meter-visual__segment" data-active={(segment + 1) / segmentList.length <= visualState.ballisticValue}></span>
      {/each}
    {:else}
      <span class="poodle-gain-reduction-meter-visual__bar"></span>
    {/if}
  </span>
</span>

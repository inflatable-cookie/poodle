<script lang="ts">
  import type { AudioControlVisualState } from "@inflatable-cookie/poodle-core";

  let {
    visualState,
    orientation,
    detents,
  }: {
    visualState: AudioControlVisualState;
    orientation: "horizontal" | "vertical";
    detents: number[];
  } = $props();
</script>

<span
  class="poodle-fader-visual"
  data-orientation={orientation}
  data-hover={visualState.hover}
  data-focus={visualState.focus}
  data-drag={visualState.drag}
  data-enabled={visualState.enabled}
  aria-hidden="true"
  style={`--poodle-fader-value: ${visualState.valueNorm};`}
>
  <span class="poodle-fader-visual__track"></span>
  <span class="poodle-fader-visual__fill"></span>
  {#each detents as detent}
    <span class="poodle-fader-visual__detent" style={`--poodle-fader-detent: ${detent};`}></span>
  {/each}
  <span class="poodle-fader-visual__thumb"></span>
</span>

<script lang="ts">
  import type { AudioControlVisualState } from "@inflatable-cookie/poodle-core";

  let { visualState }: { visualState: AudioControlVisualState } = $props();
  const sweep = $derived(visualState.valueNorm * 270);
  const indicatorRotation = $derived(-135 + sweep);
</script>

<span
  class="poodle-knob-visual"
  data-hover={visualState.hover}
  data-focus={visualState.focus}
  data-drag={visualState.drag}
  data-enabled={visualState.enabled}
  aria-hidden="true"
  style={`--poodle-knob-sweep: ${sweep}deg; --poodle-knob-indicator-rotation: ${indicatorRotation}deg;`}
>
  <span class="poodle-knob-visual__track"></span>
  <span class="poodle-knob-visual__arc"></span>
  <span class="poodle-knob-visual__indicator"></span>
</span>

<script lang="ts">
  import type { AudioSwitchVisualState } from "@inflatable-cookie/poodle-core";
  let { visualState }: { visualState: AudioSwitchVisualState } = $props();
  const position = $derived(visualState.stateCount <= 1 ? 0 : visualState.state / (visualState.stateCount - 1));
</script>

<span
  class="poodle-audio-switch-visual"
  data-state={visualState.state}
  data-pressed={visualState.pressed}
  data-lamp={visualState.lampOn}
  data-hover={visualState.hover}
  data-focus={visualState.focus}
  data-enabled={visualState.enabled}
  aria-hidden="true"
  style={`--poodle-audio-switch-position: ${position};`}
>
  <span class="poodle-audio-switch-visual__body">
    <span class="poodle-audio-switch-visual__handle"></span>
  </span>
  <span class="poodle-audio-switch-visual__lamp"></span>
</span>

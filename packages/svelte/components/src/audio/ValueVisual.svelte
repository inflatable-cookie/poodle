<script lang="ts">
  import type { AudioControlVisualState } from "@inflatable-cookie/poodle-core";

  let {
    visualState,
    text,
    kind,
  }: {
    visualState: AudioControlVisualState;
    text: string;
    kind: "readout" | "drag-number";
  } = $props();
</script>

<span
  class:poodle-value-readout-visual={kind === "readout"}
  class:poodle-drag-number-field-visual={kind === "drag-number"}
  data-hover={visualState.hover}
  data-focus={visualState.focus}
  data-drag={visualState.drag}
  data-enabled={visualState.enabled}
  aria-hidden="true"
>{text}</span>

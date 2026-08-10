<script lang="ts">
  import type { XYPadVisualState } from "@inflatable-cookie/poodle-core";
  let { visualState }: { visualState: XYPadVisualState } = $props();
</script>

<span
  class="poodle-xy-pad-visual"
  data-hover={visualState.hover}
  data-focus={visualState.focus}
  data-drag={visualState.drag}
  data-enabled={visualState.enabled}
  aria-hidden="true"
  style={`--poodle-xy-pad-x: ${visualState.xNorm}; --poodle-xy-pad-y: ${visualState.yNorm};`}
>
  <span class="poodle-xy-pad-visual__grid"></span>
  <span class="poodle-xy-pad-visual__trace-x"></span>
  <span class="poodle-xy-pad-visual__trace-y"></span>
  <span class="poodle-xy-pad-visual__thumb"></span>
</span>

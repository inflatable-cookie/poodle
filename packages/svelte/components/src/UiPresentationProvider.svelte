<script lang="ts">
  import {
    controlHeightRem,
    controlSpaceXRem,
    panelSpaceXRem,
    panelSpaceYRem,
    setUiPresentation,
  } from "./presentation";
  import type { ControlDensity, ControlSize } from "./types";

  export let density: ControlDensity = "default";
  export let sizeScale: ControlSize = "md";

  const presentation = setUiPresentation({
    density,
    sizeScale,
  });

  $: presentation.set({
    density,
    sizeScale,
  });

  $: providerStyle = [
    `--poodle-size-control-height: ${controlHeightRem(sizeScale)}rem`,
    `--poodle-space-control-x: ${controlSpaceXRem(density)}rem`,
    `--poodle-space-panel-x: ${panelSpaceXRem(density)}rem`,
    `--poodle-space-panel-y: ${panelSpaceYRem(density)}rem`,
  ].join("; ");
</script>

<div class="poodle-ui-presentation-provider" style={providerStyle}>
  <slot />
</div>

<style>
  .poodle-ui-presentation-provider {
    display: contents;
  }
</style>

<script lang="ts">
  import "@poodle/styles/ui-presentation-provider.css";
  import type { Snippet } from "svelte";
  import {
    controlHeightRem,
    controlSpaceXRem,
    panelSpaceXRem,
    panelSpaceYRem,
    setUiPresentation,
  } from "./presentation";
  import type { ControlDensity, ControlSize } from "./types";

  interface Props {
    density?: ControlDensity;
    sizeScale?: ControlSize;
    children?: Snippet;
  }

  let {
    density = "default",
    sizeScale = "md",
    children,
  }: Props = $props();

  const presentation = setUiPresentation({
    density: "default",
    sizeScale: "md",
  });

  $effect(() => {
    presentation.set({
      density,
      sizeScale,
    });
  });

  const providerStyle = $derived([
    `--poodle-size-control-height: ${controlHeightRem(sizeScale)}rem`,
    `--poodle-space-control-x: ${controlSpaceXRem(density)}rem`,
    `--poodle-space-panel-x: ${panelSpaceXRem(density)}rem`,
    `--poodle-space-panel-y: ${panelSpaceYRem(density)}rem`,
  ].join("; "));
</script>

<div class="poodle-ui-presentation-provider" style={providerStyle}>
  {@render children?.()}
</div>


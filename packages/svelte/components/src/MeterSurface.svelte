<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/meter-surface.css";
  import { onMount, untrack, type Snippet } from "svelte";
  import {
    createCanvas2dMeterSurfacePainter, createMeterSurfaceRegistry,
    type MeterBus, type MeterSurfacePainter,
  } from "@inflatable-cookie/poodle-core";
  import { setMeterSurfaceRegistry } from "./meter-surface-context";

  interface Props {
    bus?: MeterBus | null;
    painter?: MeterSurfacePainter | null;
    children?: Snippet;
  }

  let { bus = null, painter = null, children }: Props = $props();

  let rootEl: HTMLDivElement | undefined = $state();
  let viewportEl: HTMLDivElement | undefined = $state();
  let contentEl: HTMLDivElement | undefined = $state();
  let canvasEl: HTMLCanvasElement | undefined = $state();

  // The registry must exist before descendant surface-mode AudioMeters
  // initialize, so it is created during init (bus is mount-time configuration)
  // and connected on mount.
  const registry = untrack(() => (bus === null ? null : createMeterSurfaceRegistry(bus)));
  if (registry !== null) setMeterSurfaceRegistry(registry);

  onMount(() => {
    if (registry === null || rootEl === undefined || viewportEl === undefined || contentEl === undefined || canvasEl === undefined) return;
    registry.connect(
      { root: rootEl, viewport: viewportEl, content: contentEl, canvas: canvasEl },
      { painter: painter ?? createCanvas2dMeterSurfacePainter() },
    );
    return () => registry.disconnect();
  });

  export function invalidateLayout(): void {
    registry?.invalidateLayout();
  }

  export function refreshPalette(): void {
    registry?.refreshPalette();
  }
</script>

<div class="poodle-meter-surface" data-scope="meter-surface" data-part="root" bind:this={rootEl}>
  <div class="poodle-meter-surface__viewport" data-part="viewport" bind:this={viewportEl}>
    <div class="poodle-meter-surface__content" data-part="content" bind:this={contentEl}>
      {@render children?.()}
    </div>
  </div>
  <canvas class="poodle-meter-surface__canvas" data-part="canvas" aria-hidden="true" bind:this={canvasEl}></canvas>
</div>

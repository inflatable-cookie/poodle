<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { WindowSurface } from "./types";

  export let windowId: string;
  export let surfaces: WindowSurface[] = [];
  export let activeSurfaceId: string | null = null;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    activeSurfaceChange: { surfaceId: string };
    surfaceClose: { surfaceId: string };
    surfaceReorder: { order: string[] };
    surfaceDetach: { surfaceId: string };
    surfaceReceive: { surfaceId: string; fromWindowId: string };
  }>();

  $: activeSurface = surfaces.find((s) => s.id === activeSurfaceId) ?? null;
  $: isEmpty = surfaces.length === 0;
  $: isMultiSurface = surfaces.length > 1;
</script>

<div
  class="workspace-window"
  data-window-id={windowId}
  data-empty={isEmpty || undefined}
  data-multi-surface={isMultiSurface || undefined}
  aria-label={ariaLabel ?? `Workspace window ${windowId}`}
  role="region"
>
  <slot
    {windowId}
    {surfaces}
    {activeSurfaceId}
    {activeSurface}
    {isEmpty}
    {isMultiSurface}
  />
</div>

<style>
  .workspace-window {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    min-width: 0;
    min-height: 0;
    background: var(--pug-color-surface-default);
  }
</style>

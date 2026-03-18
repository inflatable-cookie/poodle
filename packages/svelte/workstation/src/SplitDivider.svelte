<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { CollapseToggle } from "@pug/svelte-primitives";
  import type { CollapseDirection } from "@pug/svelte-primitives";
  import type { SplitOrientation } from "./types";
  import ResizeHandle from "./ResizeHandle.svelte";

  export let orientation: SplitOrientation = "horizontal";
  export let showCollapseBefore = false;
  export let showCollapseAfter = false;
  export let isBeforeCollapsed = false;
  export let isAfterCollapsed = false;
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    resizeStart: { position: number };
    resizeMove: { delta: number };
    resizeEnd: { position: number };
    resizeStep: { delta: number };
    collapseBefore: { isCollapsed: boolean };
    collapseAfter: { isCollapsed: boolean };
  }>();

  $: beforeDirection = (orientation === "horizontal" ? "left" : "up") as CollapseDirection;
  $: afterDirection = (orientation === "horizontal" ? "right" : "down") as CollapseDirection;
</script>

<div
  class="split-divider"
  data-orientation={orientation}
  data-disabled={isDisabled || undefined}
>
  {#if showCollapseBefore}
    <CollapseToggle
      direction={beforeDirection}
      isCollapsed={isBeforeCollapsed}
      {isDisabled}
      ariaLabel={isBeforeCollapsed ? "Expand before" : "Collapse before"}
      on:toggle={(e) => dispatch("collapseBefore", { isCollapsed: e.detail.isCollapsed })}
    />
  {/if}

  <ResizeHandle
    {orientation}
    {isDisabled}
    ariaLabel={ariaLabel ?? "Resize"}
    on:resizeStart={(e) => dispatch("resizeStart", e.detail)}
    on:resizeMove={(e) => dispatch("resizeMove", e.detail)}
    on:resizeEnd={(e) => dispatch("resizeEnd", e.detail)}
    on:resizeStep={(e) => dispatch("resizeStep", e.detail)}
  />

  {#if showCollapseAfter}
    <CollapseToggle
      direction={afterDirection}
      isCollapsed={isAfterCollapsed}
      {isDisabled}
      ariaLabel={isAfterCollapsed ? "Expand after" : "Collapse after"}
      on:toggle={(e) => dispatch("collapseAfter", { isCollapsed: e.detail.isCollapsed })}
    />
  {/if}
</div>

<style>
  .split-divider {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .split-divider[data-orientation="horizontal"] {
    flex-direction: column;
    width: 0.625rem;
    gap: 0.0625rem;
  }

  .split-divider[data-orientation="vertical"] {
    flex-direction: row;
    height: 0.625rem;
    gap: 0.0625rem;
  }
</style>

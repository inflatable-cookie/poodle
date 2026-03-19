<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { CollapseToggle } from "@pug/svelte-primitives";
  import type { CollapseDirection } from "@pug/svelte-primitives";
  import { ResizeHandle } from "@pug/svelte-primitives";
  import type { SplitOrientation } from "@pug/svelte-primitives";

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
  $: eitherCollapsed = isBeforeCollapsed || isAfterCollapsed;
  $: hasToggles = showCollapseBefore || showCollapseAfter;
</script>

<div
  class="split-divider"
  data-orientation={orientation}
  data-disabled={isDisabled || undefined}
  data-has-toggles={hasToggles || undefined}
>
  <ResizeHandle
    {orientation}
    {isDisabled}
    ariaLabel={ariaLabel ?? "Resize"}
    on:resizeStart={(e) => dispatch("resizeStart", e.detail)}
    on:resizeMove={(e) => dispatch("resizeMove", e.detail)}
    on:resizeEnd={(e) => dispatch("resizeEnd", e.detail)}
    on:resizeStep={(e) => dispatch("resizeStep", e.detail)}
  />

  {#if hasToggles}
    <div class="split-divider__toggles">
      {#if showCollapseBefore && !isAfterCollapsed}
        <CollapseToggle
          direction={beforeDirection}
          isCollapsed={isBeforeCollapsed}
          {isDisabled}
          ariaLabel={isBeforeCollapsed ? "Expand before" : "Collapse before"}
          on:toggle={(e) => dispatch("collapseBefore", { isCollapsed: e.detail.isCollapsed })}
        />
      {/if}
      {#if showCollapseAfter && !isBeforeCollapsed}
        <CollapseToggle
          direction={afterDirection}
          isCollapsed={isAfterCollapsed}
          {isDisabled}
          ariaLabel={isAfterCollapsed ? "Expand after" : "Collapse after"}
          on:toggle={(e) => dispatch("collapseAfter", { isCollapsed: e.detail.isCollapsed })}
        />
      {/if}
    </div>
  {/if}
</div>

<style>
  .split-divider {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .split-divider[data-orientation="horizontal"] {
    width: 0.5rem;
    height: 100%;
  }

  .split-divider[data-orientation="vertical"] {
    height: 0.5rem;
    width: 100%;
  }

  /* Toggles overlay centered on the divider */
  .split-divider__toggles {
    position: absolute;
    z-index: 1;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    pointer-events: none;
  }

  .split-divider__toggles :global(*) {
    pointer-events: auto;
  }

  /* Horizontal: toggles stacked vertically, centered on the divider line */
  .split-divider[data-orientation="horizontal"] .split-divider__toggles {
    flex-direction: column;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  /* Vertical: toggles in a row, centered on the divider line */
  .split-divider[data-orientation="vertical"] .split-divider__toggles {
    flex-direction: row;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
</style>

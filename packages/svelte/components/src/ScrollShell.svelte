<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { joinStyles, overflowForDirection, scaleToSpace } from "./internal";

  import type { ScrollDirection, SpaceScale } from "./types";

  export let direction: ScrollDirection = "vertical";
  export let padding: Extract<SpaceScale, "none" | "sm" | "md"> = "none";
  export let asRole: "region" | "group" | null = null;
  export let label: string | null = null;
  export let focusable = false;

  const dispatch = createEventDispatcher<{
    scroll: Event;
  }>();

  $: needsHorizontal = direction === "horizontal" || direction === "both";

  $: viewportStyle = joinStyles([
    overflowForDirection(direction),
    `padding: ${scaleToSpace(padding)}`,
    "min-width: 0",
    "min-height: 0",
  ]);
</script>

<div class="poodle-scroll-shell">
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <div
    class="poodle-scroll-shell__viewport"
    tabindex={focusable ? 0 : undefined}
    data-focusable={focusable}
    role={asRole ?? (focusable ? "region" : undefined)}
    aria-label={label ?? (focusable ? "Scrollable content" : undefined)}
    style={viewportStyle}
    on:scroll={(event) => dispatch("scroll", event)}
  >
    <div class="poodle-scroll-shell__content" class:poodle-scroll-shell__content--h={needsHorizontal}>
      <slot />
    </div>
  </div>
</div>

<style>
  .poodle-scroll-shell {
    min-width: 0;
    min-height: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    border-radius: var(--poodle-radius-surface);
  }

  .poodle-scroll-shell__viewport {
    width: 100%;
    height: 100%;
    overscroll-behavior: contain;
    border-radius: inherit;
  }

  .poodle-scroll-shell__content--h {
    min-width: max-content;
  }

  .poodle-scroll-shell__viewport:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>

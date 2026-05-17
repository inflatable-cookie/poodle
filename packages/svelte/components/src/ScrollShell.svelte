<script lang="ts">
  import type { Snippet } from "svelte";
  import { joinStyles, overflowForDirection, scaleToSpace } from "./internal";

  import type { ScrollDirection, SpaceScale } from "./types";

  interface Props {
    direction?: ScrollDirection;
    padding?: Extract<SpaceScale, "none" | "sm" | "md">;
    asRole?: "region" | "group" | null;
    label?: string | null;
    focusable?: boolean;
    onScroll?: ((event: Event) => void) | null;
    children?: Snippet;
  }

  let {
    direction = "vertical",
    padding = "none",
    asRole = null,
    label = null,
    focusable = false,
    onScroll = null,
    children,
  }: Props = $props();

  const needsHorizontal = $derived(direction === "horizontal" || direction === "both");

  const viewportStyle = $derived(joinStyles([
    overflowForDirection(direction),
    `padding: ${scaleToSpace(padding)}`,
    "min-width: 0",
    "min-height: 0",
  ]));
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
    onscroll={(event) => onScroll?.(event)}
  >
    <div class="poodle-scroll-shell__content" class:poodle-scroll-shell__content--h={needsHorizontal}>
      {@render children?.()}
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

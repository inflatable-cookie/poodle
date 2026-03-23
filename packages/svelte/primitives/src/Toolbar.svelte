<script lang="ts">
  import { getFocusableElements } from "./internal";

  import type { Orientation } from "./types";

  export let orientation: Orientation = "horizontal";
  export let ariaLabel: string | null = null;

  let rootElement: HTMLDivElement | null = null;

  function focusSibling(direction: 1 | -1): void {
    const focusable = getFocusableElements(rootElement);

    if (focusable.length === 0) {
      return;
    }

    const currentIndex = focusable.indexOf(document.activeElement as HTMLElement);
    const nextIndex = currentIndex === -1 ? 0 : (currentIndex + direction + focusable.length) % focusable.length;
    focusable[nextIndex]?.focus();
  }
</script>

<div
  bind:this={rootElement}
  class="toolbar"
  data-orientation={orientation}
  role="toolbar"
  tabindex="0"
  aria-label={ariaLabel ?? undefined}
  on:keydown={(event) => {
    if (
      orientation === "horizontal" &&
      (event.key === "ArrowRight" || event.key === "ArrowLeft")
    ) {
      event.preventDefault();
      focusSibling(event.key === "ArrowRight" ? 1 : -1);
    }

    if (
      orientation === "vertical" &&
      (event.key === "ArrowDown" || event.key === "ArrowUp")
    ) {
      event.preventDefault();
      focusSibling(event.key === "ArrowDown" ? 1 : -1);
    }
  }}
>
  <slot />
</div>

<style>
  .toolbar {
    display: inline-flex;
    align-items: center;
    gap: var(--flint-space-inline-sm);
    padding: var(--flint-space-inline-sm);
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 78%, transparent);
    border-radius: var(--flint-radius-surface);
    background: color-mix(in srgb, var(--flint-color-background-panel) 94%, transparent);
  }

  .toolbar[data-orientation="vertical"] {
    flex-direction: column;
    align-items: stretch;
  }
</style>

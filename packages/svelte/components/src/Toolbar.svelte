<script lang="ts">
  import type { Snippet } from "svelte";
  import { getFocusableElements } from "./internal";
  import { controlHeightRem, getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    Orientation,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    orientation?: Orientation;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    ariaLabel?: string | null;
    children?: Snippet;
  }

  let {
    orientation = "horizontal",
    size = null,
    sizeRole = "chrome",
    density = null,
    ariaLabel = null,
    children,
  }: Props = $props();

  let rootElement: HTMLDivElement | null = null;
  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const toolbarStyle = $derived(`--poodle-toolbar-control-height: ${controlHeightRem(resolvedSize)}rem;`);

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
  class="poodle-toolbar"
  data-orientation={orientation}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  style={toolbarStyle}
  role="toolbar"
  tabindex="0"
  aria-label={ariaLabel ?? undefined}
  onkeydown={(event) => {
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
  {@render children?.()}
</div>

<style>
  .poodle-toolbar {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.25rem 0.375rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 78%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-recipe-toolbar-fill, color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent));
  }

  /* Size variants */
  .poodle-toolbar[data-size="xs"] { padding: 0.125rem 0.25rem; gap: 0.25rem; }
  .poodle-toolbar[data-size="sm"] { padding: 0.1875rem 0.3125rem; gap: 0.3125rem; }
  .poodle-toolbar[data-size="lg"] { padding: 0.3125rem 0.5rem; gap: 0.5rem; }
  .poodle-toolbar[data-size="xl"] { padding: 0.375rem 0.625rem; gap: 0.625rem; }

  /* Density variants */
  .poodle-toolbar[data-density="compact"] { padding-inline: 0.25rem; gap: 0.25rem; }
  .poodle-toolbar[data-density="comfortable"] { padding-inline: 0.5rem; gap: 0.5rem; }

  .poodle-toolbar[data-orientation="vertical"] {
    flex-direction: column;
    align-items: stretch;
  }
</style>

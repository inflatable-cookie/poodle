<script lang="ts">
  import { getFocusableElements } from "./internal";
  import { controlHeightRem, getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    Orientation,
    SemanticControlSizeRole,
  } from "./types";

  export let orientation: Orientation = "horizontal";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;
  export let ariaLabel: string | null = null;

  let rootElement: HTMLDivElement | null = null;
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
  $: toolbarStyle = `--poodle-toolbar-control-height: ${controlHeightRem(resolvedSize)}rem;`;

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
  data-size={resolvedSize}
  data-density={resolvedDensity}
  style={toolbarStyle}
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
    gap: calc(var(--poodle-space-control-x) * 0.75);
    min-height: var(--poodle-toolbar-control-height, var(--poodle-size-control-height));
    padding: calc(var(--poodle-space-control-x) * 0.375) calc(var(--poodle-space-control-x) * 0.5);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 78%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
  }

  .toolbar[data-density="compact"] {
    gap: calc(var(--poodle-space-control-x) * 0.625);
    padding: calc(var(--poodle-space-control-x) * 0.25) calc(var(--poodle-space-control-x) * 0.375);
  }

  .toolbar[data-density="comfortable"] {
    gap: var(--poodle-space-control-x);
    padding: calc(var(--poodle-space-control-x) * 0.5) calc(var(--poodle-space-control-x) * 0.625);
  }

  .toolbar[data-orientation="vertical"] {
    flex-direction: column;
    align-items: stretch;
  }
</style>

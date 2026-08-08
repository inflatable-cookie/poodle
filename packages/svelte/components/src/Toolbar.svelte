<script lang="ts">
  import "@inflatable-cookie/poodle-styles/toolbar.css";
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


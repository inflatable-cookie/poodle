<script module lang="ts">
  let nextPopoverId = 0;
</script>

<script lang="ts">
  import { onMount, tick, type Snippet } from "svelte";

  import { getFocusableElements } from "./internal";

  import type { OverlayPlacement, PopoverInitialFocus } from "./types";

  interface Props {
    open?: boolean | null;
    defaultOpen?: boolean;
    placement?: OverlayPlacement;
    offset?: number;
    dismissOnOutsideInteract?: boolean;
    initialFocus?: PopoverInitialFocus;
    ariaLabel?: string | null;
    block?: boolean;
    onOpenChange?: ((open: boolean) => void) | undefined;
    trigger?: Snippet<[]>;
    children?: Snippet<[]>;
  }

  let {
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    placement = "bottom-start",
    offset = 8,
    dismissOnOutsideInteract = true,
    initialFocus = "first-focusable",
    ariaLabel = null,
    block = false,
    onOpenChange = undefined,
    trigger,
    children,
  }: Props = $props();

  const popoverId = `poodle-popover-${++nextPopoverId}`;
  let rootElement = $state<HTMLDivElement | null>(null);
  let triggerElement = $state<HTMLDivElement | null>(null);
  let surfaceElement = $state<HTMLDivElement | null>(null);
  let uncontrolledOpen = $state(false);
  let previousOpen = $state(false);
  let seededDefaultOpen = $state(false);

  $effect.pre(() => {
    if (!seededDefaultOpen) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });

  const isControlled = $derived(open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  $effect(() => {
    if (!(isOpen && !previousOpen)) {
      previousOpen = isOpen;
      return;
    }

    tick().then(() => {
      if (!surfaceElement) {
        return;
      }

      if (initialFocus === "content") {
        surfaceElement.focus();
        return;
      }

      if (initialFocus === "first-focusable") {
        getFocusableElements(surfaceElement)[0]?.focus();
      }
    });

    previousOpen = isOpen;
  });

  function setOpen(nextOpen: boolean): void {
    if (isControlled) {
      open = nextOpen;
    } else {
      uncontrolledOpen = nextOpen;
    }

    onOpenChange?.(nextOpen);

    if (!nextOpen) {
      triggerElement?.focus();
    }
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!dismissOnOutsideInteract || !isOpen || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && isOpen) {
        event.preventDefault();
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<div class="poodle-popover" data-block={block} bind:this={rootElement}>
  <div
    bind:this={triggerElement}
    class="poodle-popover__trigger"
    data-block={block}
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? popoverId : undefined}
    onclick={() => setOpen(!isOpen)}
    onkeydown={(event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        setOpen(!isOpen);
      }
    }}
  >
    {@render trigger?.()}
  </div>

  {#if isOpen}
    <div
      bind:this={surfaceElement}
      id={popoverId}
      class="poodle-popover__surface"
      data-placement={placement}
      style={`--poodle-popover-offset: ${offset}px;`}
      tabindex={initialFocus === "content" ? 0 : -1}
      role="dialog"
      aria-label={ariaLabel ?? undefined}
    >
      {@render children?.()}
    </div>
  {/if}
</div>

<style>
  .poodle-popover {
    position: relative;
    display: inline-flex;
  }

  .poodle-popover[data-block="true"] {
    display: flex;
    width: 100%;
    min-width: 0;
  }

  .poodle-popover__trigger {
    display: inline-flex;
  }

  .poodle-popover__trigger[data-block="true"] {
    display: flex;
    width: 100%;
    min-width: 0;
  }

  .poodle-popover__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-popover__surface {
    position: absolute;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 14rem;
    max-width: min(24rem, 90vw);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
    background: var(--poodle-color-background-elevated);
    --poodle-surface: var(--poodle-color-background-elevated);
    box-shadow:
      inset 0 0.0625rem 0 rgba(255, 255, 255, 0.08),
      0 0.625rem 1.5rem rgba(9, 13, 18, 0.22),
      0 0.125rem 0.375rem rgba(0, 0, 0, 0.15);
  }

  .poodle-popover__surface[data-placement^="bottom"] {
    top: calc(100% + var(--poodle-popover-offset));
    left: 0;
  }

  .poodle-popover__surface[data-placement^="top"] {
    bottom: calc(100% + var(--poodle-popover-offset));
    left: 0;
  }

  .poodle-popover__surface[data-placement^="right"] {
    top: 0;
    left: calc(100% + var(--poodle-popover-offset));
  }

  .poodle-popover__surface[data-placement^="left"] {
    top: 0;
    right: calc(100% + var(--poodle-popover-offset));
  }

  .poodle-popover__surface[data-placement$="end"] {
    left: auto;
    right: 0;
  }
</style>

<script context="module" lang="ts">
  let nextPopoverId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { getFocusableElements } from "./internal";

  import type { OverlayPlacement, PopoverInitialFocus } from "./types";

  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placement: OverlayPlacement = "bottom-start";
  export let offset = 8;
  export let dismissOnOutsideInteract = true;
  export let initialFocus: PopoverInitialFocus = "first-focusable";
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
  }>();

  const popoverId = `poodle-popover-${++nextPopoverId}`;
  let rootElement: HTMLDivElement | null = null;
  let triggerElement: HTMLDivElement | null = null;
  let surfaceElement: HTMLDivElement | null = null;
  let uncontrolledOpen = defaultOpen;
  let previousOpen = false;

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;
  $: if (isOpen && !previousOpen) {
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
  }
  $: previousOpen = isOpen;

  function setOpen(nextOpen: boolean): void {
    if (isControlled) {
      open = nextOpen;
    } else {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });

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

<div class="popover" bind:this={rootElement}>
  <div
    bind:this={triggerElement}
    class="popover__trigger"
    role="button"
    tabindex="0"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? popoverId : undefined}
    on:click={() => setOpen(!isOpen)}
    on:keydown={(event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        setOpen(!isOpen);
      }
    }}
  >
    <slot name="trigger" />
  </div>

  {#if isOpen}
    <div
      bind:this={surfaceElement}
      id={popoverId}
      class="popover__surface"
      data-placement={placement}
      style={`--poodle-popover-offset: ${offset}px;`}
      tabindex={initialFocus === "content" ? 0 : -1}
      role="dialog"
      aria-label={ariaLabel ?? undefined}
    >
      <slot />
    </div>
  {/if}
</div>

<style>
  .popover {
    position: relative;
    display: inline-flex;
  }

  .popover__trigger {
    display: inline-flex;
  }

  .popover__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .popover__surface {
    position: absolute;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 14rem;
    max-width: min(24rem, 90vw);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      var(--poodle-color-border-default)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
    background: var(--poodle-color-background-elevated);
    --poodle-surface: var(--poodle-color-background-elevated);
    box-shadow: var(--poodle-treatment-surface-elevated-shadow);
  }

  .popover__surface[data-placement^="bottom"] {
    top: calc(100% + var(--poodle-popover-offset));
    left: 0;
  }

  .popover__surface[data-placement^="top"] {
    bottom: calc(100% + var(--poodle-popover-offset));
    left: 0;
  }

  .popover__surface[data-placement^="right"] {
    top: 0;
    left: calc(100% + var(--poodle-popover-offset));
  }

  .popover__surface[data-placement^="left"] {
    top: 0;
    right: calc(100% + var(--poodle-popover-offset));
  }

  .popover__surface[data-placement$="end"] {
    left: auto;
    right: 0;
  }
</style>

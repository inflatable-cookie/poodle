<script context="module" lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount, tick } from "svelte";

  import Icon from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";
  import Spinner from "./Spinner.svelte";
  import { resolveOverlayPosition } from "./overlay-position";
  import type {
    ButtonTone,
    ButtonVariant,
    ControlDensity,
    ControlSize,
    IconProp,
    OverlayPlacement,
    SemanticControlSizeRole,
  } from "./types";

  export let variant: ButtonVariant = "ghost";
  export let tone: ButtonTone = "default";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let icon: IconProp;
  export let ariaLabel: string;
  export let tooltip: string | null = null;
  export let tooltipPlacement: OverlayPlacement = "top";
  export let disabled = false;
  export let loading = false;
  /** Toggle pressed state. When provided (non-null), acts as a toggle with aria-pressed. */
  export let pressed: boolean | null = null;
  /** Initial pressed state for uncontrolled toggle mode. */
  export let defaultPressed = false;
  export let describedBy: string | null = null;
  export let type: HTMLButtonElement["type"] = "button";

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    pressedChange: { pressed: boolean };
    focus: FocusEvent;
    blur: FocusEvent;
  }>();
  const uiPresentation = getUiPresentation();

  const tooltipId = `poodle-icon-tooltip-${++nextTooltipId}`;
  let tooltipOpen = false;
  let timer: ReturnType<typeof setTimeout> | null = null;
  let buttonElement: HTMLButtonElement | null = null;
  let tooltipElement: HTMLSpanElement | null = null;
  let resolvedTooltipPlacement: OverlayPlacement = tooltipPlacement;
  let tooltipStyle = "";

  let uncontrolledPressed = defaultPressed;

  $: isUnavailable = disabled || loading;
  $: isToggle = pressed !== null || defaultPressed;
  $: pressedControlled = pressed !== null;
  $: currentPressed = pressedControlled ? pressed === true : uncontrolledPressed;
  $: tooltipText = tooltip ?? ariaLabel;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: resolvedIconSize = resolvedSize;
  $: if (tooltipOpen && tooltipText) {
    void updateTooltipPosition();
  }

  function scheduleOpen(): void {
    clearTimer();
    timer = setTimeout(() => (tooltipOpen = true), 300);
  }

  function dismiss(): void {
    clearTimer();
    tooltipOpen = false;
  }

  function clearTimer(): void {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
  }

  async function updateTooltipPosition(): Promise<void> {
    if (!tooltipOpen || !buttonElement) {
      return;
    }

    await tick();

    if (!tooltipElement) {
      return;
    }

    const nextPosition = resolveOverlayPosition(
      buttonElement.getBoundingClientRect(),
      tooltipElement.getBoundingClientRect(),
      tooltipPlacement,
    );

    resolvedTooltipPlacement = nextPosition.placement;
    tooltipStyle = `top: ${nextPosition.top}px; left: ${nextPosition.left}px;`;
  }

  function handleViewportChange(): void {
    if (tooltipOpen) {
      void updateTooltipPosition();
    }
  }

  onMount(() => {
    window.addEventListener("resize", handleViewportChange);
    window.addEventListener("scroll", handleViewportChange, true);

    return () => {
      window.removeEventListener("resize", handleViewportChange);
      window.removeEventListener("scroll", handleViewportChange, true);
    };
  });

  onDestroy(() => clearTimer());
</script>

<span
  class="poodle-icon-button-wrap"
  role="presentation"
  on:mouseenter={scheduleOpen}
  on:mouseleave={dismiss}
>
  <button
    {type}
    bind:this={buttonElement}
    class="poodle-icon-button"
    data-variant={variant}
    data-tone={tone !== "default" ? tone : undefined}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-loading={loading}
    data-pressed={isToggle ? currentPressed : undefined}
    disabled={isUnavailable}
    aria-label={ariaLabel}
    aria-describedby={tooltipOpen ? tooltipId : describedBy ?? undefined}
    aria-busy={loading ? "true" : undefined}
    aria-pressed={isToggle ? (currentPressed ? "true" : "false") : undefined}
    on:click={(event) => {
      if (isToggle) {
        const next = !currentPressed;
        if (pressedControlled) {
          pressed = next;
        } else {
          uncontrolledPressed = next;
        }
        dispatch("pressedChange", { pressed: next });
      }
      dispatch("click", event);
    }}
    on:focus={scheduleOpen}
    on:blur={dismiss}
    on:focus={(event) => dispatch("focus", event)}
    on:blur={(event) => dispatch("blur", event)}
    on:keydown={(event) => {
      if (event.key === "Escape") dismiss();
    }}
  >
    {#if loading}
      <span class="poodle-icon-button__spinner" aria-hidden="true">
        <Spinner variant="ring" size={resolvedIconSize} tone="current" />
      </span>
    {:else}
      <span class="poodle-icon-button__glyph" aria-hidden="true">
        <slot><Icon icon={icon} size={resolvedIconSize} /></slot>
      </span>
    {/if}
  </button>

  {#if tooltipOpen && tooltipText}
    <span
      id={tooltipId}
      bind:this={tooltipElement}
      class="poodle-icon-button__tooltip"
      data-placement={resolvedTooltipPlacement}
      style={tooltipStyle}
      role="tooltip"
    >
      {tooltipText}
    </span>
  {/if}
</span>

<style>
  .poodle-icon-button-wrap {
    position: relative;
    display: inline-flex;
  }

  .poodle-icon-button {
    --poodle-icon-button-fill: transparent;
    --poodle-icon-button-fill-hover: color-mix(
      in srgb,
      var(--poodle-icon-button-fill) 76%,
      var(--poodle-color-background-elevated)
    );
    --poodle-icon-button-fill-active: color-mix(
      in srgb,
      var(--poodle-icon-button-fill) 64%,
      var(--poodle-color-background-elevated)
    );
    --poodle-icon-button-border: transparent;
    --poodle-icon-button-shadow: none;
    --poodle-icon-button-text: var(--poodle-color-text-primary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-size-control-height);
    height: var(--poodle-size-control-height);
    padding: 0;
    border: 0.0625rem solid var(--poodle-icon-button-border);
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: var(--poodle-icon-button-fill);
    box-shadow: var(--poodle-icon-button-shadow);
    color: var(--poodle-icon-button-text);
    cursor: pointer;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }


  .poodle-icon-button[data-variant="primary"] {
    --poodle-icon-button-fill: var(
      --poodle-treatment-interactive-primary-fill,
      var(--poodle-color-accent-base)
    );
    --poodle-icon-button-border: var(
      --poodle-treatment-interactive-primary-border,
      color-mix(in srgb, var(--poodle-color-accent-base) 84%, black)
    );
    --poodle-icon-button-text: var(
      --poodle-treatment-interactive-primary-text,
      var(--poodle-color-text-inverse)
    );
    --poodle-icon-button-shadow: var(
      --poodle-treatment-interactive-primary-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)
    );
  }

  .poodle-icon-button[data-variant="secondary"] {
    --poodle-icon-button-fill: var(
      --poodle-treatment-interactive-fill,
      var(--poodle-color-background-surface)
    );
    --poodle-icon-button-border: var(
      --poodle-treatment-interactive-border,
      var(--poodle-color-border-default)
    );
    --poodle-icon-button-shadow: var(
      --poodle-treatment-interactive-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)
    );
  }

  .poodle-icon-button[data-tone="danger"] {
    --poodle-icon-button-fill: color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-color-background-surface));
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default));
  }

  .poodle-icon-button[data-tone="success"] {
    --poodle-icon-button-fill: color-mix(in srgb, var(--poodle-color-status-success) 16%, var(--poodle-color-background-surface));
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-success) 46%, var(--poodle-color-border-default));
  }

  .poodle-icon-button[data-variant="primary"][data-tone="danger"] {
    --poodle-icon-button-fill: var(--poodle-color-status-danger);
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 84%, black);
    --poodle-icon-button-text: var(--poodle-color-text-inverse);
  }

  .poodle-icon-button[data-variant="primary"][data-tone="success"] {
    --poodle-icon-button-fill: var(--poodle-color-status-success);
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-success) 84%, black);
    --poodle-icon-button-text: var(--poodle-color-text-inverse);
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="danger"] {
    --poodle-icon-button-fill: transparent;
    --poodle-icon-button-border: transparent;
    --poodle-icon-button-text: var(--poodle-color-status-danger);
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="success"] {
    --poodle-icon-button-fill: transparent;
    --poodle-icon-button-border: transparent;
    --poodle-icon-button-text: var(--poodle-color-status-success);
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="danger"]:hover:not(:disabled) {
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default));
    background: color-mix(in srgb, var(--poodle-color-status-danger) 10%, transparent);
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="success"]:hover:not(:disabled) {
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-success) 46%, var(--poodle-color-border-default));
    background: color-mix(in srgb, var(--poodle-color-status-success) 10%, transparent);
  }

  .poodle-icon-button[data-pressed="true"]:not([data-variant="primary"]) {
    --poodle-icon-button-fill: var(--poodle-color-accent-base);
    --poodle-icon-button-fill-hover: color-mix(in srgb, white 12%, var(--poodle-color-accent-base));
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-accent-base) 85%, black);
    --poodle-icon-button-text: var(--poodle-color-text-inverse);
    --poodle-icon-button-shadow: none;
  }

  .poodle-icon-button:hover:not(:disabled) {
    background: var(--poodle-icon-button-fill-hover);
    border-color: color-mix(in srgb, var(--poodle-icon-button-border) 74%, var(--poodle-color-text-primary));
    box-shadow: var(--poodle-treatment-interactive-shadow-active, var(--poodle-icon-button-shadow));
  }

  .poodle-icon-button:active:not(:disabled) {
    background: var(--poodle-icon-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .poodle-icon-button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-icon-button:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-icon-button__glyph,
  .poodle-icon-button__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 45%;
    height: 45%;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.875rem;
    line-height: 1;
  }

  .poodle-icon-button__glyph :global(svg) {
    width: 100%;
    height: 100%;
  }

  .poodle-icon-button__spinner {
    width: 45%;
    height: 45%;
  }

  .poodle-icon-button__spinner :global(.poodle-spinner) {
    width: 100%;
    height: 100%;
  }

  .poodle-icon-button[data-size="xs"] {
    width: 1.5rem;
    height: 1.5rem;
  }

  .poodle-icon-button[data-size="sm"] {
    width: 1.75rem;
    height: 1.75rem;
  }

  .poodle-icon-button[data-size="lg"] {
    width: 2.5rem;
    height: 2.5rem;
  }

  .poodle-icon-button[data-size="xl"] {
    width: 2.75rem;
    height: 2.75rem;
  }

  /* ── Tooltip ── */


  .poodle-icon-button__tooltip {
    position: fixed;
    z-index: var(--poodle-overlay-z-menu);
    max-width: 16rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel));
    box-shadow: var(--poodle-elevation-overlay);
    color: var(--poodle-color-text-primary);
    font-size: 0.6875rem;
    line-height: 1.35;
    white-space: nowrap;
    pointer-events: none;
  }
</style>

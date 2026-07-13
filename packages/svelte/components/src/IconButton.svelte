<script module lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import { hoverTransition, type HoverEvent as HoverMachineEvent, type HoverState } from "@poodle/headless";
  import { onDestroy, onMount, tick, type Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
  } from "./presentation";
  import { default as Spinner } from "./Spinner.svelte";
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

  interface Props {
    variant?: ButtonVariant;
    tone?: ButtonTone;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    icon: IconProp;
    ariaLabel: string;
    tooltip?: string | null;
    tooltipPlacement?: OverlayPlacement;
    disabled?: boolean;
    loading?: boolean;
    pressed?: boolean | null;
    defaultPressed?: boolean | null;
    describedBy?: string | null;
    type?: HTMLButtonElement["type"];
    onClick?: ((event: MouseEvent) => void) | null;
    onFocus?: ((event: FocusEvent) => void) | null;
    onBlur?: ((event: FocusEvent) => void) | null;
    onPressedChange?: ((pressed: boolean) => void) | null;
    children?: Snippet<[]>;
  }

  let {
    variant = "primary",
    tone = "default",
    size = null,
    sizeRole = "control",
    density = null,
    icon,
    ariaLabel,
    tooltip = null,
    tooltipPlacement = "top",
    disabled = false,
    loading = false,
    pressed = $bindable<boolean | null>(null),
    defaultPressed = null,
    describedBy = null,
    type = "button",
    onClick = null,
    onFocus = null,
    onBlur = null,
    onPressedChange = null,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const tooltipId = `poodle-icon-tooltip-${++nextTooltipId}`;
  let tooltipOpen = $state(false);
  let timer = $state<ReturnType<typeof setTimeout> | null>(null);
  let buttonElement = $state<HTMLButtonElement | null>(null);
  let tooltipElement = $state<HTMLSpanElement | null>(null);
  let resolvedTooltipPlacement = $state<OverlayPlacement>("top");
  let tooltipStyle = $state("");
  let seededDefaultPressed = $state(false);
  let uncontrolledPressed = $state(false);

  $effect.pre(() => {
    if (!seededDefaultPressed && pressed === null) {
      uncontrolledPressed = defaultPressed === true;
      seededDefaultPressed = true;
    }
  });

  const isUnavailable = $derived(disabled || loading);
  const isToggle = $derived(pressed !== null || defaultPressed !== null);
  const pressedControlled = $derived(pressed !== null);
  const currentPressed = $derived(pressedControlled ? pressed === true : uncontrolledPressed);
  const tooltipText = $derived(tooltip ?? ariaLabel);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedIconSize = $derived(resolvedSize);

  $effect(() => {
    resolvedTooltipPlacement = tooltipPlacement;
  });

  $effect(() => {
    if (tooltipOpen && tooltipText) {
      void updateTooltipPosition();
    }
  });

  let hoverMachineState: HoverState = "closed";

  function sendHover(event: HoverMachineEvent): void {
    const result = hoverTransition(hoverMachineState, { openDelayMs: 300, closeDelayMs: 0 }, event);
    hoverMachineState = result.state;

    for (const effect of result.effects) {
      if (effect.type === "clearTimer") {
        clearTimer();
      } else if (effect.type === "startTimer") {
        clearTimer();
        timer = setTimeout(() => sendHover({ type: "TIMER_FIRE" }), effect.ms);
      } else if (effect.type === "emitOpenChange") {
        tooltipOpen = effect.open;
      }
    }
  }

  function scheduleOpen(): void {
    sendHover({ type: "ENTER" });
  }

  function dismiss(): void {
    sendHover({ type: "DISMISS" });
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

  function handleFocus(event: FocusEvent): void {
    scheduleOpen();
    onFocus?.(event);
  }

  function handleBlur(event: FocusEvent): void {
    dismiss();
    onBlur?.(event);
  }

  function handleClick(event: MouseEvent): void {
    if (isToggle) {
      const next = !currentPressed;
      if (pressedControlled) {
        pressed = next;
      } else {
        uncontrolledPressed = next;
      }
      onPressedChange?.(next);
    }
    onClick?.(event);
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
  onmouseenter={scheduleOpen}
  onmouseleave={dismiss}
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
    onclick={handleClick}
    onfocus={handleFocus}
    onblur={handleBlur}
    onkeydown={(event) => {
      if (event.key === "Escape") dismiss();
    }}
  >
    {#if loading}
      <span class="poodle-icon-button__spinner" aria-hidden="true">
        <Spinner variant="ring" size={resolvedIconSize} tone="current" />
      </span>
    {:else}
      <span class="poodle-icon-button__glyph" aria-hidden="true">
        {#if children}
          {@render children()}
        {:else}
          <Icon icon={icon} size={resolvedIconSize} />
        {/if}
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
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-fill, transparent);
    --poodle-icon-button-fill-hover: var(--poodle-recipe-icon-button-fill-hover, color-mix(
      in srgb,
      var(--poodle-icon-button-fill) 76%,
      var(--poodle-color-background-elevated)
    ));
    --poodle-icon-button-fill-active: var(--poodle-recipe-icon-button-fill-active, color-mix(
      in srgb,
      var(--poodle-icon-button-fill) 64%,
      var(--poodle-color-background-elevated)
    ));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-border, transparent);
    --poodle-icon-button-shadow: var(--poodle-recipe-icon-button-shadow, none);
    --poodle-icon-button-text: var(--poodle-recipe-icon-button-text, var(--poodle-color-text-primary));
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-size-control-height);
    height: var(--poodle-size-control-height);
    padding: 0;
    border: 0.0625rem solid var(--poodle-icon-button-border);
    border-radius: var(--poodle-radius-control);
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
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-primary-fill, var(--poodle-color-accent-base));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-primary-border, color-mix(in srgb, var(--poodle-color-accent-base) 84%, black));
    --poodle-icon-button-text: var(--poodle-recipe-icon-button-primary-text, var(--poodle-color-text-inverse));
    --poodle-icon-button-shadow: var(--poodle-recipe-icon-button-primary-shadow, inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent));
  }

  .poodle-icon-button[data-variant="secondary"] {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-secondary-fill, var(--poodle-color-background-surface));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-secondary-border, var(--poodle-color-border-default));
    --poodle-icon-button-shadow: var(--poodle-recipe-icon-button-secondary-shadow, inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent));
  }

  .poodle-icon-button[data-tone="danger"] {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-danger-fill, color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-color-background-surface)));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-danger-border, color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default)));
  }

  .poodle-icon-button[data-tone="success"] {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-success-fill, color-mix(in srgb, var(--poodle-color-status-success) 16%, var(--poodle-color-background-surface)));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-success-border, color-mix(in srgb, var(--poodle-color-status-success) 46%, var(--poodle-color-border-default)));
  }

  .poodle-icon-button[data-variant="primary"][data-tone="danger"] {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-primary-danger-fill, var(--poodle-color-status-danger));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-primary-danger-border, color-mix(in srgb, var(--poodle-color-status-danger) 84%, black));
    --poodle-icon-button-text: var(--poodle-recipe-icon-button-primary-danger-text, var(--poodle-color-text-inverse));
  }

  .poodle-icon-button[data-variant="primary"][data-tone="success"] {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-primary-success-fill, var(--poodle-color-status-success));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-primary-success-border, color-mix(in srgb, var(--poodle-color-status-success) 84%, black));
    --poodle-icon-button-text: var(--poodle-recipe-icon-button-primary-success-text, var(--poodle-color-text-inverse));
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="danger"] {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-ghost-danger-fill, transparent);
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-ghost-danger-border, transparent);
    --poodle-icon-button-text: var(--poodle-recipe-icon-button-ghost-danger-text, var(--poodle-color-status-danger));
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="success"] {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-ghost-success-fill, transparent);
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-ghost-success-border, transparent);
    --poodle-icon-button-text: var(--poodle-recipe-icon-button-ghost-success-text, var(--poodle-color-status-success));
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="danger"]:hover:not(:disabled) {
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-ghost-danger-hover-border, color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default)));
    background: color-mix(in srgb, var(--poodle-color-status-danger) 10%, transparent);
  }

  .poodle-icon-button[data-variant="ghost"][data-tone="success"]:hover:not(:disabled) {
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-ghost-success-hover-border, color-mix(in srgb, var(--poodle-color-status-success) 46%, var(--poodle-color-border-default)));
    background: color-mix(in srgb, var(--poodle-color-status-success) 10%, transparent);
  }

  .poodle-icon-button[data-pressed="true"]:not([data-variant="primary"]) {
    --poodle-icon-button-fill: var(--poodle-recipe-icon-button-primary-fill, var(--poodle-color-accent-base));
    --poodle-icon-button-fill-hover: var(--poodle-recipe-icon-button-primary-fill-hover, color-mix(in srgb, white 12%, var(--poodle-color-accent-base)));
    --poodle-icon-button-border: var(--poodle-recipe-icon-button-primary-border, color-mix(in srgb, var(--poodle-color-accent-base) 85%, black));
    --poodle-icon-button-text: var(--poodle-recipe-icon-button-primary-text, var(--poodle-color-text-inverse));
    --poodle-icon-button-shadow: var(--poodle-recipe-icon-button-primary-shadow, none);
  }

  .poodle-icon-button:hover:not(:disabled) {
    background: var(--poodle-icon-button-fill-hover);
    border-color: color-mix(in srgb, var(--poodle-icon-button-border) 74%, var(--poodle-color-text-primary));
    box-shadow: var(--poodle-icon-button-shadow);
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

  .poodle-icon-button[data-size="md"] {
    width: 2.25rem;
    height: 2.25rem;
  }

  .poodle-icon-button[data-size="lg"] {
    width: 2.75rem;
    height: 2.75rem;
  }

  .poodle-icon-button[data-size="xl"] {
    width: 3.25rem;
    height: 3.25rem;
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

<script module lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import "@poodle/styles/icon-button.css";
  import { hoverTransition, type HoverEvent as HoverMachineEvent, type HoverState } from "@poodle/headless";
  import { onDestroy, type Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
  } from "./presentation";
  import { default as Spinner } from "./Spinner.svelte";
  import { anchored } from "./anchored";
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
    expanded?: boolean | null;
    controls?: string | null;
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
    expanded = null,
    controls = null,
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

  function handleFocus(event: FocusEvent): void {
    scheduleOpen();
    onFocus?.(event);
  }

  function handleBlur(event: FocusEvent): void {
    dismiss();
    onBlur?.(event);
  }

  function handleClick(event: MouseEvent): void {
    dismiss();
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
    aria-expanded={expanded === null ? undefined : expanded ? "true" : "false"}
    aria-controls={controls ?? undefined}
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
      use:anchored={{
        anchor: buttonElement,
        placement: tooltipPlacement,
        onPlacement: (next) => (resolvedTooltipPlacement = next),
      }}
      class="poodle-icon-button__tooltip"
      data-placement={resolvedTooltipPlacement}
      role="tooltip"
    >
      {tooltipText}
    </span>
  {/if}
</span>

<script context="module" lang="ts">
  let nextTooltipId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";

  import Icon from "./Icon.svelte";
  import type { ButtonTone, ButtonVariant, ControlSize, IconProp, OverlayPlacement } from "./types";

  export let variant: ButtonVariant = "ghost";
  export let tone: ButtonTone = "default";
  export let size: ControlSize = "md";
  export let icon: IconProp;
  export let ariaLabel: string;
  export let tooltip: string | null = null;
  export let tooltipPlacement: OverlayPlacement = "top";
  export let isDisabled = false;
  export let isLoading = false;
  export let isPressed: boolean | null = null;
  export let describedBy: string | null = null;
  export let type: HTMLButtonElement["type"] = "button";

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  const tooltipId = `flint-icon-tooltip-${++nextTooltipId}`;
  let tooltipOpen = false;
  let timer: ReturnType<typeof setTimeout> | null = null;

  $: isUnavailable = isDisabled || isLoading;
  $: tooltipText = tooltip ?? ariaLabel;

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

  onDestroy(() => clearTimer());
</script>

<span
  class="icon-button-wrap"
  role="presentation"
  on:mouseenter={scheduleOpen}
  on:mouseleave={dismiss}
>
  <button
    {type}
    class="icon-button"
    data-variant={variant}
    data-tone={tone !== "default" ? tone : undefined}
    data-size={size}
    data-loading={isLoading}
    data-pressed={isPressed === true}
    disabled={isUnavailable}
    aria-label={ariaLabel}
    aria-describedby={tooltipOpen ? tooltipId : describedBy ?? undefined}
    aria-busy={isLoading ? "true" : undefined}
    aria-pressed={isPressed === null ? undefined : isPressed ? "true" : "false"}
    on:click={(event) => dispatch("click", event)}
    on:focus={scheduleOpen}
    on:blur={dismiss}
    on:focus={(event) => dispatch("focus", event)}
    on:blur={(event) => dispatch("blur", event)}
    on:keydown={(event) => {
      if (event.key === "Escape") dismiss();
    }}
  >
    {#if isLoading}
      <span class="icon-button__spinner" aria-hidden="true"></span>
    {:else}
      <span class="icon-button__glyph" aria-hidden="true">
        <slot><Icon icon={icon} size="md" /></slot>
      </span>
    {/if}
  </button>

  {#if tooltipOpen && tooltipText}
    <span id={tooltipId} class="icon-button__tooltip" data-placement={tooltipPlacement} role="tooltip">
      {tooltipText}
    </span>
  {/if}
</span>

<style>
  .icon-button-wrap {
    position: relative;
    display: inline-flex;
  }

  .icon-button {
    --flint-icon-button-fill: transparent;
    --flint-icon-button-fill-hover: color-mix(
      in srgb,
      var(--flint-icon-button-fill) 76%,
      var(--flint-color-background-elevated)
    );
    --flint-icon-button-fill-active: color-mix(
      in srgb,
      var(--flint-icon-button-fill) 64%,
      var(--flint-color-background-elevated)
    );
    --flint-icon-button-border: transparent;
    --flint-icon-button-shadow: none;
    --flint-icon-button-text: var(--flint-color-text-primary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--flint-size-control-height);
    height: var(--flint-size-control-height);
    padding: 0;
    border: 0.0625rem solid var(--flint-icon-button-border);
    border-radius: var(--flint-treatment-interactive-radius, var(--flint-radius-control));
    background: var(--flint-icon-button-fill);
    box-shadow: var(--flint-icon-button-shadow);
    color: var(--flint-icon-button-text);
    cursor: pointer;
    transition:
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      box-shadow var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      transform var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .icon-button[data-size="sm"] {
    width: calc(var(--flint-size-control-height) - 0.375rem);
    height: calc(var(--flint-size-control-height) - 0.375rem);
  }

  .icon-button[data-size="lg"] {
    width: calc(var(--flint-size-control-height) + 0.375rem);
    height: calc(var(--flint-size-control-height) + 0.375rem);
  }

  .icon-button[data-variant="primary"] {
    --flint-icon-button-fill: var(
      --flint-treatment-interactive-primary-fill,
      var(--flint-color-accent-base)
    );
    --flint-icon-button-border: var(
      --flint-treatment-interactive-primary-border,
      color-mix(in srgb, var(--flint-color-accent-base) 84%, black)
    );
    --flint-icon-button-text: var(
      --flint-treatment-interactive-primary-text,
      var(--flint-color-text-inverse)
    );
    --flint-icon-button-shadow: var(
      --flint-treatment-interactive-primary-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)
    );
  }

  .icon-button[data-variant="secondary"] {
    --flint-icon-button-fill: var(
      --flint-treatment-interactive-fill,
      var(--flint-color-background-surface)
    );
    --flint-icon-button-border: var(
      --flint-treatment-interactive-border,
      var(--flint-color-border-default)
    );
    --flint-icon-button-shadow: var(
      --flint-treatment-interactive-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)
    );
  }

  .icon-button[data-tone="danger"] {
    --flint-icon-button-fill: color-mix(in srgb, var(--flint-color-status-danger) 16%, var(--flint-color-background-surface));
    --flint-icon-button-border: color-mix(in srgb, var(--flint-color-status-danger) 46%, var(--flint-color-border-default));
  }

  .icon-button[data-variant="primary"][data-tone="danger"] {
    --flint-icon-button-fill: var(--flint-color-status-danger);
    --flint-icon-button-border: color-mix(in srgb, var(--flint-color-status-danger) 84%, black);
    --flint-icon-button-text: var(--flint-color-text-inverse);
  }

  .icon-button[data-variant="ghost"][data-tone="danger"] {
    --flint-icon-button-fill: transparent;
    --flint-icon-button-border: transparent;
    --flint-icon-button-text: var(--flint-color-status-danger);
  }

  .icon-button[data-variant="ghost"][data-tone="danger"]:hover:not(:disabled) {
    --flint-icon-button-border: color-mix(in srgb, var(--flint-color-status-danger) 46%, var(--flint-color-border-default));
    background: color-mix(in srgb, var(--flint-color-status-danger) 10%, transparent);
  }

  .icon-button[data-pressed="true"] {
    background: color-mix(in srgb, var(--flint-color-accent-base) 20%, var(--flint-icon-button-fill));
    border-color: color-mix(in srgb, var(--flint-color-accent-base) 56%, var(--flint-icon-button-border));
    box-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-accent-base) 18%, transparent);
  }

  .icon-button:hover:not(:disabled) {
    background: var(--flint-icon-button-fill-hover);
    border-color: color-mix(in srgb, var(--flint-icon-button-border) 74%, var(--flint-color-text-primary));
    box-shadow: var(--flint-treatment-interactive-shadow-active, var(--flint-icon-button-shadow));
  }

  .icon-button:active:not(:disabled) {
    background: var(--flint-icon-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .icon-button:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .icon-button:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }

  .icon-button__glyph,
  .icon-button__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 45%;
    height: 45%;
    font-family: var(--flint-typography-code-family);
    font-size: 0.875rem;
    line-height: 1;
  }

  .icon-button__glyph :global(svg) {
    width: 100%;
    height: 100%;
  }

  .icon-button__spinner {
    border: 0.125rem solid color-mix(in srgb, currentColor 24%, transparent);
    border-top-color: currentColor;
    border-radius: 999px;
    animation: icon-button-spinner 0.8s linear infinite;
  }

  @keyframes icon-button-spinner {
    to {
      transform: rotate(360deg);
    }
  }

  /* ── Tooltip ── */

  .icon-button__tooltip {
    position: absolute;
    z-index: var(--flint-overlay-z-menu);
    max-width: 16rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-default) 72%, transparent);
    border-radius: calc(var(--flint-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel));
    box-shadow: var(--flint-elevation-overlay);
    color: var(--flint-color-text-primary);
    font-size: 0.6875rem;
    line-height: 1.35;
    white-space: nowrap;
    pointer-events: none;
  }

  .icon-button__tooltip[data-placement^="top"] {
    bottom: calc(100% + 0.375rem);
    left: 50%;
    transform: translateX(-50%);
  }

  .icon-button__tooltip[data-placement^="bottom"] {
    top: calc(100% + 0.375rem);
    left: 50%;
    transform: translateX(-50%);
  }

  .icon-button__tooltip[data-placement^="left"] {
    top: 50%;
    right: calc(100% + 0.375rem);
    transform: translateY(-50%);
  }

  .icon-button__tooltip[data-placement^="right"] {
    top: 50%;
    left: calc(100% + 0.375rem);
    transform: translateY(-50%);
  }
</style>

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

  const tooltipId = `poodle-icon-tooltip-${++nextTooltipId}`;
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

  .icon-button[data-size="sm"] {
    width: calc(var(--poodle-size-control-height) - 0.375rem);
    height: calc(var(--poodle-size-control-height) - 0.375rem);
  }

  .icon-button[data-size="lg"] {
    width: calc(var(--poodle-size-control-height) + 0.375rem);
    height: calc(var(--poodle-size-control-height) + 0.375rem);
  }

  .icon-button[data-variant="primary"] {
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

  .icon-button[data-variant="secondary"] {
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

  .icon-button[data-tone="danger"] {
    --poodle-icon-button-fill: color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-color-background-surface));
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default));
  }

  .icon-button[data-variant="primary"][data-tone="danger"] {
    --poodle-icon-button-fill: var(--poodle-color-status-danger);
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 84%, black);
    --poodle-icon-button-text: var(--poodle-color-text-inverse);
  }

  .icon-button[data-variant="ghost"][data-tone="danger"] {
    --poodle-icon-button-fill: transparent;
    --poodle-icon-button-border: transparent;
    --poodle-icon-button-text: var(--poodle-color-status-danger);
  }

  .icon-button[data-variant="ghost"][data-tone="danger"]:hover:not(:disabled) {
    --poodle-icon-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default));
    background: color-mix(in srgb, var(--poodle-color-status-danger) 10%, transparent);
  }

  .icon-button[data-pressed="true"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 20%, var(--poodle-icon-button-fill));
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 56%, var(--poodle-icon-button-border));
    box-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent);
  }

  .icon-button:hover:not(:disabled) {
    background: var(--poodle-icon-button-fill-hover);
    border-color: color-mix(in srgb, var(--poodle-icon-button-border) 74%, var(--poodle-color-text-primary));
    box-shadow: var(--poodle-treatment-interactive-shadow-active, var(--poodle-icon-button-shadow));
  }

  .icon-button:active:not(:disabled) {
    background: var(--poodle-icon-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .icon-button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .icon-button:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .icon-button__glyph,
  .icon-button__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 45%;
    height: 45%;
    font-family: var(--poodle-typography-code-family);
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

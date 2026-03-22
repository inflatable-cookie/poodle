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

  const tooltipId = `pug-icon-tooltip-${++nextTooltipId}`;
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
    --pug-icon-button-fill: transparent;
    --pug-icon-button-fill-hover: color-mix(
      in srgb,
      var(--pug-icon-button-fill) 76%,
      var(--pug-color-background-elevated)
    );
    --pug-icon-button-fill-active: color-mix(
      in srgb,
      var(--pug-icon-button-fill) 64%,
      var(--pug-color-background-elevated)
    );
    --pug-icon-button-border: transparent;
    --pug-icon-button-shadow: none;
    --pug-icon-button-text: var(--pug-color-text-primary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--pug-size-control-height);
    height: var(--pug-size-control-height);
    padding: 0;
    border: 0.0625rem solid var(--pug-icon-button-border);
    border-radius: var(--pug-treatment-interactive-radius, var(--pug-radius-control));
    background: var(--pug-icon-button-fill);
    box-shadow: var(--pug-icon-button-shadow);
    color: var(--pug-icon-button-text);
    cursor: pointer;
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      transform var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .icon-button[data-size="sm"] {
    width: calc(var(--pug-size-control-height) - 0.375rem);
    height: calc(var(--pug-size-control-height) - 0.375rem);
  }

  .icon-button[data-size="lg"] {
    width: calc(var(--pug-size-control-height) + 0.375rem);
    height: calc(var(--pug-size-control-height) + 0.375rem);
  }

  .icon-button[data-variant="primary"] {
    --pug-icon-button-fill: var(
      --pug-treatment-interactive-primary-fill,
      var(--pug-color-accent-base)
    );
    --pug-icon-button-border: var(
      --pug-treatment-interactive-primary-border,
      color-mix(in srgb, var(--pug-color-accent-base) 84%, black)
    );
    --pug-icon-button-text: var(
      --pug-treatment-interactive-primary-text,
      var(--pug-color-text-inverse)
    );
    --pug-icon-button-shadow: var(
      --pug-treatment-interactive-primary-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)
    );
  }

  .icon-button[data-variant="secondary"] {
    --pug-icon-button-fill: var(
      --pug-treatment-interactive-fill,
      var(--pug-color-background-surface)
    );
    --pug-icon-button-border: var(
      --pug-treatment-interactive-border,
      var(--pug-color-border-default)
    );
    --pug-icon-button-shadow: var(
      --pug-treatment-interactive-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)
    );
  }

  .icon-button[data-tone="danger"] {
    --pug-icon-button-fill: color-mix(in srgb, var(--pug-color-status-danger) 16%, var(--pug-color-background-surface));
    --pug-icon-button-border: color-mix(in srgb, var(--pug-color-status-danger) 46%, var(--pug-color-border-default));
  }

  .icon-button[data-variant="primary"][data-tone="danger"] {
    --pug-icon-button-fill: var(--pug-color-status-danger);
    --pug-icon-button-border: color-mix(in srgb, var(--pug-color-status-danger) 84%, black);
    --pug-icon-button-text: var(--pug-color-text-inverse);
  }

  .icon-button[data-variant="ghost"][data-tone="danger"] {
    --pug-icon-button-fill: transparent;
    --pug-icon-button-border: transparent;
    --pug-icon-button-text: var(--pug-color-status-danger);
  }

  .icon-button[data-variant="ghost"][data-tone="danger"]:hover:not(:disabled) {
    --pug-icon-button-border: color-mix(in srgb, var(--pug-color-status-danger) 46%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-color-status-danger) 10%, transparent);
  }

  .icon-button[data-pressed="true"] {
    background: color-mix(in srgb, var(--pug-color-accent-base) 20%, var(--pug-icon-button-fill));
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 56%, var(--pug-icon-button-border));
    box-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
  }

  .icon-button:hover:not(:disabled) {
    background: var(--pug-icon-button-fill-hover);
    border-color: color-mix(in srgb, var(--pug-icon-button-border) 74%, var(--pug-color-text-primary));
    box-shadow: var(--pug-treatment-interactive-shadow-active, var(--pug-icon-button-shadow));
  }

  .icon-button:active:not(:disabled) {
    background: var(--pug-icon-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .icon-button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .icon-button:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .icon-button__glyph,
  .icon-button__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 45%;
    height: 45%;
    font-family: var(--pug-typography-code-family);
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
    z-index: var(--pug-overlay-z-menu);
    max-width: 16rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel));
    box-shadow: var(--pug-elevation-overlay);
    color: var(--pug-color-text-primary);
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

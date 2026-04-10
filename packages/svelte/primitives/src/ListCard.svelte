<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let title: string;
  export let subtitle: string | null = null;
  export let meta: string | null = null;
  export let href: string | null = null;
  export let leadingShape: "circle" | "rounded-square" = "circle";
  export let leadingFill: "tint" | "solid" = "tint";
  export let accentColor: string | null = null;
  export let layout: "default" | "compact" = "default";
  export let interactive = false;
  export let disabled = false;
  export let selectable = false;
  export let selected = false;
  export let showReorderHandle = false;
  export let notLive = false;
  export let sash: string | null = null;
  export let sashColor: string | null = null;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    selectedChange: { selected: boolean };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isCompact = layout === "compact";
  $: isInteractive = Boolean(href) || interactive || selectable;

  function handleClick(event: MouseEvent) {
    if (disabled) return;

    if (selectable) {
      event.preventDefault();
      dispatch("selectedChange", { selected: !selected });
      return;
    }

    if (interactive || href) {
      dispatch("click", event);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (disabled || href) return;

    if ((interactive || selectable) && (event.key === "Enter" || event.key === " ")) {
      event.preventDefault();
      if (selectable) {
        dispatch("selectedChange", { selected: !selected });
      } else {
        dispatch("click", new MouseEvent("click"));
      }
    }
  }
</script>

{#if href && !disabled && !selectable}
  <a
    class="list-card"
    class:list-card--interactive={isInteractive}
    href={href}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-disabled={disabled}
    data-not-live={notLive}
    data-leading-shape={leadingShape}
    data-leading-fill={leadingFill}
    data-layout={layout}
    data-selected={selected}
    aria-label={ariaLabel ?? title}
    class:list-card--has-sash={!!sash}
    style={[
      accentColor ? `--list-card-accent: ${accentColor}` : '',
      sashColor ? `--list-card-sash: ${sashColor}` : '',
    ].filter(Boolean).join('; ') || undefined}
    on:click={handleClick}
  >
    {#if sash}
      <span class="list-card__sash" aria-label={sash}>{sash}</span>
    {/if}

    {#if showReorderHandle}
      <span class="list-card__handle" aria-hidden="true">
        <svg viewBox="0 0 16 16" fill="currentColor">
          <circle cx="5" cy="4" r="1.1"></circle>
          <circle cx="5" cy="8" r="1.1"></circle>
          <circle cx="5" cy="12" r="1.1"></circle>
          <circle cx="11" cy="4" r="1.1"></circle>
          <circle cx="11" cy="8" r="1.1"></circle>
          <circle cx="11" cy="12" r="1.1"></circle>
        </svg>
      </span>
    {/if}

    {#if $$slots.leading}
      <span class="list-card__leading">
        <slot name="leading" />
      </span>
    {/if}

    <div class="list-card__body">
      <div class="list-card__header">
        <span class="list-card__title">
          <slot name="title">{title}</slot>
        </span>
        {#if $$slots.badges}
          <span class="list-card__badges">
            <slot name="badges" />
          </span>
        {/if}
      </div>
      {#if subtitle}
        <span class="list-card__subtitle">{subtitle}</span>
      {/if}
      {#if $$slots.footer}
        <div class="list-card__footer">
          <slot name="footer" />
        </div>
      {/if}
    </div>

    {#if meta && !isCompact}
      <span class="list-card__meta">{meta}</span>
    {/if}

    {#if $$slots.actions}
      <span class="list-card__actions">
        <slot name="actions" />
      </span>
    {/if}

    {#if $$slots.trailing}
      <span class="list-card__trailing">
        <slot name="trailing" />
      </span>
    {/if}
  </a>
{:else}
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <div
    class="list-card"
    class:list-card--interactive={isInteractive}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-disabled={disabled}
    data-not-live={notLive}
    data-leading-shape={leadingShape}
    data-leading-fill={leadingFill}
    data-layout={layout}
    data-selected={selected}
    role={isInteractive ? (selectable ? "button" : "button") : undefined}
    aria-pressed={selectable ? selected : undefined}
    tabindex={isInteractive && !disabled ? 0 : -1}
    aria-label={ariaLabel ?? title}
    class:list-card--has-sash={!!sash}
    style={[
      accentColor ? `--list-card-accent: ${accentColor}` : '',
      sashColor ? `--list-card-sash: ${sashColor}` : '',
    ].filter(Boolean).join('; ') || undefined}
    on:click={handleClick}
    on:keydown={handleKeydown}
  >
    {#if sash}
      <span class="list-card__sash" aria-label={sash}>{sash}</span>
    {/if}

    {#if showReorderHandle}
      <span class="list-card__handle" aria-hidden="true">
        <svg viewBox="0 0 16 16" fill="currentColor">
          <circle cx="5" cy="4" r="1.1"></circle>
          <circle cx="5" cy="8" r="1.1"></circle>
          <circle cx="5" cy="12" r="1.1"></circle>
          <circle cx="11" cy="4" r="1.1"></circle>
          <circle cx="11" cy="8" r="1.1"></circle>
          <circle cx="11" cy="12" r="1.1"></circle>
        </svg>
      </span>
    {/if}

    {#if $$slots.leading}
      <span class="list-card__leading">
        <slot name="leading" />
      </span>
    {/if}

    <div class="list-card__body">
      <div class="list-card__header">
        <span class="list-card__title">
          <slot name="title">{title}</slot>
        </span>
        {#if $$slots.badges}
          <span class="list-card__badges">
            <slot name="badges" />
          </span>
        {/if}
      </div>
      {#if subtitle}
        <span class="list-card__subtitle">{subtitle}</span>
      {/if}
      {#if $$slots.footer}
        <div class="list-card__footer">
          <slot name="footer" />
        </div>
      {/if}
    </div>

    {#if meta && !isCompact}
      <span class="list-card__meta">{meta}</span>
    {/if}

    {#if $$slots.actions}
      <span class="list-card__actions">
        <slot name="actions" />
      </span>
    {/if}

    {#if $$slots.trailing}
      <span class="list-card__trailing">
        <slot name="trailing" />
      </span>
    {/if}
  </div>
{/if}

<style>
  .list-card {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 18%, transparent);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-surface) 88%, var(--poodle-color-text-primary));
    text-decoration: none;
    width: 100%;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .list-card--has-sash {
    position: relative;
    overflow: hidden;
  }

  .list-card--interactive {
    cursor: pointer;
  }

  .list-card--interactive:hover:not([data-disabled="true"]) {
    background: color-mix(in srgb, var(--poodle-surface) 82%, var(--poodle-color-text-primary));
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 52%, transparent);
  }

  .list-card[data-selected="true"] {
    border-color: var(--list-card-accent, var(--poodle-color-accent-base));
    box-shadow:
      0 0 0 0.0625rem var(--list-card-accent, var(--poodle-color-accent-base)),
      inset 0 0 0 0.0625rem color-mix(
        in srgb,
        var(--list-card-accent, var(--poodle-color-accent-base)) 12%,
        transparent
      );
  }

  .list-card--interactive[data-selected="true"]:hover:not([data-disabled="true"]) {
    border-color: var(--list-card-accent, var(--poodle-color-accent-base));
  }

  .list-card:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .list-card[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .list-card[data-not-live="true"] {
    border: 0.1875rem dashed color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    background: color-mix(in srgb, var(--poodle-surface) 32%, transparent);
    filter: grayscale(1);
    opacity: 0.72;
  }

  .list-card[data-not-live="true"]:hover:not([data-disabled="true"]) {
    border-color: var(--poodle-color-border-default);
    filter: grayscale(0);
    opacity: 1;
  }

  .list-card[data-layout="compact"] {
    gap: var(--poodle-space-inline-sm);
    padding: 0.5rem 0.625rem;
    min-height: 3rem;
  }

  .list-card[data-layout="compact"] .list-card__leading {
    width: 1.5rem;
    height: 1.5rem;
    font-size: 0.75rem;
  }

  .list-card[data-layout="compact"][data-leading-shape="rounded-square"] .list-card__leading {
    width: 1.75rem;
    height: 1.75rem;
  }

  .list-card[data-layout="compact"] .list-card__body {
    gap: 0;
  }

  .list-card[data-layout="compact"] .list-card__title {
    font-size: 0.875rem;
  }

  .list-card[data-layout="compact"] .list-card__subtitle,
  .list-card[data-layout="compact"] .list-card__footer {
    display: none;
  }

  .list-card__handle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 1rem;
    height: 1rem;
    color: var(--poodle-color-text-secondary);
    opacity: 0.8;
  }

  .list-card__handle svg {
    width: 100%;
    height: 100%;
  }

  .list-card__leading {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2rem;
    height: 2rem;
    overflow: hidden;
    border-radius: 999px;
    background: color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 12%, transparent);
    color: var(--list-card-accent, var(--poodle-color-accent-base));
    font-size: 0.875rem;
    font-weight: 600;
  }

  .list-card[data-leading-shape="rounded-square"] .list-card__leading {
    width: 2.75rem;
    height: 2.75rem;
    border-radius: var(--poodle-radius-control);
  }

  .list-card[data-leading-fill="solid"] .list-card__leading {
    background: var(--list-card-accent, var(--poodle-color-accent-base));
    color: #fff;
  }

  .list-card__body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
  }

  .list-card__header {
    display: flex;
    align-items: baseline;
    gap: var(--poodle-space-inline-sm);
  }

  .list-card__title {
    flex: 1;
    min-width: 0;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    font-weight: 500;
    color: var(--poodle-color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .list-card__badges {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
  }

  .list-card__subtitle {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .list-card__footer {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    margin-top: 0.125rem;
  }

  .list-card__meta {
    flex-shrink: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .list-card__trailing {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .list-card__actions {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  /* Size variants */
  .list-card[data-size="xs"] { padding: 0.375rem 0.5rem; gap: 0.375rem; }
  .list-card[data-size="xs"] .list-card__title { font-size: 0.75rem; }
  .list-card[data-size="xs"] .list-card__subtitle { font-size: 0.625rem; }
  .list-card[data-size="xs"] .list-card__meta { font-size: 0.625rem; }
  .list-card[data-size="xs"] .list-card__leading { width: 1.5rem; height: 1.5rem; font-size: 0.6875rem; }

  .list-card[data-size="sm"] { padding: 0.5rem 0.625rem; gap: 0.5rem; }
  .list-card[data-size="sm"] .list-card__title { font-size: 0.8125rem; }
  .list-card[data-size="sm"] .list-card__subtitle { font-size: 0.6875rem; }
  .list-card[data-size="sm"] .list-card__meta { font-size: 0.6875rem; }
  .list-card[data-size="sm"] .list-card__leading { width: 1.75rem; height: 1.75rem; font-size: 0.75rem; }

  .list-card[data-size="lg"] { padding: 0.75rem 1rem; gap: 0.875rem; }
  .list-card[data-size="lg"] .list-card__title { font-size: 1rem; }
  .list-card[data-size="lg"] .list-card__subtitle { font-size: 0.8125rem; }
  .list-card[data-size="lg"] .list-card__meta { font-size: 0.8125rem; }
  .list-card[data-size="lg"] .list-card__leading { width: 2.5rem; height: 2.5rem; font-size: 1rem; }

  .list-card[data-size="xl"] { padding: 0.875rem 1.125rem; gap: 1rem; }
  .list-card[data-size="xl"] .list-card__title { font-size: 1.0625rem; }
  .list-card[data-size="xl"] .list-card__subtitle { font-size: 0.875rem; }
  .list-card[data-size="xl"] .list-card__meta { font-size: 0.875rem; }
  .list-card[data-size="xl"] .list-card__leading { width: 2.75rem; height: 2.75rem; font-size: 1.125rem; }

  /* Density variants */
  .list-card[data-density="compact"] { padding-inline: 0.5rem; }
  .list-card[data-density="comfortable"] { padding-inline: 1.125rem; }

  .list-card__sash {
    position: absolute;
    top: 0.34375rem;
    left: -2.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 6rem;
    padding: 0.125rem 0;
    background: var(--list-card-sash, var(--poodle-color-positive-base, #22c55e));
    color: #fff;
    font-size: 0.5625rem;
    font-weight: 700;
    text-transform: uppercase;
    line-height: 1;
    transform: rotate(-45deg);
    pointer-events: none;
    z-index: 1;
  }
</style>

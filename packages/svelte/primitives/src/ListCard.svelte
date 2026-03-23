<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let title: string;
  export let subtitle: string | null = null;
  export let meta: string | null = null;
  export let leadingShape: "circle" | "rounded-square" = "circle";
  export let leadingFill: "tint" | "solid" = "tint";
  export let accentColor: string | null = null;
  export let isInteractive = false;
  export let isDisabled = false;
  export let isNotLive = false;
  export let sash: string | null = null;
  export let sashColor: string | null = null;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
  }>();
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
  class="list-card"
  class:list-card--interactive={isInteractive}
  data-disabled={isDisabled}
  data-not-live={isNotLive}
  data-leading-shape={leadingShape}
  data-leading-fill={leadingFill}
  role={isInteractive ? "button" : undefined}
  tabindex={isInteractive && !isDisabled ? 0 : -1}
  aria-label={ariaLabel ?? title}
  class:list-card--has-sash={!!sash}
  style={[
    accentColor ? `--list-card-accent: ${accentColor}` : '',
    sashColor ? `--list-card-sash: ${sashColor}` : '',
  ].filter(Boolean).join('; ') || undefined}
  on:click={(e) => isInteractive && !isDisabled && dispatch("click", e)}
  on:keydown={(e) => {
    if (isInteractive && !isDisabled && (e.key === "Enter" || e.key === " ")) {
      e.preventDefault();
      dispatch("click", new MouseEvent("click"));
    }
  }}
>
  {#if sash}
    <span class="list-card__sash" aria-label={sash}>{sash}</span>
  {/if}

  {#if $$slots.leading}
    <span class="list-card__leading">
      <slot name="leading" />
    </span>
  {/if}

  <div class="list-card__body">
    <div class="list-card__header">
      <span class="list-card__title">{title}</span>
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

  {#if meta}
    <span class="list-card__meta">{meta}</span>
  {/if}

  {#if $$slots.trailing}
    <span class="list-card__trailing">
      <slot name="trailing" />
    </span>
  {/if}
</div>

<style>
  .list-card {
    display: flex;
    align-items: center;
    gap: var(--flint-space-inline-md);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 18%, transparent);
    border-radius: var(--flint-radius-control);
    background: color-mix(in srgb, var(--flint-surface) 88%, var(--flint-color-text-primary));
    transition:
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .list-card--has-sash {
    position: relative;
    overflow: hidden;
  }

  .list-card--interactive {
    cursor: pointer;
  }

  .list-card--interactive:hover:not([data-disabled="true"]) {
    background: color-mix(in srgb, var(--flint-surface) 82%, var(--flint-color-text-primary));
    border-color: color-mix(in srgb, var(--flint-color-border-default) 52%, transparent);
  }

  .list-card:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .list-card[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }

  .list-card[data-not-live="true"] {
    border: 0.1875rem dashed color-mix(in srgb, var(--flint-color-border-default) 72%, transparent);
    background: color-mix(in srgb, var(--flint-surface) 32%, transparent);
    filter: grayscale(1);
    opacity: 0.72;
  }

  .list-card[data-not-live="true"]:hover:not([data-disabled="true"]) {
    border-color: var(--flint-color-border-default);
    filter: grayscale(0);
    opacity: 1;
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
    background: color-mix(in srgb, var(--list-card-accent, var(--flint-color-accent-base)) 12%, transparent);
    color: var(--list-card-accent, var(--flint-color-accent-base));
    font-size: 0.875rem;
    font-weight: 600;
  }

  .list-card[data-leading-shape="rounded-square"] .list-card__leading {
    width: 2.75rem;
    height: 2.75rem;
    border-radius: var(--flint-radius-control);
  }

  .list-card[data-leading-fill="solid"] .list-card__leading {
    background: var(--list-card-accent, var(--flint-color-accent-base));
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
    gap: var(--flint-space-inline-sm);
  }

  .list-card__title {
    flex: 1;
    min-width: 0;
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    font-weight: 500;
    color: var(--flint-color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .list-card__badges {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    gap: var(--flint-space-inline-sm);
  }

  .list-card__subtitle {
    font-size: 0.75rem;
    color: var(--flint-color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .list-card__footer {
    display: flex;
    align-items: center;
    gap: var(--flint-space-inline-md);
    margin-top: 0.125rem;
  }

  .list-card__meta {
    flex-shrink: 0;
    font-size: 0.75rem;
    color: var(--flint-color-text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .list-card__trailing {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .list-card__sash {
    position: absolute;
    top: 0.34375rem;
    left: -2.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 6rem;
    padding: 0.125rem 0;
    background: var(--list-card-sash, var(--flint-color-positive-base, #22c55e));
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

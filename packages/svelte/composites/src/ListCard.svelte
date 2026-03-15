<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let title: string;
  export let subtitle: string | null = null;
  export let meta: string | null = null;
  export let isInteractive = false;
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
  }>();
</script>

<div
  class="list-card"
  class:list-card--interactive={isInteractive}
  data-disabled={isDisabled}
  role={isInteractive ? "button" : undefined}
  tabindex={isInteractive && !isDisabled ? 0 : -1}
  aria-label={ariaLabel ?? title}
  on:click={(e) => isInteractive && !isDisabled && dispatch("click", e)}
  on:keydown={(e) => {
    if (isInteractive && !isDisabled && (e.key === "Enter" || e.key === " ")) {
      e.preventDefault();
      dispatch("click", new MouseEvent("click"));
    }
  }}
>
  {#if $$slots.leading}
    <span class="list-card__leading">
      <slot name="leading" />
    </span>
  {/if}

  <div class="list-card__content">
    <span class="list-card__title">{title}</span>
    {#if subtitle}
      <span class="list-card__subtitle">{subtitle}</span>
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
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 18%, transparent);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .list-card--interactive {
    cursor: pointer;
  }

  .list-card--interactive:hover:not([data-disabled="true"]) {
    background: color-mix(in srgb, var(--pug-color-background-elevated) 52%, var(--pug-color-background-surface));
    border-color: color-mix(in srgb, var(--pug-color-border-default) 52%, transparent);
  }

  .list-card:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .list-card[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .list-card__leading {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2rem;
    height: 2rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--pug-color-accent-base) 12%, transparent);
    color: var(--pug-color-accent-base);
    font-size: 0.875rem;
    font-weight: 600;
  }

  .list-card__content {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
  }

  .list-card__title {
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    font-weight: 500;
    color: var(--pug-color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .list-card__subtitle {
    font-size: 0.75rem;
    color: var(--pug-color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .list-card__meta {
    flex-shrink: 0;
    font-size: 0.75rem;
    color: var(--pug-color-text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .list-card__trailing {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }
</style>

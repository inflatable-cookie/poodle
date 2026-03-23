<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let title: string;
  export let description: string | null = null;
  export let href: string | null = null;
  export let badge: string | null = null;
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
  }>();

  function handleClick(event: MouseEvent): void {
    if (isDisabled) {
      event.preventDefault();
      return;
    }
    dispatch("click", event);
  }
</script>

{#if href && !isDisabled}
  <a
    class="nav-card"
    {href}
    aria-label={ariaLabel ?? title}
    data-disabled={isDisabled}
    on:click={handleClick}
  >
    {#if $$slots.icon}
      <span class="nav-card__icon" aria-hidden="true">
        <slot name="icon" />
      </span>
    {/if}
    <div class="nav-card__content">
      <span class="nav-card__title">
        {title}
        {#if badge}
          <span class="nav-card__badge">{badge}</span>
        {/if}
      </span>
      {#if description}
        <span class="nav-card__description">{description}</span>
      {/if}
    </div>
    <svg class="nav-card__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </a>
{:else}
  <button
    type="button"
    class="nav-card"
    aria-label={ariaLabel ?? title}
    disabled={isDisabled}
    data-disabled={isDisabled}
    on:click={handleClick}
  >
    {#if $$slots.icon}
      <span class="nav-card__icon" aria-hidden="true">
        <slot name="icon" />
      </span>
    {/if}
    <div class="nav-card__content">
      <span class="nav-card__title">
        {title}
        {#if badge}
          <span class="nav-card__badge">{badge}</span>
        {/if}
      </span>
      {#if description}
        <span class="nav-card__description">{description}</span>
      {/if}
    </div>
    <svg class="nav-card__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </button>
{/if}

<style>
  .nav-card {
    display: flex;
    align-items: center;
    gap: var(--flint-space-inline-md);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 32%, transparent);
    border-radius: var(--flint-radius-surface);
    background: var(--flint-color-background-surface);
    color: inherit;
    cursor: pointer;
    text-decoration: none;
    text-align: left;
    font: inherit;
    width: 100%;
    transition:
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      box-shadow var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .nav-card:hover:not(:disabled):not([data-disabled="true"]) {
    border-color: color-mix(in srgb, var(--flint-color-accent-base) 28%, var(--flint-color-border-subtle));
    background: color-mix(in srgb, var(--flint-color-background-elevated) 52%, var(--flint-color-background-surface));
  }

  .nav-card:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .nav-card[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }

  .nav-card__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2rem;
    height: 2rem;
    border-radius: var(--flint-radius-control);
    background: color-mix(in srgb, var(--flint-color-accent-base) 12%, transparent);
    color: var(--flint-color-accent-base);
    font-size: 1rem;
  }

  .nav-card__content {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .nav-card__title {
    display: flex;
    align-items: center;
    gap: var(--flint-space-inline-sm);
    font-family: var(--flint-typography-label-family);
    font-size: var(--flint-typography-label-size);
    font-weight: 600;
    color: var(--flint-color-text-primary);
  }

  .nav-card__badge {
    display: inline-flex;
    align-items: center;
    padding: 0.0625rem 0.375rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--flint-color-accent-base) 16%, transparent);
    color: var(--flint-color-accent-base);
    font-size: 0.625rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .nav-card__description {
    font-size: 0.8125rem;
    color: var(--flint-color-text-secondary);
    line-height: 1.4;
  }

  .nav-card__arrow {
    flex-shrink: 0;
    width: 1rem;
    height: 1rem;
    color: var(--flint-color-text-secondary);
    opacity: 0;
    transition: opacity var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .nav-card:hover .nav-card__arrow {
    opacity: 1;
  }
</style>

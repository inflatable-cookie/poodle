<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    description?: string | null;
    href?: string | null;
    badge?: string | null;
    disabled?: boolean;
    ariaLabel?: string | null;
    onClick?: ((event: MouseEvent) => void) | null;
    icon?: Snippet;
  }

  let {
    title,
    description = null,
    href = null,
    badge = null,
    disabled = false,
    ariaLabel = null,
    onClick = null,
    icon,
  }: Props = $props();

  function handleClick(event: MouseEvent): void {
    if (disabled) {
      event.preventDefault();
      return;
    }
    onClick?.(event);
  }
</script>

{#if href && !disabled}
  <a
    class="poodle-nav-card"
    {href}
    aria-label={ariaLabel ?? title}
    data-disabled={disabled}
    onclick={handleClick}
  >
    {#if icon}
      <span class="poodle-nav-card__icon" aria-hidden="true">
        {@render icon()}
      </span>
    {/if}
    <div class="poodle-nav-card__content">
      <span class="poodle-nav-card__title">
        {title}
        {#if badge}
          <span class="poodle-nav-card__badge">{badge}</span>
        {/if}
      </span>
      {#if description}
        <span class="poodle-nav-card__description">{description}</span>
      {/if}
    </div>
    <svg class="poodle-nav-card__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </a>
{:else}
  <button
    type="button"
    class="poodle-nav-card"
    aria-label={ariaLabel ?? title}
    disabled={disabled}
    data-disabled={disabled}
    onclick={handleClick}
  >
    {#if icon}
      <span class="poodle-nav-card__icon" aria-hidden="true">
        {@render icon()}
      </span>
    {/if}
    <div class="poodle-nav-card__content">
      <span class="poodle-nav-card__title">
        {title}
        {#if badge}
          <span class="poodle-nav-card__badge">{badge}</span>
        {/if}
      </span>
      {#if description}
        <span class="poodle-nav-card__description">{description}</span>
      {/if}
    </div>
    <svg class="poodle-nav-card__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </button>
{/if}

<style>
  .poodle-nav-card {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    padding: 0.625rem var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-surface);
    color: inherit;
    cursor: pointer;
    text-decoration: none;
    text-align: left;
    font: inherit;
    width: 100%;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-nav-card:hover:not(:disabled):not([data-disabled="true"]) {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 28%, var(--poodle-color-border-subtle));
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 52%, var(--poodle-color-background-surface));
  }

  .poodle-nav-card:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-nav-card[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-nav-card__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2rem;
    height: 2rem;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    color: var(--poodle-color-accent-base);
    font-size: 1rem;
  }

  .poodle-nav-card__content {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .poodle-nav-card__title {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: 600;
    color: var(--poodle-color-text-primary);
  }

  .poodle-nav-card__badge {
    display: inline-flex;
    align-items: center;
    padding: 0.0625rem 0.375rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    color: var(--poodle-color-accent-base);
    font-size: 0.625rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .poodle-nav-card__description {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    line-height: 1.4;
  }

  .poodle-nav-card__arrow {
    flex-shrink: 0;
    width: 1rem;
    height: 1rem;
    color: var(--poodle-color-text-secondary);
    opacity: 0;
    transition: opacity var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-nav-card:hover .poodle-nav-card__arrow {
    opacity: 1;
  }
</style>

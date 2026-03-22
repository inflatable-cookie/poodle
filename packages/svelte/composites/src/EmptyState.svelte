<script lang="ts">
  import { Icon } from "@pug/svelte-primitives";

  import type { EmptyStateVariant } from "./types";

  export let title: string;
  export let message: string | null = null;
  export let variant: EmptyStateVariant = "neutral";
  export let ariaLabel: string | null = null;
</script>

<section class="empty-state" data-variant={variant} aria-label={ariaLabel ?? title}>
  <div class="empty-state__visual" aria-hidden="true">
    {#if variant === "search"}
      <Icon name="search" />
    {:else if variant === "firstRun"}
      <Icon name="plus" />
    {:else}
      <Icon name="inbox" />
    {/if}
  </div>

  <div class="empty-state__copy">
    <h3>{title}</h3>
    {#if message}
      <p>{message}</p>
    {/if}
  </div>

  {#if $$slots.actions}
    <div class="empty-state__actions">
      <slot name="actions" />
    </div>
  {/if}
</section>

<style>
  .empty-state {
    display: grid;
    justify-items: center;
    text-align: center;
    gap: var(--pug-space-stack-md);
    padding: calc(var(--pug-space-panel-y) * 1.5) var(--pug-space-panel-x);
    border: 0.0625rem dashed var(--pug-color-border-default);
    border-radius: calc(var(--pug-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-surface) 76%, transparent);
  }

  .empty-state[data-variant="search"] {
    background: color-mix(in srgb, var(--pug-color-accent-base) 7%, transparent);
  }

  .empty-state[data-variant="firstRun"] {
    background: color-mix(in srgb, var(--pug-color-status-success) 7%, transparent);
  }

  .empty-state__visual {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.25rem;
    height: 2.25rem;
    border-radius: 999rem;
    background: color-mix(in srgb, var(--pug-color-background-panel) 90%, transparent);
    color: var(--pug-color-text-secondary);
    font-size: 1.125rem;
    font-weight: 600;
  }

  .empty-state__copy {
    display: grid;
    gap: var(--pug-space-inline-sm);
    max-width: 24rem;
  }

  .empty-state__copy h3,
  .empty-state__copy p {
    margin: 0;
  }

  .empty-state__copy h3 {
    font-size: 1.125rem;
    line-height: 1.2;
  }

  .empty-state__copy p {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .empty-state__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
  }
</style>

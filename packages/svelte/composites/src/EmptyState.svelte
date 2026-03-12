<script lang="ts">
  import type { EmptyStateVariant } from "./types";

  export let title: string;
  export let message: string | null = null;
  export let variant: EmptyStateVariant = "neutral";
  export let ariaLabel: string | null = null;
</script>

<section class="empty-state" data-variant={variant} aria-label={ariaLabel ?? title}>
  <div class="empty-state__visual" aria-hidden="true">
    {#if variant === "search"}
      ⌕
    {:else if variant === "firstRun"}
      +
    {:else}
      ○
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
    justify-items: start;
    gap: var(--pug-space-stack-md);
    padding: calc(var(--pug-space-panel-y) * 1.5) var(--pug-space-panel-x);
    border: 1px dashed var(--pug-color-border-default);
    border-radius: calc(var(--pug-radius-surface) - 2px);
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
    width: 36px;
    height: 36px;
    border-radius: 999px;
    background: color-mix(in srgb, var(--pug-color-background-panel) 90%, transparent);
    color: var(--pug-color-text-secondary);
    font-size: 18px;
    font-weight: 600;
  }

  .empty-state__copy {
    display: grid;
    gap: 6px;
  }

  .empty-state__copy h3,
  .empty-state__copy p {
    margin: 0;
  }

  .empty-state__copy h3 {
    font-size: 18px;
    line-height: 1.2;
  }

  .empty-state__copy p {
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 1.5;
  }

  .empty-state__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
  }
</style>

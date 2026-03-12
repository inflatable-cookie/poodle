<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { ToastItem } from "./types";

  export let items: ToastItem[] = [];
  export let ariaLabel = "Notifications";

  const dispatch = createEventDispatcher<{
    dismiss: { id: string };
    action: { id: string };
  }>();
</script>

<section class="toast-stack" aria-label={ariaLabel} aria-live="polite" aria-atomic="false" role="list">
  {#each items as item (item.id)}
    <article
      class="toast"
      data-tone={item.tone ?? "info"}
      role="listitem"
      aria-live={item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
    >
      <div class="toast__copy">
        <strong>{item.title}</strong>
        {#if item.message}
          <p>{item.message}</p>
        {/if}
      </div>
      <div class="toast__actions">
        {#if item.actionLabel}
          <button type="button" class="toast__action" on:click={() => dispatch("action", { id: item.id })}>
            {item.actionLabel}
          </button>
        {/if}
        <button type="button" class="toast__dismiss" aria-label={`Dismiss ${item.title}`} on:click={() => dispatch("dismiss", { id: item.id })}>
          ×
        </button>
      </div>
    </article>
  {/each}
</section>

<style>
  .toast-stack {
    display: grid;
    gap: var(--pug-space-stack-sm);
  }

  .toast {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: calc(var(--pug-radius-surface) - 2px);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 94%, transparent);
    box-shadow: var(--pug-elevation-overlay);
  }

  .toast[data-tone="info"] {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 28%, var(--pug-color-border-default));
  }

  .toast[data-tone="success"] {
    border-color: color-mix(in srgb, var(--pug-color-status-success) 28%, var(--pug-color-border-default));
  }

  .toast[data-tone="warning"] {
    border-color: color-mix(in srgb, var(--pug-color-status-warning) 28%, var(--pug-color-border-default));
  }

  .toast[data-tone="danger"] {
    border-color: color-mix(in srgb, var(--pug-color-status-danger) 28%, var(--pug-color-border-default));
  }

  .toast__copy {
    display: grid;
    gap: 4px;
  }

  .toast__copy strong,
  .toast__copy p {
    margin: 0;
  }

  .toast__copy p {
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 1.5;
  }

  .toast__actions {
    display: flex;
    align-items: start;
    gap: var(--pug-space-inline-sm);
  }

  .toast__action,
  .toast__dismiss {
    min-height: 28px;
    padding: 0 10px;
    border: 1px solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-pill);
    background: color-mix(in srgb, var(--pug-color-background-surface) 82%, transparent);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .toast__dismiss {
    width: 28px;
    padding: 0;
  }
</style>

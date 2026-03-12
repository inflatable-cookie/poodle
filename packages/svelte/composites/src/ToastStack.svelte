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
    --pug-toast-tone: var(--pug-color-accent-base);
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-toast-tone) 34%, var(--pug-color-border-default));
    border-radius: calc(var(--pug-radius-surface) - 0.125rem);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--pug-toast-tone) 12%, transparent),
        color-mix(in srgb, var(--pug-color-background-elevated) 98%, transparent) 18%
      ),
      color-mix(in srgb, var(--pug-color-background-elevated) 96%, transparent);
    box-shadow: var(--pug-elevation-overlay);
    position: relative;
    overflow: hidden;
  }

  .toast::before {
    content: "";
    position: absolute;
    inset: 0 auto 0 0;
    width: 0.1875rem;
    background: color-mix(in srgb, var(--pug-toast-tone) 82%, white 6%);
  }

  .toast[data-tone="info"] {
    --pug-toast-tone: var(--pug-color-accent-base);
  }

  .toast[data-tone="success"] {
    --pug-toast-tone: var(--pug-color-status-success);
  }

  .toast[data-tone="warning"] {
    --pug-toast-tone: var(--pug-color-status-warning);
  }

  .toast[data-tone="danger"] {
    --pug-toast-tone: var(--pug-color-status-danger);
  }

  .toast__copy {
    display: grid;
    gap: 0.25rem;
  }

  .toast__copy strong,
  .toast__copy p {
    margin: 0;
  }

  .toast__copy p {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .toast__actions {
    display: flex;
    align-items: start;
    gap: var(--pug-space-inline-sm);
  }

  .toast__action,
  .toast__dismiss {
    min-height: 1.75rem;
    padding: 0 0.625rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-pill);
    background: color-mix(in srgb, var(--pug-color-background-surface) 82%, transparent);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .toast__dismiss {
    width: 1.75rem;
    padding: 0;
  }

  .toast__action {
    border-color: color-mix(in srgb, var(--pug-toast-tone) 30%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-toast-tone) 10%, var(--pug-color-background-surface));
  }

  .toast__dismiss {
    border-color: color-mix(in srgb, var(--pug-toast-tone) 24%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-color-background-surface) 88%, transparent);
  }
</style>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Button, Icon } from "@flint/svelte-primitives";

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
      <button type="button" class="toast__dismiss" aria-label={`Dismiss ${item.title}`} on:click={() => dispatch("dismiss", { id: item.id })}>
        <Icon name="x" size="sm" />
      </button>

      <div class="toast__copy">
        <strong>{item.title}</strong>
        {#if item.message}
          <p>{item.message}</p>
        {/if}
      </div>

      {#if item.actionLabel}
        <div class="toast__actions">
          <Button variant="secondary" size="sm" on:click={() => dispatch("action", { id: item.id })}>
            {item.actionLabel}
          </Button>
        </div>
      {/if}
    </article>
  {/each}
</section>

<style>
  .toast-stack {
    display: grid;
    gap: var(--flint-space-stack-sm);
  }

  .toast {
    --flint-toast-tone: var(--flint-color-status-info, #3b82f6);
    display: grid;
    gap: var(--flint-space-stack-sm);
    padding: var(--flint-space-panel-x);
    padding-right: calc(var(--flint-space-panel-x) + 1.5rem);
    border: 0.0625rem solid color-mix(in srgb, var(--flint-toast-tone) 34%, var(--flint-color-border-default));
    border-radius: calc(var(--flint-radius-surface) - 0.125rem);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--flint-toast-tone) 12%, transparent),
        color-mix(in srgb, var(--flint-color-background-elevated) 98%, transparent) 18%
      ),
      color-mix(in srgb, var(--flint-color-background-elevated) 96%, transparent);
    box-shadow: var(--flint-elevation-overlay);
    position: relative;
    overflow: hidden;
  }

  .toast::before {
    content: "";
    position: absolute;
    inset: 0 auto 0 0;
    width: 0.1875rem;
    background: color-mix(in srgb, var(--flint-toast-tone) 82%, white 6%);
  }

  .toast[data-tone="info"] {
    --flint-toast-tone: var(--flint-color-status-info, #3b82f6);
  }

  .toast[data-tone="success"] {
    --flint-toast-tone: var(--flint-color-status-success);
  }

  .toast[data-tone="warning"] {
    --flint-toast-tone: var(--flint-color-status-warning);
  }

  .toast[data-tone="danger"] {
    --flint-toast-tone: var(--flint-color-status-danger);
  }

  .toast__dismiss {
    position: absolute;
    top: 0.375rem;
    right: 0.375rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    padding: 0;
    border: none;
    border-radius: var(--flint-radius-sm, 0.25rem);
    background: transparent;
    color: var(--flint-color-text-secondary);
    cursor: pointer;
  }

  .toast__dismiss:hover {
    color: var(--flint-color-text-primary);
    background: color-mix(in srgb, var(--flint-color-background-surface) 60%, transparent);
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
    color: var(--flint-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .toast__actions {
    display: flex;
    justify-content: flex-start;
  }
</style>

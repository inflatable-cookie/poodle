<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Button, Icon, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  import type { ToastItem } from "./types";

  export let items: ToastItem[] = [];
  export let ariaLabel = "Notifications";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    dismiss: { id: string };
    action: { id: string };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
</script>

<section class="toast-stack" aria-label={ariaLabel} aria-live="polite" aria-atomic="false" role="list" data-size={resolvedSize} data-density={resolvedDensity}>
  {#each items as item (item.id)}
    <article
      class="toast"
      data-tone={item.tone ?? "info"}
      role="listitem"
      aria-live={item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
    >
      <button type="button" class="toast__dismiss" aria-label={`Dismiss ${item.title}`} on:click={() => dispatch("dismiss", { id: item.id })}>
        <Icon name="x" />
      </button>

      <div class="toast__copy">
        <strong>{item.title}</strong>
        {#if item.message}
          <p>{item.message}</p>
        {/if}
      </div>

      {#if item.actionLabel}
        <div class="toast__actions">
          <Button variant="secondary" on:click={() => dispatch("action", { id: item.id })}>
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
    gap: var(--poodle-space-stack-sm);
  }

  .toast {
    --poodle-toast-tone: var(--poodle-color-status-info, #3b82f6);
    display: grid;
    gap: var(--poodle-space-stack-sm);
    padding: var(--poodle-space-panel-x);
    padding-right: calc(var(--poodle-space-panel-x) + 1.5rem);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-toast-tone) 34%, var(--poodle-color-border-default));
    border-radius: calc(var(--poodle-radius-surface) - 0.125rem);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--poodle-toast-tone) 12%, transparent),
        color-mix(in srgb, var(--poodle-color-background-elevated) 98%, transparent) 18%
      ),
      color-mix(in srgb, var(--poodle-color-background-elevated) 96%, transparent);
    box-shadow: var(--poodle-elevation-overlay);
    position: relative;
    overflow: hidden;
  }

  .toast::before {
    content: "";
    position: absolute;
    inset: 0 auto 0 0;
    width: 0.1875rem;
    background: color-mix(in srgb, var(--poodle-toast-tone) 82%, white 6%);
  }

  .toast[data-tone="info"] {
    --poodle-toast-tone: var(--poodle-color-status-info, #3b82f6);
  }

  .toast[data-tone="success"] {
    --poodle-toast-tone: var(--poodle-color-status-success);
  }

  .toast[data-tone="warning"] {
    --poodle-toast-tone: var(--poodle-color-status-warning);
  }

  .toast[data-tone="danger"] {
    --poodle-toast-tone: var(--poodle-color-status-danger);
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
    border-radius: var(--poodle-radius-sm, 0.25rem);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
  }

  .toast__dismiss:hover {
    color: var(--poodle-color-text-primary);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent);
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
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .toast__actions {
    display: flex;
    justify-content: flex-start;
  }

  /* ── Size variants ──────────────────────────────────────────── */

  .toast-stack[data-size="xs"] .toast {
    padding: 0.375rem 0.5rem;
    padding-right: calc(0.5rem + 1.25rem);
  }

  .toast-stack[data-size="xs"] .toast__dismiss {
    width: 1rem;
    height: 1rem;
    top: 0.25rem;
    right: 0.25rem;
  }

  .toast-stack[data-size="xs"] .toast__copy p {
    font-size: 0.6875rem;
  }

  .toast-stack[data-size="xs"] .toast__copy strong {
    font-size: 0.71875rem;
  }

  .toast-stack[data-size="sm"] .toast {
    padding: 0.5rem 0.625rem;
    padding-right: calc(0.625rem + 1.375rem);
  }

  .toast-stack[data-size="sm"] .toast__dismiss {
    width: 1.125rem;
    height: 1.125rem;
  }

  .toast-stack[data-size="sm"] .toast__copy p {
    font-size: 0.75rem;
  }

  .toast-stack[data-size="lg"] .toast {
    padding: 0.75rem 0.875rem;
    padding-right: calc(0.875rem + 1.75rem);
  }

  .toast-stack[data-size="lg"] .toast__dismiss {
    width: 1.5rem;
    height: 1.5rem;
    top: 0.5rem;
    right: 0.5rem;
  }

  .toast-stack[data-size="lg"] .toast__copy p {
    font-size: 0.875rem;
  }

  .toast-stack[data-size="lg"] .toast__copy strong {
    font-size: 0.9375rem;
  }

  .toast-stack[data-size="xl"] .toast {
    padding: 0.875rem 1rem;
    padding-right: calc(1rem + 2rem);
  }

  .toast-stack[data-size="xl"] .toast__dismiss {
    width: 1.75rem;
    height: 1.75rem;
    top: 0.5rem;
    right: 0.5rem;
  }

  .toast-stack[data-size="xl"] .toast__copy p {
    font-size: 0.9375rem;
  }

  .toast-stack[data-size="xl"] .toast__copy strong {
    font-size: 1rem;
  }

  /* Density variants */
  .toast-stack[data-density="compact"] { gap: var(--poodle-space-stack-sm); }
  .toast-stack[data-density="comfortable"] { gap: var(--poodle-space-stack-lg); }
</style>

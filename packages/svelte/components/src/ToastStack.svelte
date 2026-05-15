<script lang="ts">
  import Button from "./Button.svelte";
  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  import type { ToastItem } from "./types";

  export let items: ToastItem[] = [];
  export let ariaLabel = "Notifications";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;
  export let onDismiss: ((id: string) => void) | undefined = undefined;
  export let onAction: ((id: string) => void) | undefined = undefined;

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
</script>

<section class="poodle-toast-stack" aria-label={ariaLabel} aria-live="polite" aria-atomic="false" role="list" data-size={resolvedSize} data-density={resolvedDensity}>
  {#each items as item (item.id)}
    <article
      class="poodle-toast"
      data-tone={item.tone ?? "info"}
      role="listitem"
      aria-live={item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
    >
      <button type="button" class="poodle-toast__dismiss" aria-label={`Dismiss ${item.title}`} onclick={() => onDismiss?.(item.id)}>
        <Icon name="x" />
      </button>

      <div class="poodle-toast__copy">
        <strong>{item.title}</strong>
        {#if item.message}
          <p>{item.message}</p>
        {/if}
      </div>

      {#if item.actionLabel}
        <div class="poodle-toast__actions">
          <Button variant="secondary" size={resolvedSize} density={resolvedDensity} onClick={() => onAction?.(item.id)}>
            {item.actionLabel}
          </Button>
        </div>
      {/if}
    </article>
  {/each}
</section>

<style>
  .poodle-toast-stack {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-toast {
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

  .poodle-toast::before {
    content: "";
    position: absolute;
    inset: 0 auto 0 0;
    width: 0.1875rem;
    background: color-mix(in srgb, var(--poodle-toast-tone) 82%, white 6%);
  }

  .poodle-toast[data-tone="info"] {
    --poodle-toast-tone: var(--poodle-color-status-info, #3b82f6);
  }

  .poodle-toast[data-tone="success"] {
    --poodle-toast-tone: var(--poodle-color-status-success);
  }

  .poodle-toast[data-tone="warning"] {
    --poodle-toast-tone: var(--poodle-color-status-warning);
  }

  .poodle-toast[data-tone="danger"] {
    --poodle-toast-tone: var(--poodle-color-status-danger);
  }

  .poodle-toast__dismiss {
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

  .poodle-toast__dismiss:hover {
    color: var(--poodle-color-text-primary);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent);
  }

  .poodle-toast__copy {
    display: grid;
    gap: 0.25rem;
  }

  .poodle-toast__copy strong,
  .poodle-toast__copy p {
    margin: 0;
  }

  .poodle-toast__copy p {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-toast__actions {
    display: flex;
    justify-content: flex-start;
  }

  /* ── Size variants ──────────────────────────────────────────── */

  .poodle-toast-stack[data-size="xs"] .poodle-toast__dismiss {
    width: 1rem;
    height: 1rem;
    top: 0.25rem;
    right: 0.25rem;
  }

  .poodle-toast-stack[data-size="xs"] .poodle-toast__copy p {
    font-size: 0.6875rem;
  }

  .poodle-toast-stack[data-size="xs"] .poodle-toast__copy strong {
    font-size: 0.71875rem;
  }

  .poodle-toast-stack[data-size="sm"] .poodle-toast__dismiss {
    width: 1.125rem;
    height: 1.125rem;
  }

  .poodle-toast-stack[data-size="sm"] .poodle-toast__copy p {
    font-size: 0.75rem;
  }

  .poodle-toast-stack[data-size="lg"] .poodle-toast__dismiss {
    width: 1.5rem;
    height: 1.5rem;
    top: 0.5rem;
    right: 0.5rem;
  }

  .poodle-toast-stack[data-size="lg"] .poodle-toast__copy p {
    font-size: 0.875rem;
  }

  .poodle-toast-stack[data-size="lg"] .poodle-toast__copy strong {
    font-size: 0.9375rem;
  }

  .poodle-toast-stack[data-size="xl"] .poodle-toast__dismiss {
    width: 1.75rem;
    height: 1.75rem;
    top: 0.5rem;
    right: 0.5rem;
  }

  .poodle-toast-stack[data-size="xl"] .poodle-toast__copy p {
    font-size: 0.9375rem;
  }

  .poodle-toast-stack[data-size="xl"] .poodle-toast__copy strong {
    font-size: 1rem;
  }

  /* Density variants */
  .poodle-toast-stack[data-density="compact"] { gap: var(--poodle-space-stack-sm); }
  .poodle-toast-stack[data-density="comfortable"] { gap: var(--poodle-space-stack-lg); }

  .poodle-toast-stack[data-density="compact"] .poodle-toast {
    padding: calc(var(--poodle-space-panel-x) * 0.75);
    padding-right: calc(var(--poodle-space-panel-x) * 0.75 + 1.25rem);
  }

  .poodle-toast-stack[data-density="comfortable"] .poodle-toast {
    padding: calc(var(--poodle-space-panel-x) * 1.25);
    padding-right: calc(var(--poodle-space-panel-x) * 1.25 + 1.75rem);
  }
</style>

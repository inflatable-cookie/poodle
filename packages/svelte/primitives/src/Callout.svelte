<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import Spinner from "./Spinner.svelte";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone } from "./types";

  type CalloutAnnounceMode = "none" | "polite" | "assertive";

  export let tone: StatusTone | "neutral" = "neutral";
  export let title: string | null = null;
  export let message: string | null = null;
  export let ariaLabel: string | null = null;
  export let announceMode: CalloutAnnounceMode = "none";
  export let dismissible = false;
  export let dismissLabel = "Dismiss message";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    dismiss: void;
  }>();

  const uiPresentation = getUiPresentation();

  const toneIcon: Record<string, string> = {
    success: "check",
    warning: "triangle-alert",
    danger: "circle-x",
    info: "info",
    neutral: "info",
  };

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: role =
    announceMode === "assertive"
      ? "alert"
      : announceMode === "polite"
        ? "status"
        : undefined;
  $: ariaLive =
    announceMode === "assertive"
      ? "assertive" as const
      : announceMode === "polite"
        ? "polite" as const
        : undefined;
</script>

<section
  class="callout"
  data-tone={tone}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  aria-label={ariaLabel ?? undefined}
  role={role}
  aria-live={ariaLive}
>
  <div class="callout__body">
    <span class="callout__icon" aria-hidden="true">
      {#if $$slots.icon}
        <slot name="icon" />
      {:else if tone === "pending"}
        <Spinner variant="ring" size={resolvedSize} sizeRole="chrome" tone="accent" />
      {:else}
        <Icon name={toneIcon[tone] ?? "info"} size={resolvedSize} />
      {/if}
    </span>

    <div class="callout__content">
      {#if title}
        <strong>{title}</strong>
      {/if}
      {#if message}
        <p>{message}</p>
      {/if}
      <slot />
    </div>
  </div>

  {#if $$slots.actions}
    <div class="callout__actions">
      <slot name="actions" />
    </div>
  {/if}

  {#if dismissible}
    <button
      type="button"
      class="callout__dismiss"
      aria-label={dismissLabel}
      on:click={() => dispatch("dismiss")}
    >
      <Icon name="x" />
    </button>
  {/if}
</section>

<style>
  .callout {
    --poodle-callout-fill: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
    --poodle-callout-border: color-mix(in srgb, var(--poodle-color-border-subtle) 88%, transparent);
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto auto;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    padding: 0.625rem var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-callout-border);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-callout-fill);
    --poodle-surface: var(--poodle-callout-fill);
    color: var(--poodle-color-text-primary);
  }

  .callout[data-tone="info"] {
    --poodle-callout-fill: color-mix(in srgb, var(--poodle-color-status-info, #3b82f6) 10%, var(--poodle-color-background-panel));
    --poodle-callout-border: color-mix(in srgb, var(--poodle-color-status-info, #3b82f6) 34%, var(--poodle-color-border-default));
  }

  .callout[data-tone="success"] {
    --poodle-callout-fill: color-mix(in srgb, var(--poodle-color-status-success) 10%, var(--poodle-color-background-panel));
    --poodle-callout-border: color-mix(in srgb, var(--poodle-color-status-success) 34%, var(--poodle-color-border-default));
  }

  .callout[data-tone="warning"] {
    --poodle-callout-fill: color-mix(in srgb, var(--poodle-color-status-warning) 10%, var(--poodle-color-background-panel));
    --poodle-callout-border: color-mix(in srgb, var(--poodle-color-status-warning) 34%, var(--poodle-color-border-default));
  }

  .callout[data-tone="danger"] {
    --poodle-callout-fill: color-mix(in srgb, var(--poodle-color-status-danger) 10%, var(--poodle-color-background-panel));
    --poodle-callout-border: color-mix(in srgb, var(--poodle-color-status-danger) 34%, var(--poodle-color-border-default));
  }

  .callout[data-tone="pending"] {
    --poodle-callout-fill: color-mix(in srgb, var(--poodle-color-accent-base) 8%, var(--poodle-color-background-panel));
    --poodle-callout-border: color-mix(in srgb, var(--poodle-color-accent-base) 26%, var(--poodle-color-border-default));
  }

  .callout__body {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-self: start;
    gap: var(--poodle-space-inline-md);
    min-width: 0;
  }

  .callout__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.375rem;
    height: 1.375rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 78%, transparent);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    font-weight: 700;
    line-height: 1;
  }

  .callout__content {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .callout__content strong {
    color: var(--poodle-color-text-primary);
  }

  .callout__content :global(p) {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .callout__content strong {
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .callout__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    align-items: center;
    justify-content: flex-end;
  }

  .callout__dismiss {
    width: 1.75rem;
    height: 1.75rem;
    min-height: 0;
    margin-right: calc(-0.5 * var(--poodle-space-panel-x));
    padding: 0;
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.0625rem);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .callout__dismiss:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  @media (max-width: 45rem) {
    .callout {
      grid-template-columns: 1fr;
    }

    .callout__actions {
      justify-content: flex-start;
    }
  }

  /* Size variants */
  .callout[data-size="xs"] .callout__icon {
    width: 0.875rem;
    height: 0.875rem;
    font-size: 0.5rem;
  }
  .callout[data-size="xs"] .callout__content { font-size: 0.625rem; }
  .callout[data-size="xs"] .callout__content strong { font-size: 0.6875rem; }
  .callout[data-size="xs"] .callout__dismiss { width: 1.25rem; height: 1.25rem; }
  .callout[data-size="xs"] { gap: 0.375rem; }

  .callout[data-size="sm"] .callout__icon {
    width: 1.125rem;
    height: 1.125rem;
    font-size: 0.5625rem;
  }
  .callout[data-size="sm"] .callout__content { font-size: 0.6875rem; }
  .callout[data-size="sm"] .callout__content strong { font-size: 0.75rem; }
  .callout[data-size="sm"] .callout__dismiss { width: 1.5rem; height: 1.5rem; }
  .callout[data-size="sm"] { gap: 0.5rem; }

  .callout[data-size="lg"] .callout__icon {
    width: 1.75rem;
    height: 1.75rem;
    font-size: 0.9375rem;
  }
  .callout[data-size="lg"] .callout__content { font-size: 0.9375rem; }
  .callout[data-size="lg"] .callout__content strong { font-size: 1rem; }
  .callout[data-size="lg"] .callout__dismiss { width: 2rem; height: 2rem; }
  .callout[data-size="lg"] { gap: 0.875rem; }

  .callout[data-size="xl"] .callout__icon {
    width: 2rem;
    height: 2rem;
    font-size: 1.0625rem;
  }
  .callout[data-size="xl"] .callout__content { font-size: 1.0625rem; }
  .callout[data-size="xl"] .callout__content strong { font-size: 1.125rem; }
  .callout[data-size="xl"] .callout__dismiss { width: 2.25rem; height: 2.25rem; }
  .callout[data-size="xl"] { gap: 1rem; }

  /* Density variants */
  .callout[data-density="compact"] { padding-inline: 0.5rem; }
  .callout[data-density="comfortable"] { padding-inline: 1.25rem; }
</style>

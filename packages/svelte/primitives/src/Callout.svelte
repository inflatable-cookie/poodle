<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import type { StatusTone } from "./types";

  type CalloutAnnounceMode = "none" | "polite" | "assertive";

  export let tone: StatusTone | "neutral" = "neutral";
  export let title: string | null = null;
  export let message: string | null = null;
  export let ariaLabel: string | null = null;
  export let announceMode: CalloutAnnounceMode = "none";
  export let isDismissible = false;
  export let dismissLabel = "Dismiss message";

  const dispatch = createEventDispatcher<{
    dismiss: void;
  }>();

  const toneIcon: Record<string, string> = {
    success: "check",
    warning: "triangle-alert",
    danger: "circle-x",
    pending: "loader",
    info: "info",
    neutral: "info",
  };

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
  aria-label={ariaLabel ?? undefined}
  role={role}
  aria-live={ariaLive}
>
  <div class="callout__body">
    <span class="callout__icon" aria-hidden="true">
      {#if $$slots.icon}
        <slot name="icon" />
      {:else}
        <Icon name={toneIcon[tone] ?? "info"} size="sm" />
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

  {#if isDismissible}
    <button
      type="button"
      class="callout__dismiss"
      aria-label={dismissLabel}
      on:click={() => dispatch("dismiss")}
    >
      <Icon name="x" size="sm" />
    </button>
  {/if}
</section>

<style>
  .callout {
    --flint-callout-fill: color-mix(in srgb, var(--flint-color-background-panel) 94%, transparent);
    --flint-callout-border: color-mix(in srgb, var(--flint-color-border-subtle) 88%, transparent);
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto auto;
    align-items: center;
    gap: var(--flint-space-inline-md);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid var(--flint-callout-border);
    border-radius: var(--flint-radius-surface);
    background: var(--flint-callout-fill);
    --flint-surface: var(--flint-callout-fill);
    color: var(--flint-color-text-primary);
  }

  .callout[data-tone="info"] {
    --flint-callout-fill: color-mix(in srgb, var(--flint-color-status-info, #3b82f6) 10%, var(--flint-color-background-panel));
    --flint-callout-border: color-mix(in srgb, var(--flint-color-status-info, #3b82f6) 34%, var(--flint-color-border-default));
  }

  .callout[data-tone="success"] {
    --flint-callout-fill: color-mix(in srgb, var(--flint-color-status-success) 10%, var(--flint-color-background-panel));
    --flint-callout-border: color-mix(in srgb, var(--flint-color-status-success) 34%, var(--flint-color-border-default));
  }

  .callout[data-tone="warning"] {
    --flint-callout-fill: color-mix(in srgb, var(--flint-color-status-warning) 10%, var(--flint-color-background-panel));
    --flint-callout-border: color-mix(in srgb, var(--flint-color-status-warning) 34%, var(--flint-color-border-default));
  }

  .callout[data-tone="danger"] {
    --flint-callout-fill: color-mix(in srgb, var(--flint-color-status-danger) 10%, var(--flint-color-background-panel));
    --flint-callout-border: color-mix(in srgb, var(--flint-color-status-danger) 34%, var(--flint-color-border-default));
  }

  .callout[data-tone="pending"] {
    --flint-callout-fill: color-mix(in srgb, var(--flint-color-accent-base) 8%, var(--flint-color-background-panel));
    --flint-callout-border: color-mix(in srgb, var(--flint-color-accent-base) 26%, var(--flint-color-border-default));
  }

  .callout__body {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-self: start;
    gap: var(--flint-space-inline-md);
    min-width: 0;
  }

  .callout__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.375rem;
    height: 1.375rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--flint-color-background-surface) 78%, transparent);
    font-family: var(--flint-typography-code-family);
    font-size: 0.75rem;
    font-weight: 700;
    line-height: 1;
  }

  .callout__content {
    display: grid;
    gap: var(--flint-space-inline-sm);
    min-width: 0;
  }

  .callout__content :global(p) {
    margin: 0;
    color: var(--flint-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .callout__content strong {
    font-family: var(--flint-typography-label-family);
    font-size: var(--flint-typography-label-size);
    line-height: var(--flint-typography-label-lineHeight);
  }

  .callout__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--flint-space-inline-sm);
    align-items: center;
    justify-content: flex-end;
  }

  .callout__dismiss {
    width: 1.75rem;
    height: 1.75rem;
    min-height: 0;
    margin-right: calc(-0.5 * var(--flint-space-panel-x));
    padding: 0;
    border: 0;
    border-radius: calc(var(--flint-radius-control) - 0.0625rem);
    background: transparent;
    color: var(--flint-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .callout__dismiss:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
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
</style>

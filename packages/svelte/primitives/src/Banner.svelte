<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { BannerAnnounceMode, BannerTone } from "./types";

  export let tone: BannerTone = "info";
  export let title: string | null = null;
  export let message: string | null = null;
  export let ariaLabel: string | null = null;
  export let announceMode: BannerAnnounceMode = "polite";
  export let isDismissible = false;
  export let dismissLabel = "Dismiss message";

  const dispatch = createEventDispatcher<{
    dismiss: void;
  }>();

  $: role =
    announceMode === "assertive"
      ? "alert"
      : announceMode === "polite"
        ? "status"
        : undefined;
  $: ariaLive =
    announceMode === "assertive"
      ? "assertive"
      : announceMode === "polite"
        ? "polite"
        : undefined;
</script>

<section
  class="banner"
  data-tone={tone}
  aria-label={ariaLabel ?? undefined}
  role={role}
  aria-live={ariaLive}
>
  <div class="banner__content">
    <span class="banner__icon" aria-hidden="true">
      {#if tone === "success"}
        ✓
      {:else if tone === "warning"}
        !
      {:else if tone === "danger"}
        ×
      {:else}
        i
      {/if}
    </span>
    <div class="banner__copy">
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
    <div class="banner__actions">
      <slot name="actions" />
    </div>
  {/if}

  {#if isDismissible}
    <button
      type="button"
      class="banner__dismiss"
      aria-label={dismissLabel}
      on:click={() => dispatch("dismiss")}
    >
      ×
    </button>
  {/if}
</section>

<style>
  .banner {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto auto;
    align-items: start;
    gap: var(--pug-space-inline-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 92%, transparent);
  }

  .banner[data-tone="info"] {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 34%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-color-accent-base) 10%, transparent);
  }

  .banner[data-tone="success"] {
    border-color: color-mix(in srgb, var(--pug-color-status-success) 34%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-color-status-success) 10%, transparent);
  }

  .banner[data-tone="warning"] {
    border-color: color-mix(in srgb, var(--pug-color-status-warning) 34%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-color-status-warning) 12%, transparent);
  }

  .banner[data-tone="danger"] {
    border-color: color-mix(in srgb, var(--pug-color-status-danger) 34%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-color-status-danger) 10%, transparent);
  }

  .banner__content {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: var(--pug-space-inline-md);
    min-width: 0;
  }

  .banner__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    border-radius: calc(var(--pug-radius-control) - 0.0625rem);
    background: color-mix(in srgb, var(--pug-color-background-surface) 78%, transparent);
    color: var(--pug-color-text-primary);
    font-weight: 700;
    font-size: 0.8125rem;
  }

  .banner__copy {
    display: grid;
    gap: 0.25rem;
    min-width: 0;
  }

  .banner__copy strong,
  .banner__copy p {
    margin: 0;
  }

  .banner__copy p {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .banner__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    align-items: center;
    justify-content: flex-end;
  }

  .banner__dismiss {
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 0.0625rem);
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .banner__dismiss:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  @media (max-width: 45rem) {
    .banner {
      grid-template-columns: 1fr;
    }

    .banner__actions {
      justify-content: flex-start;
    }
  }
</style>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { Progress } from "@pug/svelte-primitives";

  export let isVisible = true;
  export let value: number | null = null;
  export let max: number = 100;
  export let message: string | null = null;
  export let canCancel = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{ cancel: void }>();

  $: isIndeterminate = value === null;
</script>

{#if isVisible}
  <div
    class="page-loading"
    role="status"
    aria-label={ariaLabel ?? "Loading"}
    aria-live="polite"
  >
    <div class="page-loading__backdrop" aria-hidden="true"></div>
    <div class="page-loading__card">
      <div class="page-loading__spinner" aria-hidden="true">
        <svg viewBox="0 0 36 36" fill="none">
          <circle
            cx="18" cy="18" r="15"
            stroke="currentColor"
            stroke-width="2.5"
            opacity="0.2"
          />
          <circle
            class="page-loading__spinner-arc"
            cx="18" cy="18" r="15"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-dasharray="70 30"
          />
        </svg>
      </div>

      {#if !isIndeterminate}
        <div class="page-loading__progress">
          <Progress {value} {max} ariaLabel={message ?? "Loading progress"} />
        </div>
      {/if}

      {#if message}
        <p class="page-loading__message">{message}</p>
      {/if}

      {#if canCancel}
        <button
          type="button"
          class="page-loading__cancel"
          on:click={() => dispatch("cancel")}
        >
          Cancel
        </button>
      {/if}
    </div>
  </div>
{/if}

<style>
  .page-loading {
    position: fixed;
    inset: 0;
    z-index: var(--pug-overlay-z-modal, 1000);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .page-loading__backdrop {
    position: absolute;
    inset: 0;
    background: color-mix(in srgb, var(--pug-color-background-base, #000) 62%, transparent);
    backdrop-filter: blur(2px);
  }

  .page-loading__card {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    min-width: 14rem;
    max-width: 20rem;
    padding: 2rem 2.5rem;
    border: 1px solid color-mix(in srgb, var(--pug-color-border-default) 42%, transparent);
    border-radius: var(--pug-radius-surface);
    background: var(--pug-color-background-elevated);
    box-shadow: var(--pug-elevation-overlay);
  }

  .page-loading__spinner {
    width: 2.5rem;
    height: 2.5rem;
    color: var(--pug-color-accent-base);
  }

  .page-loading__spinner svg {
    width: 100%;
    height: 100%;
  }

  .page-loading__spinner-arc {
    transform-origin: center;
    animation: page-loading-spin 1s linear infinite;
  }

  @keyframes page-loading-spin {
    to {
      transform: rotate(360deg);
    }
  }

  .page-loading__progress {
    width: 100%;
  }

  .page-loading__message {
    margin: 0;
    font-size: var(--pug-typography-label-size, 0.8125rem);
    color: var(--pug-color-text-secondary);
    text-align: center;
    line-height: 1.4;
  }

  .page-loading__cancel {
    padding: 0.375rem 0.875rem;
    border: 1px solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-secondary);
    font: inherit;
    font-size: var(--pug-typography-label-size, 0.8125rem);
    cursor: pointer;
    transition: background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .page-loading__cancel:hover {
    background: color-mix(in srgb, var(--pug-color-background-surface) 72%, transparent);
  }

  .page-loading__cancel:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>

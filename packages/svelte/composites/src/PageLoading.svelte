<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { Progress, Spinner } from "@poodle/svelte-primitives";

  export let visible = true;
  export let value: number | null = null;
  export let max: number = 100;
  export let message: string | null = null;
  export let canCancel = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{ cancel: void }>();

  $: isIndeterminate = value === null;
</script>

{#if visible}
  <div
    class="page-loading"
    role="status"
    aria-label={ariaLabel ?? "Loading"}
    aria-live="polite"
  >
    <div class="page-loading__backdrop" aria-hidden="true"></div>
    <div class="page-loading__card">
      <Spinner className="page-loading__spinner" variant="ring" sizeRole="prominent" tone="accent" />

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
    z-index: var(--poodle-overlay-z-modal, 1000);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .page-loading__backdrop {
    position: absolute;
    inset: 0;
    background: color-mix(in srgb, var(--poodle-color-background-base, #000) 62%, transparent);
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
    border: 1px solid color-mix(in srgb, var(--poodle-color-border-default) 42%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-elevated);
    box-shadow: var(--poodle-elevation-overlay);
  }

  .page-loading__progress {
    width: 100%;
  }

  .page-loading__message {
    margin: 0;
    font-size: var(--poodle-typography-label-size, 0.8125rem);
    color: var(--poodle-color-text-secondary);
    text-align: center;
    line-height: 1.4;
  }

  .page-loading__cancel {
    padding: 0.375rem 0.875rem;
    border: 1px solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    font: inherit;
    font-size: var(--poodle-typography-label-size, 0.8125rem);
    cursor: pointer;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .page-loading__cancel:hover {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 72%, transparent);
  }

  .page-loading__cancel:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>

<script lang="ts">
  import { default as Progress } from "./Progress.svelte";
  import { default as Spinner } from "./Spinner.svelte";

  type PageLoadingPresentation = "overlay" | "inline";

  let {
    visible = true,
    value = null,
    max = 100,
    message = null,
    canCancel = false,
    ariaLabel = null,
    presentation = "overlay",
    onCancel = undefined,
  }: {
    visible?: boolean;
    value?: number | null;
    max?: number;
    message?: string | null;
    canCancel?: boolean;
    ariaLabel?: string | null;
    presentation?: PageLoadingPresentation;
    onCancel?: (() => void) | undefined;
  } = $props();

  const isIndeterminate = $derived(value === null);
  const isOverlay = $derived(presentation === "overlay");
</script>

{#if visible}
  <div
    class="poodle-page-loading"
    data-presentation={presentation}
    role="status"
    aria-label={ariaLabel ?? "Loading"}
    aria-live="polite"
  >
    {#if isOverlay}
      <div class="poodle-page-loading__backdrop" aria-hidden="true"></div>
    {/if}
    <div class="poodle-page-loading__card">
      <Spinner className="poodle-page-loading__spinner" variant="ring" sizeRole="prominent" tone="accent" />

      {#if !isIndeterminate}
        <div class="poodle-page-loading__progress">
          <Progress {value} {max} ariaLabel={message ?? "Loading progress"} />
        </div>
      {/if}

      {#if message}
        <p class="poodle-page-loading__message">{message}</p>
      {/if}

      {#if canCancel}
        <button
          type="button"
          class="poodle-page-loading__cancel"
          onclick={() => onCancel?.()}
        >
          Cancel
        </button>
      {/if}
    </div>
  </div>
{/if}

<style>
  .poodle-page-loading {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .poodle-page-loading[data-presentation="overlay"] {
    position: fixed;
    inset: 0;
    z-index: var(--poodle-overlay-z-modal, 1000);
  }

  .poodle-page-loading[data-presentation="inline"] {
    position: relative;
    min-height: 12rem;
    padding: 3rem 1rem;
  }

  .poodle-page-loading__backdrop {
    position: absolute;
    inset: 0;
    background: color-mix(in srgb, var(--poodle-color-background-base, #000) 62%, transparent);
    backdrop-filter: blur(2px);
  }

  .poodle-page-loading__card {
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

  .poodle-page-loading[data-presentation="inline"] .poodle-page-loading__card {
    min-width: auto;
    max-width: 24rem;
    padding: 0;
    border: none;
    background: transparent;
    box-shadow: none;
  }

  .poodle-page-loading__progress {
    width: 100%;
  }

  .poodle-page-loading__message {
    margin: 0;
    font-size: var(--poodle-typography-label-size, 0.8125rem);
    color: var(--poodle-color-text-secondary);
    text-align: center;
    line-height: 1.4;
  }

  .poodle-page-loading__cancel {
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

  .poodle-page-loading__cancel:hover {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 72%, transparent);
  }

  .poodle-page-loading__cancel:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>

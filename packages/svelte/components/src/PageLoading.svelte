<script lang="ts">
  import "@poodle/styles/page-loading.css";
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


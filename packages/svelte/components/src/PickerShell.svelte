<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as Spinner } from "./Spinner.svelte";

  import type { BrowseState, PickerVariant } from "./types";

  interface Props {
    title: string;
    description?: string | null;
    variant?: PickerVariant;
    state?: BrowseState;
    ariaLabel?: string | null;
    resultCount?: number | null;
    selectionCount?: number;
    stateTitle?: string | null;
    stateMessage?: string | null;
    statusText?: string | null;
    statusId?: string | null;
    toolbar?: Snippet<[]>;
    selection?: Snippet<[]>;
    stateContent?: Snippet<[]>;
    footer?: Snippet<[]>;
    children?: Snippet<[]>;
  }

  let {
    title,
    description = null,
    variant = "inline",
    state = "ready",
    ariaLabel = null,
    resultCount = null,
    selectionCount = 0,
    stateTitle = null,
    stateMessage = null,
    statusText = null,
    statusId = null,
    toolbar,
    selection,
    stateContent,
    footer,
    children,
  }: Props = $props();
</script>

<section
  class="poodle-picker-shell"
  data-variant={variant}
  data-state={state}
  aria-label={ariaLabel ?? undefined}
>
  <div class="poodle-picker-shell__header">
    <div>
      <h3 class="poodle-picker-shell__title">{title}</h3>
      {#if description}
        <p class="poodle-picker-shell__description">{description}</p>
      {/if}
    </div>
    <div class="poodle-picker-shell__meta">
      {#if resultCount !== null}
        <span>{resultCount} results</span>
      {/if}
      <span>{selectionCount} selected</span>
    </div>
  </div>

  {#if toolbar}
    <div class="poodle-picker-shell__toolbar">
      {@render toolbar()}
    </div>
  {/if}

  {#if selection}
    <div class="poodle-picker-shell__selection">
      {@render selection()}
    </div>
  {/if}

  <!-- Screen-reader-only live region for status updates -->
  {#if statusText}
    <p class="poodle-picker-shell__status poodle-sr-only" id={statusId ?? undefined} role="status" aria-live="polite" aria-atomic="true">
      {statusText}
    </p>
  {/if}

  {#if state === "ready"}
    <div class="poodle-picker-shell__body">
      {@render children?.()}
    </div>
  {:else}
    <div class="poodle-picker-shell__state">
      {#if stateContent}
        {@render stateContent()}
      {:else}
        {#if state === "loading"}
          <span class="poodle-picker-shell__spinner" aria-hidden="true">
            <Spinner variant="grid" tone="accent" />
          </span>
        {/if}
        <strong>{stateTitle ?? "Picker state"}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}

  {#if footer}
    <div class="poodle-picker-shell__footer">
      {@render footer()}
    </div>
  {/if}
</section>

<style>
  .poodle-picker-shell {
    display: grid;
    grid-template-rows: auto;
    gap: var(--poodle-space-stack-md);
    box-sizing: border-box;
    min-width: 0;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
  }

  .poodle-picker-shell[data-variant="popover"] {
    width: 100%;
    max-width: min(32rem, calc(100vw - 2rem));
    box-shadow: var(--poodle-elevation-overlay);
  }

  .poodle-picker-shell[data-variant="modal"] {
    box-shadow: var(--poodle-elevation-dialog);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 96%, transparent);
  }

  .poodle-picker-shell__header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
  }

  .poodle-picker-shell__title,
  .poodle-picker-shell__description {
    margin: 0;
  }

  .poodle-picker-shell__title {
    font-size: 1.25rem;
    line-height: 1.2;
  }

  .poodle-picker-shell__description,
  .poodle-picker-shell__meta,
  .poodle-picker-shell__state p,
  .poodle-picker-shell__status {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-picker-shell__status {
    margin: 0;
  }

  .poodle-sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .poodle-picker-shell__meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    align-items: baseline;
  }

  .poodle-picker-shell__body {
    min-height: 0;
    overflow-y: auto;
  }

  .poodle-picker-shell__state {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    justify-items: start;
    padding: calc(var(--poodle-space-panel-y) * 1.5) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 86%, transparent);
  }

  .poodle-picker-shell__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .poodle-picker-shell__state strong,
  .poodle-picker-shell__state p {
    margin: 0;
  }
</style>

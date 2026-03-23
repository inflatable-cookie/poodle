<script lang="ts">
  import type { BrowseState, PickerVariant } from "./types";

  export let title: string;
  export let description: string | null = null;
  export let variant: PickerVariant = "inline";
  export let state: BrowseState = "ready";
  export let ariaLabel: string | null = null;
  export let resultCount: number | null = null;
  export let selectionCount = 0;
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;
  export let statusText: string | null = null;
  export let statusId: string | null = null;
</script>

<section
  class="picker-shell"
  data-variant={variant}
  data-state={state}
  aria-label={ariaLabel ?? undefined}
>
  <div class="picker-shell__header">
    <div>
      <h3 class="picker-shell__title">{title}</h3>
      {#if description}
        <p class="picker-shell__description">{description}</p>
      {/if}
    </div>
    <div class="picker-shell__meta">
      {#if resultCount !== null}
        <span>{resultCount} results</span>
      {/if}
      <span>{selectionCount} selected</span>
    </div>
  </div>

  {#if statusText}
    <p class="picker-shell__status" id={statusId ?? undefined} role="status" aria-live="polite" aria-atomic="true">
      {statusText}
    </p>
  {/if}

  {#if $$slots.toolbar}
    <div class="picker-shell__toolbar">
      <slot name="toolbar" />
    </div>
  {/if}

  {#if $$slots.selection}
    <div class="picker-shell__selection">
      <slot name="selection" />
    </div>
  {/if}

  {#if state === "ready"}
    <div class="picker-shell__body">
      <slot />
    </div>
  {:else}
    <div class="picker-shell__state">
      {#if $$slots.state}
        <slot name="state" />
      {:else}
        <strong>{stateTitle ?? "Picker state"}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}

  {#if $$slots.footer}
    <div class="picker-shell__footer">
      <slot name="footer" />
    </div>
  {/if}
</section>

<style>
  .picker-shell {
    display: grid;
    gap: var(--flint-space-stack-md);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid var(--flint-color-border-subtle);
    border-radius: var(--flint-radius-surface);
    background: color-mix(in srgb, var(--flint-color-background-panel) 94%, transparent);
  }

  .picker-shell[data-variant="popover"] {
    max-width: 30rem;
    box-shadow: var(--flint-elevation-overlay);
  }

  .picker-shell[data-variant="modal"] {
    box-shadow: var(--flint-elevation-dialog);
    background: color-mix(in srgb, var(--flint-color-background-elevated) 96%, transparent);
  }

  .picker-shell__header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: var(--flint-space-inline-md);
  }

  .picker-shell__title,
  .picker-shell__description {
    margin: 0;
  }

  .picker-shell__title {
    font-size: 1.25rem;
    line-height: 1.2;
  }

  .picker-shell__description,
  .picker-shell__meta,
  .picker-shell__state p,
  .picker-shell__status {
    color: var(--flint-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .picker-shell__status {
    margin: 0;
  }

  .picker-shell__meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--flint-space-inline-sm);
    align-items: baseline;
  }

  .picker-shell__state {
    display: grid;
    gap: var(--flint-space-stack-sm);
    padding: calc(var(--flint-space-panel-y) * 1.5) var(--flint-space-panel-x);
    border: 0.0625rem solid var(--flint-color-border-subtle);
    border-radius: var(--flint-radius-surface);
    background: color-mix(in srgb, var(--flint-color-background-surface) 86%, transparent);
  }

  .picker-shell__state strong,
  .picker-shell__state p {
    margin: 0;
  }
</style>

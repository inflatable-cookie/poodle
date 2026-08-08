<script lang="ts">
  import "@poodle/styles/picker-shell.css";
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


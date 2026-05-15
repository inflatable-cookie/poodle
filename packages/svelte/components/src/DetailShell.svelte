<script lang="ts">
  import type { Snippet } from "svelte";
  import Spinner from "./Spinner.svelte";

  import type { BrowseState } from "./types";

  interface Props {
    title?: string | null;
    scrollMode?: "shell" | "body";
    state?: Exclude<BrowseState, "no-results">;
    ariaLabel?: string | null;
    stateTitle?: string | null;
    stateMessage?: string | null;
    header?: Snippet;
    stateContent?: Snippet;
    children?: Snippet;
  }

  let {
    title = null,
    scrollMode = "body",
    state = "ready",
    ariaLabel = null,
    stateTitle = null,
    stateMessage = null,
    header,
    stateContent,
    children,
  }: Props = $props();
</script>

<section class="poodle-detail-shell" data-scroll-mode={scrollMode} aria-label={ariaLabel ?? undefined}>
  {#if header || title}
    <div class="poodle-detail-shell__header">
      {#if header}
        {@render header()}
      {:else if title}
        <h2>{title}</h2>
      {/if}
    </div>
  {/if}

  {#if state === "ready"}
    <div class="poodle-detail-shell__body">
      {@render children?.()}
    </div>
  {:else}
    <div class="poodle-detail-shell__state" data-state={state}>
      {#if stateContent}
        {@render stateContent()}
      {:else}
        {#if state === "loading"}
          <span class="poodle-detail-shell__spinner" aria-hidden="true">
            <Spinner variant="grid" tone="accent" />
          </span>
        {/if}
        <strong>{stateTitle ?? "Detail state"}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}
</section>

<style>
  .poodle-detail-shell {
    display: grid;
    gap: var(--poodle-space-stack-lg);
  }

  .poodle-detail-shell__body,
  .poodle-detail-shell__state {
    display: grid;
    gap: var(--poodle-space-stack-lg);
  }

  .poodle-detail-shell__state {
    padding: calc(var(--poodle-space-panel-y) * 2) calc(var(--poodle-space-panel-x) * 1.5);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 96%, var(--poodle-color-background-elevated));
  }

  .poodle-detail-shell__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .poodle-detail-shell__state strong,
  .poodle-detail-shell__state p {
    margin: 0;
  }

  .poodle-detail-shell__state p {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }
</style>

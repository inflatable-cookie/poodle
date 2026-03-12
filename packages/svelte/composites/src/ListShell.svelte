<script lang="ts">
  import type { BrowseState } from "./types";

  export let state: BrowseState = "ready";
  export let ariaLabel: string | null = null;
  export let itemCount: number | null = null;
  export let scrollMode: "shell" | "list" = "list";
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;
</script>

<section class="list-shell" data-scroll-mode={scrollMode} aria-label={ariaLabel ?? undefined}>
  {#if $$slots.header}
    <div class="list-shell__header">
      <slot name="header" />
    </div>
  {/if}

  {#if itemCount !== null && state === "ready"}
    <p class="list-shell__summary">{itemCount} items</p>
  {/if}

  {#if state === "ready"}
    <div class="list-shell__viewport">
      <ul class="list-shell__content">
        <slot />
      </ul>
    </div>
  {:else}
    <div class="list-shell__state" data-state={state}>
      {#if $$slots.state}
        <slot name="state" />
      {:else}
        <strong>{stateTitle ?? "Collection state"}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}

  {#if $$slots.footer}
    <div class="list-shell__footer">
      <slot name="footer" />
    </div>
  {/if}
</section>

<style>
  .list-shell {
    display: grid;
    gap: var(--pug-space-stack-md);
  }

  .list-shell__summary {
    margin: 0;
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .list-shell__viewport,
  .list-shell__state {
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 92%, transparent);
  }

  .list-shell__content {
    display: grid;
    gap: 0;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .list-shell__state {
    display: grid;
    gap: var(--pug-space-stack-sm);
    padding: calc(var(--pug-space-panel-y) * 2) calc(var(--pug-space-panel-x) * 1.5);
    text-align: left;
  }

  .list-shell__state strong,
  .list-shell__state p {
    margin: 0;
  }

  .list-shell__state p {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }
</style>

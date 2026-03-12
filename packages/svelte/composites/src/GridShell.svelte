<script lang="ts">
  import type { BrowseState, MinColumnWidth } from "./types";

  export let state: BrowseState = "ready";
  export let ariaLabel: string | null = null;
  export let itemCount: number | null = null;
  export let minColumnWidth: MinColumnWidth = "md";
  export let scrollMode: "shell" | "grid" = "grid";
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;

  $: columnWidth =
    minColumnWidth === "sm"
      ? "180px"
      : minColumnWidth === "lg"
        ? "280px"
        : "220px";
</script>

<section class="grid-shell" data-scroll-mode={scrollMode} aria-label={ariaLabel ?? undefined}>
  {#if $$slots.header}
    <div class="grid-shell__header">
      <slot name="header" />
    </div>
  {/if}

  {#if itemCount !== null && state === "ready"}
    <p class="grid-shell__summary">{itemCount} items</p>
  {/if}

  {#if state === "ready"}
    <div class="grid-shell__viewport">
      <div class="grid-shell__content" style={`--pug-grid-shell-column-width: ${columnWidth};`}>
        <slot />
      </div>
    </div>
  {:else}
    <div class="grid-shell__state" data-state={state}>
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
    <div class="grid-shell__footer">
      <slot name="footer" />
    </div>
  {/if}
</section>

<style>
  .grid-shell {
    display: grid;
    gap: var(--pug-space-stack-md);
  }

  .grid-shell__summary {
    margin: 0;
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .grid-shell__viewport,
  .grid-shell__state {
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 92%, transparent);
  }

  .grid-shell__content {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(var(--pug-grid-shell-column-width), 1fr));
    gap: var(--pug-space-inline-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
  }

  .grid-shell__state {
    display: grid;
    gap: var(--pug-space-stack-sm);
    padding: calc(var(--pug-space-panel-y) * 2) calc(var(--pug-space-panel-x) * 1.5);
  }

  .grid-shell__state strong,
  .grid-shell__state p {
    margin: 0;
  }

  .grid-shell__state p {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }
</style>

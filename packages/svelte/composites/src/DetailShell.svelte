<script lang="ts">
  import type { BrowseState } from "./types";

  export let title: string | null = null;
  export let scrollMode: "shell" | "body" = "body";
  export let state: Exclude<BrowseState, "no-results"> = "ready";
  export let ariaLabel: string | null = null;
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;
</script>

<section class="detail-shell" data-scroll-mode={scrollMode} aria-label={ariaLabel ?? undefined}>
  {#if $$slots.header || title}
    <div class="detail-shell__header">
      {#if $$slots.header}
        <slot name="header" />
      {:else if title}
        <h2>{title}</h2>
      {/if}
    </div>
  {/if}

  {#if state === "ready"}
    <div class="detail-shell__body">
      <slot />
    </div>
  {:else}
    <div class="detail-shell__state" data-state={state}>
      {#if $$slots.state}
        <slot name="state" />
      {:else}
        <strong>{stateTitle ?? "Detail state"}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}
</section>

<style>
  .detail-shell {
    display: grid;
    gap: var(--pug-space-stack-lg);
  }

  .detail-shell__body,
  .detail-shell__state {
    display: grid;
    gap: var(--pug-space-stack-lg);
  }

  .detail-shell__state {
    padding: calc(var(--pug-space-panel-y) * 2) calc(var(--pug-space-panel-x) * 1.5);
    border: 0.0625rem solid transparent;
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 96%, var(--pug-color-background-elevated));
  }

  .detail-shell__state strong,
  .detail-shell__state p {
    margin: 0;
  }

  .detail-shell__state p {
    color: var(--pug-color-text-secondary);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }
</style>

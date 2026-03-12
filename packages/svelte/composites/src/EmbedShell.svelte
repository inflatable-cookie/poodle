<script lang="ts">
  import type { AspectRatio, MediaState } from "./types";

  export let title: string;
  export let description: string | null = null;
  export let provider: string | null = null;
  export let state: MediaState = "ready";
  export let aspectRatio: AspectRatio = "video";
  export let ariaLabel: string | null = null;
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;

  $: resolvedStateTitle =
    stateTitle ??
    (state === "loading"
      ? "Loading embed"
      : state === "error"
        ? "Embed unavailable"
        : "No embedded content");
</script>

<section
  class="embed-shell"
  data-state={state}
  data-aspect-ratio={aspectRatio}
  aria-label={ariaLabel ?? title}
  aria-busy={state === "loading"}
>
  <div class="embed-shell__header">
    <div>
      <h3>{title}</h3>
      {#if description}
        <p>{description}</p>
      {/if}
    </div>
    {#if provider}
      <span class="embed-shell__provider">{provider}</span>
    {/if}
  </div>

  {#if state === "ready"}
    <div class="embed-shell__viewport">
      <slot />
    </div>
  {:else}
    <div class="embed-shell__state">
      {#if $$slots.state}
        <slot name="state" />
      {:else}
        <strong>{resolvedStateTitle}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}

  {#if $$slots.footer}
    <div class="embed-shell__footer">
      <slot name="footer" />
    </div>
  {/if}
</section>

<style>
  .embed-shell {
    display: grid;
    gap: var(--pug-space-stack-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 96%, transparent);
  }

  .embed-shell__header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: var(--pug-space-inline-md);
  }

  .embed-shell__header h3,
  .embed-shell__header p {
    margin: 0;
  }

  .embed-shell__header p,
  .embed-shell__state p {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .embed-shell__provider {
    align-self: start;
    padding: 0.375rem 0.625rem;
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 72%, transparent);
    color: var(--pug-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 600;
  }

  .embed-shell__viewport,
  .embed-shell__state {
    overflow: hidden;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: calc(var(--pug-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-surface) 78%, transparent);
  }

  .embed-shell[data-aspect-ratio="square"] .embed-shell__viewport,
  .embed-shell[data-aspect-ratio="square"] .embed-shell__state {
    aspect-ratio: 1 / 1;
  }

  .embed-shell[data-aspect-ratio="landscape"] .embed-shell__viewport,
  .embed-shell[data-aspect-ratio="landscape"] .embed-shell__state {
    aspect-ratio: 16 / 10;
  }

  .embed-shell[data-aspect-ratio="portrait"] .embed-shell__viewport,
  .embed-shell[data-aspect-ratio="portrait"] .embed-shell__state {
    aspect-ratio: 3 / 4;
  }

  .embed-shell[data-aspect-ratio="video"] .embed-shell__viewport,
  .embed-shell[data-aspect-ratio="video"] .embed-shell__state {
    aspect-ratio: 16 / 9;
  }

  .embed-shell__state {
    display: grid;
    gap: var(--pug-space-stack-sm);
    align-content: start;
    justify-items: start;
    padding: calc(var(--pug-space-panel-y) * 1.5) var(--pug-space-panel-x);
    text-align: left;
  }

  .embed-shell__state strong,
  .embed-shell__state p {
    margin: 0;
  }
</style>

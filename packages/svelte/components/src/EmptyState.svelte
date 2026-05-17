<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation } from "./presentation";
  import type { ControlDensity } from "./types";

  import type { EmptyStateSize, EmptyStateVariant } from "./types";

  interface Props {
    title: string;
    message?: string | null;
    variant?: EmptyStateVariant;
    size?: EmptyStateSize;
    density?: ControlDensity | null;
    ariaLabel?: string | null;
    visual?: Snippet;
    actions?: Snippet;
  }

  let {
    title,
    message = null,
    variant = "neutral",
    size = "default",
    density = null,
    ariaLabel = null,
    visual,
    actions,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<section class="poodle-empty-state" data-variant={variant} data-size={size} data-density={resolvedDensity} aria-label={ariaLabel ?? title}>
  <div class="poodle-empty-state__visual" aria-hidden="true">
    {#if visual}
      {@render visual()}
    {:else}
      {#if variant === "search"}
        <Icon name="search" />
      {:else if variant === "firstRun"}
        <Icon name="plus" />
      {:else}
        <Icon name="inbox" />
      {/if}
    {/if}
  </div>

  <div class="poodle-empty-state__copy">
    <h3>{title}</h3>
    {#if message}
      <p>{message}</p>
    {/if}
  </div>

  {#if actions}
    <div class="poodle-empty-state__actions">
      {@render actions()}
    </div>
  {/if}
</section>

<style>
  .poodle-empty-state {
    display: grid;
    justify-items: center;
    text-align: center;
    gap: var(--poodle-space-stack-md);
    padding: calc(var(--poodle-space-panel-y) * 1.5) var(--poodle-space-panel-x);
    border: 0.0625rem dashed var(--poodle-color-border-default);
    border-radius: calc(var(--poodle-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent);
  }

  .poodle-empty-state[data-variant="search"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 7%, transparent);
  }

  .poodle-empty-state[data-variant="firstRun"] {
    background: color-mix(in srgb, var(--poodle-color-status-success) 7%, transparent);
  }

  .poodle-empty-state__visual {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.25rem;
    height: 2.25rem;
    border-radius: 999rem;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent);
    color: var(--poodle-color-text-secondary);
    font-size: 1.125rem;
    font-weight: 600;
  }

  .poodle-empty-state[data-size="compact"] .poodle-empty-state__visual {
    width: 1.75rem;
    height: 1.75rem;
    font-size: 0.9375rem;
  }

  .poodle-empty-state__copy {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    max-width: 24rem;
  }

  .poodle-empty-state__copy h3,
  .poodle-empty-state__copy p {
    margin: 0;
  }

  .poodle-empty-state__copy h3 {
    font-size: 1.125rem;
    line-height: 1.2;
  }

  .poodle-empty-state[data-size="compact"] .poodle-empty-state__copy h3 {
    font-size: 0.9375rem;
  }

  .poodle-empty-state__copy p {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-empty-state[data-size="compact"] .poodle-empty-state__copy p {
    font-size: 0.75rem;
  }

  .poodle-empty-state__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
  }

  /* ── Density variants ─────────────────────────────────────── */

  .poodle-empty-state[data-density="compact"] {
    gap: var(--poodle-space-stack-sm);
    padding: var(--poodle-space-stack-lg) var(--poodle-space-panel-x);
  }

  .poodle-empty-state[data-density="comfortable"] {
    gap: var(--poodle-space-stack-lg);
    padding: calc(var(--poodle-space-panel-y) * 2) var(--poodle-space-panel-x);
  }
</style>

<script lang="ts">
  import "@poodle/styles/empty-state.css";
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


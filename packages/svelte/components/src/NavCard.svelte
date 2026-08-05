<script lang="ts">
  import "@poodle/styles/nav-card.css";
  import type { Snippet } from "svelte";
  import { getUiPresentation } from "./presentation.ts";

  import type { ControlDensity } from "./types.ts";

  interface Props {
    title: string;
    description?: string | null;
    href?: string | null;
    badge?: string | null;
    disabled?: boolean;
    ariaLabel?: string | null;
    density?: ControlDensity | null;
    onClick?: ((event: MouseEvent) => void) | null;
    icon?: Snippet;
  }

  let {
    title,
    description = null,
    href = null,
    badge = null,
    disabled = false,
    ariaLabel = null,
    density = null,
    onClick = null,
    icon,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  function handleClick(event: MouseEvent): void {
    if (disabled) {
      event.preventDefault();
      return;
    }
    onClick?.(event);
  }
</script>

{#if href && !disabled}
  <a
    class="poodle-nav-card"
    {href}
    aria-label={ariaLabel ?? title}
    data-disabled={disabled}
    data-density={resolvedDensity}
    onclick={handleClick}
  >
    {#if icon}
      <span class="poodle-nav-card__icon" aria-hidden="true">
        {@render icon()}
      </span>
    {/if}
    <div class="poodle-nav-card__content">
      <span class="poodle-nav-card__title">
        {title}
        {#if badge}
          <span class="poodle-nav-card__badge">{badge}</span>
        {/if}
      </span>
      {#if description}
        <span class="poodle-nav-card__description">{description}</span>
      {/if}
    </div>
    <svg class="poodle-nav-card__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </a>
{:else}
  <button
    type="button"
    class="poodle-nav-card"
    aria-label={ariaLabel ?? title}
    disabled={disabled}
    data-disabled={disabled}
    data-density={resolvedDensity}
    onclick={handleClick}
  >
    {#if icon}
      <span class="poodle-nav-card__icon" aria-hidden="true">
        {@render icon()}
      </span>
    {/if}
    <div class="poodle-nav-card__content">
      <span class="poodle-nav-card__title">
        {title}
        {#if badge}
          <span class="poodle-nav-card__badge">{badge}</span>
        {/if}
      </span>
      {#if description}
        <span class="poodle-nav-card__description">{description}</span>
      {/if}
    </div>
    <svg class="poodle-nav-card__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </button>
{/if}


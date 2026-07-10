<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    href?: string | null;
    target?: string | null;
    rel?: string | null;
    ariaLabel?: string | null;
    disabled?: boolean;
    tone?: "accent" | "inherit" | "secondary";
    className?: string;
    onClick?: ((event: MouseEvent) => void) | null;
    children?: Snippet<[]>;
  }

  let {
    href = null,
    target = null,
    rel = null,
    ariaLabel = null,
    disabled = false,
    tone = "accent",
    className = "",
    onClick = null,
    children,
  }: Props = $props();

  const rootClassName = $derived(`poodle-text-link${className ? ` ${className}` : ""}`);

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
    class={rootClassName}
    data-tone={tone}
    href={href}
    target={target ?? undefined}
    rel={rel ?? undefined}
    aria-label={ariaLabel ?? undefined}
    onclick={handleClick}
  >
    {@render children?.()}
  </a>
{:else}
  <button
    type="button"
    class={rootClassName}
    data-tone={tone}
    disabled={disabled}
    aria-label={ariaLabel ?? undefined}
    onclick={handleClick}
  >
    {@render children?.()}
  </button>
{/if}

<style>
  .poodle-text-link {
    display: inline;
    padding: 0;
    border: 0;
    background: var(--poodle-recipe-text-link-fill, transparent);
    color: var(--poodle-recipe-text-link-text, var(--poodle-color-accent-base));
    cursor: pointer;
    font: inherit;
    line-height: inherit;
    text-align: inherit;
    text-decoration: underline;
    text-decoration-color: color-mix(in srgb, currentColor 55%, transparent);
    text-decoration-thickness: 0.0625rem;
    text-underline-offset: 0.125rem;
  }

  .poodle-text-link[data-tone="inherit"] {
    color: inherit;
  }

  .poodle-text-link[data-tone="secondary"] {
    color: var(--poodle-recipe-text-link-secondary-text, var(--poodle-color-text-secondary));
  }

  .poodle-text-link:hover:not(:disabled),
  .poodle-text-link:focus-visible {
    text-decoration-color: currentColor;
  }

  .poodle-text-link:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: 0.125rem;
  }

  .poodle-text-link:disabled {
    cursor: default;
    opacity: var(--poodle-state-opacity-disabled);
  }
</style>

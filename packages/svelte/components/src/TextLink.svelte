<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/text-link.css";
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


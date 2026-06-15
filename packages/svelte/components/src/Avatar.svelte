<script lang="ts">
  interface Props {
    src?: string | null;
    alt?: string | null;
    initials?: string | null;
    ariaLabel?: string | null;
    decorative?: boolean;
    size?: "xs" | "sm" | "md" | "lg" | "xl";
    shape?: "circle" | "rounded";
    tone?: "neutral" | "accent";
  }

  let {
    src = null,
    alt = null,
    initials = null,
    ariaLabel = null,
    decorative = false,
    size = "md",
    shape = "circle",
    tone = "neutral",
  }: Props = $props();

  const fallbackText = $derived((initials ?? "?").trim().slice(0, 3).toUpperCase() || "?");
  const accessibleLabel = $derived(decorative ? undefined : ariaLabel ?? alt ?? initials ?? "Avatar");
</script>

<span
  class="poodle-avatar"
  data-size={size}
  data-shape={shape}
  data-tone={tone}
  aria-hidden={decorative ? "true" : undefined}
  aria-label={src ? undefined : accessibleLabel}
  role={!decorative && !src ? "img" : undefined}
>
  {#if src}
    <img src={src} alt={decorative ? "" : alt ?? accessibleLabel} />
  {:else}
    <span class="poodle-avatar__initials">{fallbackText}</span>
  {/if}
</span>

<style>
  .poodle-avatar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex: 0 0 auto;
    width: var(--poodle-avatar-size);
    height: var(--poodle-avatar-size);
    overflow: hidden;
    background: var(--poodle-avatar-background);
    color: var(--poodle-avatar-color);
    font-size: var(--poodle-avatar-font-size);
    font-weight: 600;
    line-height: 1;
    user-select: none;
  }

  .poodle-avatar[data-shape="circle"] {
    border-radius: 50%;
  }

  .poodle-avatar[data-shape="rounded"] {
    border-radius: var(--poodle-radius-control);
  }

  .poodle-avatar[data-tone="neutral"] {
    --poodle-avatar-background: color-mix(in srgb, var(--poodle-color-background-surface) 82%, var(--poodle-color-text-primary));
    --poodle-avatar-color: var(--poodle-color-text-secondary);
  }

  .poodle-avatar[data-tone="accent"] {
    --poodle-avatar-background: color-mix(in srgb, var(--poodle-color-accent-base) 76%, var(--poodle-color-background-surface));
    --poodle-avatar-color: var(--poodle-color-text-primary);
  }

  .poodle-avatar[data-size="xs"] {
    --poodle-avatar-size: 1.5rem;
    --poodle-avatar-font-size: 0.625rem;
  }

  .poodle-avatar[data-size="sm"] {
    --poodle-avatar-size: 2rem;
    --poodle-avatar-font-size: 0.75rem;
  }

  .poodle-avatar[data-size="md"] {
    --poodle-avatar-size: 2.75rem;
    --poodle-avatar-font-size: 1rem;
  }

  .poodle-avatar[data-size="lg"] {
    --poodle-avatar-size: 4.5rem;
    --poodle-avatar-font-size: 1.5rem;
  }

  .poodle-avatar[data-size="xl"] {
    --poodle-avatar-size: 6rem;
    --poodle-avatar-font-size: 2rem;
  }

  .poodle-avatar img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .poodle-avatar__initials {
    display: inline-flex;
  }
</style>

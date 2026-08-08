<script lang="ts">
  import "@inflatable-cookie/poodle-styles/avatar.css";
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


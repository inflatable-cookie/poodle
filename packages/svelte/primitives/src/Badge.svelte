<script lang="ts">
  export let variant: "accent" | "muted" = "accent";
  export let ariaLabel: string | null = null;
</script>

<span class="badge" data-variant={variant} aria-label={ariaLabel ?? undefined}>
  <slot />
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 1.25rem;
    padding: 0.125rem 0.4375rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    line-height: 1;
    text-transform: uppercase;
    white-space: nowrap;
  }

  .badge[data-variant="muted"] {
    background: color-mix(in srgb, var(--pug-color-background-surface) 88%, transparent);
    color: var(--pug-color-text-secondary);
  }
</style>

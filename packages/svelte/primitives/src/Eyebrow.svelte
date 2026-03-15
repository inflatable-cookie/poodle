<script lang="ts">
  export let ariaLabel: string | null = null;
</script>

<p class="eyebrow" aria-label={ariaLabel ?? undefined}>
  <slot />
</p>

<style>
  .eyebrow {
    margin: 0;
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    line-height: 1.5;
    text-transform: uppercase;
  }
</style>

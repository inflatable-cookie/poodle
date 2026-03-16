<script lang="ts">
  export let columns: 1 | 2 | 3 | 4 = 2;
  export let ariaLabel: string | null = null;
</script>

<nav class="nav-card-grid" aria-label={ariaLabel ?? undefined} style="--columns: {columns}">
  <slot />
</nav>

<style>
  .nav-card-grid {
    display: grid;
    grid-template-columns: repeat(var(--columns, 2), 1fr);
    gap: 0.75rem;
  }

  @media (max-width: 640px) {
    .nav-card-grid {
      grid-template-columns: 1fr;
    }
  }
</style>

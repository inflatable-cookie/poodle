<script lang="ts">
  import { setPillContext } from "./pill-context";

  export let ariaLabel: string | null = null;
  export let showSeparators = true;

  setPillContext({
    size: "sm",
    typography: "inherit",
  });
</script>

<div
  class="poodle-meta-bar"
  data-separators={showSeparators}
  aria-label={ariaLabel ?? undefined}
>
  <slot />
</div>

<style>
  .poodle-meta-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .poodle-meta-bar > :global(*) {
    min-width: 0;
  }

  .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"]) {
    position: relative;
    padding-inline-start: 1rem;
  }

  .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"])::before {
    content: "";
    position: absolute;
    inset-inline-start: 0.375rem;
    top: 50%;
    width: 0.25rem;
    height: 0.25rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 72%, transparent);
    transform: translateY(-50%);
  }

  .poodle-meta-bar[data-separators="true"] > :global(*:has(.poodle-pill)[data-separator="true"]) {
    padding-inline-start: 0;
  }

  .poodle-meta-bar[data-separators="true"] > :global(*:has(.poodle-pill)[data-separator="true"])::before {
    display: none;
  }

  .poodle-meta-bar > :global(.poodle-meta-item:has(.poodle-pill)) {
    --poodle-meta-item-gap: 0;
  }

  .poodle-meta-bar > :global(.poodle-meta-item:has(.poodle-pill) .poodle-meta-item__label) {
    display: none;
  }
  @media (max-width: 40rem) {
    .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"]) {
      padding-inline-start: 0.75rem;
    }

    .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"])::before {
      inset-inline-start: 0.25rem;
    }
  }
</style>

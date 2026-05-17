<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    ariaLabel?: string | null;
    children?: Snippet;
  }

  let {
    ariaLabel = null,
    children,
  }: Props = $props();
</script>

<span class="poodle-eyebrow" aria-label={ariaLabel ?? undefined}>
  {@render children?.()}
</span>

<style>
  .poodle-eyebrow {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    line-height: 1.5;
    text-transform: uppercase;
  }
</style>

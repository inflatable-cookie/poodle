<script lang="ts">
  import "@poodle/styles/meta-bar.css";
  import type { Snippet } from "svelte";
  import { setPillContext } from "./pill-context";

  interface Props {
    ariaLabel?: string | null;
    showSeparators?: boolean;
    children?: Snippet;
  }

  let {
    ariaLabel = null,
    showSeparators = true,
    children,
  }: Props = $props();

  setPillContext({
    size: "md",
    typography: "inherit",
  });
</script>

<div
  class="poodle-meta-bar"
  data-separators={showSeparators}
  aria-label={ariaLabel ?? undefined}
>
  {@render children?.()}
</div>


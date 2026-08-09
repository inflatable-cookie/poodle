<script lang="ts">
  import type { Snippet } from "svelte";
  import type { IconSet } from "./icon-registry";

  import { setIconSet } from "./icon-registry";

  // An icon set mapping kebab-case names to SVG node arrays.
  // Any icon set in this format works — lucide-static/icon-nodes.json,
  // a Phosphor equivalent, or a custom set.
  // String lookups resolve from this set first, then Poodle's scoped default
  // Lucide set.
  interface Props {
    icons: IconSet;
    children?: Snippet;
  }

  let {
    icons,
    children,
  }: Props = $props();

  const iconSet = setIconSet({});

  $effect(() => {
    iconSet.set(icons);
  });
</script>

{@render children?.()}

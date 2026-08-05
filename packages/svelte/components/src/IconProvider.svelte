<script lang="ts">
  import type { Snippet } from "svelte";
  import type { IconSet } from "./icon-registry.ts";

  import { setIconSet } from "./icon-registry.ts";

  // A complete icon set mapping kebab-case names to SVG node arrays.
  // Any icon set in this format works — lucide-static/icon-nodes.json,
  // a Phosphor equivalent, or a custom set.
  // String-based icon lookups resolve from this set first. If not found
  // and @poodle/icons-lucide is installed, icons are lazily auto-imported.
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

<script lang="ts">
  import { untrack, type Snippet } from "svelte";
  import type { IconSet } from "./icon-registry";

  import { setIconSet } from "./icon-registry";

  // A complete icon set mapping kebab-case names to SVG node arrays.
  // Any icon set in this format works — lucide-static/icon-nodes.json,
  // a Phosphor equivalent, or a custom set.
  // String-based icon lookups resolve from this set first. If not found
  // and @poodle/icons-lucide is installed, icons are lazily auto-imported.
  interface Props {
    icons: IconSet;
    children?: Snippet;
  }

  const {
    icons,
    children,
  }: Props = $props();

  setIconSet(untrack(() => icons));
</script>

{@render children?.()}

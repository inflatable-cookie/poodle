<script lang="ts">
  import type { IconSet } from "./icon-registry";

  import { setIconSet } from "./icon-registry";

  // A complete icon set mapping kebab-case names to SVG node arrays.
  // Any icon set in this format works — lucide-static/icon-nodes.json,
  // a Phosphor equivalent, or a custom set.
  // String-based icon lookups resolve from this set first. If not found
  // and @poodle/icons-lucide is installed, icons are lazily auto-imported.
  export let icons: IconSet;

  setIconSet(icons);
</script>

<slot />

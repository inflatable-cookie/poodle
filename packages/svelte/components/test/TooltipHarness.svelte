<script lang="ts">
  import Tooltip from "../src/Tooltip.svelte";

  let {
    open = null,
    content = "Save your changes",
    onOpenChange = undefined,
  }: {
    open?: boolean | null;
    content?: string;
    onOpenChange?: ((open: boolean) => void) | undefined;
  } = $props();
</script>

<!-- A real focusable trigger so the anchored bubble has an anchor and the
     aria-describedby relationship lands on an element. -->
<Tooltip {content} {open} {onOpenChange}>
  <button>Hover me</button>
</Tooltip>
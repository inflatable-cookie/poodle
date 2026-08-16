<script lang="ts">
  import ScrollShell from "../src/ScrollShell.svelte";

  interface Props {
    direction?: "vertical" | "horizontal" | "both";
    padding?: "none" | "sm" | "md";
    asRole?: "region" | "group" | null;
    label?: string | null;
    focusable?: boolean;
    onScroll?: ((event: Event) => void) | null;
  }

  let {
    direction = "vertical",
    padding = "none",
    asRole = null,
    label = null,
    focusable = false,
    onScroll = null,
  }: Props = $props();
</script>

<ScrollShell {direction} {padding} {asRole} {label} {focusable} {onScroll}>
  <span class="harness-child">Scrollable item</span>
</ScrollShell>

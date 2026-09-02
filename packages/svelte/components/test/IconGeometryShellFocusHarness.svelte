<script lang="ts">
  import IconGeometryShell from "./IconGeometryShell.svelte";

  let { progress = 0 }: { progress?: number } = $props();
</script>

<button type="button">Keep focus</button>
<IconGeometryShell pairId="chevron-left-to-chevron-right" target="to" {progress} />

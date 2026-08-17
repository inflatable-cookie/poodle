<script lang="ts">
  import StatusBar from "../src/StatusBar.svelte";
</script>

<StatusBar summary="Ready">
  {#snippet leading()}<span>main</span>{/snippet}
  {#snippet trailing()}<span>Ln 42, Col 18</span>{/snippet}
</StatusBar>
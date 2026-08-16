<script lang="ts">
  import Button from "../src/Button.svelte";
  import UiPresentationProvider from "../src/UiPresentationProvider.svelte";

  let { density = "default", sizeScale = "md" }: { density?: string; sizeScale?: string } = $props();
</script>

<UiPresentationProvider {density} {sizeScale}>
  <Button ariaLabel="Action">Action</Button>
</UiPresentationProvider>
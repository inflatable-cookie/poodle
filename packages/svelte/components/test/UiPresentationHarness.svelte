<script lang="ts">
  import type { ControlDensity, ControlSize } from "../src/types";

  import Button from "../src/Button.svelte";
  import UiPresentationProvider from "../src/UiPresentationProvider.svelte";

  let { density = "default", sizeScale = "md" }: { density?: ControlDensity; sizeScale?: ControlSize } =
    $props();
</script>

<UiPresentationProvider {density} {sizeScale}>
  <Button ariaLabel="Action">Action</Button>
</UiPresentationProvider>
<script lang="ts">
  import type { ControlSize } from "../src/types";

  import CodeEditor from "../src/CodeEditor.svelte";
  import UiPresentationProvider from "../src/UiPresentationProvider.svelte";

  let { sizeScale = "md" }: { sizeScale?: ControlSize } = $props();
</script>

<UiPresentationProvider {sizeScale}>
  <CodeEditor value="const answer = 42;" />
</UiPresentationProvider>

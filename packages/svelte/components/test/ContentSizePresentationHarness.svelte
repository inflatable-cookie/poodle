<script lang="ts">
  import type { ProseMirrorDocumentJSON } from "@inflatable-cookie/poodle-core";
  import type { ControlSize } from "../src/types";

  import MarkdownRenderer from "../src/MarkdownRenderer.svelte";
  import RichTextEditor from "../src/RichTextEditor.svelte";
  import RichTextRenderer from "../src/RichTextRenderer.svelte";
  import UiPresentationProvider from "../src/UiPresentationProvider.svelte";

  let { sizeScale = "md" }: { sizeScale?: ControlSize } = $props();
  const document: ProseMirrorDocumentJSON = {
    type: "doc",
    content: [{ type: "paragraph", content: [{ type: "text", text: "Scaled content" }] }],
  };
</script>

<UiPresentationProvider {sizeScale}>
  <RichTextEditor value={document} />
  <RichTextRenderer value={document} />
  <MarkdownRenderer value="# Scaled content" />
</UiPresentationProvider>

<script lang="ts">
  import RichTextEditor from "../../src/RichTextEditor.svelte";
  import RichTextRenderer from "../../src/RichTextRenderer.svelte";
  import type { ProseMirrorDocumentJSON } from "@inflatable-cookie/poodle-core";

  interface Props {
    value?: ProseMirrorDocumentJSON;
  }

  let { value = {
    type: "doc",
    content: [{ type: "paragraph", content: [{ type: "text", text: "hello world" }] }],
  } }: Props = $props();
</script>

<RichTextEditor {value} features={["headings", "formatting"]} />
<RichTextRenderer {value} />

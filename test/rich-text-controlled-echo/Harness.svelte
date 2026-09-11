<script lang="ts">
  import RichTextEditor from "../../packages/svelte/components/src/RichTextEditor.svelte";
  import type { ProseMirrorDocumentJSON } from "../../packages/core/src/index.ts";

  const INITIAL: ProseMirrorDocumentJSON = {
    type: "doc",
    content: [
      { type: "paragraph", content: [{ type: "text", text: "alpha beta" }] },
      { type: "paragraph", content: [{ type: "text", text: "second block" }] },
    ],
  };

  let document = $state<ProseMirrorDocumentJSON>(INITIAL);
</script>

<section data-framework="svelte">
  <button type="button" data-before>before svelte</button>
  <div class="editor-frame" data-part="controlled-editor">
    <RichTextEditor
      value={document}
      features={["formatting"]}
      ariaLabel="Svelte controlled editor"
      onChange={(next) => (document = next)}
    />
  </div>
  <pre class="readout" data-part="host-document">{JSON.stringify(document)}</pre>
</section>

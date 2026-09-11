<script lang="ts">
  import RichTextEditor from "../../packages/svelte/components/src/RichTextEditor.svelte";
  import type {
    ProseMirrorDocumentJSON,
    RichTextCommand,
  } from "../../packages/core/src/index.ts";

  const INITIAL: ProseMirrorDocumentJSON = {
    type: "doc",
    content: [
      { type: "heading", attrs: { level: 2 }, content: [{ type: "text", text: "Alpha heading" }] },
      { type: "paragraph", content: [{ type: "text", text: "Body copy" }] },
      { type: "heading", attrs: { level: 4 }, content: [{ type: "text", text: "Deep heading" }] },
    ],
  };
  const SPARSE: readonly RichTextCommand[] = ["heading-2", "heading-4"];

  // The host echoes every change straight back, exactly as the public
  // specimen does: the accepted echo must never move the caret or re-emit.
  let document = $state<ProseMirrorDocumentJSON>(INITIAL);
  let sparseDocument = $state<ProseMirrorDocumentJSON>(INITIAL);
  let changes = $state(0);

  function handleChange(next: ProseMirrorDocumentJSON): void {
    changes += 1;
    document = next;
  }
</script>

<section data-framework="svelte">
  <div class="editor-frame" data-part="heading-editor">
    <RichTextEditor
      value={document}
      features={["headings"]}
      ariaLabel="Svelte heading modes"
      onChange={handleChange}
    />
  </div>
  <div class="editor-frame editor-frame--constrained" data-part="sparse-editor">
    <RichTextEditor
      value={sparseDocument}
      features={["headings"]}
      toolbar={SPARSE}
      ariaLabel="Svelte sparse heading modes"
      onChange={(next) => (sparseDocument = next)}
    />
  </div>
  <p data-part="change-count" data-count={changes}>{changes}</p>
  <pre class="readout" data-part="host-document">{JSON.stringify(document)}</pre>
</section>

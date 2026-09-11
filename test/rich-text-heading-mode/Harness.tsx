import { useState } from "react";
import { RichTextEditor } from "../../packages/react/components/src/RichTextEditor";
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

export function Harness() {
  // The host echoes every change straight back, exactly as the public
  // specimen does: the accepted echo must never move the caret or re-emit.
  const [document, setDocument] = useState<ProseMirrorDocumentJSON>(INITIAL);
  const [sparseDocument, setSparseDocument] = useState<ProseMirrorDocumentJSON>(INITIAL);
  const [changes, setChanges] = useState(0);

  return (
    <section data-framework="react">
      <div className="editor-frame" data-part="heading-editor">
        <RichTextEditor
          value={document}
          features={["headings"]}
          ariaLabel="React heading modes"
          onChange={(next) => {
            setChanges((count) => count + 1);
            setDocument(next);
          }}
        />
      </div>
      <div className="editor-frame editor-frame--constrained" data-part="sparse-editor">
        <RichTextEditor
          value={sparseDocument}
          features={["headings"]}
          toolbar={SPARSE}
          ariaLabel="React sparse heading modes"
          onChange={setSparseDocument}
        />
      </div>
      <p data-part="change-count" data-count={changes}>
        {changes}
      </p>
      <pre className="readout" data-part="host-document">{JSON.stringify(document)}</pre>
    </section>
  );
}

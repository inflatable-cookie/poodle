import { useState } from "react";
import { RichTextEditor } from "../../packages/react/components/src/RichTextEditor";
import type { ProseMirrorDocumentJSON } from "../../packages/core/src/index.ts";

const INITIAL: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [
    { type: "paragraph", content: [{ type: "text", text: "alpha beta" }] },
    { type: "paragraph", content: [{ type: "text", text: "second block" }] },
  ],
};

export function Harness() {
  const [document, setDocument] = useState<ProseMirrorDocumentJSON>(INITIAL);
  return (
    <section data-framework="react">
      <button type="button" data-before>
        before react
      </button>
      <div className="editor-frame" data-part="controlled-editor">
        <RichTextEditor
          value={document}
          features={["formatting"]}
          ariaLabel="React controlled editor"
          onChange={setDocument}
        />
      </div>
      <pre className="readout" data-part="host-document">
        {JSON.stringify(document)}
      </pre>
    </section>
  );
}

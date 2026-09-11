import { useState } from "react";
import { CodeEditor } from "../../packages/react/components/src/CodeEditor";
import type { CodeEditorDiagnostic } from "../../packages/core/src/index.ts";

const VALUE = "const answer = 42;\nconst unused = 0;\n";
const DIAGNOSTICS: CodeEditorDiagnostic[] = [
  { id: "d1", severity: "warning", message: "unused binding", range: { from: 25, to: 31 } },
];

export function Harness() {
  const [lineNumbers, setLineNumbers] = useState(true);
  return (
    <section data-framework="react">
      <button type="button" data-before>
        before react
      </button>
      <div className="controls">
        <button
          type="button"
          data-part="line-numbers-toggle"
          aria-pressed={lineNumbers}
          onClick={() => setLineNumbers(!lineNumbers)}
        >
          {lineNumbers ? "Line numbers on" : "Line numbers off"}
        </button>
      </div>
      <div className="editor-frame" data-part="config-editor">
        <CodeEditor
          value={VALUE}
          lineNumbers={lineNumbers}
          diagnostics={DIAGNOSTICS}
          ariaLabel="React configured editor"
        />
      </div>
    </section>
  );
}

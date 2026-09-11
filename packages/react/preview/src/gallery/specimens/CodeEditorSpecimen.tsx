import { useState } from "react";
import type { ControlDensity } from "@inflatable-cookie/poodle-react";
import { CodeEditor, type CodeEditorLanguageId } from "@inflatable-cookie/poodle-react/editor";
import { createCodeEditorLanguageRegistry } from "@inflatable-cookie/poodle-react/editor/codemirror";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
import {
  CODE_DIAGNOSTIC_SOURCE,
  CODE_DIAGNOSTICS,
  CODE_JSON_SOURCE,
  CODE_TYPESCRIPT_SOURCE,
} from "../../../../../svelte/preview/src/specimens/web-editor-documents";

// Consumer-owned language registry (g18.012): this preview app installs
// exactly the grammar packages it names here. Poodle carries no grammar
// dependencies; ids are plain serializable strings, and "plain-text" needs
// no registry entry at all.
const languageRegistry = createCodeEditorLanguageRegistry({
  typescript: async () =>
    (await import("@codemirror/lang-javascript")).javascript({ typescript: true }),
  json: async () => (await import("@codemirror/lang-json")).json(),
});

const axisSource = "export const ready = true;\n";

export function CodeEditorSpecimen() {
  const [value, setValue] = useState(CODE_TYPESCRIPT_SOURCE);
  const [language, setLanguage] = useState<CodeEditorLanguageId>("typescript");
  const [lineNumbers, setLineNumbers] = useState(true);

  return (
    <SpecimenLayout
      densities={(density) => (
        <SpecimenGroup label={density}>
          <div className="code-editor-frame">
            <CodeEditor
              value={axisSource}
              language="typescript"
              languageRegistry={languageRegistry}
              density={density as ControlDensity}
              ariaLabel="Density sample"
            />
          </div>
        </SpecimenGroup>
      )}
    >
      <SpecimenGroup
        label="TypeScript editing"
        description="Host-controlled value. Typing updates the readout below."
      >
        <div className="code-editor-frame" data-part="live-editor">
          <CodeEditor
            value={value}
            language="typescript"
            languageRegistry={languageRegistry}
            ariaLabel="TypeScript editor"
            onChange={(change) => setValue(change.value)}
          />
        </div>
        <pre className="code-editor-readout" data-part="host-value">
          {value}
        </pre>
      </SpecimenGroup>

      <SpecimenGroup label="Diagnostics" description="Host-authored messages, not a language service.">
        <div className="code-editor-frame" data-part="diagnostics-editor">
          <CodeEditor
            value={CODE_DIAGNOSTIC_SOURCE}
            language="typescript"
            languageRegistry={languageRegistry}
            diagnostics={CODE_DIAGNOSTICS}
            ariaLabel="Diagnostics editor"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Read-only">
        <div className="code-editor-frame" data-part="read-only-editor">
          <CodeEditor
            value={CODE_TYPESCRIPT_SOURCE}
            language="typescript"
            languageRegistry={languageRegistry}
            readOnly
            ariaLabel="Read-only TypeScript"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Configuration">
        <div className="code-editor-controls">
          <button
            type="button"
            data-part="language-plain-text"
            aria-pressed={language === "plain-text"}
            onClick={() => setLanguage("plain-text")}
          >
            Plain text
          </button>
          <button
            type="button"
            data-part="language-typescript"
            aria-pressed={language === "typescript"}
            onClick={() => setLanguage("typescript")}
          >
            TypeScript
          </button>
          <button
            type="button"
            data-part="language-json"
            aria-pressed={language === "json"}
            onClick={() => setLanguage("json")}
          >
            JSON
          </button>
          <button
            type="button"
            data-part="line-numbers-toggle"
            aria-pressed={lineNumbers}
            onClick={() => setLineNumbers(!lineNumbers)}
          >
            {lineNumbers ? "Line numbers on" : "Line numbers off"}
          </button>
        </div>
        <div className="code-editor-frame" data-part="config-editor">
          <CodeEditor
            value={language === "json" ? CODE_JSON_SOURCE : CODE_TYPESCRIPT_SOURCE}
            language={language}
            languageRegistry={languageRegistry}
            lineNumbers={lineNumbers}
            ariaLabel="Configured editor"
          />
        </div>
      </SpecimenGroup>
      <style>{`
        .code-editor-frame { height: 16rem; }
        .code-editor-controls { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.75rem; }
        .code-editor-readout { margin: 0.75rem 0 0; padding: 0.5rem; border-radius: 0.25rem; background: var(--poodle-color-background-surface); font-size: 0.75rem; white-space: pre-wrap; max-height: 8rem; overflow: auto; }
      `}</style>
    </SpecimenLayout>
  );
}

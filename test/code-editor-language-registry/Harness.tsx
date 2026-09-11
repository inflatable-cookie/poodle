import { Component, useState, type ReactNode } from "react";
import { CodeEditor } from "../../packages/react/components/src/CodeEditor";
import { createCodeEditorLanguageRegistry } from "../../packages/react/components/src/editor-codemirror";

const VALUE = "const answer = 42;\n";

// g18.021: the TypeScript sample exercises comment, keyword, definition,
// number, and string token groups; the JSON sample exercises strings,
// numbers, and booleans with ordinary (unstyled) property names.
const TYPESCRIPT_VALUE = `// ledger
const answer = 42;
const label = "hello";
`;
const JSON_VALUE = `{\n  "answer": 42,\n  "label": "hello",\n  "live": true\n}\n`;
// g18.021: the malformed sample plants parser error nodes so the probe can
// prove invalid syntax receives the danger treatment.
const INVALID_VALUE = `// ledger
const answer = 42;
### oops ###
const broken = ;
`;

/** g18.012: the fixture consumer owns its language set and its loader counts. */
const counters = { typescript: 0, json: 0, broken: 0 };
const exposed = globalThis as Record<string, unknown>;
exposed.__registryCounters = exposed.__registryCounters ?? {};
(exposed.__registryCounters as Record<string, unknown>).react = counters;

const languageRegistry = createCodeEditorLanguageRegistry({
  typescript: async () => {
    counters.typescript += 1;
    const { javascript } = await import("@codemirror/lang-javascript");
    return javascript({ typescript: true });
  },
  json: async () => {
    counters.json += 1;
    const { json } = await import("@codemirror/lang-json");
    return json();
  },
  broken: async () => {
    counters.broken += 1;
    throw new Error("grammar exploded");
  },
});

/**
 * A mounted unknown-id refusal throws during render; an error boundary is the
 * real-browser way to catch and display it without crashing the harness.
 */
class MountRefusalBoundary extends Component<
  { children: ReactNode; onError: (message: string) => void },
  { message: string | null }
> {
  state = { message: null as string | null };

  static getDerivedStateFromError(error: unknown) {
    return { message: error instanceof Error ? error.message : String(error) };
  }

  componentDidCatch(error: unknown) {
    this.props.onError(error instanceof Error ? error.message : String(error));
  }

  render() {
    if (this.state.message !== null) return null;
    return this.props.children;
  }
}

export function Harness() {
  const [language, setLanguage] = useState("typescript");
  // Controlled value tracking: the host owns the exact text, so edits must
  // flow back through onChange or every prop sync would revert the document.
  const [value, setValue] = useState(TYPESCRIPT_VALUE);
  const [performanceMode, setPerformanceMode] = useState("full");
  const [refusalAttempt, setRefusalAttempt] = useState(false);
  const [refusalMessage, setRefusalMessage] = useState("");

  return (
    <section data-framework="react">
      <button type="button" data-before>
        before react
      </button>
      <div className="controls">
        <button type="button" data-part="language-typescript" onClick={() => setLanguage("typescript")}>
          TypeScript
        </button>
        <button type="button" data-part="language-json" onClick={() => setLanguage("json")}>
          JSON
        </button>
        <button type="button" data-part="language-plain-text" onClick={() => setLanguage("plain-text")}>
          Plain text
        </button>
        <button type="button" data-part="language-broken" onClick={() => setLanguage("broken")}>
          Broken
        </button>
        <button type="button" data-part="mode-full" onClick={() => setPerformanceMode("full")}>
          Full mode
        </button>
        <button type="button" data-part="mode-plain" onClick={() => setPerformanceMode("plain")}>
          Plain mode
        </button>
        <button
          type="button"
          data-part="sample-typescript"
          onClick={() => {
            setLanguage("typescript");
            setValue(TYPESCRIPT_VALUE);
          }}
        >
          TypeScript sample
        </button>
        <button
          type="button"
          data-part="sample-json"
          onClick={() => {
            setLanguage("json");
            setValue(JSON_VALUE);
          }}
        >
          JSON sample
        </button>
        <button
          type="button"
          data-part="sample-invalid"
          onClick={() => {
            setLanguage("typescript");
            setValue(INVALID_VALUE);
          }}
        >
          Malformed sample
        </button>
        <button
          type="button"
          data-part="mount-invalid"
          onClick={() => {
            setRefusalMessage("");
            setRefusalAttempt(true);
          }}
        >
          Mount invalid
        </button>
      </div>
      <div className="editor-frame" data-part="main-editor">
        <CodeEditor
          value={value}
          language={language}
          languageRegistry={languageRegistry}
          performanceMode={performanceMode}
          ariaLabel="React registry editor"
          onChange={(change) => setValue(change.value)}
        />
      </div>
      <MountRefusalBoundary onError={setRefusalMessage}>
        {refusalAttempt ? (
          <div className="editor-frame" data-part="refusal-editor">
            <CodeEditor
              value={VALUE}
              language="cobol"
              languageRegistry={languageRegistry}
              ariaLabel="React invalid language"
            />
          </div>
        ) : null}
      </MountRefusalBoundary>
      <p data-part="mount-refusal">{refusalMessage}</p>
    </section>
  );
}

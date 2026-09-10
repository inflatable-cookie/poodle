import { forwardRef, useEffect, useImperativeHandle, useRef, useState } from "react";
import {
  installInputModality,
  isCodeEditorValueAdmissible,
} from "@inflatable-cookie/poodle-core";
import type {
  CodeEditorChange,
  CodeEditorDiagnostic,
  CodeEditorLanguage,
  CodeEditorPerformanceMode,
  CodeEditorTabBehavior,
} from "@inflatable-cookie/poodle-core";
import type { ControlDensity } from "./types";

import "@inflatable-cookie/poodle-core/styles/code-editor.css";

import {
  assertAdmittedLanguage,
  createCodeEditorEngine,
} from "./code-editor-engine";
import type { CodeEditorActiveDiagnostic, CodeEditorEngine } from "./code-editor-engine";
import { useUiPresentation } from "./presentation";

export interface CodeEditorProps {
  value: string;
  language?: CodeEditorLanguage;
  lineNumbers?: boolean;
  searchable?: boolean;
  diagnostics?: CodeEditorDiagnostic[];
  readOnly?: boolean;
  disabled?: boolean;
  placeholder?: string;
  ariaLabel?: string;
  wrapLines?: boolean;
  tabSize?: number;
  tabBehavior?: CodeEditorTabBehavior;
  performanceMode?: CodeEditorPerformanceMode;
  density?: ControlDensity | null;
  onChange?: ((change: CodeEditorChange) => void) | null;
}

export interface CodeEditorHandle {
  focus: () => void;
}

/**
 * Web-admitted controlled code and plain-text editing surface over
 * CodeMirror 6. The engine stays private: no CodeMirror type crosses this
 * API, and the component is reached only through `./editor`.
 */
export const CodeEditor = forwardRef<CodeEditorHandle, CodeEditorProps>(function CodeEditor(
  {
    value,
    language = "plain-text",
    lineNumbers = true,
    searchable = true,
    diagnostics = [],
    readOnly = false,
    disabled = false,
    placeholder = "",
    ariaLabel = "Code editor",
    wrapLines = false,
    tabSize = 2,
    tabBehavior = "focus",
    performanceMode = "full",
    density = null,
    onChange = null,
  }: CodeEditorProps,
  ref,
) {
  assertAdmittedLanguage(language);

  const uiPresentation = useUiPresentation();
  const resolvedDensity = density ?? uiPresentation.density;
  const hostRef = useRef<HTMLDivElement | null>(null);
  const engineRef = useRef<CodeEditorEngine | null>(null);
  const onChangeRef = useRef(onChange);
  onChangeRef.current = onChange;
  const [activeDiagnostic, setActiveDiagnostic] = useState<CodeEditorActiveDiagnostic | null>(null);
  const [messageId] = useState(() => `poodle-code-editor-${Math.random().toString(36).slice(2)}`);

  useImperativeHandle(ref, () => ({
    focus: () => {
      (hostRef.current?.querySelector(".cm-content") as HTMLElement | null)?.focus();
    },
  }));

  useEffect(() => {
    installInputModality();
    const host = hostRef.current;
    if (!host) return;
    let cancelled = false;
    createCodeEditorEngine(
      host,
      {
        value,
        language,
        lineNumbers,
        searchable,
        readOnly,
        disabled,
        placeholder,
        ariaLabel,
        wrapLines,
        tabSize,
        tabBehavior,
        performanceMode,
        diagnostics,
      },
      {
        onChange: (change) => onChangeRef.current?.(change),
        onActiveDiagnostic: setActiveDiagnostic,
      },
    ).then((created) => {
      if (cancelled) {
        created.destroy();
        return;
      }
      engineRef.current = created;
    });
    return () => {
      cancelled = true;
    };
    // Mount-only: prop updates flow through the sync effect below.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    void engineRef.current?.update({
      value,
      language,
      lineNumbers,
      searchable,
      readOnly,
      disabled,
      placeholder,
      ariaLabel,
      wrapLines,
      tabSize,
      tabBehavior,
      performanceMode,
      diagnostics,
    });
  });

  return (
    <div
      className="poodle-code-editor"
      data-density={resolvedDensity}
      data-disabled={disabled || undefined}
      data-readonly={readOnly || undefined}
    >
      <div
        ref={hostRef}
        className="poodle-code-editor__viewport"
        aria-describedby={activeDiagnostic ? messageId : undefined}
      />
      {activeDiagnostic ? (
        <div
          id={messageId}
          className="poodle-code-editor__diagnostic-message"
          data-severity={activeDiagnostic.diagnostic.severity}
          role="status"
        >
          {`${activeDiagnostic.diagnostic.severity} ${activeDiagnostic.line}:${activeDiagnostic.column} — ${activeDiagnostic.diagnostic.message}`}
        </div>
      ) : null}
    </div>
  );
});

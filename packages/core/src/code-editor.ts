/**
 * CodeEditor shared TypeScript core (engine-independent).
 * Contract: docs/contracts/components/code-editor.md
 *
 * Pure value/edit/diagnostic math shared by the Svelte and React CodeMirror 6
 * wrappers. No CodeMirror import may appear here: this module ships in the
 * root barrel and must stay free of the editor engine. Engine assembly lives
 * adapter-side (`CodeEditor.svelte` / `CodeEditor.tsx`, both reached only
 * through the dedicated `./editor` entries).
 */

export type CodeEditorLanguage =
  | "plain-text"
  | "markdown"
  | "json"
  | "yaml"
  | "toml"
  | "javascript"
  | "typescript"
  | "html"
  | "css"
  | "rust"
  | "shell";

export const CODE_EDITOR_LANGUAGES: readonly CodeEditorLanguage[] = [
  "plain-text",
  "markdown",
  "json",
  "yaml",
  "toml",
  "javascript",
  "typescript",
  "html",
  "css",
  "rust",
  "shell",
] as const;

/** Closed admitted set. Unknown strings never silently fall back. */
export function isCodeEditorLanguage(value: string): value is CodeEditorLanguage {
  return (CODE_EDITOR_LANGUAGES as readonly string[]).includes(value);
}

export interface CodeEditorRange {
  from: number;
  to: number;
}

export interface CodeEditorDiagnostic {
  id: string;
  severity: "error" | "warning" | "info";
  message: string;
  range: CodeEditorRange;
  source?: string | null;
  code?: string | null;
}

export interface CodeEditorTextEdit {
  range: CodeEditorRange;
  insert: string;
}

export interface CodeEditorChange {
  value: string;
  edits: CodeEditorTextEdit[];
}

export type CodeEditorTabBehavior = "focus" | "indent";
export type CodeEditorPerformanceMode = "full" | "plain";

/**
 * Supported document envelope: valid UTF-8 through 2 MiB inclusive. Hosts
 * refuse larger sources before mounting; Poodle never loads files.
 */
export const CODE_EDITOR_MAX_BYTES = 2 * 1024 * 1024;

export function codeEditorByteLength(value: string): number {
  return new TextEncoder().encode(value).length;
}

export function isCodeEditorValueAdmissible(value: string): boolean {
  return codeEditorByteLength(value) <= CODE_EDITOR_MAX_BYTES;
}

function isValidRange(range: CodeEditorRange, docLength: number): boolean {
  return (
    Number.isInteger(range.from) &&
    Number.isInteger(range.to) &&
    range.from >= 0 &&
    range.to >= range.from &&
    range.to <= docLength
  );
}

/**
 * Translate one committed engine transaction into the public change payload.
 * Raw edits carry offsets into `previous`; the result sorts them, verifies
 * they are non-overlapping, and derives the exact complete next value by
 * replaying them against `previous`. Throws on overlap so a corrupt engine
 * translation can never emit a value that disagrees with its edits.
 */
export function toCodeEditorChange(
  previous: string,
  rawEdits: readonly CodeEditorTextEdit[],
): CodeEditorChange {
  const edits = [...rawEdits].sort((a, b) => a.range.from - b.range.from || a.range.to - b.range.to);
  for (const edit of edits) {
    if (!isValidRange(edit.range, previous.length)) {
      throw new Error("code-editor: edit range out of bounds for the previous value");
    }
  }
  for (let i = 1; i < edits.length; i += 1) {
    const prior = edits[i - 1];
    const current = edits[i];
    if (prior && current && current.range.from < prior.range.to) {
      throw new Error("code-editor: overlapping edits in one transaction");
    }
  }
  let value = "";
  let cursor = 0;
  for (const edit of edits) {
    value += previous.slice(cursor, edit.range.from) + edit.insert;
    cursor = edit.range.to;
  }
  value += previous.slice(cursor);
  return { value, edits };
}

/** Replay check for the oracle: applying `edits` to `previous` must equal `value`. */
export function applyCodeEditorEdits(previous: string, edits: readonly CodeEditorTextEdit[]): string {
  let value = "";
  let cursor = 0;
  for (const edit of edits) {
    value += previous.slice(cursor, edit.range.from) + edit.insert;
    cursor = edit.range.to;
  }
  return value + previous.slice(cursor);
}

export interface ValidatedDiagnostics {
  valid: CodeEditorDiagnostic[];
  /** Ids omitted as invalid (bad range) or duplicate. Never clamped. */
  refused: string[];
}

/**
 * Host diagnostics stay host-owned: invalid or out-of-bounds ranges are
 * omitted (never clamped onto different text) and duplicate ids are invalid.
 */
export function validateCodeEditorDiagnostics(
  diagnostics: readonly CodeEditorDiagnostic[],
  docLength: number,
): ValidatedDiagnostics {
  const seen = new Set<string>();
  const valid: CodeEditorDiagnostic[] = [];
  const refused: string[] = [];
  for (const diagnostic of diagnostics) {
    if (
      typeof diagnostic.id !== "string" ||
      diagnostic.id === "" ||
      seen.has(diagnostic.id) ||
      !isValidRange(diagnostic.range, docLength)
    ) {
      refused.push(typeof diagnostic.id === "string" ? diagnostic.id : "");
      continue;
    }
    seen.add(diagnostic.id);
    valid.push(diagnostic);
  }
  return { valid, refused };
}

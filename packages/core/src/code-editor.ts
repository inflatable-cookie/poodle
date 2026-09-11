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

/**
 * Serializable, consumer-defined language id. Poodle fixes no vocabulary:
 * `plain-text` is built in, and every other id means whatever the host's
 * language registry says it means. Unknown ids fail closed; they never
 * silently fall back to plain text.
 */
export type CodeEditorLanguageId = string;

/** The one built-in language. It never needs a registry or grammar package. */
export const CODE_EDITOR_PLAIN_TEXT: CodeEditorLanguageId = "plain-text";

/**
 * Lazy language loader. Opaque to the engines: the value is produced by a
 * substrate adapter (CodeMirror), which owns the loader's return type and the
 * runtime guarantee that a resolved value really is a language extension.
 */
export type CodeEditorLanguageLoader = () => Promise<unknown>;

/**
 * The opaque registry both web engines resolve the selected `language` id
 * through. Constructed through an explicit substrate adapter subpath
 * (`./editor/codemirror`); the component prop accepts only this shape, never
 * arbitrary editor extensions, themes, or keymaps.
 */
export interface CodeEditorLanguageRegistry {
  /** True when the registry admits the id. Synchronous, fail-closed gate. */
  has(language: CodeEditorLanguageId): boolean;
  /**
   * Load the language for an admitted id. Each id loads exactly once per
   * registry instance: the first load's promise (settled or rejected) is
   * memoized, so switching back never re-invokes the loader. Unknown ids
   * reject; they never fall back.
   */
  load(language: CodeEditorLanguageId): Promise<unknown>;
}

/** Registry construction input: id/loader pairs or a plain record. */
export type CodeEditorLanguageRegistryInput =
  | Iterable<readonly [CodeEditorLanguageId, CodeEditorLanguageLoader]>
  | Record<string, CodeEditorLanguageLoader>;

/**
 * Shared registry construction and refusal semantics for every framework
 * adapter. Fail closed: empty or non-string ids, non-function loaders,
 * duplicate ids, and a `plain-text` entry (built in; registries admit syntax
 * languages only) throw before any registry exists.
 */
export function createCodeEditorLanguageRegistry(
  input: CodeEditorLanguageRegistryInput,
): CodeEditorLanguageRegistry {
  const loaders = new Map<CodeEditorLanguageId, CodeEditorLanguageLoader>();
  const pairs: Iterable<readonly [CodeEditorLanguageId, CodeEditorLanguageLoader]> =
    Symbol.iterator in input ? input : Object.entries(input);
  for (const [language, loader] of pairs) {
    if (typeof language !== "string" || language === "") {
      throw new Error("code-editor: language ids must be non-empty strings.");
    }
    if (language === CODE_EDITOR_PLAIN_TEXT) {
      throw new Error(
        `code-editor: "${CODE_EDITOR_PLAIN_TEXT}" is built in; registries admit syntax languages only.`,
      );
    }
    if (typeof loader !== "function") {
      throw new Error(`code-editor: language "${language}" needs a lazy loader function.`);
    }
    if (loaders.has(language)) {
      throw new Error(`code-editor: duplicate language "${language}" in the registry.`);
    }
    loaders.set(language, loader);
  }
  const loaded = new Map<CodeEditorLanguageId, Promise<unknown>>();
  return {
    has(language) {
      return loaders.has(language);
    },
    load(language) {
      const memoized = loaded.get(language);
      if (memoized) return memoized;
      const loader = loaders.get(language);
      if (!loader) {
        return Promise.reject(
          new Error(
            `code-editor: unsupported language "${String(language)}". Register it through a language registry or request "${CODE_EDITOR_PLAIN_TEXT}" explicitly.`,
          ),
        );
      }
      const pending = loader();
      loaded.set(language, pending);
      return pending;
    },
  };
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

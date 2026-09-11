/**
 * CodeMirror 6 engine assembly for the React CodeEditor.
 * Reached only through the dedicated `./editor` entry; never the root barrel.
 *
 * No top-level DOM access: constructing the EditorView happens exclusively in
 * `createCodeEditorEngine`, which wrappers call after mount. Importing this
 * module (including on the server) loads the engine code but never touches
 * the document or spawns workers.
 */

import { closeSearchPanel, searchKeymap } from "@codemirror/search";
import {
  Compartment,
  EditorState,
  Prec,
  RangeSetBuilder,
  StateField,
  Transaction,
} from "@codemirror/state";
import type { ChangeSet, Extension, StateEffect, TransactionSpec } from "@codemirror/state";
import {
  Decoration,
  EditorView,
  keymap,
  lineNumbers,
  placeholder,
} from "@codemirror/view";
import type { DecorationSet, KeyBinding, ViewUpdate } from "@codemirror/view";
import {
  defaultKeymap,
  history,
  historyKeymap,
  indentMore,
} from "@codemirror/commands";
import {
  installCodeEditorFocusEntry,
  isCodeEditorLanguage,
  toCodeEditorChange,
  validateCodeEditorDiagnostics,
} from "@inflatable-cookie/poodle-core";
import type {
  CodeEditorChange,
  CodeEditorDiagnostic,
  CodeEditorLanguage,
  CodeEditorPerformanceMode,
  CodeEditorTabBehavior,
  CodeEditorTextEdit,
} from "@inflatable-cookie/poodle-core";

import { languageFor } from "./code-editor-languages";

export interface CodeEditorEngineOptions {
  value: string;
  language: CodeEditorLanguage;
  lineNumbers: boolean;
  searchable: boolean;
  readOnly: boolean;
  disabled: boolean;
  placeholder: string;
  ariaLabel: string;
  wrapLines: boolean;
  tabSize: number;
  tabBehavior: CodeEditorTabBehavior;
  performanceMode: CodeEditorPerformanceMode;
  diagnostics: readonly CodeEditorDiagnostic[];
}

export interface CodeEditorActiveDiagnostic {
  diagnostic: CodeEditorDiagnostic;
  line: number;
  column: number;
}

export interface CodeEditorEngineCallbacks {
  onChange: (change: CodeEditorChange) => void;
  onActiveDiagnostic: (active: CodeEditorActiveDiagnostic | null) => void;
}

export interface CodeEditorEngine {
  update: (next: Partial<CodeEditorEngineOptions>) => Promise<void>;
  focusDiagnostic: (direction: 1 | -1) => void;
  destroy: () => void;
}

/** Fail closed: unknown language strings never silently fall back. */
export function assertAdmittedLanguage(language: string): asserts language is CodeEditorLanguage {
  if (!isCodeEditorLanguage(language)) {
    throw new Error(
      `code-editor: unsupported language "${language}". Request "plain-text" explicitly or use an admitted language.`,
    );
  }
}

function normalizeTabSize(tabSize: number): number {
  return Number.isInteger(tabSize) && tabSize > 0 ? tabSize : 2;
}

/**
 * The engine owns line-break representation: CR and CRLF load as LF. The
 * comparison honors that without changing any text the host can observe.
 */
function diagnosticSignature(diagnostics: readonly CodeEditorDiagnostic[]): string {
  return JSON.stringify(diagnostics);
}

function sameDocumentText(propValue: string, docText: string): boolean {
  return propValue === docText || propValue.replace(/\r\n?/g, "\n") === docText;
}

/**
 * Translate one committed engine transaction into the public change payload.
 * Offsets bind the document the transaction started from. Exported for the
 * focused transaction fixtures; not part of the public package surface.
 */
export function transactionToChange(previous: string, changes: ChangeSet): CodeEditorChange {
  const edits: CodeEditorTextEdit[] = [];
  changes.iterChanges((fromA, toA, _fromB, _toB, inserted) => {
    edits.push({ range: { from: fromA, to: toA }, insert: inserted.toString() });
  });
  return toCodeEditorChange(previous, edits);
}

export async function createCodeEditorEngine(
  host: HTMLElement,
  initial: CodeEditorEngineOptions,
  callbacks: CodeEditorEngineCallbacks,
): Promise<CodeEditorEngine> {
  if (typeof initial.value !== "string") {
    throw new Error("code-editor: value must be a string; the component has no uncontrolled mode.");
  }
  assertAdmittedLanguage(initial.language);

  let options: CodeEditorEngineOptions = { ...initial };
  let applyingHostValue = false;
  let exitArmed = false;
  let activeDiagnosticIndex: number | null = null;

  const languageCompartment = new Compartment();
  const behaviorCompartment = new Compartment();
  const readOnlyCompartment = new Compartment();
  const diagnosticsCompartment = new Compartment();
  const wrapCompartment = new Compartment();
  const tabSizeCompartment = new Compartment();

  function buildDiagnosticDecorations(valid: CodeEditorDiagnostic[]): DecorationSet {
    const builder = new RangeSetBuilder<Decoration>();
    const ordered = [...valid].sort(
      (a, b) => a.range.from - b.range.from || a.range.to - b.range.to,
    );
    for (const diagnostic of ordered) {
      builder.add(
        diagnostic.range.from,
        diagnostic.range.to,
        Decoration.mark({
          class: `poodle-code-editor__diagnostic--${diagnostic.severity}`,
          attributes: { "data-poodle-diagnostic": diagnostic.id },
        }),
      );
    }
    return builder.finish();
  }

  function reportDiagnostics(docLength: number): CodeEditorDiagnostic[] {
    const { valid, refused } = validateCodeEditorDiagnostics(options.diagnostics, docLength);
    if (refused.length > 0) {
      console.warn(
        `code-editor: omitted ${refused.length} diagnostic(s) with invalid ranges or duplicate ids`,
        refused,
      );
    }
    return valid;
  }

  function diagnosticField(valid: CodeEditorDiagnostic[]) {
    return StateField.define<DecorationSet>({
      create: () => buildDiagnosticDecorations(valid),
      update: (decorations, transaction) =>
        transaction.docChanged ? buildDiagnosticDecorations(valid) : decorations.map(transaction.changes),
      provide: (field) => EditorView.decorations.from(field),
    });
  }

  function behaviorExtension(): Extension {
    const bindings: KeyBinding[] = [
      ...defaultKeymap,
      ...historyKeymap,
      ...(options.searchable && !options.disabled ? searchKeymap : []),
      {
        key: "Escape",
        run: (view) => {
          exitArmed = true;
          closeSearchPanel(view);
          return false;
        },
      },
      {
        key: "F8",
        run: () => {
          focusDiagnostic(1);
          return true;
        },
        shift: () => {
          focusDiagnostic(-1);
          return true;
        },
      },
    ];
    if (options.tabBehavior === "indent" && !options.disabled) {
      bindings.push({
        key: "Tab",
        run: (view) => {
          if (exitArmed) {
            exitArmed = false;
            return false;
          }
          indentMore(view);
          return true;
        },
      });
    }
    return Prec.highest(keymap.of(bindings));
  }

  function announceDiagnostic(valid: CodeEditorDiagnostic[], index: number | null): void {
    const diagnostic = index === null ? undefined : valid[index];
    if (!diagnostic) {
      callbacks.onActiveDiagnostic(null);
      return;
    }
    const line = view.state.doc.lineAt(diagnostic.range.from);
    callbacks.onActiveDiagnostic({
      diagnostic,
      line: line.number,
      column: diagnostic.range.from - line.from + 1,
    });
  }

  function focusDiagnostic(direction: 1 | -1): void {
    if (options.disabled) return;
    const valid = reportDiagnostics(view.state.doc.length).sort(
      (a, b) => a.range.from - b.range.from || a.range.to - b.range.to,
    );
    if (valid.length === 0) {
      activeDiagnosticIndex = null;
      announceDiagnostic(valid, null);
      return;
    }
    activeDiagnosticIndex =
      activeDiagnosticIndex === null
        ? direction === 1
          ? 0
          : valid.length - 1
        : (activeDiagnosticIndex + direction + valid.length) % valid.length;
    const diagnostic = valid[activeDiagnosticIndex];
    if (!diagnostic) {
      announceDiagnostic(valid, null);
      return;
    }
    view.dispatch({ selection: { anchor: diagnostic.range.from } });
    view.focus();
    const line = view.state.doc.lineAt(diagnostic.range.from);
    callbacks.onActiveDiagnostic({
      diagnostic,
      line: line.number,
      column: diagnostic.range.from - line.from + 1,
    });
  }

  const view = new EditorView({
    parent: host,
    state: EditorState.create({
      doc: options.value,
      extensions: [
        history(),
        languageCompartment.of(await languageFor(options.language, options.performanceMode)),
        behaviorCompartment.of(behaviorExtension()),
        readOnlyCompartment.of([
          EditorState.readOnly.of(options.readOnly || options.disabled),
          EditorView.editable.of(!options.disabled),
        ]),
        diagnosticsCompartment.of(diagnosticField(reportDiagnostics(options.value.length))),
        wrapCompartment.of(options.wrapLines ? EditorView.lineWrapping : []),
        ...(options.lineNumbers ? [lineNumbers()] : []),
        ...(options.placeholder ? [placeholder(options.placeholder)] : []),
        tabSizeCompartment.of(EditorState.tabSize.of(normalizeTabSize(options.tabSize))),
        EditorView.updateListener.of((update: ViewUpdate) => {
          if (!update.docChanged || applyingHostValue) return;
          callbacks.onChange(
            transactionToChange(update.startState.doc.toString(), update.changes),
          );
        }),
      ],
    }),
  });

  view.contentDOM.setAttribute("aria-label", options.ariaLabel);
  if (options.disabled) view.contentDOM.setAttribute("aria-disabled", "true");

  // The outer focus treatment is a keyboard-entry affordance. It lives on the
  // root element as a local attribute, never on the document modality.
  const disposeFocusEntry = installCodeEditorFocusEntry(host);

  let updateEpoch = 0;

  async function update(next: Partial<CodeEditorEngineOptions>): Promise<void> {
    // Change detection against the applied options: wrappers resend the full
    // prop set on every sync, so a key present with an unchanged value must
    // not reconfigure anything. In particular, resending `diagnostics` must
    // not clear the F8 active diagnostic or rebuild its decorations.
    const previous = options;
    options = { ...options, ...next };
    if (typeof next.language === "string") assertAdmittedLanguage(next.language);
    const epoch = ++updateEpoch;
    const effects: StateEffect<unknown>[] = [];
    if (
      (next.language !== undefined && next.language !== previous.language) ||
      (next.performanceMode !== undefined && next.performanceMode !== previous.performanceMode)
    ) {
      const language = await languageFor(options.language, options.performanceMode);
      if (epoch !== updateEpoch) return;
      effects.push(languageCompartment.reconfigure(language) as StateEffect<unknown>);
    }
    if (
      (next.searchable !== undefined && next.searchable !== previous.searchable) ||
      (next.tabBehavior !== undefined && next.tabBehavior !== previous.tabBehavior) ||
      (next.disabled !== undefined && next.disabled !== previous.disabled)
    ) {
      effects.push(behaviorCompartment.reconfigure(behaviorExtension()));
    }
    if (
      (next.readOnly !== undefined && next.readOnly !== previous.readOnly) ||
      (next.disabled !== undefined && next.disabled !== previous.disabled)
    ) {
      effects.push(
        readOnlyCompartment.reconfigure([
          EditorState.readOnly.of(options.readOnly || options.disabled),
          EditorView.editable.of(!options.disabled),
        ]),
      );
    }
    if (
      next.diagnostics !== undefined &&
      diagnosticSignature(next.diagnostics) !== diagnosticSignature(previous.diagnostics)
    ) {
      activeDiagnosticIndex = null;
      announceDiagnostic([], null);
      effects.push(
        diagnosticsCompartment.reconfigure(
          diagnosticField(reportDiagnostics(view.state.doc.length)),
        ),
      );
    }
    if (next.wrapLines !== undefined && next.wrapLines !== previous.wrapLines) {
      effects.push(wrapCompartment.reconfigure(options.wrapLines ? EditorView.lineWrapping : []));
    }
    if (next.tabSize !== undefined && next.tabSize !== previous.tabSize) {
      effects.push(
        tabSizeCompartment.reconfigure(
          EditorState.tabSize.of(normalizeTabSize(options.tabSize)),
        ),
      );
    }
    if (next.ariaLabel !== undefined && next.ariaLabel !== previous.ariaLabel) {
      view.contentDOM.setAttribute("aria-label", options.ariaLabel);
    }
    if (next.disabled !== undefined && next.disabled !== previous.disabled) {
      if (options.disabled) view.contentDOM.setAttribute("aria-disabled", "true");
      else view.contentDOM.removeAttribute("aria-disabled");
    }
    const spec: TransactionSpec = {};
    if (next.value !== undefined && !sameDocumentText(next.value, view.state.doc.toString())) {
      spec.changes = [{ from: 0, to: view.state.doc.length, insert: next.value }];
    }
    const hasEffects = effects.length > 0;
    if (hasEffects) spec.effects = effects;
    if (spec.changes !== undefined || hasEffects) {
      applyingHostValue = true;
      try {
        view.dispatch(spec, { annotations: Transaction.addToHistory.of(false) });
      } finally {
        applyingHostValue = false;
      }
    }
  }

  function destroy(): void {
    disposeFocusEntry();
    closeSearchPanel(view);
    view.destroy();
  }

  return { update, focusDiagnostic, destroy };
}

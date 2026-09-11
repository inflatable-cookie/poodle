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
  HighlightStyle,
  syntaxHighlighting,
  syntaxTree,
} from "@codemirror/language";
import { tags as lezerTags } from "@lezer/highlight";
import {
  Decoration,
  EditorView,
  keymap,
  lineNumbers,
  placeholder,
  ViewPlugin,
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
  toCodeEditorChange,
  validateCodeEditorDiagnostics,
} from "@inflatable-cookie/poodle-core";
import type {
  CodeEditorChange,
  CodeEditorDiagnostic,
  CodeEditorLanguageId,
  CodeEditorLanguageRegistry,
  CodeEditorPerformanceMode,
  CodeEditorTabBehavior,
  CodeEditorTextEdit,
} from "@inflatable-cookie/poodle-core";

export interface CodeEditorEngineOptions {
  value: string;
  language: CodeEditorLanguageId;
  languageRegistry: CodeEditorLanguageRegistry | null;
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

/**
 * Fail closed: an unknown id never silently becomes plain text. `plain-text`
 * is built in; every other id must be admitted by the registry the host
 * supplied.
 */
export function assertAdmittedLanguage(
  language: string,
  languageRegistry: CodeEditorLanguageRegistry | null,
): asserts language is CodeEditorLanguageId {
  if (language === "plain-text") return;
  if (!languageRegistry || !languageRegistry.has(language)) {
    throw new Error(
      `code-editor: unsupported language "${language}". Register it through a language registry or request "plain-text" explicitly.`,
    );
  }
}

/** Substrate shape check: a loaded value must look like a CodeMirror extension. */
function isLanguageExtension(value: unknown): value is Extension {
  return (
    Array.isArray(value) ||
    (typeof value === "object" && value !== null && "extension" in value)
  );
}

/**
 * Token-bound syntax presentation (g18.021). One private highlight style maps
 * stable Lezer tag groups onto Poodle semantic CSS variables, so a live theme
 * change restyles mounted editors without engine recreation. Ordinary names
 * stay unmapped and inherit the primary text colour; weight or style never
 * substitutes for colour.
 *
 * Poodle ships no grammar: these tags are base editor presentation machinery,
 * and the style is installed exclusively with a full-mode non-plain language.
 */
const codeEditorHighlightStyle = HighlightStyle.define([
  {
    // Comments, metadata, and their subtags read as secondary text.
    tag: [lezerTags.comment, lezerTags.meta],
    color: "var(--poodle-color-text-secondary)",
  },
  {
    // The whole keyword family — control, definition, module, and operator
    // keywords — reads as accent. `null`, `super`, `this`, and boolean
    // literals carry their own more specific rules below.
    tag: lezerTags.keyword,
    color: "var(--poodle-color-accent-base)",
  },
  {
    // Strings (and string specialisations such as templates) read as success.
    tag: lezerTags.string,
    color: "var(--poodle-color-status-success)",
  },
  {
    // Numbers, booleans, and literal constants read as info.
    tag: [lezerTags.number, lezerTags.bool, lezerTags.null, lezerTags.atom],
    color: "var(--poodle-color-status-info)",
  },
  {
    // Types and definitions read as warning; ordinary names stay primary.
    tag: [
      lezerTags.typeName,
      lezerTags.className,
      lezerTags.definition(lezerTags.variableName),
      lezerTags.definition(lezerTags.propertyName),
    ],
    color: "var(--poodle-color-status-warning)",
  },
  {
    // Grammars that tag malformed input directly read as danger. Parser error
    // nodes of grammars that do not are marked by the plugin below.
    tag: lezerTags.invalid,
    color: "var(--poodle-color-status-danger)",
  },
]);

/**
 * Parser error nodes carry no Lezer tag, so the tag-bound style cannot reach
 * them. This private plugin marks visible error nodes with the danger token
 * so invalid syntax is legible without replacing host diagnostics. The style
 * is an inline CSS variable, so it follows live theme changes like the rest
 * of the presentation.
 */
const invalidSyntaxMark = Decoration.mark({
  class: "poodle-code-editor__syntax-invalid",
  attributes: { style: "color: var(--poodle-color-status-danger)" },
});

function invalidSyntaxDecorations(view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const tree = syntaxTree(view.state);
  for (const { from, to } of view.visibleRanges) {
    tree.iterate({
      from,
      to,
      enter: (node) => {
        if (!node.type.isError) return true;
        if (node.to > node.from) builder.add(node.from, node.to, invalidSyntaxMark);
        return false;
      },
    });
  }
  return builder.finish();
}

const invalidSyntaxHighlighter = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = invalidSyntaxDecorations(view);
    }

    update(update: ViewUpdate) {
      if (update.docChanged || update.viewportChanged) {
        this.decorations = invalidSyntaxDecorations(update.view);
      }
    }
  },
  { decorations: (plugin) => plugin.decorations },
);

/** Full-mode, non-plain syntax presentation installed with the language. */
const codeEditorSyntaxPresentation: Extension = [
  syntaxHighlighting(codeEditorHighlightStyle),
  invalidSyntaxHighlighter,
];

/**
 * Resolve the active id through the host's registry. Plain text (and plain
 * performance mode) load no language and no syntax presentation; anything
 * else must be admitted and must resolve to a real language extension before
 * it can reach the editor, and it carries the private token-bound
 * presentation with it.
 */
async function resolveLanguageExtension(
  options: Pick<CodeEditorEngineOptions, "language" | "languageRegistry" | "performanceMode">,
): Promise<Extension> {
  if (options.performanceMode === "plain" || options.language === "plain-text") return [];
  const registry = options.languageRegistry;
  if (!registry) {
    throw new Error(
      `code-editor: unsupported language "${options.language}". Register it through a language registry or request "plain-text" explicitly.`,
    );
  }
  assertAdmittedLanguage(options.language, registry);
  const loaded = await registry.load(options.language);
  if (!isLanguageExtension(loaded)) {
    throw new Error(
      `code-editor: language "${options.language}" did not resolve to a CodeMirror language extension.`,
    );
  }
  return [loaded, codeEditorSyntaxPresentation];
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
  assertAdmittedLanguage(initial.language, initial.languageRegistry);

  let options: CodeEditorEngineOptions = { ...initial };
  let applyingHostValue = false;
  let exitArmed = false;
  let activeDiagnosticIndex: number | null = null;

  const languageCompartment = new Compartment();
  const behaviorCompartment = new Compartment();
  const readOnlyCompartment = new Compartment();
  const diagnosticsCompartment = new Compartment();
  const wrapCompartment = new Compartment();
  const lineNumbersCompartment = new Compartment();
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
        languageCompartment.of(await resolveLanguageExtension(options)),
        behaviorCompartment.of(behaviorExtension()),
        readOnlyCompartment.of([
          EditorState.readOnly.of(options.readOnly || options.disabled),
          EditorView.editable.of(!options.disabled),
        ]),
        diagnosticsCompartment.of(diagnosticField(reportDiagnostics(options.value.length))),
        wrapCompartment.of(options.wrapLines ? EditorView.lineWrapping : []),
        lineNumbersCompartment.of(options.lineNumbers ? lineNumbers() : []),
        ...(options.placeholder ? [placeholder(options.placeholder)] : []),
        tabSizeCompartment.of(EditorState.tabSize.of(normalizeTabSize(options.tabSize))),
        EditorView.updateListener.of((update: ViewUpdate) => {
          if (!update.docChanged || applyingHostValue) return;
          // A committed user edit transaction — typing, deletion, line
          // commands, indentation, clipboard, history, drop — dismisses the
          // entry treatment in the same update that changes the document.
          focusEntry.dismiss();
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
  // root element as a local attribute, never on the document modality. Real
  // editing intent is transaction-driven: every committed user edit below
  // dismisses, so bindings that repurpose key names (indent Tab, copy-line
  // arrows) and readOnly context are honored by construction.
  const focusEntry = installCodeEditorFocusEntry(host);

  let updateEpoch = 0;

  async function update(next: Partial<CodeEditorEngineOptions>): Promise<void> {
    // Change detection against the applied options: wrappers resend the full
    // prop set on every sync, so a key present with an unchanged value must
    // not reconfigure anything. In particular, resending `diagnostics` must
    // not clear the F8 active diagnostic or rebuild its decorations.
    const previous = options;
    options = { ...options, ...next };
    if (typeof next.language === "string") {
      assertAdmittedLanguage(next.language, options.languageRegistry);
    }
    const epoch = ++updateEpoch;
    const effects: StateEffect<unknown>[] = [];
    if (
      (next.language !== undefined && next.language !== previous.language) ||
      (next.performanceMode !== undefined &&
        next.performanceMode !== previous.performanceMode) ||
      (next.languageRegistry !== undefined && next.languageRegistry !== previous.languageRegistry)
    ) {
      const language = await resolveLanguageExtension(options);
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
    if (next.lineNumbers !== undefined && next.lineNumbers !== previous.lineNumbers) {
      effects.push(lineNumbersCompartment.reconfigure(options.lineNumbers ? lineNumbers() : []));
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
    focusEntry.destroy();
    closeSearchPanel(view);
    view.destroy();
  }

  return { update, focusDiagnostic, destroy };
}

/**
 * CodeEditor component-local focus-entry presentation.
 *
 * The shared document modality tracks the last input device document-wide; it
 * cannot distinguish Tab navigation into the editor from ordinary typing
 * inside it. The outer focus treatment is an entry affordance, so the
 * presentation lives here instead, without ever writing the document
 * attribute (the document modality stays the source of truth for every other
 * control).
 *
 * Arming reads the document modality at the moment focus enters the editing
 * surface — the same policy every other composite control's CSS applies:
 * pointerdown always precedes focus, so pointer entry reads `pointer` and
 * never arms, while Tab/Shift+Tab entry reads `keyboard` and arms. The
 * difference is that armed state does not follow later keydowns: the first
 * real editing intent — insertion, deletion, composition, clipboard mutation,
 * undo, or redo — dismisses it. Focus leaving the component resets it.
 *
 * Contract: docs/contracts/components/code-editor.md §6 Focus.
 */

import { getInputModality } from "./input-modality";

/** Root data attribute carrying the keyboard entry presentation. */
export const CODE_EDITOR_FOCUS_ENTRY_ATTR = "data-focus-entry";

/** Keys that move the caret or drive editor chrome; never editing intent. */
const NON_EDITING_KEYS = new Set([
  "Tab",
  "Escape",
  "F8",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
  "ArrowUp",
  "End",
  "Home",
  "PageDown",
  "PageUp",
]);

/** Named keys CodeMirror handles in its keymap (preventing `beforeinput`). */
const EDITING_NAMED_KEYS = new Set(["Backspace", "Delete", "Enter"]);

/** History and clipboard chords (Ctrl/Cmd+X, V, Z, Y) are editing intent. */
const EDITING_CHORD_KEYS = new Set(["x", "v", "z", "y"]);

const MODIFIER_KEYS = new Set([
  "Alt",
  "AltGraph",
  "CapsLock",
  "Control",
  "Fn",
  "FnLock",
  "Hyper",
  "Meta",
  "NumLock",
  "ScrollLock",
  "Shift",
  "Super",
  "Symbol",
  "SymbolLock",
]);

/** `beforeinput` input types that mutate text or editing history. */
const EDITING_INPUT_TYPES = new Set([
  "insertText",
  "insertReplacementText",
  "insertFromPaste",
  "insertFromDrop",
  "insertCompositionText",
  "insertLineBreak",
  "insertParagraph",
  "deleteContent",
  "deleteContentBackward",
  "deleteContentForward",
  "deleteByCut",
  "deleteByDrag",
  "historyUndo",
  "historyRedo",
]);

/**
 * Whether the event target is a surface whose keys mean editing: the
 * CodeMirror content, the search panel, or the host itself. Keys on inert
 * chrome never count.
 */
function isEditableSurface(target: EventTarget | null, host: HTMLElement): boolean {
  return (
    target instanceof Element &&
    host.contains(target) &&
    target.matches(".poodle-code-editor, .cm-content, .cm-panel, .cm-panel *")
  );
}

/**
 * Install the component-local focus-entry presentation listeners on one
 * CodeEditor viewport host. Returns the disposer. The document input-modality
 * attribute is never written.
 */
export function installCodeEditorFocusEntry(host: HTMLElement): () => void {
  const editorRoot = host.closest<HTMLElement>(".poodle-code-editor");
  if (!editorRoot) {
    return () => {};
  }
  const root: HTMLElement = editorRoot;

  function arm(): void {
    root.setAttribute(CODE_EDITOR_FOCUS_ENTRY_ATTR, "keyboard");
  }

  function disarm(): void {
    root.removeAttribute(CODE_EDITOR_FOCUS_ENTRY_ATTR);
  }

  function onFocusin(event: FocusEvent): void {
    // Pointer entry reads `pointer` because pointerdown precedes focus;
    // keyboard navigation entry reads `keyboard`.
    if (getInputModality() !== "keyboard") return;
    arm();
  }

  function onFocusout(event: FocusEvent): void {
    // relatedTarget is the element receiving focus; inside the editor keeps
    // the entry state (search panel round trips), anything else resets it.
    if (event.relatedTarget instanceof Node && root.contains(event.relatedTarget)) return;
    root.removeAttribute(CODE_EDITOR_FOCUS_ENTRY_ATTR);
  }

  function onKeydown(event: Event): void {
    if (!(event instanceof KeyboardEvent)) return;
    if (!isEditableSurface(event.target, host)) return;
    if (MODIFIER_KEYS.has(event.key)) return;
    if (NON_EDITING_KEYS.has(event.key)) return;
    const isChord =
      (event.ctrlKey || event.metaKey) && EDITING_CHORD_KEYS.has(event.key.toLowerCase());
    const isTyping =
      event.key.length === 1 ||
      EDITING_NAMED_KEYS.has(event.key) ||
      event.key === "Dead" ||
      event.key === "Process" ||
      event.isComposing;
    if (!isChord && !isTyping) return;
    disarm();
  }

  function onBeforeinput(event: Event): void {
    if (!(event instanceof InputEvent)) return;
    if (!isEditableSurface(event.target, host)) return;
    if (!EDITING_INPUT_TYPES.has(event.inputType)) return;
    disarm();
  }

  host.addEventListener("focusin", onFocusin, true);
  host.addEventListener("focusout", onFocusout, true);
  host.addEventListener("keydown", onKeydown, true);
  host.addEventListener("beforeinput", onBeforeinput, true);
  return () => {
    host.removeEventListener("focusin", onFocusin, true);
    host.removeEventListener("focusout", onFocusout, true);
    host.removeEventListener("keydown", onKeydown, true);
    host.removeEventListener("beforeinput", onBeforeinput, true);
  };
}

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
 * Arming reads the document modality at focus entry — pointerdown always
 * precedes focus, so pointer entry reads `pointer` and never arms, while
 * keyboard navigation entry reads `keyboard` and arms. Dismissal is
 * transaction-driven: the engine reports committed edit transactions, so
 * bindings that repurpose bare key names (indent-mode Tab, copy-line arrows)
 * and readOnly context are honored by construction instead of re-derived
 * from key names here. Focus leaving the component resets the state.
 *
 * Contract: docs/contracts/components/code-editor.md §6 Focus.
 */

import { getInputModality } from "./input-modality";

/** Root data attribute carrying the keyboard entry presentation. */
export const CODE_EDITOR_FOCUS_ENTRY_ATTR = "data-focus-entry";

/** Handle over the installed entry presentation. */
export interface CodeEditorFocusEntryHandle {
  /**
   * Dismiss the entry presentation. The engine calls this when a committed
   * user edit transaction changes the document.
   */
  dismiss: () => void;
  /** Remove every listener. */
  destroy: () => void;
}

const INERT_DISPOSAL: CodeEditorFocusEntryHandle = {
  dismiss: () => {},
  destroy: () => {},
};

/**
 * Install the component-local focus-entry presentation listeners on one
 * CodeEditor viewport host. The document input-modality attribute is never
 * written.
 */
export function installCodeEditorFocusEntry(host: HTMLElement): CodeEditorFocusEntryHandle {
  const editorRoot = host.closest<HTMLElement>(".poodle-code-editor");
  if (!editorRoot) {
    return INERT_DISPOSAL;
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
    disarm();
  }

  function onPointerdown(): void {
    // A pointer press inside the editor means pointer operation: the entry
    // affordance yields even when earlier keyboard navigation armed it.
    disarm();
  }

  function onCompositionstart(event: Event): void {
    // IME composition commits to the document after dismissal can act; only
    // the editing surface counts, not IME inside editor chrome like find.
    if (!(event.target instanceof Element) || !event.target.closest(".cm-content")) return;
    disarm();
  }

  host.addEventListener("focusin", onFocusin, true);
  host.addEventListener("focusout", onFocusout, true);
  host.addEventListener("pointerdown", onPointerdown, true);
  host.addEventListener("compositionstart", onCompositionstart, true);
  return {
    dismiss: disarm,
    destroy() {
      host.removeEventListener("focusin", onFocusin, true);
      host.removeEventListener("focusout", onFocusout, true);
      host.removeEventListener("pointerdown", onPointerdown, true);
      host.removeEventListener("compositionstart", onCompositionstart, true);
    },
  };
}

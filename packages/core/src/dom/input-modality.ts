/**
 * Document input-modality tracker. Composite web controls gate their focus
 * treatment on `:root[data-poodle-input-modality="keyboard"]` so pointer
 * focus does not paint a ring. The attribute is the source of truth.
 *
 * `what-input` shape: capture-phase, passive listeners; modifier-only keys
 * do not flip to keyboard. Safe default is `keyboard` (autofocus after load).
 */

export type InputModality = "keyboard" | "pointer";

export const INPUT_MODALITY_ATTR = "data-poodle-input-modality";

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

const LISTENER_OPTS: AddEventListenerOptions = { capture: true, passive: true };

const installed = new WeakSet<Document>();

function resolveDocument(doc?: Document): Document | undefined {
  if (doc) {
    return doc;
  }
  if (typeof document === "undefined") {
    return undefined;
  }
  return document;
}

function writeModality(target: Document, modality: InputModality): void {
  target.documentElement.setAttribute(INPUT_MODALITY_ATTR, modality);
}

/**
 * Idempotent per-document installer. No-op without a `document` (SSR).
 * Listeners attach once; later calls leave the existing set in place.
 */
export function installInputModality(doc?: Document): void {
  const target = resolveDocument(doc);
  if (!target?.documentElement) {
    return;
  }
  if (installed.has(target)) {
    if (!target.documentElement.hasAttribute(INPUT_MODALITY_ATTR)) {
      writeModality(target, "keyboard");
    }
    return;
  }
  installed.add(target);

  if (!target.documentElement.hasAttribute(INPUT_MODALITY_ATTR)) {
    writeModality(target, "keyboard");
  }

  const onKeyDown = (event: KeyboardEvent): void => {
    if (MODIFIER_KEYS.has(event.key)) {
      return;
    }
    writeModality(target, "keyboard");
  };
  const onPointer = (): void => {
    writeModality(target, "pointer");
  };

  target.addEventListener("keydown", onKeyDown, LISTENER_OPTS);
  target.addEventListener("pointerdown", onPointer, LISTENER_OPTS);
  target.addEventListener("mousedown", onPointer, LISTENER_OPTS);
  target.addEventListener("touchstart", onPointer, LISTENER_OPTS);
}

export function getInputModality(doc?: Document): InputModality {
  const target = resolveDocument(doc);
  const value = target?.documentElement?.getAttribute(INPUT_MODALITY_ATTR);
  return value === "pointer" ? "pointer" : "keyboard";
}

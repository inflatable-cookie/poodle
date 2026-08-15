/**
 * Generic DOM editing helpers for conformance actions. Operate on native
 * input/textarea elements; no component identifier lives here.
 */

import { fireEvent } from "@testing-library/dom";

function asControl(el: HTMLElement | null): HTMLInputElement | HTMLTextAreaElement | null {
  if (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) return el;
  return el?.querySelector("input, textarea") ?? null;
}

type CompositionOrigin = { prefix: string; suffix: string };

const compositionOrigin = new WeakMap<Element, CompositionOrigin>();

export function insertIntoControl(el: HTMLElement | null, text: string): number {
  const control = asControl(el);
  if (!control || control.disabled || control.readOnly) return control?.value.length ?? 0;
  control.focus();
  const start = control.selectionStart ?? control.value.length;
  const end = control.selectionEnd ?? start;
  const next = `${control.value.slice(0, start)}${text}${control.value.slice(end)}`;
  fireEvent.input(control, { target: { value: next } });
  const caret = start + text.length;
  control.setSelectionRange(caret, caret);
  return caret;
}

export function selectOnControl(el: HTMLElement | null, start: number, end: number): void {
  const control = asControl(el);
  if (!control) return;
  control.focus();
  control.setSelectionRange(start, end);
}

export function composeOnControl(
  el: HTMLElement | null,
  text: string,
  phase: "start" | "update" | "commit",
): void {
  const control = asControl(el);
  if (!control || control.disabled || control.readOnly) return;
  control.focus();
  const start = control.selectionStart ?? control.value.length;
  const origin = compositionOrigin.get(control) ?? {
    prefix: control.value.slice(0, start),
    suffix: control.value.slice(control.selectionEnd ?? start),
  };
  if (phase === "start") {
    compositionOrigin.set(control, origin);
    fireEvent.compositionStart(control, { data: "" });
    return;
  }
  const composed = `${origin.prefix}${text}${origin.suffix}`;
  if (phase === "update") {
    fireEvent.compositionUpdate(control, { data: text });
    return;
  }
  // Commit input stays composing so the control buffers; compositionend is
  // the single valueChange. Splice from the origin captured at start, not
  // from a caret that already sits after the preview.
  fireEvent.input(control, {
    data: text,
    isComposing: true,
    target: { value: composed },
  });
  fireEvent.compositionEnd(control, { data: text });
  compositionOrigin.delete(control);
  const caret = origin.prefix.length + text.length;
  control.setSelectionRange(caret, caret);
}

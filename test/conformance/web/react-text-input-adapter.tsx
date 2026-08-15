/**
 * React TextInput conformance adapter (g14.006).
 */

import { act, cleanup, fireEvent, render, type RenderResult } from "@testing-library/react";

import type { RuntimeAdapter, TraceEntry } from "./runner";
import { composeOnControl, insertIntoControl, selectOnControl } from "./text-actions";
import { ReactTextInputHost } from "./hosts/ReactTextInputHost";

function partElement(root: HTMLElement | null, part: string): HTMLElement | null {
  if (!root) return null;
  if (part === "root") return root;
  if (part === "control") return root.querySelector(".poodle-text-input__control");
  if (part === "clear") return root.querySelector(".poodle-text-input__clear");
  if (part === "prefix") return root.querySelector(".poodle-text-input__affix--prefix");
  if (part === "suffix") return root.querySelector(".poodle-text-input__affix--suffix");
  return null;
}

export class ReactTextInputAdapter implements RuntimeAdapter {
  readonly runtime = "react";
  private _trace: TraceEntry[] = [];
  private host: RenderResult | null = null;
  private root: HTMLElement | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.host = render(
      <ReactTextInputHost
        fixture={fixture}
        onValueChange={(value: string) => {
          this._trace.push({ event: "valueChange", payload: { value } });
        }}
        onSubmit={(value: string) => {
          this._trace.push({ event: "submit", payload: { value } });
        }}
        onCancel={() => {
          this._trace.push({ event: "cancel" });
        }}
        onClear={() => {
          this._trace.push({ event: "clear" });
        }}
      />,
    );
    this.root = this.host.container.querySelector(".poodle-text-input") as HTMLElement | null;
  }

  rootElement(): HTMLElement | null {
    return this.root;
  }

  async press(part: string, _input: "pointer" | "keyboard" = "pointer"): Promise<void> {
    const el = partElement(this.root, part);
    if (!el) return;
    fireEvent.click(el);
  }

  async dismiss(_part: string): Promise<void> {}
  async pointer(_part: string, _target: "inside" | "outside"): Promise<void> {}
  async scrub(_part: string, _fraction: number, _phase: "press" | "drag" | "release"): Promise<void> {}

  async flush(): Promise<void> {
    await act(async () => undefined);
  }

  focus(part: string): void {
    partElement(this.root, part)?.focus();
  }

  async key(part: string, key: string): Promise<void> {
    const el = partElement(this.root, part);
    if (!el) return;
    el.focus();
    fireEvent.keyDown(el, { key });
  }

  async insert(part: string, text: string): Promise<void> {
    insertIntoControl(partElement(this.root, part), text);
  }

  async select(part: string, start: number, end: number): Promise<void> {
    selectOnControl(partElement(this.root, part), start, end);
  }

  async compose(part: string, text: string, phase: "start" | "update" | "commit"): Promise<void> {
    composeOnControl(partElement(this.root, part), text, phase);
  }

  trace(): TraceEntry[] {
    return this._trace;
  }

  cleanup(): void {
    cleanup();
    this.host = null;
    this.root = null;
  }
}

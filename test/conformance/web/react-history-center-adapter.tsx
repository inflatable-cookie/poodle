/**
 * React HistoryCenter conformance adapter (g14.007). The same driving surface
 * as the Svelte adapter; resolution stays the interface's job in both.
 */

import { cleanup, fireEvent, render } from "@testing-library/react";
import { act } from "react";

import {
  historyCenterInterface,
  serializeInterface,
} from "../../../packages/core/src/conformance";
import { ReactHistoryCenterHost } from "./hosts/ReactHistoryCenterHost";
import { resolvePart, type RuntimeAdapter, type TraceEntry } from "./runner";

const iface = serializeInterface(historyCenterInterface);

export class ReactHistoryCenterAdapter implements RuntimeAdapter {
  readonly runtime = "react";
  private _trace: TraceEntry[] = [];
  private root: HTMLElement | null = null;

  mount(fixture: {
    props: Record<string, unknown>;
    regions: Record<string, string>;
    host?: Record<string, unknown>;
  }): void {
    this._trace = [];
    const result = render(
      <ReactHistoryCenterHost
        props={fixture.props}
        host={fixture.host ?? {}}
        trace={this._trace}
      />,
    );
    this.root = result.container.querySelector(
      ".poodle-history-center-popover",
    ) as HTMLElement | null;
  }

  rootElement(): HTMLElement | null {
    return this.root;
  }

  private element(part: string): HTMLElement | null {
    return this.root ? resolvePart(this.root, iface, part) : null;
  }

  async press(part: string, input: "pointer" | "keyboard"): Promise<void> {
    const element = this.element(part);
    if (!element) return;
    element.focus();
    if (input === "keyboard") {
      fireEvent.keyDown(element, { key: "Enter" });
    }
    fireEvent.click(element);
    await this.flush();
  }

  focus(part: string): void {
    this.element(part)?.focus();
  }

  async key(part: string, key: string): Promise<void> {
    const element = this.element(part);
    if (!element) return;
    fireEvent.keyDown(element, { key });
    await this.flush();
  }

  async insert(part: string, text: string): Promise<void> {
    const element = this.element(part);
    if (!(element instanceof HTMLInputElement)) return;
    // The seeded name is selected when the rename opens, so typing replaces it.
    fireEvent.change(element, { target: { value: text } });
    await this.flush();
  }

  async dismiss(part: string): Promise<void> {
    const element = this.element(part) ?? this.root;
    if (!element) return;
    fireEvent.keyDown(element, { key: "Escape" });
    await this.flush();
  }

  async pointer(part: string, target: "inside" | "outside"): Promise<void> {
    if (target === "outside") {
      const outside = this.root?.ownerDocument.body;
      if (outside) fireEvent.pointerDown(outside);
    } else {
      const element = this.element(part);
      if (element) fireEvent.pointerDown(element);
    }
    await this.flush();
  }

  async scrub(): Promise<void> {
    // HistoryCenter has no scrubbable part; the corpus authors none.
  }

  async flush(): Promise<void> {
    await act(async () => {
      await Promise.resolve();
    });
  }

  trace(): TraceEntry[] {
    return [...this._trace];
  }

  cleanup(): void {
    cleanup();
    this.root = null;
  }
}

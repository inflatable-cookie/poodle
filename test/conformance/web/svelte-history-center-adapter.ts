/**
 * Svelte HistoryCenter conformance adapter (g14.007).
 *
 * Resolution is the interface's job, not this file's: parts are found through
 * the shared runner's descriptors, so no class name or part list is written
 * here. The adapter's only job is to drive real interactions and hand back the
 * trace the host recorded.
 */

import { cleanup, fireEvent, render } from "@testing-library/svelte";
import { tick } from "svelte";

import {
  historyCenterInterface,
  serializeInterface,
} from "../../../packages/core/src/conformance";
import HistoryCenterHost from "./hosts/HistoryCenterHost.svelte";
import { resolvePart, type RuntimeAdapter, type TraceEntry } from "./runner";

const iface = serializeInterface(historyCenterInterface);

export class SvelteHistoryCenterAdapter implements RuntimeAdapter {
  readonly runtime = "svelte";
  private _trace: TraceEntry[] = [];
  private host: ReturnType<typeof render<HistoryCenterHost>> | null = null;
  private root: HTMLElement | null = null;

  mount(fixture: {
    props: Record<string, unknown>;
    regions: Record<string, string>;
    host?: Record<string, unknown>;
  }): void {
    this._trace = [];
    // Every prop under `props`: `host` and `trace` collide with Svelte's own
    // render options otherwise.
    this.host = render(HistoryCenterHost, {
      props: {
        props: fixture.props,
        host: fixture.host ?? {},
        trace: this._trace,
      },
    });
    this.root = this.host.container.querySelector(
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
    // The seeded name is selected when the rename opens, so typing replaces
    // it — one `input` event, which is what happy-dom gives a real edit.
    await fireEvent.input(element, { target: { value: text } });
    await this.flush();
  }

  /** The real dismissal route: a document-level Escape. */
  async dismiss(part: string): Promise<void> {
    const element = this.element(part) ?? this.root;
    if (!element) return;
    await fireEvent.keyDown(element, { key: "Escape" });
    await this.flush();
  }

  async pointer(part: string, target: "inside" | "outside"): Promise<void> {
    if (target === "outside") {
      const outside = this.root?.ownerDocument.body;
      if (outside) await fireEvent.pointerDown(outside);
    } else {
      const element = this.element(part);
      if (element) await fireEvent.pointerDown(element);
    }
    await this.flush();
  }

  async scrub(): Promise<void> {
    // HistoryCenter has no scrubbable part; the corpus authors none.
  }

  async flush(): Promise<void> {
    await tick();
    await tick();
  }

  trace(): TraceEntry[] {
    return [...this._trace];
  }

  cleanup(): void {
    cleanup();
    this.root = null;
    this.host = null;
  }
}

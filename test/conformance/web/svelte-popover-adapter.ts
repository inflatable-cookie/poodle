/** Svelte Popover conformance adapter (g14.005). */

import { cleanup, fireEvent, render } from "@testing-library/svelte";
import { flushSync, tick } from "svelte";

import type { RuntimeAdapter, TraceEntry } from "./runner";
import PopoverHost from "./hosts/PopoverHost.svelte";

export class SveltePopoverAdapter implements RuntimeAdapter {
  readonly runtime = "svelte";
  private _trace: TraceEntry[] = [];
  private host: ReturnType<typeof render<PopoverHost>> | null = null;
  private root: HTMLElement | null = null;

  mount(fixture: {
    props: Record<string, unknown>;
    regions: Record<string, string>;
    host?: Record<string, unknown>;
  }): void {
    this._trace = [];
    this.host = render(PopoverHost, {
      fixture,
      onOpenChange: (open: boolean) =>
        this._trace.push({ event: "openChange", payload: { open } }),
    });
    this.root = this.host.container.querySelector(".poodle-popover") as HTMLElement | null;
  }

  rootElement(): HTMLElement | null {
    return this.root;
  }

  private partElement(part: string): HTMLElement | null {
    if (!this.root) return null;
    if (part === "root") return this.root;
    if (part === "trigger") return this.root.querySelector(".poodle-popover__trigger");
    if (part === "surface") {
      // The surface is portalled out of the root; resolve from the document.
      return this.root.ownerDocument.querySelector(".poodle-popover__surface");
    }
    return null;
  }

  async press(part: string, input: "pointer" | "keyboard"): Promise<void> {
    const element = this.partElement(part);
    if (!element) return;
    if (input === "keyboard") {
      // The trigger is a role=button div: Enter is handled by the component's
      // own keydown path. A real browser would then prevent the default
      // click, so no synthetic click is driven here — one toggle, one event.
      element.focus();
      fireEvent.keyDown(element, { key: "Enter" });
    } else {
      element.focus();
      fireEvent.click(element);
    }
    await this.flush();
  }

  focus(part: string): void {
    this.partElement(part)?.focus();
  }

  async key(part: string, key: string): Promise<void> {
    const element = this.partElement(part);
    if (!element) return;
    element.focus();
    fireEvent.keyDown(element, { key: key === "Space" ? " " : key });
  }

  async dismiss(_part: string): Promise<void> {
    // The real document-level Escape route: the dismissable-layer stack
    // listens for document keydown while a layer is open.
    const document = this.root?.ownerDocument ?? globalThis.document;
    fireEvent.keyDown(document.body, { key: "Escape" });
  }

  async pointer(part: string, target: "inside" | "outside"): Promise<void> {
    if (target === "outside") {
      // A real outside pointer press: target the document body, outside the
      // mounted component's containment set. The press also moves focus off
      // the component, like a real pointer-down outside it (the web browser
      // blurs the focused element; GPUI moves focus to the window host).
      const document = this.root?.ownerDocument ?? globalThis.document;
      (document.activeElement as HTMLElement | null)?.blur?.();
      fireEvent.mouseDown(document.body);
    } else {
      const element = this.partElement(part);
      if (!element) return;
      // A pointer-down on a non-focusable target blurs the focused element
      // (the browser default action); the surface content is not focusable.
      (this.root?.ownerDocument.activeElement as HTMLElement | null)?.blur?.();
      fireEvent.mouseDown(element);
    }
  }

  async scrub(): Promise<void> {}

  async flush(): Promise<void> {
    flushSync();
    // The focus-on-open effect waits on Svelte's tick; give it a microtask
    // boundary, then flush the resulting DOM again.
    await tick();
    flushSync();

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

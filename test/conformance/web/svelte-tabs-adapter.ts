/** Svelte Tabs conformance adapter (g14.004). */

import { cleanup, fireEvent, render } from "@testing-library/svelte";
import { flushSync } from "svelte";

import type { RuntimeAdapter, TraceEntry } from "./runner";
import TabsHost from "./hosts/TabsHost.svelte";

function partElement(root: HTMLElement, part: string): HTMLElement | null {
  if (part === "root") return root;
  if (part === "list") return root.querySelector(".poodle-tabs__list");
  const [base, key] = part.split(":", 2);
  const selector = base === "trigger" ? ".poodle-tabs__tab" : base === "panel" ? ".poodle-tabs__panel" : null;
  if (!selector || !key) return null;
  return Array.from(root.querySelectorAll<HTMLElement>(selector))
    .find((candidate) => candidate.dataset.value === key) ?? null;
}

export class SvelteTabsAdapter implements RuntimeAdapter {
  readonly runtime = "svelte";
  private _trace: TraceEntry[] = [];
  private host: ReturnType<typeof render<TabsHost>> | null = null;
  private root: HTMLElement | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.host = render(TabsHost, {
      fixture,
      onValueChange: (value: string) => this._trace.push({ event: "valueChange", payload: { value } }),
    });
    this.root = this.host.container.querySelector(".poodle-tabs");
  }

  rootElement(): HTMLElement | null { return this.root; }

  async press(part: string, input: "pointer" | "keyboard"): Promise<void> {
    const element = this.root ? partElement(this.root, part) : null;
    if (!element) return;
    if (input === "keyboard") {
      element.focus();
      // A browser synthesizes button activation from Enter. happy-dom does
      // not, so drive the resulting click while preserving keyboard focus.
      await fireEvent.click(element);
    } else {
      await fireEvent.click(element);
    }
  }

  focus(part: string): void { if (this.root) partElement(this.root, part)?.focus(); }

  async key(part: string, key: string): Promise<void> {
    const element = this.root ? partElement(this.root, part) : null;
    if (!element) return;
    element.focus();
    await fireEvent.keyDown(element, { key });
  }

  async scrub(): Promise<void> {}
  async flush(): Promise<void> { flushSync(); }
  trace(): TraceEntry[] { return [...this._trace]; }
  cleanup(): void { cleanup(); this.root = null; this.host = null; }
}

/**
 * Svelte runtime adapter for the conformance runner: mounts the real Svelte
 * Button, exposes the mounted root for the shared observer, and performs
 * actions through real events. The controlled-pressed host path ($state in a
 * small host component) is the same shape a real consumer uses.
 */

import { render, fireEvent, cleanup } from "@testing-library/svelte";
import { flushSync } from "svelte";

import type { RuntimeAdapter, TraceEntry } from "./runner";
import ButtonHost from "./hosts/ButtonHost.svelte";

export class SvelteButtonAdapter implements RuntimeAdapter {
  readonly runtime = "svelte";
  private _trace: TraceEntry[] = [];
  private host: ReturnType<typeof render<ButtonHost>> | null = null;
  private root: HTMLElement | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.host = render(ButtonHost, {
      fixture,
      onPress: () => {
        this._trace.push({ event: "press" });
      },
      onPressedChange: (pressed: boolean) => {
        this._trace.push({ event: "pressedChange", payload: { pressed } });
      },
    });
    this.root = this.host.container.querySelector("button.poodle-button") as HTMLElement | null;
  }

  rootElement(): HTMLElement | null {
    return this.root;
  }

  async press(_part: string, input: "pointer" | "keyboard"): Promise<void> {
    const root = this.root;
    if (!root) return;
    if (input === "keyboard") {
      // Keyboard activation is the browser's default action for a focused
      // button: Enter fires the click. happy-dom implements no default
      // actions, so the harness performs the browser default (keydown then
      // click) — the same event sequence a real browser produces.
      root.focus();
      fireEvent.keyDown(root, { key: "Enter" });
      fireEvent.click(root);
    } else {
      fireEvent.click(root);
    }
    await this.flush();
  }

  async flush(): Promise<void> {
    flushSync();
  }

  focus(part: string): void {
    if (part === "root") this.root?.focus();
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

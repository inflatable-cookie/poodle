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
      // Click-to-focus is the browser's default on pointer press too;
      // happy-dom leaves focus untouched, so the harness performs the
      // default a real browser would.
      root.focus();
      fireEvent.click(root);
    }
    await this.flush();
  }

  async dismiss(_part: string): Promise<void> {
    // The corpus of this profile does not exercise the dismissal route.
  }

  async pointer(_part: string, _target: "inside" | "outside"): Promise<void> {
    // The corpus of this profile does not exercise outside pointer intent.
  }

  async flush(): Promise<void> {
    flushSync();
  }

  focus(part: string): void {
    if (part === "root") this.root?.focus();
  }

  async key(_part: string, _key: string): Promise<void> {
    // Button corpus does not exercise key actions.
  }

  async scrub(_part: string, _fraction: number, _phase: "press" | "drag" | "release"): Promise<void> {
    // Button corpus does not exercise scrub actions.
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

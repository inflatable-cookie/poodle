/**
 * React runtime adapter for the conformance runner: mounts the real React
 * Button, exposes the mounted root for the shared observer, and performs
 * actions through real events. The controlled-pressed host path (rerender
 * with the updated prop) is the same shape a real consumer uses.
 */

import { render, fireEvent, cleanup, act, type RenderResult } from "@testing-library/react";

import type { RuntimeAdapter, TraceEntry } from "./runner";
import { Button } from "../../../packages/react/components/src/Button";

export class ReactButtonAdapter implements RuntimeAdapter {
  readonly runtime = "react";
  private _trace: TraceEntry[] = [];
  private host: RenderResult | null = null;
  private state: { pressed: boolean | null } = { pressed: null };
  private root: HTMLElement | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.state = { pressed: (fixture.props.pressed as boolean | null) ?? null };
    const props: Record<string, unknown> = { ...fixture.props };
    if (fixture.regions.label !== undefined) {
      props.children = fixture.regions.label;
    }
    props.leadingIcon = fixture.regions.leading ?? null;
    props.trailingIcon = fixture.regions.trailing ?? null;
    props.onClick = () => {
      this._trace.push({ event: "press" });
    };
    props.onPressedChange = (pressed: boolean) => {
      this._trace.push({ event: "pressedChange", payload: { pressed } });
      if (this.state.pressed !== null) {
        this.state.pressed = pressed;
        this.host?.rerender(<Button {...(props as never)} pressed={pressed} />);
      }
    };
    this.host = render(<Button {...(props as never)} />);
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

  async flush(): Promise<void> {
    await act(async () => {});
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

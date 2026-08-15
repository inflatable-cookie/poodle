/** React Tabs conformance adapter (g14.004). */

import { act, cleanup, fireEvent, render, type RenderResult } from "@testing-library/react";
import { useState } from "react";

import { Tabs } from "../../../packages/react/components/src/Tabs";
import type { RuntimeAdapter, TraceEntry } from "./runner";

function partElement(root: HTMLElement, part: string): HTMLElement | null {
  if (part === "root") return root;
  if (part === "list") return root.querySelector(".poodle-tabs__list");
  const [base, key] = part.split(":", 2);
  const selector = base === "trigger" ? ".poodle-tabs__tab" : base === "panel" ? ".poodle-tabs__panel" : null;
  if (!selector || !key) return null;
  return Array.from(root.querySelectorAll<HTMLElement>(selector))
    .find((candidate) => candidate.dataset.value === key) ?? null;
}

function Host(props: {
  fixture: { props: Record<string, unknown>; regions: Record<string, string> };
  onValueChange: (value: string) => void;
}) {
  const [value, setValue] = useState((props.fixture.props.value as string | null | undefined) ?? null);
  return (
    <Tabs
      {...(props.fixture.props as never)}
      value={value}
      onValueChange={(next) => {
        props.onValueChange(next);
        setValue(next);
      }}
    >
      {(activeValue) => <>{props.fixture.regions.panel} · {activeValue}</>}
    </Tabs>
  );
}

export class ReactTabsAdapter implements RuntimeAdapter {
  readonly runtime = "react";
  private _trace: TraceEntry[] = [];
  private host: RenderResult | null = null;
  private root: HTMLElement | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.host = render(
      <Host
        fixture={fixture}
        onValueChange={(value) => this._trace.push({ event: "valueChange", payload: { value } })}
      />,
    );
    this.root = this.host.container.querySelector(".poodle-tabs");
  }

  rootElement(): HTMLElement | null { return this.root; }

  async press(part: string, input: "pointer" | "keyboard"): Promise<void> {
    const element = this.root ? partElement(this.root, part) : null;
    if (!element) return;
    if (input === "keyboard") {
      act(() => element.focus());
      // A browser synthesizes button activation from Enter. happy-dom does
      // not, so drive the resulting click while preserving keyboard focus.
      await act(async () => {
        fireEvent.click(element);
      });
    } else {
      await act(async () => {
        fireEvent.click(element);
      });
    }
  }

  focus(part: string): void {
    if (!this.root) return;
    const element = partElement(this.root, part);
    if (element) act(() => element.focus());
  }

  async key(part: string, key: string): Promise<void> {
    const element = this.root ? partElement(this.root, part) : null;
    if (!element) return;
    act(() => element.focus());
    await act(async () => {
      fireEvent.keyDown(element, { key });
    });
  }

  async scrub(): Promise<void> {}
  async dismiss(_part: string): Promise<void> {
    // The corpus of this profile does not exercise the dismissal route.
  }

  async pointer(_part: string, _target: "inside" | "outside"): Promise<void> {
    // The corpus of this profile does not exercise outside pointer intent.
  }

  async flush(): Promise<void> { await act(async () => {}); }
  trace(): TraceEntry[] { return [...this._trace]; }
  cleanup(): void { cleanup(); this.root = null; this.host = null; }
}

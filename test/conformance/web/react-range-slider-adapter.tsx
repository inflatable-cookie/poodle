/**
 * React RangeSlider conformance adapter (g14.003).
 */

import { render, fireEvent, cleanup, act, type RenderResult } from "@testing-library/react";
import { useState } from "react";

import type { RuntimeAdapter, TraceEntry } from "./runner";
import { RangeSlider } from "../../../packages/react/components/src/RangeSlider";

function partElement(root: HTMLElement, part: string): HTMLElement | null {
  if (part === "root") return root;
  if (part === "lower") {
    return root.querySelector(
      ".poodle-range-slider__control--lower, .poodle-range-slider__embedded-control--lower",
    );
  }
  if (part === "upper") {
    return root.querySelector(
      ".poodle-range-slider__control--upper, .poodle-range-slider__embedded-control--upper",
    );
  }
  return null;
}

function Host(props: {
  fixture: { props: Record<string, unknown>; regions: Record<string, string> };
  onValueChange: (value: [number, number]) => void;
  onValueCommit: (value: [number, number]) => void;
}) {
  const initial = (props.fixture.props.value as [number, number] | undefined) ?? [0, 100];
  const [value, setValue] = useState<[number, number]>(initial);
  return (
    <RangeSlider
      {...(props.fixture.props as never)}
      value={value}
      onValueChange={(next) => {
        props.onValueChange(next);
        setValue(next);
      }}
      onValueCommit={(next) => {
        props.onValueCommit(next);
        setValue(next);
      }}
    />
  );
}

export class ReactRangeSliderAdapter implements RuntimeAdapter {
  readonly runtime = "react";
  private _trace: TraceEntry[] = [];
  private host: RenderResult | null = null;
  private root: HTMLElement | null = null;
  private activeThumb: "lower" | "upper" | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.activeThumb = null;
    this.host = render(
      <Host
        fixture={fixture}
        onValueChange={(value) => {
          this._trace.push({ event: "valueChange", payload: { value } });
        }}
        onValueCommit={(value) => {
          this._trace.push({ event: "valueCommit", payload: { value } });
        }}
      />,
    );
    this.root = this.host.container.querySelector(".poodle-range-slider") as HTMLElement | null;
  }

  rootElement(): HTMLElement | null {
    return this.root;
  }

  async press(_part: string, _input: "pointer" | "keyboard"): Promise<void> {}

  async flush(): Promise<void> {
    await act(async () => {});
  }

  focus(part: string): void {
    partElement(this.root ?? document.body, part)?.focus();
  }

  async key(part: string, key: string): Promise<void> {
    const el = partElement(this.root ?? document.body, part);
    if (!el) return;
    el.focus();
    if (el instanceof HTMLInputElement && el.type === "range") {
      const step = Number(el.step) || 1;
      const min = Number(el.min);
      const max = Number(el.max);
      let next = Number(el.value);
      if (key === "ArrowRight" || key === "ArrowUp") next = Math.min(max, next + step);
      else if (key === "ArrowLeft" || key === "ArrowDown") next = Math.max(min, next - step);
      else if (key === "Home") next = min;
      else if (key === "End") next = max;
      else {
        fireEvent.keyDown(el, { key });
        await this.flush();
        return;
      }
      fireEvent.input(el, { target: { value: String(next) } });
      fireEvent.keyUp(el, { key });
      await this.flush();
      return;
    }
    fireEvent.keyDown(el, { key });
    await this.flush();
  }

  async scrub(
    _part: string,
    fraction: number,
    phase: "press" | "drag" | "release",
  ): Promise<void> {
    const root = this.root;
    if (!root || root.getAttribute("data-disabled") === "true") return;

    const lower = partElement(root, "lower");
    const upper = partElement(root, "upper");
    if (lower instanceof HTMLInputElement && upper instanceof HTMLInputElement) {
      const min = Number(lower.min);
      const max = Number(upper.max);
      const raw = min + fraction * (max - min);
      if (phase === "press" || this.activeThumb == null) {
        const lowerValue = Number(lower.value);
        const upperValue = Number(upper.value);
        this.activeThumb =
          Math.abs(raw - lowerValue) <= Math.abs(raw - upperValue) ? "lower" : "upper";
      }
      const target = this.activeThumb === "lower" ? lower : upper;
      if (phase === "press" || phase === "drag") {
        fireEvent.input(target, { target: { value: String(raw) } });
      }
      if (phase === "release") {
        fireEvent.mouseUp(target);
        this.activeThumb = null;
      }
      await this.flush();
      return;
    }

    const rect = root.getBoundingClientRect();
    const x = rect.left + fraction * Math.max(rect.width, 1);
    const y = rect.top + rect.height / 2;
    const eventInit = { clientX: x, clientY: y, button: 0, pointerId: 1 };
    if (phase === "press") fireEvent.pointerDown(root, eventInit);
    else if (phase === "drag") fireEvent.pointerMove(root, eventInit);
    else fireEvent.pointerUp(root, eventInit);
    await this.flush();
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

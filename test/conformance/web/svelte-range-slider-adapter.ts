/**
 * Svelte RangeSlider conformance adapter (g14.003). Mounts the real component
 * through a controlled host and drives scrub/key through DOM events.
 */

import { render, fireEvent, cleanup } from "@testing-library/svelte";
import { flushSync } from "svelte";

import type { RuntimeAdapter, TraceEntry } from "./runner";
import RangeSliderHost from "./hosts/RangeSliderHost.svelte";

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

export class SvelteRangeSliderAdapter implements RuntimeAdapter {
  readonly runtime = "svelte";
  private _trace: TraceEntry[] = [];
  private host: ReturnType<typeof render<RangeSliderHost>> | null = null;
  private root: HTMLElement | null = null;
  private activeThumb: "lower" | "upper" | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.activeThumb = null;
    this.host = render(RangeSliderHost, {
      fixture,
      onValueChange: (value: [number, number]) => {
        this._trace.push({ event: "valueChange", payload: { value } });
      },
      onValueCommit: (value: [number, number]) => {
        this._trace.push({ event: "valueCommit", payload: { value } });
      },
    });
    this.root = this.host.container.querySelector(".poodle-range-slider") as HTMLElement | null;
  }

  rootElement(): HTMLElement | null {
    return this.root;
  }

  async press(_part: string, _input: "pointer" | "keyboard"): Promise<void> {
    // RangeSlider corpus uses scrub/key, not press.
  }

  async flush(): Promise<void> {
    flushSync();
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
      // happy-dom has no range-input default actions; perform the browser default.
      el.value = String(next);
      el.dispatchEvent(new Event("input", { bubbles: true }));
      el.dispatchEvent(new Event("change", { bubbles: true }));
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
        target.value = String(raw);
        target.dispatchEvent(new Event("input", { bubbles: true }));
      }
      if (phase === "release") {
        target.dispatchEvent(new Event("change", { bubbles: true }));
        this.activeThumb = null;
      }
      await this.flush();
      return;
    }

    // Embedded variant: pointer path on the root.
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

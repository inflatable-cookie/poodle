/**
 * Svelte runtime adapter for the conformance runner: mounts the real Svelte
 * Button, resolves parts against the real DOM, and performs actions through
 * real events. The controlled-pressed host path ($set on the $bindable prop)
 * is the same shape a real consumer uses.
 */

import { render, fireEvent, cleanup } from "@testing-library/svelte";
import { flushSync } from "svelte";

import type { RuntimeAdapter, TraceEntry, RuntimeObservation, PartObservation } from "./runner";
import { observeRootGeometry, observeRootChannels } from "./runner";
import ButtonHost from "./hosts/ButtonHost.svelte";

interface HostState {
  pressed: boolean | null;
}

export class SvelteButtonAdapter implements RuntimeAdapter {
  readonly runtime = "svelte";
  private _trace: TraceEntry[] = [];
  private host: ReturnType<typeof render<ButtonHost>> | null = null;
  private state: HostState = { pressed: null };
  private root: HTMLElement | null = null;

  mount(fixture: { props: Record<string, unknown>; regions: Record<string, string> }): void {
    this._trace = [];
    this.state = { pressed: (fixture.props.pressed as boolean | null) ?? null };
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

  observe(): RuntimeObservation {
    const root = this.root;
    if (!root) {
      return { runtime: this.runtime, component: "button", parts: {}, trace: [...this._trace] };
    }
    const doc = root.ownerDocument;
    const labelEl = root.querySelector<HTMLElement>(".poodle-button__label");
    const iconSpans = Array.from(root.querySelectorAll<HTMLElement>(".poodle-button__icon"));
    const spinnerEl = root.querySelector<HTMLElement>(".poodle-button__spinner");
    const chevronEl = root.querySelector<HTMLElement>(".poodle-button__chevron");
    const hasLeading = root.hasAttribute("data-has-leading");
    const hasTrailing = root.hasAttribute("data-has-trailing");

    const baseStates: Record<string, boolean> = {
      disabled: root.hasAttribute("disabled"),
      loading: root.getAttribute("data-loading") === "true",
      pressed: root.getAttribute("aria-pressed") === "true",
      focused: doc.activeElement === root,
      focusVisible: doc.activeElement === root && root.matches(":focus-visible"),
    };

    const tokenRoles: Record<string, string> = {
      variant: root.getAttribute("data-variant") ?? "",
      tone: root.getAttribute("data-tone") ?? "default",
      size: root.getAttribute("data-size") ?? "",
      density: root.getAttribute("data-density") ?? "",
      fit: root.getAttribute("data-fit") ?? "default",
      truncate: root.getAttribute("data-truncate") ?? "",
    };

    const part = (el: HTMLElement | null, extra?: Partial<PartObservation>): PartObservation => {
      const present = Boolean(el);
      if (!el) {
        return {
          present,
          role: null,
          name: null,
          text: null,
          icon: null,
          states: {},
          tokenRoles: {},
          focusable: false,
          focused: false,
          focusVisible: false,
          geometry: {},
          channels: {},
        };
      }
      return {
        present,
        role: null,
        name: null,
        text: el.textContent,
        icon: null,
        states: {},
        tokenRoles: {},
        focusable: false,
        focused: false,
        focusVisible: false,
        geometry: {},
        channels: {},
        ...extra,
      };
    };

    const name =
      root.getAttribute("aria-label") ?? labelEl?.textContent?.trim() ?? root.textContent?.trim();

    const leadingEl = hasLeading ? iconSpans[0] ?? null : null;
    const trailingEl = hasTrailing ? iconSpans[iconSpans.length - 1] ?? null : null;

    return {
      runtime: this.runtime,
      component: "button",
      parts: {
        root: {
          present: true,
          role: "button",
          name: name ?? null,
          text: null,
          icon: null,
          states: baseStates,
          tokenRoles,
          focusable: !root.hasAttribute("disabled"),
          focused: baseStates.focused,
          focusVisible: baseStates.focusVisible,
          geometry: observeRootGeometry(root),
          channels: observeRootChannels(root),
        },
        label: part(labelEl),
        leadingIcon: part(leadingEl),
        trailingIcon: part(trailingEl),
        spinner: part(spinnerEl),
        chevron: part(chevronEl),
      },
      trace: [...this._trace],
    };
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

import { fireEvent, render, screen } from "@testing-library/svelte";
import { hydrate, tick, unmount } from "svelte";
import { describe, expect, it } from "vitest";

import Harness from "./PopoverRetainedHarness.svelte";
import SsrHarness from "./ssr/PopoverSsrHarness.svelte";

// Server output from PopoverSsrHarness with an initially open interactive
// trigger. Keeping the real hydration markers here lets the client suite prove
// the server-advertised controls relationship survives hydration. The paired
// server suite separately renders this shape and verifies request-local ids.
const OPEN_INTERACTIVE_SSR_HTML =
  '<!--[--><div data-poodle-theme-root=""><!--[0--><!--$s1--><div data-scope="popover" data-part="root" data-state="open" data-block="false" data-placement="bottom-start" data-surface-width="content" class="poodle-popover"><div data-part="trigger" data-state="open" data-block="false" data-disabled="false" class="poodle-popover__trigger"><!--[0--><button type="button" class="poodle-button" data-variant="secondary" data-size="md" data-density="default" data-loading="false" aria-expanded="true" aria-controls="poodle-popover-s1"><!--[-1--><!--]--> <!--[-1--><!--]--> <!--[0--><span class="poodle-button__label"><!---->Open<!----></span><!--]--> <!--[-1--><!--]--> <!--[-1--><!--]--></button><!----><!--]--></div> <!--[0--><div data-part="surface" data-state="open" data-placement="bottom-start" data-surface-width="content" id="poodle-popover-s1" role="dialog" tabindex="-1" class="poodle-popover__surface" style=""><button type="button" data-testid="surface-action">Surface action</button><!----></div><!--]--></div><!--]--></div><!--]-->';

/**
 * Two shipped Popover defects, retained from the rejected g14 conformance
 * pilot (g14.005, g14.007) as focused regressions. The pilot corpus is gone;
 * these are the claims worth keeping.
 */
describe("Popover — retained regressions", () => {
  it("keeps a controlled open request inert while disabled", async () => {
    // g14.005: a controlled `open: true` host with `disabled: true` rendered
    // the surface anyway, past the machine's own guard.
    render(Harness, { props: { open: true, disabled: true } });

    expect(screen.queryByTestId("surface-action")).toBeNull();
  });

  it("renders the surface for a controlled open request when enabled", async () => {
    render(Harness, { props: { open: true } });

    expect(screen.getByTestId("surface-action")).toBeTruthy();
  });

  it("restores focus to the interactive trigger, not its wrapper", async () => {
    // g14.007: with `triggerIsInteractive` the wrapper observes clicks without
    // becoming a button. Restoring focus to the wrapper left the operator on
    // something Enter could not activate.
    render(Harness, { props: { defaultOpen: true, triggerIsInteractive: true } });

    const surfaceAction = screen.getByTestId("surface-action");
    surfaceAction.focus();
    await fireEvent.keyDown(document, { key: "Escape" });

    expect(document.activeElement).toBe(screen.getByTestId("inner-trigger"));
  });
});

/**
 * g15.041: interactive trigger semantics. In interactive mode the wrapper is
 * inert layout — no role, tab stop, keydown handler, or ARIA — and the
 * `PopoverTriggerState` payload lands on the caller's real control.
 *
 * Server-render evidence for the same semantics lives in
 * `test/ssr/PopoverSsr.test.ts` (the `svelte-components-ssr` project).
 */
describe("Popover — interactive trigger semantics (g15.041)", () => {
  function wrapperOf(container: HTMLElement): HTMLElement {
    const wrapper = container.querySelector<HTMLElement>(".poodle-popover__trigger");
    if (!wrapper) throw new Error("trigger wrapper not rendered");
    return wrapper;
  }

  function surfaceId(): string {
    const surface = document.querySelector<HTMLElement>('[data-part="surface"]');
    if (!surface?.id) throw new Error("surface not rendered");
    return surface.id;
  }

  it("renders the wrapper roleless and untabbable with the disclosure ARIA on the real control", () => {
    const { container } = render(Harness, { props: { triggerIsInteractive: true } });

    const wrapper = wrapperOf(container);
    expect(wrapper.getAttribute("role")).toBeNull();
    expect(wrapper.getAttribute("tabindex")).toBeNull();
    expect(wrapper.getAttribute("aria-expanded")).toBeNull();
    expect(wrapper.getAttribute("aria-controls")).toBeNull();
    expect(wrapper.getAttribute("aria-disabled")).toBeNull();

    const control = screen.getByTestId("inner-trigger");
    expect(control.getAttribute("aria-expanded")).toBe("false");
    expect(control.getAttribute("aria-controls")).toBeNull();
  });

  it("threads expanded and controls onto the real control while open", async () => {
    render(Harness, { props: { triggerIsInteractive: true } });

    const control = screen.getByTestId("inner-trigger");
    await fireEvent.click(control);

    expect(control.getAttribute("aria-expanded")).toBe("true");
    expect(control.getAttribute("aria-controls")).toBe(surfaceId());
  });

  it("toggles repeatedly from the real control, click and keyboard activation alike", async () => {
    render(Harness, { props: { triggerIsInteractive: true } });

    const control = screen.getByTestId("inner-trigger");

    // Keyboard: the wrapper owns no keydown handler in interactive mode, so a
    // bare keydown must not toggle — happy-dom cannot replay the browser's
    // native Enter/Space activation of a real button, which arrives as a
    // click and is covered by the click assertions in this same test.
    await fireEvent.keyDown(control, { key: "Enter" });
    await fireEvent.keyDown(control, { key: " " });
    expect(screen.queryByTestId("surface-action")).toBeNull();

    await fireEvent.click(control);
    expect(screen.getByTestId("surface-action")).toBeTruthy();
    expect(control.getAttribute("aria-expanded")).toBe("true");

    await fireEvent.click(control);
    expect(screen.queryByTestId("surface-action")).toBeNull();
    expect(control.getAttribute("aria-expanded")).toBe("false");

    await fireEvent.click(control);
    expect(screen.getByTestId("surface-action")).toBeTruthy();
  });

  it("closes on outside pointerdown and on Escape", async () => {
    render(Harness, { props: { triggerIsInteractive: true, defaultOpen: true } });

    const control = screen.getByTestId("inner-trigger");
    await fireEvent.mouseDown(document.body);
    expect(screen.queryByTestId("surface-action")).toBeNull();
    expect(document.activeElement).toBe(control);

    await fireEvent.click(control);
    expect(screen.getByTestId("surface-action")).toBeTruthy();
    await fireEvent.keyDown(document, { key: "Escape" });
    expect(screen.queryByTestId("surface-action")).toBeNull();
    expect(document.activeElement).toBe(control);
  });

  it("follows a controlled open prop driven through onOpenChange", async () => {
    const calls: boolean[] = [];
    const onOpenChange = (next: boolean) => calls.push(next);
    const { rerender } = render(Harness, {
      props: { triggerIsInteractive: true, open: false, onOpenChange },
    });

    expect(screen.queryByTestId("surface-action")).toBeNull();

    // The host applies the requested state, as a controlled host does.
    await rerender({ triggerIsInteractive: true, open: true, onOpenChange });
    const control = screen.getByTestId("inner-trigger");
    expect(screen.getByTestId("surface-action")).toBeTruthy();
    expect(control.getAttribute("aria-controls")).toBe(surfaceId());

    // A toggle request against a controlled popover emits the intent…
    await fireEvent.click(control);
    expect(calls).toEqual([false]);

    // …and the host drives the close by re-passing the prop.
    await rerender({ triggerIsInteractive: true, open: false, onOpenChange });
    expect(screen.queryByTestId("surface-action")).toBeNull();
    expect(control.getAttribute("aria-expanded")).toBe("false");
  });

  it("runs uncontrolled from defaultOpen", () => {
    render(Harness, { props: { triggerIsInteractive: true, defaultOpen: true } });

    const control = screen.getByTestId("inner-trigger");
    expect(screen.getByTestId("surface-action")).toBeTruthy();
    expect(control.getAttribute("aria-expanded")).toBe("true");
    expect(control.getAttribute("aria-controls")).toBe(surfaceId());
  });

  it("carries disabled onto the real control and blocks opening", async () => {
    render(Harness, { props: { triggerIsInteractive: true, disabled: true } });

    const control = screen.getByTestId("inner-trigger");
    expect((control as HTMLButtonElement).disabled).toBe(true);

    await fireEvent.click(control);
    expect(screen.queryByTestId("surface-action")).toBeNull();
  });

  it("hydrates the server-advertised controls relationship without changing its id", async () => {
    const target = document.createElement("div");
    target.innerHTML = OPEN_INTERACTIVE_SSR_HTML;
    document.body.appendChild(target);

    const component = hydrate(SsrHarness, {
      target,
      props: { triggerIsInteractive: true, defaultOpen: true },
    });
    await tick();

    const control = target.querySelector<HTMLElement>(".poodle-button");
    const surface = screen.getByTestId("surface-action").closest<HTMLElement>('[data-part="surface"]');
    expect(control?.getAttribute("aria-controls")).toBe("poodle-popover-s1");
    expect(surface?.id).toBe("poodle-popover-s1");

    await unmount(component);
    target.remove();
  });
});

describe("Popover — default trigger mode unchanged (g15.041)", () => {
  it("keeps role, tab stop, and disclosure ARIA on the wrapper", async () => {
    const { container } = render(Harness, { props: {} });

    const wrapper = container.querySelector<HTMLElement>(".poodle-popover__trigger");
    expect(wrapper?.getAttribute("role")).toBe("button");
    expect(wrapper?.getAttribute("tabindex")).toBe("0");
    expect(wrapper?.getAttribute("aria-expanded")).toBe("false");
    expect(wrapper?.getAttribute("aria-controls")).toBeNull();

    await fireEvent.click(wrapper!);
    expect(wrapper?.getAttribute("aria-expanded")).toBe("true");
    expect(wrapper?.getAttribute("aria-controls")).toBe(
      document.querySelector<HTMLElement>('[data-part="surface"]')?.id,
    );
  });

  it("opens from Enter on the wrapper in default mode", async () => {
    const { container } = render(Harness, { props: {} });

    const wrapper = container.querySelector<HTMLElement>(".poodle-popover__trigger")!;
    await fireEvent.keyDown(wrapper, { key: "Enter" });

    expect(screen.getByTestId("surface-action")).toBeTruthy();
  });
});

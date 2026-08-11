import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import DialogInitialFocusHarness from "./DialogInitialFocusHarness.svelte";
import FormDialogInitialFocusHarness from "./FormDialogInitialFocusHarness.svelte";

/**
 * `initialFocus` resolution on the Dialog open edge (g13.009).
 *
 * Covers the contract cases: "auto" skips header chrome and lands on the
 * first body focusable; "none" focuses nothing; a selector resolves within
 * the surface; an unmatched selector falls back to "auto"; an already-focused
 * element inside the surface is not stolen; FormDialog focuses its first
 * field by default and a consumer override wins.
 *
 * The open-edge resolution runs inside `tick().then(...)`, so each assertion
 * flushes a macrotask first. The guard case focuses the field synchronously
 * after render, before that microtask chain resolves.
 */
async function flush(): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, 0));
}

describe("Dialog initialFocus (svelte)", () => {
  it('"auto" skips the close button and lands on the first body focusable', async () => {
    render(DialogInitialFocusHarness, { props: { initialFocus: "auto" } });
    await flush();

    expect(document.activeElement).toBe(screen.getByTestId("first-field"));
  });

  it('defaults to "auto"', async () => {
    render(DialogInitialFocusHarness, {});
    await flush();

    expect(document.activeElement).toBe(screen.getByTestId("first-field"));
  });

  it('"none" focuses nothing', async () => {
    render(DialogInitialFocusHarness, { props: { initialFocus: "none" } });
    await flush();

    const surface = document.querySelector(".poodle-dialog__surface");
    expect(surface).toBeTruthy();
    expect(surface!.contains(document.activeElement)).toBe(false);
  });

  it("a selector resolves within the surface", async () => {
    render(DialogInitialFocusHarness, { props: { initialFocus: "[data-testid='in-body-button']" } });
    await flush();

    expect(document.activeElement).toBe(screen.getByTestId("in-body-button"));
  });

  it("an unmatched selector falls back to \"auto\" behaviour", async () => {
    render(DialogInitialFocusHarness, { props: { initialFocus: "#does-not-exist" } });
    await flush();

    expect(document.activeElement).toBe(screen.getByTestId("first-field"));
  });

  it("does not steal focus from an element already focused inside the surface", async () => {
    render(DialogInitialFocusHarness, { props: { initialFocus: "auto" } });
    const field = screen.getByTestId("first-field");
    field.focus();
    await flush();

    expect(document.activeElement).toBe(field);
  });
});

describe("FormDialog initialFocus (svelte)", () => {
  it("focuses its first field by default", async () => {
    render(FormDialogInitialFocusHarness, {});
    await flush();

    expect(document.activeElement).toBe(screen.getByTestId("form-first-field"));
  });

  it("a consumer-supplied initialFocus wins over the default", async () => {
    render(FormDialogInitialFocusHarness, {
      props: { initialFocus: "[data-testid='form-second-field']" },
    });
    await flush();

    expect(document.activeElement).toBe(screen.getByTestId("form-second-field"));
  });
});

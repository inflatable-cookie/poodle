import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Dialog } from "../src/Dialog";
import { FormDialog } from "../src/FormDialog";

/**
 * `initialFocus` resolution on the Dialog open edge (g13.009), React side.
 *
 * Mirrors the Svelte suite: "auto" skips header chrome and lands on the
 * first body focusable; "none" focuses nothing; a selector resolves within
 * the surface; an unmatched selector falls back to "auto"; an already-focused
 * element inside the surface is not stolen; FormDialog focuses its first
 * field by default and a consumer override wins.
 *
 * The React resolution runs from the surface ref callback during the open
 * commit, so focus is observable synchronously after `render` returns. The
 * guard case relies on child refs attaching before the parent surface ref.
 */
describe("Dialog initialFocus (react)", () => {
  it('"auto" skips the close button and lands on the first body focusable', () => {
    render(
      <Dialog open title="Settings" showCloseButton initialFocus="auto">
        <input data-testid="first-field" type="text" />
        <button type="button" data-testid="in-body-button">
          In body
        </button>
      </Dialog>,
    );

    expect(document.activeElement).toBe(document.querySelector('[data-testid="first-field"]'));
  });

  it('defaults to "auto"', () => {
    render(
      <Dialog open title="Settings" showCloseButton>
        <input data-testid="first-field" type="text" />
      </Dialog>,
    );

    expect(document.activeElement).toBe(document.querySelector('[data-testid="first-field"]'));
  });

  it('"none" focuses nothing', () => {
    render(
      <Dialog open title="Settings" showCloseButton initialFocus="none">
        <input data-testid="first-field" type="text" />
      </Dialog>,
    );

    const surface = document.querySelector(".poodle-dialog__surface");
    expect(surface).toBeTruthy();
    expect(surface!.contains(document.activeElement)).toBe(false);
  });

  it("a selector resolves within the surface", () => {
    render(
      <Dialog open title="Settings" showCloseButton initialFocus="[data-testid='in-body-button']">
        <input data-testid="first-field" type="text" />
        <button type="button" data-testid="in-body-button">
          In body
        </button>
      </Dialog>,
    );

    expect(document.activeElement).toBe(document.querySelector('[data-testid="in-body-button"]'));
  });

  it('an unmatched selector falls back to "auto" behaviour', () => {
    render(
      <Dialog open title="Settings" showCloseButton initialFocus="#does-not-exist">
        <input data-testid="first-field" type="text" />
      </Dialog>,
    );

    expect(document.activeElement).toBe(document.querySelector('[data-testid="first-field"]'));
  });

  it("does not steal focus from an element already focused inside the surface", () => {
    render(
      <Dialog open title="Settings" showCloseButton initialFocus="auto">
        <input data-testid="first-field" type="text" />
        <input
          data-testid="already-focused"
          type="text"
          ref={(node) => {
            // Child refs attach before the parent surface ref, so this focus
            // lands before the dialog's open-edge resolution runs.
            if (node) node.focus();
          }}
        />
      </Dialog>,
    );

    expect(document.activeElement).toBe(document.querySelector('[data-testid="already-focused"]'));
  });
});

describe("FormDialog initialFocus (react)", () => {
  it("focuses its first field by default", () => {
    render(
      <FormDialog
        open
        title="Edit"
        body={() => (
          <>
            <input data-testid="form-first-field" type="text" />
            <input data-testid="form-second-field" type="text" />
          </>
        )}
      />,
    );

    expect(document.activeElement).toBe(document.querySelector('[data-testid="form-first-field"]'));
  });

  it("a consumer-supplied initialFocus wins over the default", () => {
    render(
      <FormDialog
        open
        title="Edit"
        initialFocus="[data-testid='form-second-field']"
        body={() => (
          <>
            <input data-testid="form-first-field" type="text" />
            <input data-testid="form-second-field" type="text" />
          </>
        )}
      />,
    );

    expect(document.activeElement).toBe(document.querySelector('[data-testid="form-second-field"]'));
  });
});

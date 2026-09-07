import { createElement } from "react";
import { render as renderReact } from "@testing-library/react";
import { render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import {
  getInputModality,
  INPUT_MODALITY_ATTR,
  installInputModality,
} from "@inflatable-cookie/poodle-core";
import { TokenInput as ReactTokenInput } from "@inflatable-cookie/poodle-react";

import TokenInput from "../../components/src/TokenInput.svelte";

function freshDocument(): Document {
  return document.implementation.createHTMLDocument("input-modality");
}

describe("installInputModality tracker", () => {
  it("defaults to keyboard and flips pointer → keyboard → pointer", () => {
    const doc = freshDocument();
    installInputModality(doc);
    expect(getInputModality(doc)).toBe("keyboard");
    expect(doc.documentElement.getAttribute(INPUT_MODALITY_ATTR)).toBe("keyboard");

    doc.dispatchEvent(new Event("pointerdown", { bubbles: true }));
    expect(getInputModality(doc)).toBe("pointer");

    doc.dispatchEvent(new KeyboardEvent("keydown", { key: "Tab", bubbles: true }));
    expect(getInputModality(doc)).toBe("keyboard");

    doc.dispatchEvent(new Event("pointerdown", { bubbles: true }));
    expect(getInputModality(doc)).toBe("pointer");
  });

  it("ignores modifier-only keydown", () => {
    const doc = freshDocument();
    installInputModality(doc);
    doc.dispatchEvent(new Event("pointerdown", { bubbles: true }));
    doc.dispatchEvent(new KeyboardEvent("keydown", { key: "Shift", bubbles: true }));
    expect(getInputModality(doc)).toBe("pointer");
  });

  it("installs one listener set when called twice on the same document", () => {
    const doc = freshDocument();
    const spy = vi.spyOn(doc, "addEventListener");
    installInputModality(doc);
    const first = spy.mock.calls.length;
    installInputModality(doc);
    expect(first).toBe(4);
    expect(spy.mock.calls.length).toBe(first);
    spy.mockRestore();
  });

  it("is a no-op without document", () => {
    vi.stubGlobal("document", undefined);
    try {
      expect(() => installInputModality()).not.toThrow();
      expect(getInputModality()).toBe("keyboard");
    } finally {
      vi.unstubAllGlobals();
    }
  });
});

describe("TokenInput shells install the tracker", () => {
  afterEach(() => {
    document.documentElement.removeAttribute(INPUT_MODALITY_ATTR);
  });

  it("writes the root attribute when Svelte TokenInput mounts", () => {
    render(TokenInput, { props: { ariaLabel: "Tags" } });
    expect(document.documentElement.getAttribute(INPUT_MODALITY_ATTR)).toBe("keyboard");
  });

  it("writes the root attribute when React TokenInput mounts", () => {
    renderReact(createElement(ReactTokenInput, { ariaLabel: "Tags" }));
    expect(document.documentElement.getAttribute(INPUT_MODALITY_ATTR)).toBe("keyboard");
  });
});

import { afterEach, describe, expect, test } from "bun:test";
import { Window } from "happy-dom";

import { getFocusableElements, trapFocusKeydown } from "../src/dom/focus.ts";

let activeWindow: Window | null = null;

function createDom(): { window: Window; document: Document } {
  const window = new Window();
  activeWindow = window;
  return { window, document: window.document as unknown as Document };
}

afterEach(() => {
  activeWindow?.close();
  activeWindow = null;
});

function pressTab(
  window: Window,
  surface: HTMLElement,
  shiftKey = false,
): KeyboardEvent {
  const event = new window.KeyboardEvent("keydown", {
    key: "Tab",
    shiftKey,
    cancelable: true,
  }) as unknown as KeyboardEvent;
  trapFocusKeydown(surface, event);
  return event;
}

describe("getFocusableElements (happy-dom)", () => {
  test("includes position: fixed elements even when offsetParent is null", () => {
    const { document } = createDom();
    const root = document.createElement("div");
    const fixed = document.createElement("button");
    fixed.style.position = "fixed";
    Object.defineProperty(fixed, "offsetParent", { value: null, configurable: true });
    root.appendChild(fixed);
    document.body.appendChild(root);

    expect(getFocusableElements(root)).toEqual([fixed]);
  });

  test("filters out hidden elements and elements with no client rects", () => {
    const { document } = createDom();
    const root = document.createElement("div");

    const visible = document.createElement("button");
    const hiddenAttr = document.createElement("button");
    hiddenAttr.setAttribute("hidden", "");
    const zeroRects = document.createElement("button");
    zeroRects.getClientRects = () => [] as unknown as DOMRectList;

    root.append(visible, hiddenAttr, zeroRects);
    document.body.appendChild(root);

    expect(getFocusableElements(root)).toEqual([visible]);
  });

  test("matches a[href], area[href], [contenteditable], and summary; excludes non-link [href] and contenteditable=false", () => {
    const { document } = createDom();
    const root = document.createElement("div");

    const anchor = document.createElement("a");
    anchor.setAttribute("href", "#top");
    const anchorNoHref = document.createElement("a");
    const map = document.createElement("map");
    const area = document.createElement("area");
    area.setAttribute("href", "#region");
    map.appendChild(area);
    const linkEl = document.createElement("link");
    linkEl.setAttribute("href", "style.css");
    const editable = document.createElement("div");
    editable.setAttribute("contenteditable", "");
    const editableTrue = document.createElement("div");
    editableTrue.setAttribute("contenteditable", "true");
    const editableFalse = document.createElement("div");
    editableFalse.setAttribute("contenteditable", "false");
    const details = document.createElement("details");
    const summary = document.createElement("summary");
    summary.textContent = "More";
    details.appendChild(summary);

    root.append(anchor, anchorNoHref, map, linkEl, editable, editableTrue, editableFalse, details);
    document.body.appendChild(root);

    expect(getFocusableElements(root)).toEqual([
      anchor,
      area,
      editable,
      editableTrue,
      summary,
    ]);
  });

  test("excludes candidates removed from sequential tab order via negative tabindex, type=hidden, or inert", () => {
    const { document } = createDom();
    const root = document.createElement("div");

    const button = document.createElement("button");
    const editableNegative = document.createElement("div");
    editableNegative.setAttribute("contenteditable", "true");
    editableNegative.setAttribute("tabindex", "-1");
    const buttonNegative = document.createElement("button");
    buttonNegative.setAttribute("tabindex", "-1");
    const linkNegative = document.createElement("a");
    linkNegative.setAttribute("href", "#skip");
    linkNegative.setAttribute("tabindex", "-2");
    const hiddenInput = document.createElement("input");
    hiddenInput.type = "hidden";
    const inertWrapper = document.createElement("div");
    inertWrapper.setAttribute("inert", "");
    const inertButton = document.createElement("button");
    inertWrapper.appendChild(inertButton);

    root.append(button, editableNegative, buttonNegative, linkNegative, hiddenInput, inertWrapper);
    document.body.appendChild(root);

    expect(getFocusableElements(root)).toEqual([button]);
  });
});

describe("trapFocusKeydown (happy-dom)", () => {
  test("wraps Tab and Shift+Tab around a button followed by a contenteditable[tabindex=-1] element", () => {
    const { window, document } = createDom();
    const surface = document.createElement("div");
    surface.tabIndex = -1;
    const button = document.createElement("button");
    const editableNegative = document.createElement("div");
    editableNegative.setAttribute("contenteditable", "true");
    editableNegative.setAttribute("tabindex", "-1");
    surface.append(button, editableNegative);
    document.body.appendChild(surface);

    button.focus();
    const forward = pressTab(window, surface, false);
    expect(forward.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(button);

    surface.focus();
    const backward = pressTab(window, surface, true);
    expect(backward.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(button);
  });
  test("wraps Shift+Tab to last when focus is on the surface itself", () => {
    const { window, document } = createDom();
    const surface = document.createElement("div");
    surface.tabIndex = -1;
    const first = document.createElement("button");
    const last = document.createElement("button");
    surface.append(first, last);
    document.body.appendChild(surface);

    surface.focus();
    expect(document.activeElement).toBe(surface);

    const event = pressTab(window, surface, true);
    expect(event.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(last);
  });

  test("wraps Shift+Tab to last and Tab to first when focus is outside the focusable list", () => {
    const { window, document } = createDom();
    const outside = document.createElement("button");
    const surface = document.createElement("div");
    surface.tabIndex = -1;
    const first = document.createElement("button");
    const last = document.createElement("button");
    surface.append(first, last);
    document.body.append(outside, surface);

    outside.focus();
    const back = pressTab(window, surface, true);
    expect(back.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(last);

    outside.focus();
    const forward = pressTab(window, surface, false);
    expect(forward.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(first);
  });

  test("wraps first<->last and lets middle elements tab naturally", () => {
    const { window, document } = createDom();
    const surface = document.createElement("div");
    const first = document.createElement("button");
    const middle = document.createElement("button");
    const last = document.createElement("button");
    surface.append(first, middle, last);
    document.body.appendChild(surface);

    first.focus();
    const wrapBack = pressTab(window, surface, true);
    expect(wrapBack.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(last);

    const wrapForward = pressTab(window, surface, false);
    expect(wrapForward.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(first);

    middle.focus();
    const middleForward = pressTab(window, surface, false);
    expect(middleForward.defaultPrevented).toBe(false);
    const middleBack = pressTab(window, surface, true);
    expect(middleBack.defaultPrevented).toBe(false);
  });

  test("pins focus to the surface when there are no focusable children", () => {
    const { window, document } = createDom();
    const surface = document.createElement("div");
    surface.tabIndex = -1;
    document.body.appendChild(surface);

    const event = pressTab(window, surface, false);
    expect(event.defaultPrevented).toBe(true);
    expect(document.activeElement).toBe(surface);
  });
});

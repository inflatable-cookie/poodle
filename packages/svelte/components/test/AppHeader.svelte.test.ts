import { readFileSync } from "node:fs";
import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import AppHeader from "../src/AppHeader.svelte";
import AppHeaderCenterHarness from "./AppHeaderCenterHarness.svelte";
import AppHeaderElementHarness from "./AppHeaderElementHarness.svelte";

// Element access (g13.014): `bind:element` yields the raw `<header>` DOM
// element (never a handle object), so a host can attach behaviour to the
// root. The binding must not change the rendered output.
describe("AppHeader (svelte)", () => {
  it("bind:element resolves to the rendered header element", () => {
    render(AppHeaderElementHarness);
    expect(screen.getByTestId("bound-element").textContent).toBe("HEADER poodle-app-header");
  });

  it("renders the header with unchanged anatomy", () => {
    const { container } = render(AppHeader, { props: { title: "Finch", dragRegion: true } });
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header).toBeTruthy();
    expect(header.getAttribute("data-drag-region")).toBe("true");
    expect(header.getAttribute("aria-label")).toBe("Finch");
    expect(header.querySelector("strong")?.textContent).toBe("Finch");
  });

  it("renders data-drag-region=false when dragRegion is false (default)", () => {
    const { container } = render(AppHeader, { props: { title: "Finch" } });
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header.getAttribute("data-drag-region")).toBe("false");
  });
});

// Centre region (g13-b017): presence is the layout signal. The CSS is stubbed
// by vitest, so these tests inject the real stylesheet to assert the computed
// grid (ruling 3: the default header must be byte-identical and pixel-identical
// — the media query is evaluated by happy-dom at stylesheet parse time, so the
// narrow cases set the window width before injecting).
const appHeaderCss = readFileSync(
  new URL("../../../core/src/styles/app-header.css", `file://${import.meta.dirname}/`),
  "utf8",
);

function injectAppHeaderStyles(widthPx?: number): void {
  if (widthPx !== undefined) {
    (window as unknown as { happyDOM: { setInnerWidth(w: number): void } }).happyDOM.setInnerWidth(widthPx);
  }
  const style = document.createElement("style");
  style.textContent = appHeaderCss;
  document.head.appendChild(style);
}

// Svelte 5 emits `<!---->` hydration markers and whitespace-only text nodes
// around dynamic content; those are compiler plumbing, not region markup. This
// strips them so the comparison is over the real element structure — the same
// content the pre-g13-b017 component emitted.
function normalizedHtml(element: HTMLElement): string {
  const clone = element.cloneNode(true) as HTMLElement;
  const walker = document.createTreeWalker(
    clone,
    NodeFilter.SHOW_COMMENT | NodeFilter.SHOW_TEXT,
  );
  const toRemove: Node[] = [];
  let node = walker.nextNode();
  while (node) {
    if (node.nodeType === Node.COMMENT_NODE) {
      toRemove.push(node);
    } else if (node.nodeType === Node.TEXT_NODE && !node.textContent!.trim()) {
      toRemove.push(node);
    }
    node = walker.nextNode();
  }
  for (const doomed of toRemove) doomed.parentNode?.removeChild(doomed);
  return clone.innerHTML;
}

// Computed grid, with the stylesheet's line breaks collapsed (the multi-line
// `grid-template-columns` source serializes verbatim into the computed value).
function gridOf(element: HTMLElement): string {
  return getComputedStyle(element).gridTemplateColumns.replace(/\s+/g, " ");
}

// The pre-g13-b017 region markup: three flat siblings, no centre, no trailing
// wrapper, no data-center attribute. Byte-identical to before the centre
// region existed.
const DEFAULT_REGION_MARKUP =
  '<div class="poodle-app-header__identity"><div class="poodle-app-header__title-group"><strong>Finch</strong></div></div>' +
  '<div class="poodle-app-header__actions">New</div>' +
  '<div class="poodle-app-header__utility">Settings</div>';

const CENTERED_REGION_MARKUP =
  '<div class="poodle-app-header__identity"><div class="poodle-app-header__title-group"><strong>Finch</strong></div></div>' +
  '<div class="poodle-app-header__center">Centre</div>' +
  '<div class="poodle-app-header__trailing">' +
  '<div class="poodle-app-header__actions">New</div>' +
  '<div class="poodle-app-header__utility">Settings</div>' +
  "</div>";

describe("AppHeader centre region (svelte)", () => {
  it("keeps the default region markup byte-identical when center is absent", () => {
    const { container } = render(AppHeaderCenterHarness, {
      props: { center: false },
    });
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header.hasAttribute("data-center")).toBe(false);
    expect(normalizedHtml(header)).toBe(DEFAULT_REGION_MARKUP);
  });

  it("keeps the default computed grid when center is absent", () => {
    injectAppHeaderStyles();
    const { container } = render(AppHeader, { props: { title: "Finch" } });
    const header = container.querySelector("header.poodle-app-header")!;
    expect(gridOf(header)).toBe("minmax(0, 1fr) auto auto");
  });

  it("emits the centre region and trailing wrapper when center is present", () => {
    injectAppHeaderStyles();
    const { container } = render(AppHeaderCenterHarness, {
      props: { center: true },
    });
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header.getAttribute("data-center")).toBe("");
    expect(normalizedHtml(header)).toBe(CENTERED_REGION_MARKUP);
    expect(gridOf(header)).toBe(
      "minmax(0, 1fr) auto minmax(0, 1fr)",
    );
  });

  it("reflows a centred header and collapses the default at ≤45rem", () => {
    injectAppHeaderStyles(600); // 37.5rem — the media query matches at parse time
    const plain = render(AppHeaderCenterHarness, { props: { center: false } });
    const centred = render(AppHeaderCenterHarness, { props: { center: true } });
    const plainHeader = plain.container.querySelector("header.poodle-app-header")!;
    const centredHeader = centred.container.querySelector("header.poodle-app-header")!;
    expect(gridOf(plainHeader)).toBe("1fr");
    expect(gridOf(centredHeader)).toBe(
      "auto minmax(0, 1fr) auto",
    );
  });
});

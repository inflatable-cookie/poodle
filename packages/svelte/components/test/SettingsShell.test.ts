import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { readFileSync } from "node:fs";
import { createRawSnippet } from "svelte";
import { describe, expect, it, vi } from "vitest";

import SettingsShell from "../src/SettingsShell.svelte";

// The required behaviors cover the frame's layout contract (independent
// scroll regions, one-line truncated group titles), which the CSS owns —
// vitest stubs CSS, so these tests inject the real sheets (the AppHeader
// pattern) before asserting computed style.

const settingsShellCss = readFileSync(
  new URL("../../../core/src/styles/settings-shell.css", `file://${import.meta.dirname}/`),
  "utf8",
);
const sidebarNavCss = readFileSync(
  new URL("../../../core/src/styles/sidebar-nav.css", `file://${import.meta.dirname}/`),
  "utf8",
);

function injectStyles(): void {
  for (const css of [settingsShellCss, sidebarNavCss]) {
    const style = document.createElement("style");
    style.textContent = css;
    document.head.appendChild(style);
  }
}

const groups = [
  { id: "general", label: "General", items: [{ value: "general", label: "General" }] },
];

const pageSnippet = createRawSnippet(() => ({
  render: () => "<p class=\"settings-page-marker\">Page content</p>",
}));

function baseProps(overrides: Record<string, unknown> = {}): Record<string, unknown> {
  return {
    groups,
    activePageId: "general",
    pageTitle: "General",
    open: true,
    ...overrides,
  };
}

describe("SettingsShell (svelte)", () => {
  it("truncates group labels on one line and carries a native title tooltip", () => {
    injectStyles();
    const longLabel = "Keyboard Shortcuts & Input & More";
    render(SettingsShell, {
      props: baseProps({
        groups: [{ id: "g1", label: longLabel, items: [{ value: "a", label: "A" }] }],
      }),
    });

    const title = document.querySelector(".poodle-sidebar-nav__group-title") as HTMLElement;
    expect(title).not.toBeNull();
    expect(title.getAttribute("title")).toBe(longLabel);
    const style = getComputedStyle(title);
    expect(style.whiteSpace).toBe("nowrap");
    expect(style.overflow).toBe("hidden");
    expect(style.textOverflow).toBe("ellipsis");
  });

  it("renders exactly one close affordance in the whole shell", () => {
    render(SettingsShell, { props: baseProps() });

    expect(screen.getAllByRole("button", { name: /close/i })).toHaveLength(1);
    expect(screen.queryByRole("button", { name: "Close" })).toBeNull();
  });

  it("keeps the page rendered while a query is live — search narrows the rail, it does not replace the page", async () => {
    const narrowed = [
      { id: "storage", label: "Storage & Backups", items: [{ value: "storage", label: "Storage" }] },
    ];
    const { rerender } = render(SettingsShell, {
      props: baseProps({ page: pageSnippet, searchQuery: "" }),
    });

    // No query: full rail, page renders.
    expect(document.querySelector(".settings-page-marker")).not.toBeNull();
    expect(screen.getByRole("button", { name: "General" })).not.toBeNull();

    // Query live and the host narrowed the groups: the page is STILL there.
    await rerender(baseProps({ page: pageSnippet, searchQuery: "stor", groups: narrowed }));
    expect(document.querySelector(".settings-page-marker")).not.toBeNull();
    expect(screen.getByRole("button", { name: "Storage" })).not.toBeNull();
    expect(screen.queryByRole("button", { name: "General" })).toBeNull();

    // The removed results panel must not come back.
    expect(document.querySelector(".poodle-settings-shell__results")).toBeNull();
    expect(document.querySelector(".poodle-settings-shell__result-list")).toBeNull();
  });

  it("keeps navigation and page in separately scrollable regions, outside the search and header", () => {
    injectStyles();
    render(SettingsShell, { props: baseProps({ page: pageSnippet }) });

    const navViewport = document.querySelector(
      ".poodle-settings-shell__nav .poodle-scroll-shell__viewport",
    ) as HTMLElement;
    const pageViewport = document.querySelector(
      ".poodle-settings-shell__page-stack .poodle-scroll-shell__viewport",
    ) as HTMLElement;
    expect(navViewport).not.toBeNull();
    expect(pageViewport).not.toBeNull();
    expect(getComputedStyle(navViewport).overflowY).toBe("auto");
    expect(getComputedStyle(pageViewport).overflowY).toBe("auto");
    expect(navViewport).not.toBe(pageViewport);

    // Search lives in the dialog header bar, outside both scroll regions.
    const searchInput = document.querySelector(".poodle-settings-shell__search input") as HTMLElement;
    expect(navViewport.contains(searchInput)).toBe(false);
    expect(pageViewport.contains(searchInput)).toBe(false);
  });

  it("fires onNavigate with the page id from the rail", async () => {
    const onNavigate = vi.fn();
    render(SettingsShell, { props: baseProps({ onNavigate }) });

    await fireEvent.click(screen.getByRole("button", { name: "General" }));
    expect(onNavigate).toHaveBeenCalledWith("general");
  });

  it("fires onRequestClose on a close attempt and stays open against a refusal", async () => {
    const onRequestClose = vi.fn();
    render(SettingsShell, {
      props: baseProps({ closeRefusedReason: "Apply or discard this page before leaving.", onRequestClose }),
    });

    await fireEvent.click(screen.getByRole("button", { name: "Close settings" }));
    expect(onRequestClose).toHaveBeenCalledOnce();
    expect(screen.getByRole("dialog", { name: "Settings" })).toBeTruthy();

    // Escape is a close attempt too, and the refusal holds.
    await fireEvent.keyDown(document.body, { key: "Escape" });
    expect(onRequestClose).toHaveBeenCalledTimes(2);
    expect(screen.getByRole("dialog", { name: "Settings" })).toBeTruthy();
  });

  it("closes when the host does not refuse", async () => {
    render(SettingsShell, { props: baseProps() });

    await fireEvent.click(screen.getByRole("button", { name: "Close settings" }));
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("renders a refused close as an announced warning callout, not an error", () => {
    render(SettingsShell, {
      props: baseProps({ closeRefusedReason: "Apply or discard this page before leaving." }),
    });

    const callout = document.querySelector(".poodle-callout") as HTMLElement;
    expect(callout).not.toBeNull();
    expect(callout.getAttribute("data-tone")).toBe("warning");
    expect(callout.getAttribute("role")).toBe("status");
    expect(callout.getAttribute("aria-live")).toBe("polite");
    expect(callout.textContent).toContain("Apply or discard this page before leaving.");
    // Not an error treatment: nothing announces assertively.
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("distinguishes an empty scope from a query that matched nothing", () => {
    // No pages at all, no query: the scope is empty.
    render(SettingsShell, { props: baseProps({ groups: [], searchQuery: "" }) });
    expect(screen.getByText("No settings pages")).toBeTruthy();
    expect(screen.getByText("This scope has no settings pages yet.")).toBeTruthy();
    expect(document.querySelector(".poodle-sidebar-nav")).toBeNull();

    cleanup();

    // Pages exist but the host's filter removed them all: say so instead.
    render(SettingsShell, { props: baseProps({ groups: [], searchQuery: "xyzzy" }) });
    expect(screen.getByText("No matches")).toBeTruthy();
    expect(screen.getByText("No settings match your search.")).toBeTruthy();
    expect(screen.queryByText("No settings pages")).toBeNull();
  });

  it("defaults the dialog's accessible name to the title, and lets a host override it", async () => {
    const { rerender } = render(SettingsShell, { props: baseProps() });
    expect(screen.getByRole("dialog").getAttribute("aria-label")).toBe("Settings");

    await rerender(baseProps({ ariaLabel: "Nucleus settings" }));
    expect(screen.getByRole("dialog").getAttribute("aria-label")).toBe("Nucleus settings");
  });
});

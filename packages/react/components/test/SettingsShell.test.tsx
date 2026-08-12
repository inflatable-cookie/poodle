import { fireEvent, render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";

import { SettingsShell, type SettingsShellProps } from "../src/SettingsShell";

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

function baseProps(overrides: Partial<SettingsShellProps> = {}): SettingsShellProps {
  return {
    groups,
    activePageId: "general",
    pageTitle: "General",
    open: true,
    ...overrides,
  };
}

// Controlled host wrapper: the shell is fully controlled in React, so a close
// only takes effect when the host updates `open` (the Svelte binding does the
// same job implicitly there).
function Host({ onOpenChange, ...props }: SettingsShellProps) {
  const [open, setOpen] = useState(true);
  return (
    <SettingsShell
      {...props}
      open={open}
      onOpenChange={(next) => {
        setOpen(next);
        onOpenChange?.(next);
      }}
    />
  );
}

describe("SettingsShell (react)", () => {
  it("truncates group labels on one line and carries a native title tooltip", () => {
    injectStyles();
    const longLabel = "Keyboard Shortcuts & Input & More";
    render(<SettingsShell {...baseProps({ groups: [{ id: "g1", label: longLabel, items: [{ value: "a", label: "A" }] }] })} />);

    const title = document.querySelector(".poodle-sidebar-nav__group-title") as HTMLElement;
    expect(title).not.toBeNull();
    expect(title.getAttribute("title")).toBe(longLabel);
    const style = getComputedStyle(title);
    expect(style.whiteSpace).toBe("nowrap");
    expect(style.overflow).toBe("hidden");
    expect(style.textOverflow).toBe("ellipsis");
  });

  it("renders exactly one close affordance in the whole shell", () => {
    render(<SettingsShell {...baseProps()} />);

    expect(screen.getAllByRole("button", { name: /close/i })).toHaveLength(1);
    expect(screen.queryByRole("button", { name: "Close" })).toBeNull();
  });

  it("replaces the page region with results while searching and restores it when cleared", () => {
    const results = [
      { pageId: "storage", pageLabel: "Storage", anchorId: "disks", anchorLabel: "Disks" },
      { pageId: "backup", pageLabel: "Backup" },
    ];
    const page = <p className="settings-page-marker">Page content</p>;

    const { rerender } = render(<SettingsShell {...baseProps({ page, searchResults: null })} />);
    expect(document.querySelector(".settings-page-marker")).not.toBeNull();
    expect(document.querySelector(".poodle-settings-shell__results")).toBeNull();

    rerender(<SettingsShell {...baseProps({ page, searchResults: results })} />);
    expect(document.querySelector(".poodle-settings-shell__results")).not.toBeNull();
    expect(document.querySelector(".settings-page-marker")).toBeNull();
    expect(screen.getByRole("button", { name: /Storage/ })).not.toBeNull();
    expect(screen.getByRole("button", { name: /Backup/ })).not.toBeNull();

    rerender(<SettingsShell {...baseProps({ page, searchResults: null })} />);
    expect(document.querySelector(".settings-page-marker")).not.toBeNull();
    expect(document.querySelector(".poodle-settings-shell__results")).toBeNull();
  });

  it("keeps navigation and page in separately scrollable regions, outside the search and header", () => {
    injectStyles();
    render(<SettingsShell {...baseProps({ page: <p>Page content</p> })} />);

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

    const searchInput = document.querySelector(".poodle-settings-shell__search input") as HTMLElement;
    const pageHeader = document.querySelector(".poodle-settings-shell__page-header") as HTMLElement;
    expect(navViewport.contains(searchInput)).toBe(false);
    expect(pageViewport.contains(searchInput)).toBe(false);
    expect(navViewport.contains(pageHeader)).toBe(false);
    expect(pageViewport.contains(pageHeader)).toBe(false);
  });

  it("fires onNavigate with the page id, and with the anchor id when a result carries one", () => {
    const onNavigate = vi.fn();
    const { rerender } = render(<SettingsShell {...baseProps({ onNavigate })} />);

    fireEvent.click(screen.getByRole("button", { name: "General" }));
    expect(onNavigate).toHaveBeenCalledWith("general");

    const results = [
      { pageId: "storage", pageLabel: "Storage", anchorId: "disks", anchorLabel: "Disks" },
    ];
    rerender(<SettingsShell {...baseProps({ onNavigate, searchResults: results })} />);
    fireEvent.click(screen.getByRole("button", { name: /Storage/ }));
    expect(onNavigate).toHaveBeenCalledWith("storage", "disks");

    const anchorless = [{ pageId: "backup", pageLabel: "Backup" }];
    rerender(<SettingsShell {...baseProps({ onNavigate, searchResults: anchorless })} />);
    fireEvent.click(screen.getByRole("button", { name: "Backup" }));
    expect(onNavigate).toHaveBeenCalledWith("backup", null);
  });

  it("fires onRequestClose on a close attempt and stays open against a refusal", () => {
    const onRequestClose = vi.fn();
    render(
      <SettingsShell
        {...baseProps({
          closeRefusedReason: "Apply or discard this page before leaving.",
          onRequestClose,
        })}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: "Close settings" }));
    expect(onRequestClose).toHaveBeenCalledOnce();
    // The host never received a close — the shell refused on the reason.
    expect(screen.getByRole("dialog", { name: "Settings" })).toBeTruthy();
  });

  it("closes when the host does not refuse", () => {
    render(<Host {...baseProps()} />);

    fireEvent.click(screen.getByRole("button", { name: "Close settings" }));
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("renders a refused close as an announced warning callout, not an error", () => {
    render(
      <SettingsShell
        {...baseProps({ closeRefusedReason: "Apply or discard this page before leaving." })}
      />,
    );

    const callout = document.querySelector(".poodle-callout") as HTMLElement;
    expect(callout).not.toBeNull();
    expect(callout.getAttribute("data-tone")).toBe("warning");
    expect(callout.getAttribute("role")).toBe("status");
    expect(callout.getAttribute("aria-live")).toBe("polite");
    expect(callout.textContent).toContain("Apply or discard this page before leaving.");
    // Not an error treatment: nothing announces assertively.
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("renders the designed empty states for empty groups and empty results", () => {
    render(<SettingsShell {...baseProps({ groups: [] })} />);
    expect(screen.getByText("No settings pages")).toBeTruthy();
    expect(document.querySelector(".poodle-empty-state")).not.toBeNull();
    expect(document.querySelector(".poodle-sidebar-nav")).toBeNull();

    render(<SettingsShell {...baseProps({ searchResults: [] })} />);
    expect(screen.getByText("No results")).toBeTruthy();
    expect(screen.getByText("No settings match your search.")).toBeTruthy();
    expect(document.querySelector(".poodle-settings-shell__result-list")).toBeNull();
  });
});

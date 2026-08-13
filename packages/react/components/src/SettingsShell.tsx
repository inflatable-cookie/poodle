import "@inflatable-cookie/poodle-core/styles/settings-shell.css";

import { useMemo, useRef, useState, type ReactNode } from "react";

import { Callout } from "./Callout";
import { Dialog } from "./Dialog";
import { EmptyState } from "./EmptyState";
import { ScrollShell } from "./ScrollShell";
import { SidebarNav } from "./SidebarNav";
import { Surface } from "./Surface";
import { TextInput } from "./TextInput";

// Structural types, declared locally (R3): Poodle never imports Longhorn
// and never learns what a storage profile or a keymap is. These shapes are
// what the current host needs; a host maps its own domain onto them.
export interface SettingsNavGroup {
  id: string;
  label: string;
  items: { value: string; label: string }[];
}

export interface SettingsSearchResult {
  pageId: string;
  pageLabel: string;
  anchorId?: string;
  anchorLabel?: string;
}

export interface SettingsShellProps {
  groups?: SettingsNavGroup[];
  activePageId?: string | null;
  pageTitle?: string | null;
  searchQuery?: string;
  defaultSearchQuery?: string;
  searchResults?: SettingsSearchResult[] | null;
  open?: boolean | null;
  defaultOpen?: boolean;
  title?: string | null;
  closeLabel?: string;
  closeRefusedReason?: string | null;
  page?: ReactNode;
  onNavigate?: (pageId: string, anchorId?: string | null) => void;
  onRequestClose?: () => void;
  onOpenChange?: (open: boolean) => void;
  onSearchQueryChange?: (value: string) => void;
}

export function SettingsShell({
  groups = [],
  activePageId = null,
  pageTitle = null,
  searchQuery: controlledSearchQuery,
  defaultSearchQuery = "",
  searchResults = null,
  open = null,
  defaultOpen = false,
  title = "Settings",
  closeLabel = "Close settings",
  closeRefusedReason = null,
  page,
  onNavigate,
  onRequestClose,
  onOpenChange,
  onSearchQueryChange,
}: SettingsShellProps) {
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [uncontrolledQuery, setUncontrolledQuery] = useState(defaultSearchQuery);
  const nextSearchId = useRef(0);
  const searchId = useMemo(() => `poodle-settings-search-${nextSearchId.current++}`, []);

  const isOpen = open === null ? uncontrolledOpen : open;
  const searchQuery = controlledSearchQuery === undefined ? uncontrolledQuery : controlledSearchQuery;
  const isSearching = searchResults !== null;
  const hasResults = isSearching && (searchResults?.length ?? 0) > 0;

  function handleSearchChange(value: string): void {
    if (controlledSearchQuery === undefined) setUncontrolledQuery(value);
    onSearchQueryChange?.(value);
  }

  function handleOpenChange(next: boolean): void {
    // Refused close: the host's `closeRefusedReason` keeps the shell open.
    // The shell never closes itself against a refusal, and never invents a
    // reason of its own (R1.7).
    if (!next && closeRefusedReason) {
      return;
    }
    if (open === null) setUncontrolledOpen(next);
    onOpenChange?.(next);
  }

  function handleResultActivation(result: SettingsSearchResult): void {
    onNavigate?.(result.pageId, result.anchorId ?? null);
  }

  return (
    <Dialog
      open={isOpen}
      title={title}
      ariaLabel={title}
      width="xl"
      showCloseButton
      closeLabel={closeLabel}
      onRequestClose={onRequestClose}
      onOpenChange={handleOpenChange}
      header={
        /* Title, search and the dialog's own close read as one bar. Dialog's
           `__header-row` is the flex container and its close button is our
           sibling, so this node owns only the left-of-close span. */
        <div className="poodle-settings-shell__dialog-header">
          <strong className="poodle-settings-shell__dialog-title">{title}</strong>
          <div className="poodle-settings-shell__search">
            <TextInput
              id={searchId}
              type="search"
              placeholder="Search settings"
              ariaLabel="Search settings"
              value={searchQuery}
              showClearButton
              onValueChange={handleSearchChange}
            />
          </div>
        </div>
      }
    >
      <div className="poodle-settings-shell">
        <aside className="poodle-settings-shell__nav">
          <Surface tone="panel" border="subtle" padding="none">
            <ScrollShell direction="vertical">
              {groups.length === 0 ? (
                <EmptyState
                  variant="neutral"
                  size="compact"
                  title="No settings pages"
                  message="This scope has no settings pages yet."
                />
              ) : (
                <SidebarNav groups={groups} value={activePageId} ariaLabel="Settings pages" onValueChange={onNavigate} />
              )}
            </ScrollShell>
          </Surface>
        </aside>

        <div className="poodle-settings-shell__page">
          {closeRefusedReason ? (
            <div className="poodle-settings-shell__notice">
              <Callout tone="warning" announceMode="polite" message={closeRefusedReason} />
            </div>
          ) : null}

          {isSearching ? (
            <div className="poodle-settings-shell__results" data-empty={!hasResults}>
              {hasResults ? (
                <ScrollShell direction="vertical" padding="md">
                  <ul className="poodle-settings-shell__result-list" aria-label="Settings search results">
                    {(searchResults ?? []).map((result) => (
                      <li key={`${result.pageId}:${result.anchorId ?? ""}`}>
                        <button
                          type="button"
                          className="poodle-settings-shell__result"
                          onClick={() => handleResultActivation(result)}
                        >
                          <span className="poodle-settings-shell__result-label">{result.pageLabel}</span>
                          {result.anchorLabel ? <span className="poodle-settings-shell__result-anchor">{result.anchorLabel}</span> : null}
                        </button>
                      </li>
                    ))}
                  </ul>
                </ScrollShell>
              ) : (
                <EmptyState
                  variant="search"
                  size="compact"
                  title="No results"
                  message="No settings match your search."
                />
              )}
            </div>
          ) : (
            // No visible page heading or description: the nav rail already
            // names the current page, and the page snippet owns its own intro.
            // `pageTitle` becomes this region's accessible name instead, so the
            // name survives for screen readers without being drawn.
            <section className="poodle-settings-shell__page-stack" aria-label={pageTitle ?? undefined}>
              <ScrollShell direction="vertical" padding="md">
                {page}
              </ScrollShell>
            </section>
          )}
        </div>
      </div>
    </Dialog>
  );
}

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

export interface SettingsShellProps {
  groups?: SettingsNavGroup[];
  activePageId?: string | null;
  pageTitle?: string | null;
  searchQuery?: string;
  defaultSearchQuery?: string;
  open?: boolean | null;
  defaultOpen?: boolean;
  title?: string | null;
  /* The dialog's accessible name. Defaults to `title`, but hosts want them
     different: every app's visible title is "Settings", while a screen-reader
     user with several windows open needs "Nucleus settings" to tell them
     apart. */
  ariaLabel?: string | null;
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
  open = null,
  defaultOpen = false,
  title = "Settings",
  ariaLabel = null,
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
  const dialogName = ariaLabel ?? title;
  /* Search narrows the rail; the host filters `groups` (only it knows that a
     query can match an anchor inside a page). The shell just needs to know a
     query is live, so an empty rail reads as "no matches" not "no pages". */
  const isFiltering = searchQuery.trim().length > 0;

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

  return (
    <Dialog
      open={isOpen}
      title={title}
      ariaLabel={dialogName}
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
              {groups.length === 0 && isFiltering ? (
                /* A query is live and nothing matched. "No settings pages"
                   would be wrong here — there are pages, none match. */
                <EmptyState
                  variant="search"
                  size="compact"
                  title="No matches"
                  message="No settings match your search."
                />
              ) : groups.length === 0 ? (
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

          {/* The page always renders. Search narrows the rail rather than
              replacing the page, so what you were reading stays put while you
              filter. No visible page heading either: the rail already names the
              current page, and the node owns its own intro. `pageTitle` is this
              region's accessible name only. */}
          <section className="poodle-settings-shell__page-stack" aria-label={pageTitle ?? undefined}>
            <ScrollShell direction="vertical" padding="md">
              {page}
            </ScrollShell>
          </section>
        </div>
      </div>
    </Dialog>
  );
}

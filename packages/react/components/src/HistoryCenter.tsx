import { useEffect, useRef, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/history-center.css";
import {
  historyCenterKeydownEvent,
  historyCenterRows,
  historyCenterTransition,
  isForkPoint,
  trapFocusKeydown,
  type HistoryCenterContext,
  type HistoryCenterEvent,
  type HistoryCenterRow,
} from "@inflatable-cookie/poodle-core";

import { Button } from "./Button";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { Popover } from "./Popover";
import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  HistoryBranch,
  HistoryEntry,
  HistoryStatus,
  OverlayPlacement,
  SemanticControlSizeRole,
} from "./types";

export interface HistoryCenterProps {
  entries?: HistoryEntry[];
  totalEntries?: number;
  hasMoreEntries?: boolean;
  branches?: HistoryBranch[] | null;
  totalBranches?: number;
  hasMoreBranches?: boolean;
  canUndo?: boolean;
  canRedo?: boolean;
  busy?: boolean;
  status?: HistoryStatus;
  statusMessage?: string | null;
  rejection?: string | null;
  maxBranchNameBytes?: number;
  open?: boolean | null;
  defaultOpen?: boolean;
  placement?: OverlayPlacement;
  undoLabel?: string;
  redoLabel?: string;
  listLabel?: string;
  title?: string;
  emptyMessage?: string;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onUndo?: (() => void) | null;
  onRedo?: (() => void) | null;
  onOpenChange?: ((open: boolean) => void) | null;
  onSelectEntry?: ((id: string) => void) | null;
  onCheckout?: ((branchId: string, entryId: string) => void) | null;
  onRenameBranch?: ((branchId: string, name: string) => void) | null;
  onLoadMoreEntries?: ((offset: number) => void) | null;
  onLoadMoreBranches?: ((offset: number) => void) | null;
}

export function HistoryCenter({
  entries = [],
  totalEntries = 0,
  hasMoreEntries = false,
  branches = null,
  totalBranches = 0,
  hasMoreBranches = false,
  canUndo = false,
  canRedo = false,
  busy = false,
  status = "idle",
  statusMessage = null,
  rejection = null,
  maxBranchNameBytes = 256,
  open = null,
  defaultOpen = false,
  placement = "bottom-end",
  undoLabel = "Undo",
  redoLabel = "Redo",
  listLabel = "History",
  title = "History",
  emptyMessage = "No history entries yet.",
  ariaLabel = null,
  size = null,
  sizeRole = "chrome",
  density = null,
  onUndo = null,
  onRedo = null,
  onOpenChange = null,
  onSelectEntry = null,
  onCheckout = null,
  onRenameBranch = null,
  onLoadMoreEntries = null,
  onLoadMoreBranches = null,
}: HistoryCenterProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [focusIndex, setFocusIndex] = useState(0);
  const [expandedBranchIds, setExpandedBranchIds] = useState<string[]>([]);
  const [displayedRejection, setDisplayedRejection] = useState<string | null>(null);
  const [renamingBranchId, setRenamingBranchId] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const sectionRef = useRef<HTMLElement | null>(null);
  const listRef = useRef<HTMLUListElement | null>(null);
  const renameInputRef = useRef<HTMLInputElement | null>(null);
  const lastRejectionProp = useRef<string | null>(null);

  const isOpen = open === null ? uncontrolledOpen : open;
  const hasBranches = branches !== null;
  const branchList = branches ?? [];
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const rows = historyCenterRows(entries, branches, expandedBranchIds);

  const machineContext: HistoryCenterContext = {
    entries,
    branches,
    expandedBranchIds,
    focusIndex,
    rejection: displayedRejection,
  };

  const sendRef = useRef<(event: HistoryCenterEvent) => void>(() => {});
  sendRef.current = (event: HistoryCenterEvent) => {
    const result = historyCenterTransition(isOpen ? "open" : "closed", machineContext, event);
    setFocusIndex(result.context.focusIndex);
    setExpandedBranchIds(result.context.expandedBranchIds);
    setDisplayedRejection(result.context.rejection);

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitOpenChange":
          if (open === null) setUncontrolledOpen(effect.open);
          onOpenChange?.(effect.open);
          break;
        case "focusRow":
          // Executed by the focusIndex effect below, which runs after commit.
          break;
        case "emitSelectEntry":
          onSelectEntry?.(effect.id);
          break;
        case "emitCheckout":
          onCheckout?.(effect.branchId, effect.entryId);
          break;
        case "emitRenameBranch":
          onRenameBranch?.(effect.branchId, effect.name);
          break;
      }
    }
  };

  // Transient rejection: a *new* non-null prop value displays; dismissal is
  // local and never re-shows the same value. The host clearing the prop
  // clears the notice.
  useEffect(() => {
    if (rejection === lastRejectionProp.current) return;
    lastRejectionProp.current = rejection;
    sendRef.current(rejection === null ? { type: "DISMISS_REJECTION" } : { type: "SHOW_REJECTION", message: rejection });
  }, [rejection]);

  // Roving row focus follows the machine's focusIndex after commit.
  useEffect(() => {
    const rowEl = listRef.current?.querySelector<HTMLElement>(`[data-row-index="${focusIndex}"]`);
    (rowEl?.querySelector<HTMLElement>("button, input") ?? rowEl)?.focus();
  }, [focusIndex]);

  // Inline rename input takes focus and selects its content when it appears.
  useEffect(() => {
    if (renamingBranchId !== null && renameInputRef.current) {
      renameInputRef.current.focus();
      renameInputRef.current.select();
    }
  }, [renamingBranchId]);

  function handleOpenChange(next: boolean): void {
    sendRef.current(next ? { type: "OPEN" } : { type: "CLOSE" });
  }

  function handleListKeydown(event: React.KeyboardEvent): void {
    if (event.target instanceof HTMLInputElement) {
      return;
    }

    const machineEvent = historyCenterKeydownEvent(event.key);

    if (machineEvent === null) {
      return;
    }

    event.preventDefault();

    if (machineEvent.type === "ACTIVATE_ROW") {
      const rowEl = (event.target as HTMLElement).closest<HTMLElement>("[data-row-index]");

      if (rowEl) {
        sendRef.current({ type: "ACTIVATE_ROW", index: Number(rowEl.dataset.rowIndex) });
      }

      return;
    }

    sendRef.current(machineEvent);
  }

  function handleSurfaceKeydown(event: React.KeyboardEvent): void {
    trapFocusKeydown(sectionRef.current, event.nativeEvent);
  }

  function handleRowClick(index: number): void {
    sendRef.current({ type: "ACTIVATE_ROW", index });
  }

  function toggleFork(entryId: string): void {
    sendRef.current({ type: "TOGGLE_BRANCHES", entryId });
  }

  function dismissRejection(): void {
    sendRef.current({ type: "DISMISS_REJECTION" });
  }

  function startRename(branch: HistoryBranch): void {
    setRenamingBranchId(branch.id);
    setRenameValue(branch.name ?? branch.id);
  }

  function commitRename(branchId: string): void {
    if (renamingBranchId !== branchId) {
      return;
    }

    sendRef.current({ type: "RENAME", branchId, name: renameValue });
    setRenamingBranchId(null);
  }

  function cancelRename(): void {
    setRenamingBranchId(null);
  }

  function handleRenameKeydown(event: React.KeyboardEvent, branchId: string): void {
    if (event.key === "Enter") {
      event.preventDefault();
      commitRename(branchId);
    } else if (event.key === "Escape") {
      event.preventDefault();
      cancelRename();
    }
  }

  function rowKey(row: HistoryCenterRow): string {
    return row.kind === "entry" ? `entry:${row.entry.id}` : `branch:${row.branch.id}`;
  }

  function branchRowIndex(rows: HistoryCenterRow[], branch: HistoryBranch): number {
    return rows.findIndex((candidate) => candidate.kind === "branch" && candidate.branch.id === branch.id);
  }

  const entryTotal = `${totalEntries} ${totalEntries === 1 ? "entry" : "entries"}`;
  const branchTotal = `${totalBranches} ${totalBranches === 1 ? "branch" : "branches"}`;
  const summary = hasBranches ? `${entryTotal} · ${branchTotal}` : entryTotal;

  return (
    <div className="poodle-history-center-popover">
      <span className="poodle-history-center__trigger">
        <IconButton
          icon="undo"
          ariaLabel={undoLabel}
          tooltip={undoLabel}
          variant="ghost"
          size={resolvedSize}
          density={resolvedDensity}
          disabled={!canUndo || busy}
          onClick={onUndo}
        />

        <Popover
          open={isOpen}
          placement={placement}
          initialFocus="first-focusable"
          triggerIsInteractive
          ariaLabel={ariaLabel ?? title}
          surfaceMinWidth="min(28rem, calc(100vw - 2rem))"
          surfaceMaxWidth="min(38rem, calc(100vw - 2rem))"
          onOpenChange={handleOpenChange}
          trigger={
            <IconButton
              icon="list"
              ariaLabel={listLabel}
              tooltip={listLabel}
              variant="ghost"
              size={resolvedSize}
              density={resolvedDensity}
              expanded={isOpen}
            />
          }
        >
          <section
            ref={sectionRef}
            className="poodle-history-center"
            data-size={resolvedSize}
            data-density={resolvedDensity}
            aria-label={ariaLabel ?? title}
            onKeyDown={handleSurfaceKeydown}
          >
            <header className="poodle-history-center__header">
              <h2>{title}</h2>
              <p>{summary}</p>
            </header>

            {displayedRejection !== null ? (
              <div className="poodle-history-center__rejection" role="status">
                <p>{displayedRejection}</p>
                <IconButton
                  icon="circle-x"
                  ariaLabel="Dismiss"
                  tooltip="Dismiss"
                  variant="ghost"
                  tone="danger"
                  size="xs"
                  density={resolvedDensity}
                  onClick={dismissRejection}
                />
              </div>
            ) : null}

            {entries.length === 0 ? (
              status === "loading" ? (
                <div className="poodle-history-center__loading" role="status">
                  <Spinner variant="ring" size="sm" tone="muted" />
                  <span>{statusMessage ?? "Loading history…"}</span>
                </div>
              ) : (
                <div className="poodle-history-center__empty">
                  <EmptyState title={title} message={emptyMessage} size="compact" />
                </div>
              )
            ) : (
              <>
                <ul
                  ref={listRef}
                  className="poodle-history-center__list"
                  aria-label={listLabel}
                  onKeyDown={handleListKeydown}
                >
                  {rows.map((row) =>
                    row.kind === "entry" ? (
                      <li
                        key={rowKey(row)}
                        data-row-index={row.index}
                        data-part="entry"
                        data-position={row.entry.position}
                        data-checkpoint={hasBranches && row.entry.checkpoint === true ? "true" : undefined}
                        data-fork={hasBranches && isForkPoint(row.entry) ? "true" : undefined}
                      >
                        <div className="poodle-history-center__item-row">
                          <button
                            type="button"
                            className="poodle-history-center__item-content"
                            tabIndex={focusIndex === row.index ? 0 : -1}
                            onClick={() => handleRowClick(row.index)}
                          >
                            {hasBranches && row.entry.checkpoint ? (
                              <span className="poodle-history-center__pin">
                                <Icon icon="git-commit-horizontal" size={resolvedSize} />
                              </span>
                            ) : (
                              <span className="poodle-history-center__position-marker" data-position={row.entry.position} />
                            )}
                            <span className="poodle-history-center__item-copy">
                              <span className="poodle-history-center__item-label">{row.entry.label}</span>
                              {row.entry.groupId ? (
                                <span className="poodle-history-center__item-meta">{row.entry.groupId}</span>
                              ) : null}
                            </span>
                          </button>

                          {hasBranches && isForkPoint(row.entry) ? (
                            <button
                              type="button"
                              className="poodle-history-center__fork-indicator"
                              aria-label={
                                expandedBranchIds.includes(row.entry.id)
                                  ? `Collapse branches at ${row.entry.label}`
                                  : `Show branches at ${row.entry.label}`
                              }
                              aria-expanded={expandedBranchIds.includes(row.entry.id)}
                              tabIndex={focusIndex === row.index ? 0 : -1}
                              onClick={() => toggleFork(row.entry.id)}
                            >
                              <Icon
                                icon={expandedBranchIds.includes(row.entry.id) ? "chevron-down" : "chevron-right"}
                                size="xs"
                              />
                            </button>
                          ) : null}
                        </div>

                        {expandedBranchIds.includes(row.entry.id) ? (
                          <ul className="poodle-history-center__branches">
                            {branchList.map((branch) => (
                              <li
                                key={branch.id}
                                data-row-index={branchRowIndex(rows, branch)}
                                data-part="branch"
                                data-current={branch.current ? "true" : undefined}
                              >
                                {renamingBranchId === branch.id ? (
                                  <input
                                    ref={renameInputRef}
                                    className="poodle-history-center__rename-input"
                                    aria-label={`Rename branch ${branch.name ?? branch.id}`}
                                    maxLength={maxBranchNameBytes}
                                    value={renameValue}
                                    onChange={(event) => setRenameValue(event.target.value)}
                                    onKeyDown={(event) => handleRenameKeydown(event, branch.id)}
                                    onBlur={() => commitRename(branch.id)}
                                  />
                                ) : (
                                  <div className="poodle-history-center__branch-actions">
                                    <button
                                      type="button"
                                      className="poodle-history-center__branch-content"
                                      tabIndex={focusIndex === branchRowIndex(rows, branch) ? 0 : -1}
                                      onClick={() => handleRowClick(branchRowIndex(rows, branch))}
                                    >
                                      <span className="poodle-history-center__branch-name">{branch.name ?? branch.id}</span>
                                      {branch.entryCount !== undefined ? (
                                        <span className="poodle-history-center__branch-count">
                                          {branch.entryCount} {branch.entryCount === 1 ? "entry" : "entries"}
                                        </span>
                                      ) : null}
                                      {branch.current ? (
                                        <span className="poodle-history-center__branch-current-badge">Current</span>
                                      ) : null}
                                    </button>
                                    <IconButton
                                      icon="edit"
                                      ariaLabel={`Rename ${branch.name ?? branch.id}`}
                                      tooltip="Rename branch"
                                      variant="ghost"
                                      size="xs"
                                      density={resolvedDensity}
                                      onClick={() => startRename(branch)}
                                    />
                                  </div>
                                )}
                              </li>
                            ))}
                          </ul>
                        ) : null}
                      </li>
                    ) : null,
                  )}
                </ul>

                {status === "loading" ? (
                  <div className="poodle-history-center__loading" role="status">
                    <Spinner variant="ring" size="sm" tone="muted" />
                    <span>{statusMessage ?? "Loading history…"}</span>
                  </div>
                ) : null}

                {hasMoreEntries && onLoadMoreEntries ? (
                  <div className="poodle-history-center__load-more">
                    <Button
                      variant="ghost"
                      size="xs"
                      density={resolvedDensity}
                      onClick={() => onLoadMoreEntries?.(entries.length)}
                    >
                      Load more entries
                    </Button>
                  </div>
                ) : null}

                {hasBranches && hasMoreBranches && onLoadMoreBranches ? (
                  <div className="poodle-history-center__load-more">
                    <Button
                      variant="ghost"
                      size="xs"
                      density={resolvedDensity}
                      onClick={() => onLoadMoreBranches?.(branchList.length)}
                    >
                      Load more branches
                    </Button>
                  </div>
                ) : null}
              </>
            )}
          </section>
        </Popover>

        <IconButton
          icon="redo"
          ariaLabel={redoLabel}
          tooltip={redoLabel}
          variant="ghost"
          size={resolvedSize}
          density={resolvedDensity}
          disabled={!canRedo || busy}
          onClick={onRedo}
        />
      </span>
    </div>
  );
}

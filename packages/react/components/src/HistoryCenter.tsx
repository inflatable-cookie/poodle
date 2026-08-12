import { useEffect, useRef, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/history-center.css";
import {
  historyCenterKeydownEvent,
  historyCenterRows,
  historyCenterTransition,
  trapFocusKeydown,
  type HistoryBranch,
  type HistoryCenterContext,
  type HistoryCenterEvent,
  type HistoryCenterRow,
  type HistoryEntry,
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
  HistoryStatus,
  OverlayPlacement,
  SemanticControlSizeRole,
} from "./types";

export interface HistoryCenterProps {
  /** Branch records in supplied order. `null` disables the tree (no rows,
   *  every row event inert). */
  branches?: HistoryBranch[] | null;
  /** Per-branch entry path (`branchId` -> root-to-head entries). `null`
   *  when `branches` is null. */
  paths?: Record<string, HistoryEntry[]> | null;
  totalEntries?: number;
  totalBranches?: number;
  hasMoreEntries?: boolean;
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
  /** Entry activation; always the entry actually clicked, on the branch
   *  that owns its run. */
  onNavigateEntry?: ((branchId: string, entryId: string) => void) | null;
  onRenameBranch?: ((branchId: string, name: string) => void) | null;
  onLoadMoreEntries?: ((offset: number) => void) | null;
  onLoadMoreBranches?: ((offset: number) => void) | null;
}

type EntryRow = Extract<HistoryCenterRow, { kind: "entry" }>;
type CaptionRow = Extract<HistoryCenterRow, { kind: "caption" }>;

export function HistoryCenter({
  branches = null,
  paths = null,
  totalEntries = 0,
  totalBranches = 0,
  hasMoreEntries = false,
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
  onNavigateEntry = null,
  onRenameBranch = null,
  onLoadMoreEntries = null,
  onLoadMoreBranches = null,
}: HistoryCenterProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [focusIndex, setFocusIndex] = useState(0);
  const [displayedRejection, setDisplayedRejection] = useState<string | null>(null);
  const [renamingBranchId, setRenamingBranchId] = useState<string | null>(null);
  const [renamingIndex, setRenamingIndex] = useState(-1);
  const [renameValue, setRenameValue] = useState("");
  const sectionRef = useRef<HTMLElement | null>(null);
  const listRef = useRef<HTMLUListElement | null>(null);
  const renameInputRef = useRef<HTMLInputElement | null>(null);
  const lastRejectionProp = useRef<string | null>(null);
  const pendingFocusRestore = useRef<number | null>(null);

  const isOpen = open === null ? uncontrolledOpen : open;
  const hasBranches = branches !== null;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const rows = historyCenterRows(branches, paths);
  const entryRows = rows.filter((row): row is EntryRow => row.kind === "entry");
  const entryRowCount = entryRows.length;
  // The data's own "present": the newest authority-supplied timestamp. The
  // caption relative time is derived from supplied data (ruling D2) — there
  // is no clock and no `Date.now()` anywhere.
  const newestRecordedAt = entryRows.reduce<number | undefined>((newest, row) => {
    const at = row.entry.recordedAtMs;
    return at !== undefined && (newest === undefined || at > newest) ? at : newest;
  }, undefined);

  const machineContext: HistoryCenterContext = {
    branches,
    paths,
    focusIndex,
    rejection: displayedRejection,
  };

  const sendRef = useRef<(event: HistoryCenterEvent) => void>(() => {});
  sendRef.current = (event: HistoryCenterEvent) => {
    const result = historyCenterTransition(isOpen ? "open" : "closed", machineContext, event);
    setFocusIndex(result.context.focusIndex);
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
        case "emitNavigateEntry":
          onNavigateEntry?.(effect.branchId, effect.entryId);
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
  // Deferred a microtask past the Popover's open-focus effect: focusing inside
  // the same commit lets the popover's initial focus land on the input's
  // focus window, blur it, and spuriously blur-commit the rename.
  useEffect(() => {
    if (renamingBranchId !== null && renameInputRef.current) {
      const input = renameInputRef.current;
      const frame = requestAnimationFrame(() => {
        input.focus();
        input.select();
      });
      return () => cancelAnimationFrame(frame);
    }
  }, [renamingBranchId]);

  // After a rename commits or cancels, the input unmounts — return focus to
  // the caption's rename button so keyboard users stay anchored to the row.
  useEffect(() => {
    if (pendingFocusRestore.current === null) return;
    const index = pendingFocusRestore.current;
    pendingFocusRestore.current = null;
    listRef.current?.querySelector<HTMLElement>(`[data-row-index="${index}"] [data-rename-button]`)?.focus();
  });

  function handleOpenChange(next: boolean): void {
    sendRef.current(next ? { type: "OPEN" } : { type: "CLOSE" });
  }

  function handleListKeydown(event: React.KeyboardEvent): void {
    // The rename input owns its keys (commit/cancel). The caption rename
    // button keeps native activation for Enter/Space (start a rename instead
    // of roving-row activation), but arrows still drive roving focus.
    if (event.target instanceof HTMLInputElement) {
      return;
    }

    if (
      event.target instanceof HTMLElement &&
      event.target.closest("[data-rename-button]") !== null &&
      (event.key === "Enter" || event.key === " ")
    ) {
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

  function dismissRejection(): void {
    sendRef.current({ type: "DISMISS_REJECTION" });
  }

  function startRename(branch: HistoryBranch, index: number): void {
    setRenamingBranchId(branch.id);
    setRenamingIndex(index);
    setRenameValue(branch.name ?? branch.id);
  }

  function finishRename(): void {
    pendingFocusRestore.current = renamingIndex;
    setRenamingBranchId(null);
    setRenamingIndex(-1);
  }

  function commitRename(branchId: string): void {
    if (renamingBranchId !== branchId) {
      return;
    }

    sendRef.current({ type: "RENAME", branchId, name: renameValue });
    finishRename();
  }

  function cancelRename(): void {
    finishRename();
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
    return row.kind === "entry" ? `entry:${row.entry.id}` : `caption:${row.branch.id}`;
  }

  // The run's own lane shape (drawn by the renderer from the row's lane
  // metadata; the stitcher's flags are always true, only depth saturates).
  function laneKind(row: EntryRow): string {
    if (row.depth === 0) {
      return "trunk";
    }
    if (row.lane.start && row.lane.end) {
      return "single";
    }
    if (row.lane.start) {
      return "elbow";
    }
    if (row.lane.end) {
      return "end";
    }
    return "continue";
  }

  // Short-form relative time, derived purely from supplied timestamps. The
  // reference is the data's newest recordedAtMs — stable across renders and
  // runtimes, no clock.
  function formatRelativeTime(diffMs: number): string {
    const seconds = Math.round(Math.max(0, diffMs) / 1000);

    if (seconds < 60) {
      return "just now";
    }

    const minutes = Math.round(seconds / 60);

    if (minutes < 60) {
      return `${minutes}m ago`;
    }

    const hours = Math.round(minutes / 60);

    if (hours < 24) {
      return `${hours}h ago`;
    }

    const days = Math.round(hours / 24);

    if (days < 7) {
      return `${days}d ago`;
    }

    return `${Math.round(days / 7)}w ago`;
  }

  // A run caption takes its relative time from its own run's most recent
  // entry — the run's head (its last stitched entry row) — and renders no
  // time at all when the field is absent (ruling D2).
  function captionMeta(row: CaptionRow): { count: number; time: string | null } {
    let count = 0;
    let runHead: EntryRow | undefined;

    for (const candidate of entryRows) {
      if (candidate.branchId === row.branch.id) {
        count += 1;
        runHead = candidate;
      }
    }

    const at = runHead?.entry.recordedAtMs;
    const time =
      at !== undefined && newestRecordedAt !== undefined ? formatRelativeTime(newestRecordedAt - at) : null;

    return { count, time };
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

            {rows.length === 0 ? (
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
                        className="poodle-history-center__row"
                        data-row-index={row.index}
                        data-part="entry"
                        data-depth={row.depth}
                        data-position={row.entry.position}
                        data-checkpoint={row.entry.checkpoint === true ? "true" : undefined}
                        aria-level={row.depth + 1}
                      >
                        <span className="poodle-history-center__lanes" aria-hidden="true">
                          {Array.from({ length: row.depth }, (_, level) => (
                            <span key={level} className="poodle-history-center__lane" data-lane="ancestor" />
                          ))}
                          <span className="poodle-history-center__lane" data-lane={laneKind(row)} />
                        </span>
                        <button
                          type="button"
                          className="poodle-history-center__entry-content"
                          tabIndex={focusIndex === row.index ? 0 : -1}
                          onClick={() => handleRowClick(row.index)}
                        >
                          {row.entry.checkpoint === true ? (
                            <span className="poodle-history-center__pin">
                              <Icon icon="git-commit-horizontal" size={resolvedSize} />
                            </span>
                          ) : (
                            <span className="poodle-history-center__position-marker" data-position={row.entry.position} />
                          )}
                          <span className="poodle-history-center__entry-copy">
                            <span className="poodle-history-center__entry-label">{row.entry.label}</span>
                            {row.entry.groupId ? (
                              <span className="poodle-history-center__entry-meta">{row.entry.groupId}</span>
                            ) : null}
                          </span>
                        </button>
                      </li>
                    ) : (
                      (() => {
                        const meta = captionMeta(row);
                        return (
                          <li
                            key={rowKey(row)}
                            className="poodle-history-center__row"
                            data-row-index={row.index}
                            data-part="caption"
                            data-depth={row.depth}
                            data-current={row.branch.current ? "true" : undefined}
                            aria-level={row.depth + 1}
                          >
                            <span className="poodle-history-center__lanes" aria-hidden="true">
                              {Array.from({ length: row.depth }, (_, level) => (
                                <span key={level} className="poodle-history-center__lane" data-lane="ancestor" />
                              ))}
                              <span className="poodle-history-center__lane" data-lane="caption" />
                            </span>
                            {renamingBranchId === row.branch.id ? (
                              <input
                                ref={renameInputRef}
                                className="poodle-history-center__rename-input"
                                aria-label={`Rename branch ${row.branch.name ?? row.branch.id}`}
                                maxLength={maxBranchNameBytes}
                                value={renameValue}
                                onChange={(event) => setRenameValue(event.target.value)}
                                onKeyDown={(event) => handleRenameKeydown(event, row.branch.id)}
                                onBlur={() => commitRename(row.branch.id)}
                              />
                            ) : (
                              <>
                                <span className="poodle-history-center__caption-copy">
                                  <span className="poodle-history-center__caption-name">
                                    {row.branch.name ?? row.branch.id}
                                  </span>
                                  <span className="poodle-history-center__caption-meta">
                                    {meta.count} {meta.count === 1 ? "entry" : "entries"}
                                    {meta.time !== null ? ` · ${meta.time}` : ""}
                                  </span>
                                  {row.branch.current ? (
                                    <span className="poodle-history-center__branch-current-badge">Current</span>
                                  ) : null}
                                </span>
                                <button
                                  type="button"
                                  className="poodle-history-center__caption-rename"
                                  data-rename-button="true"
                                  aria-label={`Rename ${row.branch.name ?? row.branch.id}`}
                                  title="Rename branch"
                                  tabIndex={focusIndex === row.index ? 0 : -1}
                                  onClick={() => startRename(row.branch, row.index)}
                                >
                                  <Icon icon="edit" size="xs" />
                                </button>
                              </>
                            )}
                          </li>
                        );
                      })()
                    ),
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
                      onClick={() => onLoadMoreEntries?.(entryRowCount)}
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
                      onClick={() => onLoadMoreBranches?.(branches?.length ?? 0)}
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

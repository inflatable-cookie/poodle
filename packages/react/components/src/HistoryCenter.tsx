import { useEffect, useRef, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/history-center.css";
import {
  historyCenterKeydownEvent,
  historyCenterTransition,
  historyCenterVisibleRows,
  trapFocusKeydown,
  type HistoryCenterContext,
  type HistoryCenterEvent,
  type HistoryCenterOpenFork,
  type HistoryCenterRejectionCode,
  type HistoryCenterRow,
  type HistoryCenterRowId,
  type HistoryContinuation,
  type HistoryPathPage,
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
  /** Root path pages in fetch order (newest page first). `null` disables
   *  the list: no rows render and every row event is inert. */
  pages?: HistoryPathPage[] | null;
  canUndo?: boolean;
  canRedo?: boolean;
  busy?: boolean;
  status?: HistoryStatus;
  statusMessage?: string | null;
  /** A rejection code the host's bridge mapped from the protocol; the
   *  component owns the display copy. `null` clears the notice. */
  rejection?: HistoryCenterRejectionCode | null;
  /** Host op 1 result: the continuations at an anchor, fed back after
   *  `onLoadContinuations`. Diffed by reference; a new non-null value
   *  dispatches CONTINUATIONS_LOADED. */
  continuationsResult?: { entryId: string; continuations: HistoryContinuation[] } | null;
  /** Host op 2 result: a continuation run's pages (fetch order), fed back
   *  after `onLoadContinuationRun`. Diffed by reference; a new non-null
   *  value dispatches RUN_LOADED. */
  runResult?: { fromEntryId: string; pages: HistoryPathPage[] } | null;
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
   *  that owns its run (`null` on the spine — the host's own branch). */
  onNavigateEntry?: ((branchId: string | null, entryId: string) => void) | null;
  onRenameBranch?: ((branchId: string, name: string) => void) | null;
  /** Host op 1: load the continuations at the anchor entry. */
  onLoadContinuations?: ((entryId: string) => void) | null;
  /** Host op 2: load the run starting at the fork's first entry. */
  onLoadContinuationRun?: ((fromEntryId: string) => void) | null;
  /** Host op 3: prefer the picked continuation as the redo target. */
  onPreferContinuation?: ((entryId: string) => void) | null;
}

type EntryRow = Extract<HistoryCenterRow, { kind: "entry" }>;

export function HistoryCenter({
  pages = null,
  canUndo = false,
  canRedo = false,
  busy = false,
  status = "idle",
  statusMessage = null,
  rejection = null,
  continuationsResult = null,
  runResult = null,
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
  onLoadContinuations = null,
  onLoadContinuationRun = null,
  onPreferContinuation = null,
}: HistoryCenterProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  // Machine-owned disclosure tree: holds only what is open (R5); dropped on
  // close by the machine's CLOSE transition.
  const [openForks, setOpenForks] = useState<ReadonlyMap<string, HistoryCenterOpenFork> | null>(null);
  // Machine-owned roving focus identity over the visible rows.
  const [focusedRow, setFocusedRow] = useState<HistoryCenterRowId | null>(null);
  const [displayedRejection, setDisplayedRejection] = useState<string | null>(null);
  const [renamingBranchId, setRenamingBranchId] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const sectionRef = useRef<HTMLElement | null>(null);
  const listRef = useRef<HTMLUListElement | null>(null);
  const renameInputRef = useRef<HTMLInputElement | null>(null);
  const lastRejectionProp = useRef<HistoryCenterRejectionCode | null>(null);
  const lastContinuationsResult = useRef<typeof continuationsResult>(null);
  const lastRunResult = useRef<typeof runResult>(null);
  const pendingFocusRestore = useRef<string | null>(null);
  // Set when the machine emits a focusRow effect; the focusedRow effect below
  // only moves DOM focus then — a clampFocus identity change (e.g. after
  // DISCLOSE) must not steal focus from the disclosure button the user used.
  const focusRequestedRef = useRef(false);

  const isOpen = open === null ? uncontrolledOpen : open;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  // R1: ONE loop over the flat visible-row derivation. No recursion, no
  // nested component. depth drives padding and nothing else.
  const rows = historyCenterVisibleRows(pages, openForks);
  const entryRows = rows.filter((row): row is EntryRow => row.kind === "entry");
  const entryTotal = `${entryRows.length} ${entryRows.length === 1 ? "entry" : "entries"}`;
  // The data's own "present": the newest authority-supplied timestamp. The
  // run-header relative time is derived from supplied data only (ruling D2) —
  // there is no clock and no `Date.now()` anywhere.
  const newestRecordedAt = entryRows.reduce<number | undefined>((newest, row) => {
    const at = row.entry.recordedAtMs;
    return at !== undefined && (newest === undefined || at > newest) ? at : newest;
  }, undefined);

  const machineContext: HistoryCenterContext = {
    pages,
    open: openForks,
    focusRow: focusedRow,
    rejection: displayedRejection,
  };

  const sendRef = useRef<(event: HistoryCenterEvent) => void>(() => {});
  sendRef.current = (event: HistoryCenterEvent) => {
    const result = historyCenterTransition(isOpen ? "open" : "closed", machineContext, event);
    setOpenForks(result.context.open);
    setFocusedRow(result.context.focusRow);
    setDisplayedRejection(result.context.rejection);

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitOpenChange":
          if (open === null) setUncontrolledOpen(effect.open);
          onOpenChange?.(effect.open);
          break;
        case "focusRow":
          focusRequestedRef.current = true;
          break;
        case "emitNavigateEntry":
          onNavigateEntry?.(effect.branchId, effect.entryId);
          break;
        case "emitRenameBranch":
          onRenameBranch?.(effect.branchId, effect.name);
          break;
        case "loadContinuations":
          onLoadContinuations?.(effect.entryId);
          break;
        case "loadContinuationRun":
          onLoadContinuationRun?.(effect.fromEntryId);
          break;
        case "preferContinuation":
          onPreferContinuation?.(effect.entryId);
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
    sendRef.current(rejection === null ? { type: "DISMISS_REJECTION" } : { type: "SHOW_REJECTION", code: rejection });
  }, [rejection]);

  // Host op results: the host resolves a callback and feeds the result back.
  // Diffed by reference — a *new* non-null value dispatches the matching
  // loaded event; null or the same reference does nothing.
  useEffect(() => {
    if (continuationsResult === lastContinuationsResult.current) return;
    lastContinuationsResult.current = continuationsResult;
    if (continuationsResult !== null) {
      sendRef.current({
        type: "CONTINUATIONS_LOADED",
        entryId: continuationsResult.entryId,
        continuations: continuationsResult.continuations,
      });
    }
  }, [continuationsResult]);

  useEffect(() => {
    if (runResult === lastRunResult.current) return;
    lastRunResult.current = runResult;
    if (runResult !== null) {
      sendRef.current({ type: "RUN_LOADED", fromEntryId: runResult.fromEntryId, pages: runResult.pages });
    }
  }, [runResult]);

  // Roving row focus follows the machine's identity after commit — but only
  // when the machine asked for it (FOCUS_MOVE): clampFocus identity changes
  // after a disclosure toggle must not steal focus from the disclosure button.
  useEffect(() => {
    if (focusedRow === null || !focusRequestedRef.current) return;
    focusRequestedRef.current = false;
    const rowEl = listRef.current?.querySelector<HTMLElement>(rowSelector(focusedRow));
    (rowEl?.querySelector<HTMLElement>(
      ".poodle-history-center__entry-content, .poodle-history-center__picker, .poodle-history-center__not-yet-loaded",
    ) ?? rowEl)?.focus();
  }, [focusedRow]);

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
  // the run header's rename button so keyboard users stay anchored to the row.
  useEffect(() => {
    if (pendingFocusRestore.current === null) return;
    const branchId = pendingFocusRestore.current;
    pendingFocusRestore.current = null;
    listRef.current?.querySelector<HTMLElement>(
      `[data-part="run-header-rename"][data-branch="${CSS.escape(branchId)}"]`,
    )?.focus();
  });

  // ── Row identity helpers (R1) ─────────────────────────────────────────

  function rowIdOf(row: HistoryCenterRow): HistoryCenterRowId {
    return row.kind === "entry"
      ? { kind: "entry", entryId: row.entry.id }
      : { kind: row.kind, entryId: row.anchorEntryId };
  }

  function rowKey(row: HistoryCenterRow): string {
    return row.kind === "entry" ? `entry:${row.entry.id}` : `${row.kind}:${row.anchorEntryId}`;
  }

  function rowSelector(id: HistoryCenterRowId): string {
    return `[data-row-kind="${id.kind}"][data-row-entry="${CSS.escape(id.entryId)}"]`;
  }

  function rowFocused(row: HistoryCenterRow): boolean {
    return (
      focusedRow !== null &&
      focusedRow.kind === row.kind &&
      focusedRow.entryId === rowIdOf(row).entryId
    );
  }

  // ── Disclosure tree lookups (display only; topology stays in core) ────

  function* walkLevels(
    open: ReadonlyMap<string, HistoryCenterOpenFork> | null,
  ): Generator<HistoryCenterOpenFork> {
    if (open === null) {
      return;
    }
    for (const level of open.values()) {
      yield level;
      yield* walkLevels(level.inner as ReadonlyMap<string, HistoryCenterOpenFork> | null);
    }
  }

  /** Whether a fork is open at the entry, at any depth (aria-expanded). */
  function hasLevel(entryId: string): boolean {
    for (const level of walkLevels(openForks)) {
      if (level.anchorEntryId === entryId) {
        return true;
      }
    }
    return false;
  }

  /** The open level whose chosen fork is the given run's first entry. */
  function levelForFork(forkId: string): HistoryCenterOpenFork | null {
    for (const level of walkLevels(openForks)) {
      if (level.chosen?.entryId === forkId) {
        return level;
      }
    }
    return null;
  }

  /** The run's first entry row doubles as the opened region: it carries the
   *  chosen fork's branch name, run entry count and rename affordance (R6). */
  function isFirstRunRow(row: EntryRow): boolean {
    return row.forkId !== null && row.entry.id === row.forkId;
  }

  function runHeaderFor(row: EntryRow): { branchId: string; name: string; entryCount: number } | null {
    if (!isFirstRunRow(row) || row.branchId === null || row.forkId === null) {
      return null;
    }
    const chosen = levelForFork(row.forkId)?.chosen;
    if (chosen === null || chosen === undefined) {
      return null;
    }
    return {
      branchId: chosen.branchId,
      name: chosen.branchName ?? chosen.branchId,
      entryCount: chosen.entryCount,
    };
  }

  // Short-form relative time, derived purely from supplied timestamps. The
  // reference is the data's newest recordedAtMs — stable across renders and
  // runtimes, no clock (D2).
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

  /** The run's head — its last visible row — supplies the relative time. */
  function runHeadTime(forkId: string): string | null {
    let at: number | undefined;
    for (const row of entryRows) {
      if (row.forkId === forkId) {
        at = row.entry.recordedAtMs;
      }
    }
    if (at === undefined || newestRecordedAt === undefined) {
      return null;
    }
    return formatRelativeTime(newestRecordedAt - at);
  }

  /** The picker's tentative pick, for the confirm enablement rule (R4). */
  function pickedContinuation(
    continuations: HistoryContinuation[],
    pickedEntryId: string | null,
  ): HistoryContinuation | undefined {
    if (pickedEntryId === null) {
      return undefined;
    }
    return continuations.find((fork) => fork.entryId === pickedEntryId);
  }

  // ── Handlers ──────────────────────────────────────────────────────────

  function handleOpenChange(next: boolean): void {
    sendRef.current(next ? { type: "OPEN" } : { type: "CLOSE" });
  }

  function handleListKeydown(event: React.KeyboardEvent): void {
    // The rename input owns its keys (commit/cancel). Disclosure, picker and
    // rename buttons keep native Enter/Space activation (they are not row
    // activation); arrows still drive roving focus from anywhere in the row.
    if (event.target instanceof HTMLInputElement) {
      return;
    }

    if (
      event.target instanceof HTMLElement &&
      (event.target.closest("[data-part=\"fork-disclosure\"]") !== null ||
        event.target.closest("[data-part=\"picker-option\"]") !== null ||
        event.target.closest("[data-part=\"picker-confirm\"]") !== null ||
        event.target.closest("[data-part=\"run-header-rename\"]") !== null) &&
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
      const rowEl = (event.target as HTMLElement).closest<HTMLElement>("[data-row-kind]");

      if (rowEl) {
        const kind = rowEl.dataset.rowKind as HistoryCenterRowId["kind"] | undefined;
        const entryId = rowEl.dataset.rowEntry;

        if (kind && entryId) {
          sendRef.current({ type: "ACTIVATE_ROW", row: { kind, entryId } });
        }
      }

      return;
    }

    sendRef.current(machineEvent);
  }

  function handleSurfaceKeydown(event: React.KeyboardEvent): void {
    trapFocusKeydown(sectionRef.current, event.nativeEvent);
  }

  function handleRowClick(row: HistoryCenterRow): void {
    sendRef.current({ type: "ACTIVATE_ROW", row: rowIdOf(row) });
  }

  function disclose(entryId: string): void {
    sendRef.current({ type: "DISCLOSE", entryId });
  }

  function dismissRejection(): void {
    sendRef.current({ type: "DISMISS_REJECTION" });
  }

  // ── Inline rename (opened region, R6) ────────────────────────────────

  function startRename(branchId: string, name: string): void {
    setRenamingBranchId(branchId);
    setRenameValue(name);
  }

  function finishRename(): void {
    pendingFocusRestore.current = renamingBranchId;
    setRenamingBranchId(null);
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

  return (
    <div className="poodle-history-center-popover" data-scope="history-center" data-part="root">
      <span className="poodle-history-center__trigger" data-part="trigger">
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
            data-part="surface"
            data-state={isOpen ? "open" : "closed"}
            data-size={resolvedSize}
            data-density={resolvedDensity}
            aria-label={ariaLabel ?? title}
            onKeyDown={handleSurfaceKeydown}
          >
            <header className="poodle-history-center__header">
              <h2>{title}</h2>
              <p>{entryTotal}</p>
            </header>

            {displayedRejection !== null ? (
              <div className="poodle-history-center__rejection" data-part="rejection" role="status">
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
              ) : status === "failed" ? (
                <div className="poodle-history-center__loading" role="status">
                  <span>{statusMessage ?? "History failed to load."}</span>
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
                  data-part="list"
                  aria-label={listLabel}
                  onKeyDown={handleListKeydown}
                >
                  {rows.map((row) => {
                    const rowId = rowIdOf(row);
                    const depth = row.depth;
                    return (
                      <li
                        key={rowKey(row)}
                        className="poodle-history-center__row"
                        data-part={row.kind}
                        data-row-kind={row.kind}
                        data-row-entry={rowId.entryId}
                        data-depth={depth}
                        data-position={row.kind === "entry" ? row.entry.position : undefined}
                        data-checkpoint={
                          row.kind === "entry" && row.entry.checkpoint === true ? "true" : undefined
                        }
                        data-fork-count={row.kind === "entry" ? row.forkCount : undefined}
                        data-parent-entry={
                          row.kind === "entry" ? (row.parentEntryId ?? undefined) : undefined
                        }
                        data-fork-id={row.kind === "entry" ? (row.forkId ?? undefined) : undefined}
                        aria-level={depth + 1}
                        style={{ "--poodle-history-center-depth": depth } as React.CSSProperties}
                      >
                        {row.kind === "entry" ? (
                          (() => {
                            const header = runHeaderFor(row);
                            const openAt = hasLevel(row.entry.id);
                            return (
                              <>
                                <button
                                  type="button"
                                  className="poodle-history-center__entry-content"
                                  data-open={openAt ? "true" : undefined}
                                  tabIndex={rowFocused(row) ? 0 : -1}
                                  onClick={() => handleRowClick(row)}
                                >
                                  {row.entry.checkpoint === true ? (
                                    <span className="poodle-history-center__pin">
                                      <Icon name="git-commit-horizontal" size={resolvedSize} />
                                    </span>
                                  ) : (
                                    <span
                                      className="poodle-history-center__position-marker"
                                      data-position={row.entry.position}
                                    />
                                  )}
                                  <span className="poodle-history-center__entry-copy">
                                    <span className="poodle-history-center__entry-label">{row.entry.label}</span>
                                    {row.entry.groupId ? (
                                      <span className="poodle-history-center__entry-meta">{row.entry.groupId}</span>
                                    ) : null}
                                  </span>
                                </button>

                                {header !== null ? (
                                  // The opened region: the run's first entry row carries
                                  // the chosen fork's name, count and rename (R6).
                                  <div
                                    className="poodle-history-center__run-header"
                                    data-part="run-header"
                                    data-branch={header.branchId}
                                  >
                                    {renamingBranchId === header.branchId ? (
                                      <input
                                        ref={renameInputRef}
                                        className="poodle-history-center__rename-input"
                                        data-part="run-header-rename-input"
                                        aria-label={`Rename branch ${header.name}`}
                                        maxLength={maxBranchNameBytes}
                                        value={renameValue}
                                        onChange={(event) => setRenameValue(event.target.value)}
                                        onKeyDown={(event) => handleRenameKeydown(event, header.branchId)}
                                        onBlur={() => {
                                          // Commit the branch currently being renamed; a
                                          // blur fired by the input's own teardown (after
                                          // commit/cancel) is a no-op.
                                          if (renamingBranchId !== null) commitRename(renamingBranchId);
                                        }}
                                      />
                                    ) : (
                                      <>
                                        <span className="poodle-history-center__run-header-copy">
                                          <span className="poodle-history-center__run-header-name">
                                            {header.name}
                                          </span>
                                          <span className="poodle-history-center__run-header-meta">
                                            {header.entryCount} {header.entryCount === 1 ? "entry" : "entries"}
                                            {runHeadTime(row.forkId!) !== null ? ` · ${runHeadTime(row.forkId!)}` : ""}
                                          </span>
                                        </span>
                                        <button
                                          type="button"
                                          className="poodle-history-center__run-header-rename"
                                          data-part="run-header-rename"
                                          data-branch={header.branchId}
                                          aria-label={`Rename ${header.name}`}
                                          title="Rename branch"
                                          tabIndex={rowFocused(row) ? 0 : -1}
                                          onClick={() => startRename(header.branchId, header.name)}
                                        >
                                          <Icon name="edit" size="xs" />
                                        </button>
                                      </>
                                    )}
                                  </div>
                                ) : null}

                                {row.forkCount > 0 ? (
                                  <button
                                    type="button"
                                    className="poodle-history-center__fork"
                                    data-part="fork-disclosure"
                                    data-open={openAt ? "true" : undefined}
                                    aria-label={
                                      openAt
                                        ? `Hide ${row.forkCount} ${row.forkCount === 1 ? "continuation" : "continuations"}`
                                        : `Show ${row.forkCount} ${row.forkCount === 1 ? "continuation" : "continuations"}`
                                    }
                                    aria-expanded={openAt}
                                    tabIndex={rowFocused(row) ? 0 : -1}
                                    onClick={() => disclose(row.entry.id)}
                                  >
                                    <Icon name="git-branch" size={resolvedSize} />
                                    {row.forkCount > 1 ? (
                                      <span className="poodle-history-center__fork-badge" data-part="fork-badge">
                                        {row.forkCount}
                                      </span>
                                    ) : null}
                                    <span className="poodle-history-center__fork-chevron" aria-hidden="true">
                                      <Icon name="chevron-right" size={resolvedSize} />
                                    </span>
                                  </button>
                                ) : null}
                              </>
                            );
                          })()
                        ) : row.kind === "picker" ? (
                          (() => {
                            const picked = pickedContinuation(row.continuations, row.pickedEntryId);
                            return (
                              <div
                                className="poodle-history-center__picker"
                                data-part="picker"
                                data-anchor={row.anchorEntryId}
                                tabIndex={rowFocused(row) ? 0 : -1}
                              >
                                <div className="poodle-history-center__picker-options" role="group" aria-label="Continuations">
                                  {row.continuations.map((fork) => (
                                    <button
                                      key={fork.entryId}
                                      type="button"
                                      className="poodle-history-center__picker-option"
                                      data-part="picker-option"
                                      data-value={fork.entryId}
                                      data-preferred={fork.preferred ? "true" : undefined}
                                      aria-pressed={row.pickedEntryId === fork.entryId}
                                      onClick={() =>
                                        sendRef.current({ type: "PICK_CONTINUATION", entryId: fork.entryId })
                                      }
                                    >
                                      <span className="poodle-history-center__picker-option-copy">
                                        <span className="poodle-history-center__picker-option-name">{fork.label}</span>
                                        <span className="poodle-history-center__picker-option-branch">
                                          {fork.branchName ?? fork.branchId}
                                        </span>
                                      </span>
                                      {fork.preferred ? (
                                        <span className="poodle-history-center__preferred-badge" data-part="preferred-badge">
                                          Preferred
                                        </span>
                                      ) : null}
                                    </button>
                                  ))}
                                </div>
                                <div className="poodle-history-center__picker-actions" data-part="picker-confirm">
                                  <Button
                                    variant="ghost"
                                    size="xs"
                                    density={resolvedDensity}
                                    disabled={picked === undefined || picked.preferred}
                                    onClick={() => sendRef.current({ type: "CONFIRM" })}
                                  >
                                    Choose
                                  </Button>
                                </div>
                              </div>
                            );
                          })()
                        ) : (
                          <div
                            className="poodle-history-center__not-yet-loaded"
                            data-part="not-yet-loaded"
                            data-anchor={row.anchorEntryId}
                            tabIndex={rowFocused(row) ? 0 : -1}
                          >
                            <Spinner variant="ring" size="xs" tone="muted" />
                            <span>Loading…</span>
                          </div>
                        )}
                      </li>
                    );
                  })}
                </ul>

                {status === "loading" ? (
                  <div className="poodle-history-center__loading" role="status">
                    <Spinner variant="ring" size="sm" tone="muted" />
                    <span>{statusMessage ?? "Loading history…"}</span>
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

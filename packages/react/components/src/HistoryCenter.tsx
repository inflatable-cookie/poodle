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

import { AlertDialog } from "./AlertDialog";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { Menu } from "./Menu";
import { Popover } from "./Popover";
import { Select } from "./Select";
import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  HistoryStatus,
  MenuItem,
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
  /** Host op 3: checkout the picked continuation — the selected fork
   *  becomes the primary history. Poodle's word; the host maps the
   *  callback onto its own prefer operation (R2a). */
  onCheckoutContinuation?: ((entryId: string) => void) | null;
  /** Host op 4 (opt-in, b033 R4): delete the selected fork. The delete
   *  IconButton renders only when this callback is supplied — absent
   *  callback, absent button; no disabled stand-in. Poodle deletes nothing
   *  itself, shows no confirmation of its own (that is the host's call),
   *  and does not guess at the resulting history: the host runs the
   *  operation and supplies new pages. */
  onDeleteContinuation?: ((entryId: string) => void) | null;
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
  onCheckoutContinuation = null,
  onDeleteContinuation = null,
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
  /** The fork the operator asked to delete, held while the confirmation is
   *  open. `null` whenever no confirmation is showing. */
  const [deleteTarget, setDeleteTarget] = useState<HistoryContinuation | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const sectionRef = useRef<HTMLElement | null>(null);
  const listRef = useRef<HTMLUListElement | null>(null);
  const renameInputRef = useRef<HTMLInputElement | null>(null);
  const lastRejectionProp = useRef<HistoryCenterRejectionCode | null>(null);
  const lastContinuationsResult = useRef<typeof continuationsResult>(null);
  const lastRunResult = useRef<typeof runResult>(null);
  const pendingFocusRestore = useRef(false);
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
  // picker trigger's relative time is derived from supplied data only (ruling
  // D2) — there is no clock and no `Date.now()` anywhere.
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
        case "checkoutContinuation":
          onCheckoutContinuation?.(effect.entryId);
          break;
        case "deleteContinuation":
          onDeleteContinuation?.(effect.entryId);
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
  // the picker's pencil so keyboard users stay anchored to the row. The
  // pencil is an IconButton (R4), which forwards no data attributes — the
  // wrapper carries the part/branch and the button inside is the focus
  // target (b032 R3). b033: the run header is gone, so the shared pencil is
  // the only rename control left.
  useEffect(() => {
    if (!pendingFocusRestore.current) return;
    pendingFocusRestore.current = false;
    listRef.current
      ?.querySelector<HTMLElement>('[data-part="picker-actions"] .poodle-menu__trigger')
      ?.focus();
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

  /**
   * Relative time from a diff, mirroring the Svelte runtime exactly. The
   * reference is the data's newest recordedAtMs — stable across renders and
   * runtimes, no clock (b028 D2).
   */
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

  /** The picker trigger's meta line: the shown fork's entry count, plus the
   *  run's derived relative time when its rows are visible (b033 R3 — the
   *  run header's count and time survive inside the trigger). */
  function pickerMeta(fork: HistoryContinuation): string {
    const count = `${fork.entryCount} ${fork.entryCount === 1 ? "entry" : "entries"}`;
    const time = runHeadTime(fork.entryId);
    return time === null ? count : `${count} · ${time}`;
  }

  /**
   * The fork row's actions, as one menu.
   *
   * Checkout is not the primary way to switch fork — clicking any entry in the
   * run navigates to it and checks that fork out. This item is the narrower
   * case: make the fork primary *without* moving the current position.
   *
   * R1 (g13-034): the row's `disabled` signal governs the Select alone — the
   * menu never inherits it. With one fork there is nothing to choose between
   * (the Select is disabled), but the auto-chosen single fork still counts as
   * picked: checkout and rename live on their own gates. Checkout is disabled
   * only when nothing is picked, when the picked fork is already the current
   * line (`preferred`), or while a rename is open.
   */
  function pickerActions(
    picked: HistoryContinuation | undefined,
    renameTarget: { branchId: string; name: string } | null,
  ): MenuItem[] {
    const items: MenuItem[] = [
      {
        value: "rename",
        label: "Rename",
        disabled: renameTarget === null || renamingBranchId !== null,
      },
      {
        value: "checkout",
        label: "Checkout",
        disabled: picked === undefined || picked.preferred || renamingBranchId !== null,
      },
    ];

    // Opt-in (b033 R4): absent callback, absent item.
    if (onDeleteContinuation !== null) {
      items.push({ value: "separator", label: "", kind: "separator" });
      items.push({
        value: "delete",
        label: "Delete",
        tone: "danger",
        disabled: picked === undefined || renamingBranchId !== null,
      });
    }

    return items;
  }

  /** Routes a menu selection to the action it names. */
  function runPickerAction(
    value: string,
    picked: HistoryContinuation | undefined,
    renameTarget: { branchId: string; name: string } | null,
  ): void {
    if (value === "rename" && renameTarget !== null) {
      startRename(renameTarget.branchId, renameTarget.name);
      return;
    }
    if (value === "checkout") {
      sendRef.current({ type: "CONFIRM" });
      return;
    }
    if (value === "delete" && picked !== undefined) {
      // Deleting a fork discards work and Poodle cannot undo it, so the
      // operator confirms before the command leaves. This reverses b033's R4,
      // which left confirmation to the host: every host would have had to
      // build the same dialog, and one that forgot would ship a menu item
      // that destroys history on a single click.
      setDeleteTarget(picked);
    }
  }

  function confirmDelete(): void {
    const target = deleteTarget;
    setDeleteTarget(null);
    if (target !== null) {
      sendRef.current({ type: "DELETE_CONTINUATION", entryId: target.entryId });
    }
    pendingFocusRestore.current = true;
  }

  function cancelDelete(): void {
    setDeleteTarget(null);
    pendingFocusRestore.current = true;
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

  /** The fork behind a Select option value — the option carries only the
   *  value and label; the branch name and the current marker come from the
   *  fork record (R4). */
  function forkForValue(continuations: HistoryContinuation[], entryId: string): HistoryContinuation | undefined {
    return continuations.find((fork) => fork.entryId === entryId);
  }

  // ── Handlers ──────────────────────────────────────────────────────────

  function handleOpenChange(next: boolean): void {
    sendRef.current(next ? { type: "OPEN" } : { type: "CLOSE" });
  }

  function handleListKeydown(event: React.KeyboardEvent): void {
    // The rename input owns its keys (commit/cancel). Disclosure buttons
    // keep native Enter/Space activation (they are not row activation);
    // arrows still drive roving focus from anywhere in the row.
    if (event.target instanceof HTMLInputElement) {
      return;
    }

    // The picker's Select owns every key on its trigger (arrows open the
    // listbox, Enter/Space pick an option); the machine never maps them.
    if (event.target instanceof HTMLElement && event.target.closest('[data-part="picker-select"]') !== null) {
      return;
    }

    if (
      event.target instanceof HTMLElement &&
      event.target.closest('[data-part="fork-disclosure"]') !== null &&
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

  // ── Inline rename (picker row; b033 R3 — the run header is gone, the
  //    shared pencil serves both fork counts) ───────────────────────────

  function startRename(branchId: string, name: string): void {
    setRenamingBranchId(branchId);
    setRenameValue(name);
  }

  function finishRename(): void {
    pendingFocusRestore.current = true;
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
    // The rename input owns its keys end to end: Escape must cancel the
    // rename, not close the popover — the document-level dismiss layer
    // ignores `preventDefault`, so the key stops propagating here.
    if (event.key === "Enter") {
      event.preventDefault();
      event.stopPropagation();
      commitRename(branchId);
    } else if (event.key === "Escape") {
      event.preventDefault();
      event.stopPropagation();
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
            /* A bare glyph, not an IconButton. The disclosure sits between
               undo and redo, and it must read narrower than them rather than
               as a third control of equal weight. It stays a real button, so
               the keyboard and assistive tech still reach it; only the button
               chrome and its padding go away. */
            <button
              type="button"
              className="poodle-history-center__list-trigger"
              data-part="list-trigger"
              data-size={resolvedSize}
              aria-label={listLabel}
              aria-expanded={isOpen}
              title={listLabel}
            >
              <Icon name="chevron-down" size={resolvedSize} density={resolvedDensity} />
            </button>
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
                            const renameTarget =
                              picked === undefined
                                ? null
                                : { branchId: picked.branchId, name: picked.branchName ?? picked.branchId };
                            return (
                              <div
                                className="poodle-history-center__picker"
                                data-part="picker"
                                data-anchor={row.anchorEntryId}
                                data-disabled={row.disabled ? "true" : undefined}
                                tabIndex={rowFocused(row) ? 0 : -1}
                              >
                                {/* b033: one picker row serves every open level
                                    (forkCount >= 1). The Select is disabled on
                                    the row's signal when a single fork leaves
                                    nothing to choose between (R3); its trigger
                                    carries the fork icon, name, branch name,
                                    entry count and derived relative time (R1,
                                    R3 — the run header's facts survive in the
                                    trigger). The row is Select, rename pencil,
                                    opt-in delete, checkout (R1, R4): the
                                    pencil renames whichever fork the Select
                                    shows, the delete IconButton renders only
                                    when the host supplies its callback and
                                    emits a command for the selected fork — no
                                    confirmation inside Poodle — and while a
                                    rename is open the inline input takes the
                                    Select's place (b032 R3). The trigger and
                                    the options both carry the fork label and
                                    its branch name. */}
                                <div
                                  className="poodle-history-center__picker-controls"
                                  data-part="picker-select"
                                >
                                  {renameTarget !== null && renamingBranchId === renameTarget.branchId ? (
                                    <input
                                      ref={renameInputRef}
                                      className="poodle-history-center__rename-input"
                                      data-part="picker-rename-input"
                                      aria-label={`Rename branch ${renameTarget.name}`}
                                      maxLength={maxBranchNameBytes}
                                      value={renameValue}
                                      onChange={(event) => setRenameValue(event.target.value)}
                                      onKeyDown={(event) => handleRenameKeydown(event, renameTarget.branchId)}
                                      onBlur={() => {
                                        // Commit the branch currently being
                                        // renamed; a blur fired by the input's
                                        // own teardown (after commit/cancel) is
                                        // a no-op.
                                        if (renamingBranchId !== null) commitRename(renamingBranchId);
                                      }}
                                    />
                                  ) : (
                                    <Select
                                      value={row.pickedEntryId}
                                      options={row.continuations.map((fork) => ({
                                        value: fork.entryId,
                                        label: fork.label,
                                      }))}
                                      size="xs"
                                      density={resolvedDensity}
                                      variant="default"
                                      disabled={row.disabled}
                                      ariaLabel="Continuations"
                                      placeholder="Choose a fork…"
                                      onValueChange={(entryId) =>
                                        sendRef.current({ type: "PICK_CONTINUATION", entryId })
                                      }
                                      trigger={({ selectedOption, placeholder }) => {
                                        const fork =
                                          selectedOption === null
                                            ? undefined
                                            : forkForValue(row.continuations, selectedOption.value);
                                        return selectedOption === null ? (
                                          <span
                                            className="poodle-history-center__picker-value"
                                            data-placeholder="true"
                                          >
                                            {placeholder}
                                          </span>
                                        ) : (
                                          <span className="poodle-history-center__picker-value">
                                            <Icon name="git-branch" size="sm" />
                                            <span className="poodle-history-center__picker-option-copy">
                                              <span className="poodle-history-center__picker-option-name">
                                                {selectedOption.label}
                                              </span>
                                              {fork !== undefined ? (
                                                <>
                                                  <span className="poodle-history-center__picker-option-branch">
                                                    {fork.branchName ?? fork.branchId}
                                                  </span>
                                                </>
                                              ) : null}
                                            </span>
                                            {fork !== undefined ? (
                                                  <span className="poodle-history-center__picker-option-meta">
                                                    {pickerMeta(fork)}
                                                  </span>
                                            ) : null}
                                          </span>
                                        );
                                      }}
                                      option={({ option }) => {
                                        const fork = forkForValue(row.continuations, option.value);
                                        return (
                                          <span className="poodle-history-center__picker-option-copy">
                                            <span className="poodle-history-center__picker-option-name">
                                              {option.label}
                                            </span>
                                            <span className="poodle-history-center__picker-option-branch">
                                              {fork?.branchName ?? fork?.branchId}
                                            </span>
                                          </span>
                                        );
                                      }}
                                    />
                                  )}
                                  {/* One actions menu, not three buttons. Clicking any entry in a fork's
                                      run already navigates and checks that fork out, so none of these is
                                      the row's primary action — checkout here exists only to activate a
                                      fork without moving the current position. */}
                                  <span className="poodle-history-center__picker-actions" data-part="picker-actions">
                                    <Menu
                                      items={pickerActions(picked, renameTarget)}
                                      size="xs"
                                      density={resolvedDensity}
                                      ariaLabel="Fork actions"
                                      triggerAriaLabel={
                                        picked === undefined
                                          ? "Fork actions"
                                          : `Actions for ${picked.branchName ?? picked.branchId}`
                                      }
                                      onAction={(value) => runPickerAction(value, picked, renameTarget)}
                                      trigger={
                                          <IconButton
                                            icon="ellipsis"
                                            ariaLabel="Fork actions"
                                            variant="ghost"
                                            size="xs"
                                            density={resolvedDensity}
                                          />
                                        }
                                    />
                                  </span>
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

      {/* One dialog for the whole component, not one per row: only ever a
          single fork is being deleted. It renders outside the Popover, which
          is safe — Dialog registers its dismiss layer while the Popover is on
          top, so the ancestry rule (b031) spares the Popover from its own
          outside-dismissal and the history list is still there when the
          operator cancels. */}
      <AlertDialog
        open={deleteTarget !== null}
        tone="danger"
        title="Delete this fork?"
        description="The fork and its entries go for good. This cannot be undone."
        itemLabel="Fork"
        itemValue={deleteTarget === null ? null : deleteTarget.label}
        confirmLabel="Delete"
        cancelLabel="Cancel"
        onConfirm={confirmDelete}
        onCancel={cancelDelete}
        onOpenChange={(next) => {
          if (!next) {
            cancelDelete();
          }
        }}
        size={resolvedSize}
        density={resolvedDensity}
      />
    </div>
  );
}

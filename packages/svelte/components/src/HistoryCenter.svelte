<script lang="ts">
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
  import { tick } from "svelte";

  import { default as EmptyState } from "./EmptyState.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Popover } from "./Popover.svelte";
  import { default as Select } from "./Select.svelte";
  import { default as Spinner } from "./Spinner.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    HistoryStatus,
    OverlayPlacement,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
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

  let {
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
    open = $bindable<boolean | null>(null),
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
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);
  // Machine-owned disclosure tree: holds only what is open (R5); dropped on
  // close by the machine's CLOSE transition.
  let openForks = $state<ReadonlyMap<string, HistoryCenterOpenFork> | null>(null);
  // Machine-owned roving focus identity over the visible rows.
  let focusedRow = $state<HistoryCenterRowId | null>(null);
  let displayedRejection = $state<string | null>(null);
  let renamingBranchId = $state<string | null>(null);
  let renameValue = $state("");
  let renameInputElement = $state<HTMLInputElement | null>(null);
  let sectionElement = $state<HTMLElement | null>(null);
  let listElement = $state<HTMLUListElement | null>(null);

  $effect.pre(() => {
    if (!seededDefaultOpen) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });

  const isOpen = $derived(open === null ? uncontrolledOpen : open);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  // R1: ONE loop over the flat visible-row derivation. No recursion, no
  // nested component, no svelte:self. depth drives padding and nothing else.
  const rows = $derived(historyCenterVisibleRows(pages, openForks));
  const entryRows = $derived(rows.filter((row): row is Extract<HistoryCenterRow, { kind: "entry" }> => row.kind === "entry"));
  const entryTotal = $derived(`${entryRows.length} ${entryRows.length === 1 ? "entry" : "entries"}`);
  // The data's own "present": the newest authority-supplied timestamp. The
  // picker trigger's relative time is derived from supplied data only (ruling
  // D2) — there is no clock and no `Date.now()` anywhere.
  const newestRecordedAt = $derived(
    entryRows.reduce<number | undefined>((newest, row) => {
      const at = row.entry.recordedAtMs;
      return at !== undefined && (newest === undefined || at > newest) ? at : newest;
    }, undefined),
  );

  const machineContext = $derived<HistoryCenterContext>({
    pages,
    open: openForks,
    focusRow: focusedRow,
    rejection: displayedRejection,
  });

  function send(event: HistoryCenterEvent): void {
    const result = historyCenterTransition(isOpen ? "open" : "closed", machineContext, event);
    openForks = result.context.open;
    focusedRow = result.context.focusRow;
    displayedRejection = result.context.rejection;

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitOpenChange": {
          if (open === null) uncontrolledOpen = effect.open;
          else open = effect.open;
          onOpenChange?.(effect.open);
          break;
        }
        case "focusRow":
          tick().then(() => {
            const rowEl = listElement?.querySelector<HTMLElement>(rowSelector(effect.row));
            (rowEl?.querySelector<HTMLElement>(
              ".poodle-history-center__entry-content, .poodle-history-center__picker, .poodle-history-center__not-yet-loaded",
            ) ?? rowEl)?.focus();
          });
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
  }

  // Transient rejection: a *new* non-null prop value displays; dismissal is
  // local and never re-shows the same value. The host clearing the prop
  // clears the notice.
  let lastRejectionProp: HistoryCenterRejectionCode | null = null;

  $effect(() => {
    if (rejection === lastRejectionProp) {
      return;
    }

    lastRejectionProp = rejection;

    if (rejection === null) {
      send({ type: "DISMISS_REJECTION" });
    } else {
      send({ type: "SHOW_REJECTION", code: rejection });
    }
  });

  // Host op results: the host resolves a callback and feeds the result back.
  // Diffed by reference — a *new* non-null value dispatches the matching
  // loaded event; null or the same reference does nothing.
  let lastContinuationsResult: typeof continuationsResult = null;

  $effect(() => {
    if (continuationsResult === lastContinuationsResult) {
      return;
    }

    lastContinuationsResult = continuationsResult;

    if (continuationsResult !== null) {
      send({
        type: "CONTINUATIONS_LOADED",
        entryId: continuationsResult.entryId,
        continuations: continuationsResult.continuations,
      });
    }
  });

  let lastRunResult: typeof runResult = null;

  $effect(() => {
    if (runResult === lastRunResult) {
      return;
    }

    lastRunResult = runResult;

    if (runResult !== null) {
      send({ type: "RUN_LOADED", fromEntryId: runResult.fromEntryId, pages: runResult.pages });
    }
  });

  // ── Row identity helpers (R1) ─────────────────────────────────────────

  function rowIdOf(row: HistoryCenterRow): HistoryCenterRowId {
    return row.kind === "entry"
      ? { kind: "entry", entryId: row.entry.id }
      : { kind: row.kind, entryId: row.anchorEntryId };
  }

  function sameRowId(a: HistoryCenterRowId, b: HistoryCenterRowId): boolean {
    return a.kind === b.kind && a.entryId === b.entryId;
  }

  function rowKey(row: HistoryCenterRow): string {
    return row.kind === "entry" ? `entry:${row.entry.id}` : `${row.kind}:${row.anchorEntryId}`;
  }

  function rowSelector(id: HistoryCenterRowId): string {
    return `[data-row-kind="${id.kind}"][data-row-entry="${CSS.escape(id.entryId)}"]`;
  }

  function rowFocused(row: HistoryCenterRow): boolean {
    return focusedRow !== null && sameRowId(focusedRow, rowIdOf(row));
  }

  // ── Disclosure tree lookups (display only; topology stays in core) ────

  function* walkLevels(open: ReadonlyMap<string, HistoryCenterOpenFork> | null): Generator<HistoryCenterOpenFork> {
    if (open === null) {
      return;
    }
    for (const level of open.values()) {
      yield level;
      yield* walkLevels(level.inner);
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

  /** The picker trigger's meta line: the shown fork's entry count, plus the
   *  run's derived relative time when its rows are visible (b033 R3 — the
   *  run header's count and time survive inside the trigger). */
  function pickerMeta(fork: HistoryContinuation): string {
    const count = `${fork.entryCount} ${fork.entryCount === 1 ? "entry" : "entries"}`;
    const time = runHeadTime(fork.entryId);
    return time === null ? count : `${count} · ${time}`;
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
    send(next ? { type: "OPEN" } : { type: "CLOSE" });
  }

  function handleListKeydown(event: KeyboardEvent): void {
    // The rename input owns its keys (commit/cancel). Disclosure buttons
    // keep native Enter/Space activation (they are not row activation);
    // arrows still drive roving focus from anywhere in the row.
    if (event.target instanceof HTMLInputElement) {
      return;
    }

    // The picker's Select owns every key on its trigger (arrows open the
    // listbox, Enter/Space pick an option); the machine never maps them.
    if (event.target instanceof HTMLElement && event.target.closest("[data-part=\"picker-select\"]") !== null) {
      return;
    }

    if (
      event.target instanceof HTMLElement &&
      event.target.closest("[data-part=\"fork-disclosure\"]") !== null &&
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
          send({ type: "ACTIVATE_ROW", row: { kind, entryId } });
        }
      }

      return;
    }

    send(machineEvent);
  }

  function handleSurfaceKeydown(event: KeyboardEvent): void {
    trapFocusKeydown(sectionElement, event);
  }

  function handleRowClick(row: HistoryCenterRow): void {
    send({ type: "ACTIVATE_ROW", row: rowIdOf(row) });
  }

  function disclose(entryId: string): void {
    send({ type: "DISCLOSE", entryId });
  }

  function dismissRejection(): void {
    send({ type: "DISMISS_REJECTION" });
  }

  // ── Inline rename (picker row; b033 R3 — the run header is gone, the
  //    shared pencil serves both fork counts) ───────────────────────────

  function startRename(branchId: string, name: string): void {
    renamingBranchId = branchId;
    renameValue = name;
  }

  function finishRename(): void {
    const branchId = renamingBranchId;
    renamingBranchId = null;
    tick().then(() => {
      const branch = CSS.escape(branchId ?? "");
      // The picker pencil is an IconButton (R4), which forwards no data
      // attributes — the wrapper carries the part/branch and the button
      // inside is the focus target (b032 R3). b033: the run header is gone,
      // so the shared pencil is the only rename control left. Focus returns
      // to it on commit or cancel.
      listElement
        ?.querySelector<HTMLElement>(`[data-part="picker-rename"][data-branch="${branch}"]`)
        ?.querySelector<HTMLElement>("button")
        ?.focus();
    });
  }

  function commitRename(branchId: string): void {
    if (renamingBranchId !== branchId) {
      return;
    }

    send({ type: "RENAME", branchId, name: renameValue });
    finishRename();
  }

  function cancelRename(): void {
    finishRename();
  }

  function handleRenameKeydown(event: KeyboardEvent, branchId: string): void {
    // The rename input owns its keys end to end. Escape must cancel the
    // rename, not close the popover: the surface is portalled, so Svelte's
    // delegated keydown listener and the dismiss layer both sit on
    // `document`, where `stopPropagation` does not stop same-node listeners
    // — `stopImmediatePropagation` does.
    if (event.key === "Enter") {
      event.preventDefault();
      event.stopImmediatePropagation();
      commitRename(branchId);
    } else if (event.key === "Escape") {
      event.preventDefault();
      event.stopImmediatePropagation();
      cancelRename();
    }
  }

  $effect(() => {
    if (renamingBranchId !== null && renameInputElement) {
      // Defer past the Popover's open-focus microtask: focusing inside the
      // same flush as the mount lets the popover's initial focus land on the
      // input's focus window, blur it, and spuriously blur-commit the rename.
      tick().then(() => {
        renameInputElement?.focus();
        renameInputElement?.select();
      });
    }
  });
</script>

<div class="poodle-history-center-popover" data-scope="history-center" data-part="root">
  <span class="poodle-history-center__trigger" data-part="trigger">
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
      {placement}
      initialFocus="first-focusable"
      triggerIsInteractive
      ariaLabel={ariaLabel ?? title}
      surfaceMinWidth="min(28rem, calc(100vw - 2rem))"
      surfaceMaxWidth="min(38rem, calc(100vw - 2rem))"
      onOpenChange={handleOpenChange}
    >
      {#snippet trigger()}
        <IconButton
          icon="list"
          ariaLabel={listLabel}
          tooltip={listLabel}
          variant="ghost"
          size={resolvedSize}
          density={resolvedDensity}
          expanded={isOpen}
        />
      {/snippet}

      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <section
        class="poodle-history-center"
        data-part="surface"
        data-state={isOpen ? "open" : "closed"}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        aria-label={ariaLabel ?? title}
        bind:this={sectionElement}
        onkeydown={handleSurfaceKeydown}
      >
        <header class="poodle-history-center__header">
          <h2>{title}</h2>
          <p>{entryTotal}</p>
        </header>

        {#if displayedRejection !== null}
          <div class="poodle-history-center__rejection" data-part="rejection" role="status">
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
        {/if}

        {#if rows.length === 0}
          {#if status === "loading"}
            <div class="poodle-history-center__loading" role="status">
              <Spinner variant="ring" size="sm" tone="muted" />
              <span>{statusMessage ?? "Loading history…"}</span>
            </div>
          {:else if status === "failed"}
            <div class="poodle-history-center__loading" role="status">
              <span>{statusMessage ?? "History failed to load."}</span>
            </div>
          {:else}
            <div class="poodle-history-center__empty">
              <EmptyState title={title} message={emptyMessage} size="compact" />
            </div>
          {/if}
        {:else}
          <ul
            class="poodle-history-center__list"
            data-part="list"
            aria-label={listLabel}
            bind:this={listElement}
            onkeydown={handleListKeydown}
          >
            {#each rows as row (rowKey(row))}
              {@const rowId = rowIdOf(row)}
              {@const depth = row.depth}
              <li
                class="poodle-history-center__row"
                data-part={row.kind}
                data-row-kind={row.kind}
                data-row-entry={rowId.entryId}
                data-depth={depth}
                data-position={row.kind === "entry" ? row.entry.position : undefined}
                data-checkpoint={row.kind === "entry" && row.entry.checkpoint === true ? "true" : undefined}
                data-fork-count={row.kind === "entry" ? row.forkCount : undefined}
                data-parent-entry={row.kind === "entry" ? (row.parentEntryId ?? undefined) : undefined}
                data-fork-id={row.kind === "entry" ? (row.forkId ?? undefined) : undefined}
                aria-level={depth + 1}
                style={`--poodle-history-center-depth: ${depth}`}
              >
                {#if row.kind === "entry"}
                  {@const openAt = hasLevel(row.entry.id)}
                  <button
                    type="button"
                    class="poodle-history-center__entry-content"
                    data-open={openAt ? "true" : undefined}
                    tabindex={rowFocused(row) ? 0 : -1}
                    onclick={() => handleRowClick(row)}
                  >
                    {#if row.entry.checkpoint === true}
                      <span class="poodle-history-center__pin">
                        <Icon name="git-commit-horizontal" size={resolvedSize} />
                      </span>
                    {:else}
                      <span class="poodle-history-center__position-marker" data-position={row.entry.position}></span>
                    {/if}
                    <span class="poodle-history-center__entry-copy">
                      <span class="poodle-history-center__entry-label">{row.entry.label}</span>
                      {#if row.entry.groupId}
                        <span class="poodle-history-center__entry-meta">{row.entry.groupId}</span>
                      {/if}
                    </span>
                  </button>

                  {#if row.forkCount > 0}
                    <button
                      type="button"
                      class="poodle-history-center__fork"
                      data-part="fork-disclosure"
                      data-open={openAt ? "true" : undefined}
                      aria-label={openAt
                        ? `Hide ${row.forkCount} ${row.forkCount === 1 ? "continuation" : "continuations"}`
                        : `Show ${row.forkCount} ${row.forkCount === 1 ? "continuation" : "continuations"}`}
                      aria-expanded={openAt}
                      tabindex={rowFocused(row) ? 0 : -1}
                      onclick={() => disclose(row.entry.id)}
                    >
                      <Icon name="git-branch" size={resolvedSize} />
                      {#if row.forkCount > 1}
                        <span class="poodle-history-center__fork-badge" data-part="fork-badge">{row.forkCount}</span>
                      {/if}
                      <span class="poodle-history-center__fork-chevron" aria-hidden="true">
                        <Icon name="chevron-right" size={resolvedSize} />
                      </span>
                    </button>
                  {/if}
                {:else if row.kind === "picker"}
                  {@const picked = pickedContinuation(row.continuations, row.pickedEntryId)}
                  {@const renameTarget = picked === undefined ? null : { branchId: picked.branchId, name: picked.branchName ?? picked.branchId }}
                  <div
                    class="poodle-history-center__picker"
                    data-part="picker"
                    data-anchor={row.anchorEntryId}
                    data-disabled={row.disabled ? "true" : undefined}
                    tabindex={rowFocused(row) ? 0 : -1}
                  >
                    <!-- b033: one picker row serves every open level
                         (forkCount >= 1). The Select is disabled on the
                         row's signal when a single fork leaves nothing to
                         choose between (R3); its trigger carries the fork
                         icon, name, branch name, entry count and derived
                         relative time (R1, R3 — the run header's facts
                         survive in the trigger). The row is Select, rename
                         pencil, opt-in delete, checkout (R1, R4): the
                         pencil renames whichever fork the Select shows,
                         the delete IconButton renders only when the host
                         supplies its callback and emits a command for the
                         selected fork — no confirmation inside Poodle —
                         and while a rename is open the inline input takes
                         the Select's place (b032 R3). The trigger and the
                         options both carry the fork label and its branch
                         name. -->
                    <div class="poodle-history-center__picker-controls" data-part="picker-select">
                      {#if renameTarget !== null && renamingBranchId === renameTarget.branchId}
                        <input
                          bind:this={renameInputElement}
                          class="poodle-history-center__rename-input"
                          data-part="picker-rename-input"
                          aria-label={`Rename branch ${renameTarget.name}`}
                          maxlength={maxBranchNameBytes}
                          bind:value={renameValue}
                          onkeydown={(event) => handleRenameKeydown(event, renameTarget.branchId)}
                          onblur={() => {
                            // Commit the branch currently being renamed; a
                            // blur fired by the input's own teardown (after
                            // commit/cancel) is a no-op.
                            if (renamingBranchId !== null) commitRename(renamingBranchId);
                          }}
                        />
                      {:else}
                      <Select
                        value={row.pickedEntryId}
                        options={row.continuations.map((fork) => ({ value: fork.entryId, label: fork.label }))}
                        size="xs"
                        density={resolvedDensity}
                        variant="default"
                        disabled={row.disabled}
                        ariaLabel="Continuations"
                        placeholder="Choose a fork…"
                        onValueChange={(entryId) => send({ type: "PICK_CONTINUATION", entryId })}
                      >
                        {#snippet trigger({ selectedOption, placeholder })}
                          {#if selectedOption === null}
                            <span class="poodle-history-center__picker-value" data-placeholder="true">{placeholder}</span>
                          {:else}
                            {@const fork = forkForValue(row.continuations, selectedOption.value)}
                            <span class="poodle-history-center__picker-value">
                              <Icon name="git-branch" size={resolvedSize} />
                              <span class="poodle-history-center__picker-option-copy">
                                <span class="poodle-history-center__picker-option-name">{selectedOption.label}</span>
                                {#if fork !== undefined}
                                  <span class="poodle-history-center__picker-option-branch">
                                    {fork.branchName ?? fork.branchId}
                                  </span>
                                  <span class="poodle-history-center__picker-option-meta">{pickerMeta(fork)}</span>
                                {/if}
                              </span>
                            </span>
                          {/if}
                        {/snippet}
                        {#snippet option({ option })}
                          {@const fork = forkForValue(row.continuations, option.value)}
                          <span class="poodle-history-center__picker-option-copy">
                            <span class="poodle-history-center__picker-option-name">{option.label}</span>
                            <span class="poodle-history-center__picker-option-branch">
                              {fork?.branchName ?? fork?.branchId}
                            </span>
                          </span>
                        {/snippet}
                      </Select>
                      {/if}
                      <span
                        class="poodle-history-center__picker-rename"
                        data-part="picker-rename"
                        data-branch={renameTarget?.branchId}
                      >
                        <IconButton
                          icon="edit"
                          ariaLabel={renameTarget === null ? "Rename fork" : `Rename ${renameTarget.name}`}
                          tooltip="Rename branch"
                          variant="ghost"
                          size="xs"
                          density={resolvedDensity}
                          disabled={renameTarget === null}
                          onClick={() => {
                            if (renameTarget !== null) startRename(renameTarget.branchId, renameTarget.name);
                          }}
                        />
                      </span>
                      {#if onDeleteContinuation !== null}
                        <span class="poodle-history-center__picker-delete" data-part="picker-delete">
                          <IconButton
                            icon="trash-2"
                            ariaLabel={picked === undefined ? "Delete fork" : `Delete ${picked.branchName ?? picked.branchId}`}
                            tooltip="Delete fork"
                            variant="ghost"
                            tone="danger"
                            size="xs"
                            density={resolvedDensity}
                            disabled={picked === undefined}
                            onClick={() => {
                              if (picked !== undefined) {
                                send({ type: "DELETE_CONTINUATION", entryId: picked.entryId });
                              }
                            }}
                          />
                        </span>
                      {/if}
                      <span class="poodle-history-center__picker-checkout" data-part="picker-checkout">
                        <IconButton
                          icon="check"
                          ariaLabel="Checkout"
                          tooltip="Checkout"
                          variant="ghost"
                          size="xs"
                          density={resolvedDensity}
                          disabled={row.disabled || picked === undefined || picked.preferred || renamingBranchId !== null}
                          onClick={() => send({ type: "CONFIRM" })}
                        />
                      </span>
                    </div>
                  </div>
                {:else}
                  <div
                    class="poodle-history-center__not-yet-loaded"
                    data-part="not-yet-loaded"
                    data-anchor={row.anchorEntryId}
                    tabindex={rowFocused(row) ? 0 : -1}
                  >
                    <Spinner variant="ring" size="xs" tone="muted" />
                    <span>Loading…</span>
                  </div>
                {/if}
              </li>
            {/each}
          </ul>

          {#if status === "loading"}
            <div class="poodle-history-center__loading" role="status">
              <Spinner variant="ring" size="sm" tone="muted" />
              <span>{statusMessage ?? "Loading history…"}</span>
            </div>
          {/if}
        {/if}
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

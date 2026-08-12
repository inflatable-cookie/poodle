<script lang="ts">
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
  import { tick } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as EmptyState } from "./EmptyState.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Popover } from "./Popover.svelte";
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

  let {
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
    onLoadMoreEntries = null,
    onLoadMoreBranches = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);
  let focusIndex = $state(0);
  let displayedRejection = $state<string | null>(null);
  let renamingBranchId = $state<string | null>(null);
  let renamingIndex = $state(-1);
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
  const hasBranches = $derived(branches !== null);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const rows = $derived(historyCenterRows(branches, paths));
  type EntryRow = Extract<HistoryCenterRow, { kind: "entry" }>;
  const entryRows = $derived(rows.filter((row): row is EntryRow => row.kind === "entry"));
  const entryRowCount = $derived(entryRows.length);
  // The data's own "present": the newest authority-supplied timestamp. The
  // caption relative time is derived from supplied data (ruling D2) — there
  // is no clock and no `Date.now()` anywhere.
  const newestRecordedAt = $derived(
    entryRows.reduce<number | undefined>((newest, row) => {
      const at = row.entry.recordedAtMs;
      return at !== undefined && (newest === undefined || at > newest) ? at : newest;
    }, undefined),
  );

  const machineContext = $derived<HistoryCenterContext>({
    branches,
    paths,
    focusIndex,
    rejection: displayedRejection,
  });

  function send(event: HistoryCenterEvent): void {
    const result = historyCenterTransition(isOpen ? "open" : "closed", machineContext, event);
    focusIndex = result.context.focusIndex;
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
            const rowEl = listElement?.querySelector<HTMLElement>(`[data-row-index="${effect.index}"]`);
            (rowEl?.querySelector<HTMLElement>("button, input") ?? rowEl)?.focus();
          });
          break;
        case "emitNavigateEntry":
          onNavigateEntry?.(effect.branchId, effect.entryId);
          break;
        case "emitRenameBranch":
          onRenameBranch?.(effect.branchId, effect.name);
          break;
      }
    }
  }

  // Transient rejection: a *new* non-null prop value displays; dismissal is
  // local and never re-shows the same value. The host clearing the prop
  // clears the notice.
  let lastRejectionProp: string | null = null;

  $effect(() => {
    if (rejection === lastRejectionProp) {
      return;
    }

    lastRejectionProp = rejection;

    if (rejection === null) {
      send({ type: "DISMISS_REJECTION" });
    } else {
      send({ type: "SHOW_REJECTION", message: rejection });
    }
  });

  function handleOpenChange(next: boolean): void {
    send(next ? { type: "OPEN" } : { type: "CLOSE" });
  }

  function handleListKeydown(event: KeyboardEvent): void {
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
        send({ type: "ACTIVATE_ROW", index: Number(rowEl.dataset.rowIndex) });
      }

      return;
    }

    send(machineEvent);
  }

  function handleSurfaceKeydown(event: KeyboardEvent): void {
    trapFocusKeydown(sectionElement, event);
  }

  function handleRowClick(index: number): void {
    send({ type: "ACTIVATE_ROW", index });
  }

  function dismissRejection(): void {
    send({ type: "DISMISS_REJECTION" });
  }

  function startRename(branch: HistoryBranch, index: number): void {
    renamingBranchId = branch.id;
    renamingIndex = index;
    renameValue = branch.name ?? branch.id;
  }

  function finishRename(): void {
    const index = renamingIndex;
    renamingBranchId = null;
    renamingIndex = -1;
    tick().then(() => {
      listElement?.querySelector<HTMLElement>(`[data-row-index="${index}"] [data-rename-button]`)?.focus();
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
    if (event.key === "Enter") {
      event.preventDefault();
      commitRename(branchId);
    } else if (event.key === "Escape") {
      event.preventDefault();
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
  function captionMeta(row: Extract<HistoryCenterRow, { kind: "caption" }>): { count: number; time: string | null } {
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

  const entryTotal = $derived(`${totalEntries} ${totalEntries === 1 ? "entry" : "entries"}`);
  const branchTotal = $derived(`${totalBranches} ${totalBranches === 1 ? "branch" : "branches"}`);
  const summary = $derived(hasBranches ? `${entryTotal} · ${branchTotal}` : entryTotal);
</script>

<div class="poodle-history-center-popover">
  <span class="poodle-history-center__trigger">
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
        data-size={resolvedSize}
        data-density={resolvedDensity}
        aria-label={ariaLabel ?? title}
        bind:this={sectionElement}
        onkeydown={handleSurfaceKeydown}
      >
        <header class="poodle-history-center__header">
          <h2>{title}</h2>
          <p>{summary}</p>
        </header>

        {#if displayedRejection !== null}
          <div class="poodle-history-center__rejection" role="status">
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
          {:else}
            <div class="poodle-history-center__empty">
              <EmptyState title={title} message={emptyMessage} size="compact" />
            </div>
          {/if}
        {:else}
          <ul
            class="poodle-history-center__list"
            aria-label={listLabel}
            bind:this={listElement}
            onkeydown={handleListKeydown}
          >
            {#each rows as row (rowKey(row))}
              {#if row.kind === "entry"}
                <li
                  class="poodle-history-center__row"
                  data-row-index={row.index}
                  data-part="entry"
                  data-depth={row.depth}
                  data-position={row.entry.position}
                  data-checkpoint={row.entry.checkpoint === true ? "true" : undefined}
                  aria-level={row.depth + 1}
                >
                  <span class="poodle-history-center__lanes" aria-hidden="true">
                    {#each Array(row.depth) as _, level (level)}
                      <span class="poodle-history-center__lane" data-lane="ancestor"></span>
                    {/each}
                    <span class="poodle-history-center__lane" data-lane={laneKind(row)}></span>
                  </span>
                  <button
                    type="button"
                    class="poodle-history-center__entry-content"
                    tabindex={focusIndex === row.index ? 0 : -1}
                    onclick={() => handleRowClick(row.index)}
                  >
                    {#if row.entry.checkpoint === true}
                      <span class="poodle-history-center__pin">
                        <Icon icon="git-commit-horizontal" size={resolvedSize} />
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
                </li>
              {:else}
                {@const meta = captionMeta(row)}
                <li
                  class="poodle-history-center__row"
                  data-row-index={row.index}
                  data-part="caption"
                  data-depth={row.depth}
                  data-current={row.branch.current ? "true" : undefined}
                  aria-level={row.depth + 1}
                >
                  <span class="poodle-history-center__lanes" aria-hidden="true">
                    {#each Array(row.depth) as _, level (level)}
                      <span class="poodle-history-center__lane" data-lane="ancestor"></span>
                    {/each}
                    <span class="poodle-history-center__lane" data-lane="caption"></span>
                  </span>
                  {#if renamingBranchId === row.branch.id}
                    <input
                      bind:this={renameInputElement}
                      class="poodle-history-center__rename-input"
                      aria-label={`Rename branch ${row.branch.name ?? row.branch.id}`}
                      maxlength={maxBranchNameBytes}
                      bind:value={renameValue}
                      onkeydown={(event) => handleRenameKeydown(event, row.branch.id)}
                      onblur={() => commitRename(row.branch.id)}
                    />
                  {:else}
                    <span class="poodle-history-center__caption-copy">
                      <span class="poodle-history-center__caption-name">{row.branch.name ?? row.branch.id}</span>
                      <span class="poodle-history-center__caption-meta">
                        {meta.count} {meta.count === 1 ? "entry" : "entries"}{meta.time !== null ? ` · ${meta.time}` : ""}
                      </span>
                      {#if row.branch.current}
                        <span class="poodle-history-center__branch-current-badge">Current</span>
                      {/if}
                    </span>
                    <button
                      type="button"
                      class="poodle-history-center__caption-rename"
                      data-rename-button="true"
                      aria-label={`Rename ${row.branch.name ?? row.branch.id}`}
                      title="Rename branch"
                      tabindex={focusIndex === row.index ? 0 : -1}
                      onclick={() => startRename(row.branch, row.index)}
                    >
                      <Icon icon="edit" size="xs" />
                    </button>
                  {/if}
                </li>
              {/if}
            {/each}
          </ul>

          {#if status === "loading"}
            <div class="poodle-history-center__loading" role="status">
              <Spinner variant="ring" size="sm" tone="muted" />
              <span>{statusMessage ?? "Loading history…"}</span>
            </div>
          {/if}

          {#if hasMoreEntries && onLoadMoreEntries}
            <div class="poodle-history-center__load-more">
              <Button
                variant="ghost"
                size="xs"
                density={resolvedDensity}
                onClick={() => onLoadMoreEntries?.(entryRowCount)}
              >
                Load more entries
              </Button>
            </div>
          {/if}

          {#if hasBranches && hasMoreBranches && onLoadMoreBranches}
            <div class="poodle-history-center__load-more">
              <Button
                variant="ghost"
                size="xs"
                density={resolvedDensity}
                onClick={() => onLoadMoreBranches?.(branches?.length ?? 0)}
              >
                Load more branches
              </Button>
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

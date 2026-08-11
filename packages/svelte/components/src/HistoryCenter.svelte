<script lang="ts">
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
    HistoryBranch,
    HistoryEntry,
    HistoryStatus,
    OverlayPlacement,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
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

  let {
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
    onSelectEntry = null,
    onCheckout = null,
    onRenameBranch = null,
    onLoadMoreEntries = null,
    onLoadMoreBranches = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);
  let focusIndex = $state(0);
  let expandedBranchIds = $state<string[]>([]);
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
  const hasBranches = $derived(branches !== null);
  const branchList = $derived(branches ?? []);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const rows = $derived.by(() => historyCenterRows(entries, branches, expandedBranchIds));

  const machineContext = $derived<HistoryCenterContext>({
    entries,
    branches,
    expandedBranchIds,
    focusIndex,
    rejection: displayedRejection,
  });

  function send(event: HistoryCenterEvent): void {
    const result = historyCenterTransition(isOpen ? "open" : "closed", machineContext, event);
    focusIndex = result.context.focusIndex;
    expandedBranchIds = result.context.expandedBranchIds;
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

  function toggleFork(entryId: string): void {
    send({ type: "TOGGLE_BRANCHES", entryId });
  }

  function dismissRejection(): void {
    send({ type: "DISMISS_REJECTION" });
  }

  function startRename(branch: HistoryBranch): void {
    renamingBranchId = branch.id;
    renameValue = branch.name ?? branch.id;
  }

  function commitRename(branchId: string): void {
    if (renamingBranchId !== branchId) {
      return;
    }

    send({ type: "RENAME", branchId, name: renameValue });
    renamingBranchId = null;
  }

  function cancelRename(): void {
    renamingBranchId = null;
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
      renameInputElement.focus();
      renameInputElement.select();
    }
  });

  function rowKey(row: HistoryCenterRow): string {
    return row.kind === "entry" ? `entry:${row.entry.id}` : `branch:${row.branch.id}`;
  }

  function branchRowIndex(rows: HistoryCenterRow[], branch: HistoryBranch): number {
    return rows.findIndex((candidate) => candidate.kind === "branch" && candidate.branch.id === branch.id);
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
      surfaceMinWidth="min(23rem, calc(100vw - 2rem))"
      surfaceMaxWidth="min(30rem, calc(100vw - 2rem))"
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

        {#if entries.length === 0}
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
          <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
          <ul
            class="poodle-history-center__list"
            aria-label={listLabel}
            bind:this={listElement}
            onkeydown={handleListKeydown}
          >
            {#each rows as row (rowKey(row))}
              {#if row.kind === "entry"}
                <li
                  data-row-index={row.index}
                  data-part="entry"
                  data-position={row.entry.position}
                  data-checkpoint={hasBranches && row.entry.checkpoint === true ? "true" : undefined}
                  data-fork={hasBranches && isForkPoint(row.entry) ? "true" : undefined}
                >
                  <div class="poodle-history-center__item-row">
                    <button
                      type="button"
                      class="poodle-history-center__item-content"
                      tabindex={focusIndex === row.index ? 0 : -1}
                      onclick={() => handleRowClick(row.index)}
                    >
                      {#if hasBranches && row.entry.checkpoint}
                        <Icon icon="git-commit-horizontal" size={resolvedSize} class="poodle-history-center__pin" />
                      {:else}
                        <span class="poodle-history-center__position-marker" data-position={row.entry.position}></span>
                      {/if}
                      <span class="poodle-history-center__item-copy">
                        <span class="poodle-history-center__item-label">{row.entry.label}</span>
                        {#if row.entry.groupId}
                          <span class="poodle-history-center__item-meta">{row.entry.groupId}</span>
                        {/if}
                      </span>
                    </button>

                    {#if hasBranches && isForkPoint(row.entry)}
                      <button
                        type="button"
                        class="poodle-history-center__fork-indicator"
                        aria-label={expandedBranchIds.includes(row.entry.id)
                          ? `Collapse branches at ${row.entry.label}`
                          : `Show branches at ${row.entry.label}`}
                        aria-expanded={expandedBranchIds.includes(row.entry.id)}
                        tabindex={focusIndex === row.index ? 0 : -1}
                        onclick={() => toggleFork(row.entry.id)}
                      >
                        <Icon
                          icon={expandedBranchIds.includes(row.entry.id) ? "chevron-down" : "chevron-right"}
                          size="xs"
                        />
                      </button>
                    {/if}
                  </div>

                  {#if expandedBranchIds.includes(row.entry.id)}
                    <ul class="poodle-history-center__branches">
                      {#each branchList as branch (branch.id)}
                        <li
                          data-row-index={branchRowIndex(rows, branch)}
                          data-part="branch"
                          data-current={branch.current ? "true" : undefined}
                        >
                          {#if renamingBranchId === branch.id}
                            <input
                              bind:this={renameInputElement}
                              class="poodle-history-center__rename-input"
                              aria-label={`Rename branch ${branch.name ?? branch.id}`}
                              maxlength={maxBranchNameBytes}
                              bind:value={renameValue}
                              onkeydown={(event) => handleRenameKeydown(event, branch.id)}
                              onblur={() => commitRename(branch.id)}
                            />
                          {:else}
                            <div class="poodle-history-center__branch-actions">
                              <button
                                type="button"
                                class="poodle-history-center__branch-content"
                                tabindex={focusIndex === branchRowIndex(rows, branch) ? 0 : -1}
                                onclick={() => handleRowClick(branchRowIndex(rows, branch))}
                              >
                                <span class="poodle-history-center__branch-name">{branch.name ?? branch.id}</span>
                                {#if branch.entryCount !== undefined}
                                  <span class="poodle-history-center__branch-count">
                                    {branch.entryCount} {branch.entryCount === 1 ? "entry" : "entries"}
                                  </span>
                                {/if}
                                {#if branch.current}
                                  <span class="poodle-history-center__branch-current-badge">Current</span>
                                {/if}
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
                          {/if}
                        </li>
                      {/each}
                    </ul>
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
                onClick={() => onLoadMoreEntries?.(entries.length)}
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
                onClick={() => onLoadMoreBranches?.(branchList.length)}
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

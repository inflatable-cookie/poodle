<script lang="ts">
  import type { Snippet } from "svelte";

  import Button from "./Button.svelte";
  import Field from "./Field.svelte";
  import IconButton from "./IconButton.svelte";
  import Pill from "./Pill.svelte";
  import Select from "./Select.svelte";
  import TextInput from "./TextInput.svelte";
  import UiPresentationProvider from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  import type {
    AuditLogEntry,
    LogActionType,
    LogActor,
    LogEntry,
    LogFilter,
    LogLevel,
  } from "./types";

  export let entries: LogEntry[] = [];
  export let variant: "auto" | "stream" | "audit" = "auto";
  export let maxEntries = 500;
  export let autoScroll = true;
  export let filterLevel: LogLevel | null = null;
  export let filterText = "";
  export let ariaLabel = "Log output";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  export let loading = false;
  export let error: string | null = null;
  export let emptyMessage = "No log entries found";
  export let filters: LogFilter[] = [];
  export let filterValues: Record<string, string> = {};
  export let page = 1;
  export let pageSize = 50;
  export let total: number | undefined = undefined;
  export let onFilterChange: ((field: string, value: string) => void) | undefined = undefined;
  export let onClearFilters: (() => void) | undefined = undefined;
  export let onPageChange: ((page: number) => void) | undefined = undefined;
  export let onRefresh: (() => void) | undefined = undefined;
  export let onExport: (() => void) | undefined = undefined;
  export let actionIcon: Snippet<[LogActionType]> | undefined = undefined;
  export let entryDetails: Snippet<[AuditLogEntry]> | undefined = undefined;
  export let getActionType: ((action: string) => LogActionType) | undefined = undefined;
  export let formatAction: ((action: string) => string) | undefined = undefined;
  export let formatResourceType: ((resourceType: string) => string) | undefined = undefined;
  export let getActorHref: ((actor: LogActor) => string) | undefined = undefined;
  export let getResourceHref:
    | ((resourceType: string, resourceId: string, action: string) => string | null)
    | undefined = undefined;

  let scrollContainer: HTMLDivElement | null = null;
  let isUserScrolled = false;
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: resolvedVariant =
    variant === "auto" ? (entries.some(isAuditEntry) ? "audit" : "stream") : variant;

  $: displayEntries =
    resolvedVariant === "stream"
      ? entries
          .filter(isStreamEntry)
          .filter((entry) => !filterLevel || entry.level === filterLevel)
          .filter(
            (entry) => !filterText || entry.message.toLowerCase().includes(filterText.toLowerCase()),
          )
          .slice(-maxEntries)
      : [];

  $: levelCounts = {
    info: entries.filter((entry) => isStreamEntry(entry) && entry.level === "info").length,
    warn: entries.filter((entry) => isStreamEntry(entry) && entry.level === "warn").length,
    error: entries.filter((entry) => isStreamEntry(entry) && entry.level === "error").length,
  };

  $: auditEntries = resolvedVariant === "audit" ? entries.filter(isAuditEntry) : [];
  $: hasAuditToolbar = filters.length > 0 || !!onRefresh || !!onExport;
  $: hasActiveFilters = Object.values(filterValues).some((value) => value?.trim());
  $: totalPages = total ? Math.max(1, Math.ceil(total / pageSize)) : 1;
  $: showPagination = total !== undefined && total > pageSize;

  function isStreamEntry(entry: LogEntry): entry is Extract<LogEntry, { message: string }> {
    return "message" in entry;
  }

  function isAuditEntry(entry: LogEntry): entry is AuditLogEntry {
    return "occurredAt" in entry;
  }

  function handleScroll(): void {
    if (!scrollContainer) return;
    const { scrollTop, scrollHeight, clientHeight } = scrollContainer;
    isUserScrolled = scrollHeight - scrollTop - clientHeight > 32;
  }

  function scrollToBottom(): void {
    isUserScrolled = false;
    queueMicrotask(() => {
      if (scrollContainer) {
        scrollContainer.scrollTop = scrollContainer.scrollHeight;
      }
    });
  }

  $: if (resolvedVariant === "stream" && autoScroll && !isUserScrolled && scrollContainer) {
    scrollContainer.scrollTop = scrollContainer.scrollHeight;
  }

  function formatStreamTimestamp(ts: Date | string | number): string {
    const date = ts instanceof Date ? ts : new Date(ts);
    return date.toLocaleTimeString("en-US", { hour12: false, fractionalSecondDigits: 3 });
  }

  function formatRelativeTime(value: string): string {
    const timestamp = new Date(value).getTime();
    const diffMs = timestamp - Date.now();
    const diffMinutes = Math.round(diffMs / 60000);
    const diffHours = Math.round(diffMs / 3600000);
    const diffDays = Math.round(diffMs / 86400000);
    const formatter = new Intl.RelativeTimeFormat("en", { numeric: "auto" });

    if (Math.abs(diffMinutes) < 60) {
      return formatter.format(diffMinutes, "minute");
    }

    if (Math.abs(diffHours) < 24) {
      return formatter.format(diffHours, "hour");
    }

    return formatter.format(diffDays, "day");
  }

  function resolveActionType(action: string): LogActionType {
    if (getActionType) return getActionType(action);

    const normalized = action.toLowerCase();
    if (normalized.includes("create")) return "create";
    if (normalized.includes("update") || normalized.includes("edit")) return "update";
    if (normalized.includes("delete") || normalized.includes("remove")) return "delete";
    if (normalized.includes("restore") || normalized.includes("recover")) return "restore";
    if (normalized.includes("upload")) return "upload";
    if (normalized === "login" || normalized === "sign_in") return "login";
    if (normalized === "logout" || normalized === "sign_out") return "logout";
    if (
      normalized.includes("role") ||
      normalized.includes("suspend") ||
      normalized.includes("permission")
    ) {
      return "security";
    }
    return "other";
  }

  function resolveActionTone(actionType: LogActionType): "neutral" | "info" | "success" | "warning" | "danger" {
    switch (actionType) {
      case "create":
      case "restore":
        return "success";
      case "delete":
        return "danger";
      case "update":
      case "upload":
        return "info";
      case "security":
        return "warning";
      default:
        return "neutral";
    }
  }

  function resolveActionLabel(action: string): string {
    return formatAction ? formatAction(action) : action.replace(/_/g, " ");
  }

  function resolveResourceLabel(resourceType: string): string {
    return formatResourceType ? formatResourceType(resourceType) : resourceType.replace(/_/g, " ");
  }

  function resolveActorName(entry: AuditLogEntry): string {
    if (!entry.actor) return "System";
    return entry.actor.name ?? entry.actor.email ?? `User ${entry.actor.id.slice(0, 8)}`;
  }

  function requestPage(pageValue: number): void {
    if (!onPageChange) return;
    const nextPage = Math.max(1, Math.min(pageValue, totalPages));
    if (nextPage !== page) {
      onPageChange(nextPage);
    }
  }
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  {#if resolvedVariant === "audit"}
    <section
      class="poodle-log-list poodle-log-list--audit"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      aria-label={ariaLabel}
    >
      {#if hasAuditToolbar}
        <div class="poodle-log-list__toolbar">
          {#if filters.length > 0}
            <div class="poodle-log-list__filters">
              {#each filters as filter}
                <div class="poodle-log-list__filter">
                  {#if filter.type === "select" && filter.options}
                    <Field id={`log-filter-${filter.field}`} label={filter.label} let:describedBy>
                      <Select
                        id={`log-filter-${filter.field}`}
                        name={filter.field}
                        options={[{ value: "", label: filter.placeholder ?? "All" }, ...filter.options]}
                        value={filterValues[filter.field] ?? ""}
                        describedBy={describedBy}
                        on:valueChange={(event) => onFilterChange?.(filter.field, event.detail.value)}
                      />
                    </Field>
                  {:else if filter.type === "date"}
                    <Field id={`log-filter-${filter.field}`} label={filter.label} let:describedBy>
                      <TextInput
                        id={`log-filter-${filter.field}`}
                        type="date"
                        value={filterValues[filter.field] ?? ""}
                        describedBy={describedBy}
                        on:valueChange={(event) => onFilterChange?.(filter.field, event.detail.value)}
                      />
                    </Field>
                  {/if}
                </div>
              {/each}

              {#if hasActiveFilters && onClearFilters}
                <Button variant="ghost" size="sm" leadingIcon="x" on:click={onClearFilters}>
                  Clear
                </Button>
              {/if}
            </div>
          {/if}

          <div class="poodle-log-list__toolbar-actions">
            {#if onRefresh}
              <IconButton
                icon="refresh-cw"
                variant="ghost"
                size="sm"
                loading={loading}
                ariaLabel="Refresh"
                on:click={onRefresh}
              />
            {/if}
            {#if onExport}
              <Button variant="ghost" size="sm" leadingIcon="download" disabled={loading} on:click={onExport}>
                Export
              </Button>
            {/if}
          </div>
        </div>
      {/if}

      <div class="poodle-log-list__content">
        {#if loading && auditEntries.length === 0}
          <div class="poodle-log-list__status">
            <p>Loading log entries...</p>
          </div>
        {:else if error}
          <div class="poodle-log-list__status poodle-log-list__status--error" role="alert">
            <p>{error}</p>
          </div>
        {:else if auditEntries.length === 0}
          <div class="poodle-log-list__status">
            <p>{emptyMessage}</p>
          </div>
        {:else}
          <ul class="poodle-log-list__audit-entries">
            {#each auditEntries as entry (entry.id)}
              {@const actionType = resolveActionType(entry.action)}
              {@const actorName = resolveActorName(entry)}
              {@const resourceHref = getResourceHref?.(entry.resourceType, entry.resourceId, entry.action) ?? null}
              <li class="poodle-log-list__audit-entry">
                <div class="poodle-log-list__audit-icon" data-action-type={actionType}>
                  {#if actionIcon}
                    {@render actionIcon(actionType)}
                  {:else}
                    {actionType.slice(0, 1).toUpperCase()}
                  {/if}
                </div>

                <div class="poodle-log-list__audit-body">
                  <div class="poodle-log-list__audit-main">
                    <span class="poodle-log-list__audit-actor">
                      {#if entry.actor && getActorHref}
                        <a href={getActorHref(entry.actor)} class="poodle-log-list__audit-link">{actorName}</a>
                      {:else}
                        {actorName}
                      {/if}
                    </span>

                    <Pill tone={resolveActionTone(actionType)} appearance="badge" size="sm">
                      {resolveActionLabel(entry.action)}
                    </Pill>

                    {#if resourceHref}
                      <a href={resourceHref} class="poodle-log-list__audit-link poodle-log-list__audit-resource">
                        {resolveResourceLabel(entry.resourceType)}
                        {#if entry.resourceLabel}
                          <span class="poodle-log-list__audit-resource-label">"{entry.resourceLabel}"</span>
                        {/if}
                      </a>
                    {:else}
                      <span class="poodle-log-list__audit-resource">
                        {resolveResourceLabel(entry.resourceType)}
                        {#if entry.resourceLabel}
                          <span class="poodle-log-list__audit-resource-label">"{entry.resourceLabel}"</span>
                        {/if}
                      </span>
                    {/if}

                    <time class="poodle-log-list__audit-time" datetime={entry.occurredAt}>
                      {formatRelativeTime(entry.occurredAt)}
                    </time>
                  </div>

                  {#if entryDetails}
                    <div class="poodle-log-list__audit-details">
                      {@render entryDetails(entry)}
                    </div>
                  {/if}
                </div>
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      {#if showPagination}
        <div class="poodle-log-list__pagination">
          <span class="poodle-log-list__pagination-info">
            Showing {(page - 1) * pageSize + 1}-{Math.min(page * pageSize, total ?? 0)} of {total}
          </span>

          <div class="poodle-log-list__pagination-controls">
            <IconButton
              icon="chevron-left"
              variant="ghost"
              size="sm"
              ariaLabel="Previous page"
              disabled={page <= 1 || loading}
              on:click={() => requestPage(page - 1)}
            />
            <span class="poodle-log-list__pagination-page">Page {page} of {totalPages}</span>
            <IconButton
              icon="chevron-right"
              variant="ghost"
              size="sm"
              ariaLabel="Next page"
              disabled={page >= totalPages || loading}
              on:click={() => requestPage(page + 1)}
            />
          </div>
        </div>
      {/if}
    </section>
  {:else}
    <div
      class="poodle-log-list poodle-log-list--stream"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role="log"
      aria-label={ariaLabel}
    >
      <div class="poodle-log-list__toolbar">
        <div class="poodle-log-list__stream-filters">
          <button
            type="button"
            class="poodle-log-list__filter-btn"
            class:poodle-active={filterLevel === null}
            on:click={() => (filterLevel = null)}
          >
            All <span class="poodle-log-list__count">{entries.length}</span>
          </button>
          <button
            type="button"
            class="poodle-log-list__filter-btn poodle-log-list__filter-btn--info"
            class:poodle-active={filterLevel === "info"}
            on:click={() => (filterLevel = filterLevel === "info" ? null : "info")}
          >
            Info <span class="poodle-log-list__count">{levelCounts.info}</span>
          </button>
          <button
            type="button"
            class="poodle-log-list__filter-btn poodle-log-list__filter-btn--warn"
            class:poodle-active={filterLevel === "warn"}
            on:click={() => (filterLevel = filterLevel === "warn" ? null : "warn")}
          >
            Warn <span class="poodle-log-list__count">{levelCounts.warn}</span>
          </button>
          <button
            type="button"
            class="poodle-log-list__filter-btn poodle-log-list__filter-btn--error"
            class:poodle-active={filterLevel === "error"}
            on:click={() => (filterLevel = filterLevel === "error" ? null : "error")}
          >
            Error <span class="poodle-log-list__count">{levelCounts.error}</span>
          </button>
        </div>

        <input
          type="text"
          class="poodle-log-list__search"
          placeholder="Filter logs..."
          bind:value={filterText}
          aria-label="Filter log messages"
        />
      </div>

      <div class="poodle-log-list__scroll" bind:this={scrollContainer} on:scroll={handleScroll}>
        {#if displayEntries.length === 0}
          <div class="poodle-log-list__empty">No log entries{filterLevel || filterText ? " match filters" : ""}</div>
        {:else}
          {#each displayEntries as entry (entry.id ?? `${entry.timestamp}-${entry.message}`)}
            <div class="poodle-log-list__entry" data-level={entry.level}>
              <time class="poodle-log-list__ts">{formatStreamTimestamp(entry.timestamp)}</time>
              <span class="poodle-log-list__level">{entry.level.toUpperCase()}</span>
              <span class="poodle-log-list__msg">{entry.message}</span>
            </div>
          {/each}
        {/if}
      </div>

      {#if isUserScrolled && autoScroll}
        <button
          type="button"
          class="poodle-log-list__scroll-btn"
          on:click={scrollToBottom}
          aria-label="Scroll to latest"
        >
          New entries
        </button>
      {/if}
    </div>
  {/if}
</UiPresentationProvider>

<style>
  .poodle-log-list {
    display: flex;
    flex-direction: column;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-panel);
    overflow: hidden;
    position: relative;
  }

  .poodle-log-list__toolbar {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
    flex-wrap: wrap;
  }

  .poodle-log-list--stream .poodle-log-list__toolbar {
    align-items: center;
  }

  .poodle-log-list__stream-filters,
  .poodle-log-list__filters {
    display: flex;
    align-items: flex-end;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .poodle-log-list__filter {
    min-width: 10rem;
  }

  .poodle-log-list__toolbar-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-left: auto;
  }

  .poodle-log-list__filter-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.1875rem 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-label-size);
    font-family: var(--poodle-typography-code-family);
    line-height: 1;
  }

  .poodle-log-list__filter-btn.poodle-active {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 42%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .poodle-log-list__search {
    flex: 1;
    min-width: 10rem;
    padding: 0.1875rem 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-size: var(--poodle-typography-label-size);
    font-family: var(--poodle-typography-code-family);
    outline: none;
  }

  .poodle-log-list__search:focus {
    border-color: var(--poodle-color-accent-focusRing);
  }

  .poodle-log-list__content {
    min-height: 12rem;
  }

  .poodle-log-list__status {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 12rem;
    padding: 2rem 1rem;
    color: var(--poodle-color-text-secondary);
    text-align: center;
  }

  .poodle-log-list__status--error {
    color: var(--poodle-color-status-danger, #ef4444);
  }

  .poodle-log-list__audit-entries {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .poodle-log-list__audit-entry {
    display: flex;
    gap: 0.75rem;
    padding: 0.875rem 1rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
  }

  .poodle-log-list__audit-entry:last-child {
    border-bottom: none;
  }

  .poodle-log-list__audit-icon {
    width: 1.75rem;
    height: 1.75rem;
    border-radius: 999px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 88%, transparent);
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 700;
  }

  .poodle-log-list__audit-icon[data-action-type="create"],
  .poodle-log-list__audit-icon[data-action-type="restore"] {
    background: color-mix(in srgb, var(--poodle-color-status-success, #22c55e) 16%, transparent);
    color: var(--poodle-color-status-success, #22c55e);
  }

  .poodle-log-list__audit-icon[data-action-type="delete"] {
    background: color-mix(in srgb, var(--poodle-color-status-danger, #ef4444) 16%, transparent);
    color: var(--poodle-color-status-danger, #ef4444);
  }

  .poodle-log-list__audit-icon[data-action-type="update"],
  .poodle-log-list__audit-icon[data-action-type="upload"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    color: var(--poodle-color-accent-base);
  }

  .poodle-log-list__audit-icon[data-action-type="security"] {
    background: color-mix(in srgb, var(--poodle-color-status-warning, #f59e0b) 16%, transparent);
    color: var(--poodle-color-status-warning, #f59e0b);
  }

  .poodle-log-list__audit-body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .poodle-log-list__audit-main {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
    font-size: 0.875rem;
    line-height: 1.4;
  }

  .poodle-log-list__audit-actor {
    font-weight: 600;
    color: var(--poodle-color-text-primary);
  }

  .poodle-log-list__audit-link {
    color: inherit;
    text-decoration: none;
  }

  .poodle-log-list__audit-link:hover {
    text-decoration: underline;
  }

  .poodle-log-list__audit-resource {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-log-list__audit-resource-label {
    font-style: italic;
    color: var(--poodle-color-text-primary);
  }

  .poodle-log-list__audit-details {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  .poodle-log-list__audit-time {
    margin-left: auto;
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .poodle-log-list__pagination {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .poodle-log-list__pagination-info,
  .poodle-log-list__pagination-page {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  .poodle-log-list__pagination-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .poodle-log-list__scroll {
    max-height: 20rem;
    overflow-y: auto;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1.6;
  }

  .poodle-log-list__entry {
    display: flex;
    gap: 0.625rem;
    padding: 0.125rem 0.5rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
  }

  .poodle-log-list__entry[data-level="warn"] {
    background: color-mix(in srgb, var(--poodle-color-status-warning, #f59e0b) 8%, transparent);
  }

  .poodle-log-list__entry[data-level="error"] {
    background: color-mix(in srgb, var(--poodle-color-status-danger, #ef4444) 8%, transparent);
  }

  .poodle-log-list__ts,
  .poodle-log-list__level {
    flex-shrink: 0;
  }

  .poodle-log-list__level {
    width: 3rem;
    text-align: right;
    font-weight: 600;
  }

  .poodle-log-list__msg {
    flex: 1;
    min-width: 0;
    word-break: break-word;
  }

  .poodle-log-list__empty {
    padding: 1rem;
    color: var(--poodle-color-text-secondary);
    text-align: center;
  }

  .poodle-log-list__scroll-btn {
    position: absolute;
    bottom: 0.5rem;
    left: 50%;
    transform: translateX(-50%);
    border: none;
    border-radius: 999px;
    padding: 0.375rem 0.75rem;
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-accent-onBase, white);
    box-shadow: var(--poodle-shadow-md);
    cursor: pointer;
  }
</style>

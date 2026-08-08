<script lang="ts">
  import "@inflatable-cookie/poodle-styles/log-list.css";
  import type { Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as Field } from "./Field.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { default as Select } from "./Select.svelte";
  import { default as TextLink } from "./TextLink.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    AuditLogEntry,
    ControlDensity,
    ControlSize,
    LogActionType,
    LogActor,
    LogEntry,
    LogFilter,
    LogLevel,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    entries?: LogEntry[];
    variant?: "auto" | "stream" | "audit";
    maxEntries?: number;
    autoScroll?: boolean;
    filterLevel?: LogLevel | null;
    filterText?: string;
    ariaLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    loading?: boolean;
    error?: string | null;
    emptyMessage?: string;
    filters?: LogFilter[];
    filterValues?: Record<string, string>;
    page?: number;
    pageSize?: number;
    total?: number | undefined;
    onFilterChange?: ((field: string, value: string) => void) | undefined;
    onClearFilters?: (() => void) | undefined;
    onPageChange?: ((page: number) => void) | undefined;
    onRefresh?: (() => void) | undefined;
    onExport?: (() => void) | undefined;
    actionIcon?: Snippet<[LogActionType]> | undefined;
    entryDetails?: Snippet<[AuditLogEntry]> | undefined;
    getActionType?: ((action: string) => LogActionType) | undefined;
    formatAction?: ((action: string) => string) | undefined;
    formatResourceType?: ((resourceType: string) => string) | undefined;
    getActorHref?: ((actor: LogActor) => string) | undefined;
    getResourceHref?:
      | ((resourceType: string, resourceId: string, action: string) => string | null)
      | undefined;
  }

  let {
    entries = [],
    variant = "auto",
    maxEntries = 500,
    autoScroll = true,
    filterLevel = $bindable<LogLevel | null>(null),
    filterText = $bindable(""),
    ariaLabel = "Log output",
    size = null,
    sizeRole = "control",
    density = null,
    loading = false,
    error = null,
    emptyMessage = "No log entries found",
    filters = [],
    filterValues = {},
    page = 1,
    pageSize = 50,
    total = undefined,
    onFilterChange = undefined,
    onClearFilters = undefined,
    onPageChange = undefined,
    onRefresh = undefined,
    onExport = undefined,
    actionIcon = undefined,
    entryDetails = undefined,
    getActionType = undefined,
    formatAction = undefined,
    formatResourceType = undefined,
    getActorHref = undefined,
    getResourceHref = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let scrollContainer = $state<HTMLDivElement | null>(null);
  let isUserScrolled = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedVariant = $derived(
    variant === "auto" ? (entries.some(isAuditEntry) ? "audit" : "stream") : variant,
  );
  const displayEntries = $derived(
    resolvedVariant === "stream"
      ? entries
          .filter(isStreamEntry)
          .filter((entry) => !filterLevel || entry.level === filterLevel)
          .filter(
            (entry) => !filterText || entry.message.toLowerCase().includes(filterText.toLowerCase()),
          )
          .slice(-maxEntries)
      : [],
  );
  const levelCounts = $derived({
    info: entries.filter((entry) => isStreamEntry(entry) && entry.level === "info").length,
    warn: entries.filter((entry) => isStreamEntry(entry) && entry.level === "warn").length,
    error: entries.filter((entry) => isStreamEntry(entry) && entry.level === "error").length,
  });
  const auditEntries = $derived(resolvedVariant === "audit" ? entries.filter(isAuditEntry) : []);
  const hasAuditToolbar = $derived(filters.length > 0 || !!onRefresh || !!onExport);
  const hasActiveFilters = $derived(Object.values(filterValues).some((value) => value?.trim()));
  const totalPages = $derived(total ? Math.max(1, Math.ceil(total / pageSize)) : 1);
  const showPagination = $derived(total !== undefined && total > pageSize);

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

  $effect(() => {
    if (resolvedVariant === "stream" && autoScroll && !isUserScrolled && scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight;
    }
  });

  function formatStreamTimestamp(timestamp: Date | string | number): string {
    const date = timestamp instanceof Date ? timestamp : new Date(timestamp);
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
                    <Field id={`log-filter-${filter.field}`} label={filter.label}>
                      {#snippet control({ describedBy })}
                      <Select
                        id={`log-filter-${filter.field}`}
                        name={filter.field}
                        options={[{ value: "", label: filter.placeholder ?? "All" }, ...(filter.options ?? [])]}
                        value={filterValues[filter.field] ?? ""}
                        describedBy={describedBy}
                        onValueChange={(nextValue) => onFilterChange?.(filter.field, nextValue)}
                      />
                      {/snippet}
                    </Field>
                  {:else if filter.type === "date"}
                    <Field id={`log-filter-${filter.field}`} label={filter.label}>
                      {#snippet control({ describedBy })}
                      <TextInput
                        id={`log-filter-${filter.field}`}
                        type="date"
                        value={filterValues[filter.field] ?? ""}
                        describedBy={describedBy}
                        onValueChange={(nextValue) => onFilterChange?.(filter.field, nextValue)}
                      />
                      {/snippet}
                    </Field>
                  {/if}
                </div>
              {/each}

              {#if hasActiveFilters && onClearFilters}
                <Button variant="ghost" size="sm" leadingIcon="x" onClick={onClearFilters}>
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
                onClick={onRefresh}
              />
            {/if}
            {#if onExport}
              <Button variant="ghost" size="sm" leadingIcon="download" disabled={loading} onClick={onExport}>
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
                        <TextLink href={getActorHref(entry.actor)} className="poodle-log-list__audit-link" tone="inherit">
                          {actorName}
                        </TextLink>
                      {:else}
                        {actorName}
                      {/if}
                    </span>

                    <Pill tone={resolveActionTone(actionType)} appearance="badge" size="sm">
                      {resolveActionLabel(entry.action)}
                    </Pill>

                    {#if resourceHref}
                      <TextLink href={resourceHref} className="poodle-log-list__audit-link poodle-log-list__audit-resource" tone="inherit">
                        {resolveResourceLabel(entry.resourceType)}
                        {#if entry.resourceLabel}
                          <span class="poodle-log-list__audit-resource-label">"{entry.resourceLabel}"</span>
                        {/if}
                      </TextLink>
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
              onClick={() => requestPage(page - 1)}
            />
            <span class="poodle-log-list__pagination-page">Page {page} of {totalPages}</span>
            <IconButton
              icon="chevron-right"
              variant="ghost"
              size="sm"
              ariaLabel="Next page"
              disabled={page >= totalPages || loading}
              onClick={() => requestPage(page + 1)}
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
            onclick={() => (filterLevel = null)}
          >
            All <span class="poodle-log-list__count">{entries.length}</span>
          </button>
          <button
            type="button"
            class="poodle-log-list__filter-btn poodle-log-list__filter-btn--info"
            class:poodle-active={filterLevel === "info"}
            onclick={() => (filterLevel = filterLevel === "info" ? null : "info")}
          >
            Info <span class="poodle-log-list__count">{levelCounts.info}</span>
          </button>
          <button
            type="button"
            class="poodle-log-list__filter-btn poodle-log-list__filter-btn--warn"
            class:poodle-active={filterLevel === "warn"}
            onclick={() => (filterLevel = filterLevel === "warn" ? null : "warn")}
          >
            Warn <span class="poodle-log-list__count">{levelCounts.warn}</span>
          </button>
          <button
            type="button"
            class="poodle-log-list__filter-btn poodle-log-list__filter-btn--error"
            class:poodle-active={filterLevel === "error"}
            onclick={() => (filterLevel = filterLevel === "error" ? null : "error")}
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

      <div class="poodle-log-list__scroll" bind:this={scrollContainer} onscroll={handleScroll}>
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
          onclick={scrollToBottom}
          aria-label="Scroll to latest"
        >
          New entries
        </button>
      {/if}
    </div>
  {/if}
</UiPresentationProvider>


import { useEffect, useRef, useState, type ReactNode } from "react";

import "@poodle/styles/log-list.css";

import { Button } from "./Button";
import { Field } from "./Field";
import { IconButton } from "./IconButton";
import { Pill } from "./Pill";
import { resolveSemanticControlSize, UiPresentationProvider, useUiPresentation } from "./presentation";
import { Select } from "./Select";
import { TextInput } from "./TextInput";
import { TextLink } from "./TextLink";
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
  StreamLogEntry,
} from "./types";

export interface LogListProps {
  entries?: LogEntry[];
  variant?: "auto" | "stream" | "audit";
  maxEntries?: number;
  autoScroll?: boolean;
  filterLevel?: LogLevel | null;
  defaultFilterLevel?: LogLevel | null;
  filterText?: string;
  defaultFilterText?: string;
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
  total?: number;
  onFilterChange?: (field: string, value: string) => void;
  onClearFilters?: () => void;
  onPageChange?: (page: number) => void;
  onFilterLevelChange?: (level: LogLevel | null) => void;
  onFilterTextChange?: (text: string) => void;
  onRefresh?: () => void;
  onExport?: () => void;
  actionIcon?: (actionType: LogActionType) => ReactNode;
  entryDetails?: (entry: AuditLogEntry) => ReactNode;
  getActionType?: (action: string) => LogActionType;
  formatAction?: (action: string) => string;
  formatResourceType?: (resourceType: string) => string;
  getActorHref?: (actor: LogActor) => string;
  getResourceHref?: (resourceType: string, resourceId: string, action: string) => string | null;
}

function isStreamEntry(entry: LogEntry): entry is StreamLogEntry {
  return "message" in entry;
}

function isAuditEntry(entry: LogEntry): entry is AuditLogEntry {
  return "occurredAt" in entry;
}

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

  if (Math.abs(diffMinutes) < 60) return formatter.format(diffMinutes, "minute");
  if (Math.abs(diffHours) < 24) return formatter.format(diffHours, "hour");
  return formatter.format(diffDays, "day");
}

function defaultActionType(action: string): LogActionType {
  const normalized = action.toLowerCase();
  if (normalized.includes("create")) return "create";
  if (normalized.includes("update") || normalized.includes("edit")) return "update";
  if (normalized.includes("delete") || normalized.includes("remove")) return "delete";
  if (normalized.includes("restore") || normalized.includes("recover")) return "restore";
  if (normalized.includes("upload")) return "upload";
  if (normalized === "login" || normalized === "sign_in") return "login";
  if (normalized === "logout" || normalized === "sign_out") return "logout";
  if (normalized.includes("role") || normalized.includes("suspend") || normalized.includes("permission")) {
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

export function LogList({
  entries = [],
  variant = "auto",
  maxEntries = 500,
  autoScroll = true,
  filterLevel,
  defaultFilterLevel = null,
  filterText,
  defaultFilterText = "",
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
  total,
  onFilterChange,
  onClearFilters,
  onPageChange,
  onFilterLevelChange,
  onFilterTextChange,
  onRefresh,
  onExport,
  actionIcon,
  entryDetails,
  getActionType,
  formatAction,
  formatResourceType,
  getActorHref,
  getResourceHref,
}: LogListProps) {
  const uiPresentation = useUiPresentation();

  const scrollContainerRef = useRef<HTMLDivElement | null>(null);
  const [isUserScrolled, setIsUserScrolled] = useState(false);
  const [uncontrolledFilterLevel, setUncontrolledFilterLevel] = useState<LogLevel | null>(defaultFilterLevel);
  const [uncontrolledFilterText, setUncontrolledFilterText] = useState(defaultFilterText);

  const currentFilterLevel = filterLevel !== undefined ? filterLevel : uncontrolledFilterLevel;
  const currentFilterText = filterText !== undefined ? filterText : uncontrolledFilterText;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedVariant = variant === "auto" ? (entries.some(isAuditEntry) ? "audit" : "stream") : variant;
  const displayEntries =
    resolvedVariant === "stream"
      ? entries
          .filter(isStreamEntry)
          .filter((entry) => !currentFilterLevel || entry.level === currentFilterLevel)
          .filter((entry) => !currentFilterText || entry.message.toLowerCase().includes(currentFilterText.toLowerCase()))
          .slice(-maxEntries)
      : [];
  const levelCounts = {
    info: entries.filter((entry) => isStreamEntry(entry) && entry.level === "info").length,
    warn: entries.filter((entry) => isStreamEntry(entry) && entry.level === "warn").length,
    error: entries.filter((entry) => isStreamEntry(entry) && entry.level === "error").length,
  };
  const auditEntries = resolvedVariant === "audit" ? entries.filter(isAuditEntry) : [];
  const hasAuditToolbar = filters.length > 0 || !!onRefresh || !!onExport;
  const hasActiveFilters = Object.values(filterValues).some((value) => value?.trim());
  const totalPages = total ? Math.max(1, Math.ceil(total / pageSize)) : 1;
  const showPagination = total !== undefined && total > pageSize;

  function setFilterLevel(next: LogLevel | null): void {
    if (filterLevel === undefined) setUncontrolledFilterLevel(next);
    onFilterLevelChange?.(next);
  }

  useEffect(() => {
    const container = scrollContainerRef.current;
    if (resolvedVariant === "stream" && autoScroll && !isUserScrolled && container) {
      container.scrollTop = container.scrollHeight;
    }
  });

  const resolveActionType = getActionType ?? defaultActionType;
  const resolveActionLabel = (action: string) => (formatAction ? formatAction(action) : action.replace(/_/g, " "));
  const resolveResourceLabel = (resourceType: string) =>
    formatResourceType ? formatResourceType(resourceType) : resourceType.replace(/_/g, " ");
  const resolveActorName = (entry: AuditLogEntry) =>
    !entry.actor ? "System" : (entry.actor.name ?? entry.actor.email ?? `User ${entry.actor.id.slice(0, 8)}`);

  function requestPage(pageValue: number): void {
    if (!onPageChange) return;
    const nextPage = Math.max(1, Math.min(pageValue, totalPages));
    if (nextPage !== page) onPageChange(nextPage);
  }

  const levelFilterButton = (label: string, level: LogLevel | null, count: number, modifier?: string) => (
    <button
      type="button"
      className={[
        "poodle-log-list__filter-btn",
        modifier ? `poodle-log-list__filter-btn--${modifier}` : "",
        currentFilterLevel === level ? "poodle-active" : "",
      ]
        .filter(Boolean)
        .join(" ")}
      onClick={() => setFilterLevel(level === null ? null : currentFilterLevel === level ? null : level)}
    >
      {label} <span className="poodle-log-list__count">{count}</span>
    </button>
  );

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      {resolvedVariant === "audit" ? (
        <section
          className="poodle-log-list poodle-log-list--audit"
          data-size={resolvedSize}
          data-density={resolvedDensity}
          aria-label={ariaLabel}
        >
          {hasAuditToolbar ? (
            <div className="poodle-log-list__toolbar">
              {filters.length > 0 ? (
                <div className="poodle-log-list__filters">
                  {filters.map((filter) => (
                    <div key={filter.field} className="poodle-log-list__filter">
                      {filter.type === "select" && filter.options ? (
                        <Field
                          id={`log-filter-${filter.field}`}
                          label={filter.label}
                          control={({ describedBy }) => (
                            <Select
                              id={`log-filter-${filter.field}`}
                              name={filter.field}
                              options={[{ value: "", label: filter.placeholder ?? "All" }, ...(filter.options ?? [])]}
                              value={filterValues[filter.field] ?? ""}
                              describedBy={describedBy}
                              onValueChange={(nextValue) => onFilterChange?.(filter.field, nextValue)}
                            />
                          )}
                        />
                      ) : filter.type === "date" ? (
                        <Field
                          id={`log-filter-${filter.field}`}
                          label={filter.label}
                          control={({ describedBy }) => (
                            <TextInput
                              id={`log-filter-${filter.field}`}
                              type="date"
                              value={filterValues[filter.field] ?? ""}
                              describedBy={describedBy}
                              onValueChange={(nextValue) => onFilterChange?.(filter.field, nextValue)}
                            />
                          )}
                        />
                      ) : null}
                    </div>
                  ))}

                  {hasActiveFilters && onClearFilters ? (
                    <Button variant="ghost" size="sm" leadingIcon="x" onClick={onClearFilters}>
                      Clear
                    </Button>
                  ) : null}
                </div>
              ) : null}

              <div className="poodle-log-list__toolbar-actions">
                {onRefresh ? (
                  <IconButton icon="refresh-cw" variant="ghost" size="sm" loading={loading} ariaLabel="Refresh" onClick={onRefresh} />
                ) : null}
                {onExport ? (
                  <Button variant="ghost" size="sm" leadingIcon="download" disabled={loading} onClick={onExport}>
                    Export
                  </Button>
                ) : null}
              </div>
            </div>
          ) : null}

          <div className="poodle-log-list__content">
            {loading && auditEntries.length === 0 ? (
              <div className="poodle-log-list__status">
                <p>Loading log entries...</p>
              </div>
            ) : error ? (
              <div className="poodle-log-list__status poodle-log-list__status--error" role="alert">
                <p>{error}</p>
              </div>
            ) : auditEntries.length === 0 ? (
              <div className="poodle-log-list__status">
                <p>{emptyMessage}</p>
              </div>
            ) : (
              <ul className="poodle-log-list__audit-entries">
                {auditEntries.map((entry) => {
                  const actionType = resolveActionType(entry.action);
                  const actorName = resolveActorName(entry);
                  const resourceHref = getResourceHref?.(entry.resourceType, entry.resourceId, entry.action) ?? null;
                  const resourceContent = (
                    <>
                      {resolveResourceLabel(entry.resourceType)}
                      {/* Svelte's template keeps the newline as a space here. */}
                      {entry.resourceLabel ? (
                        <>
                          {" "}
                          <span className="poodle-log-list__audit-resource-label">"{entry.resourceLabel}"</span>
                        </>
                      ) : null}
                    </>
                  );
                  return (
                    <li key={entry.id} className="poodle-log-list__audit-entry">
                      <div className="poodle-log-list__audit-icon" data-action-type={actionType}>
                        {actionIcon ? actionIcon(actionType) : actionType.slice(0, 1).toUpperCase()}
                      </div>

                      <div className="poodle-log-list__audit-body">
                        <div className="poodle-log-list__audit-main">
                          <span className="poodle-log-list__audit-actor">
                            {entry.actor && getActorHref ? (
                              <TextLink href={getActorHref(entry.actor)} className="poodle-log-list__audit-link" tone="inherit">
                                {actorName}
                              </TextLink>
                            ) : (
                              actorName
                            )}
                          </span>

                          <Pill tone={resolveActionTone(actionType)} appearance="badge" size="sm">
                            {resolveActionLabel(entry.action)}
                          </Pill>

                          {resourceHref ? (
                            <TextLink
                              href={resourceHref}
                              className="poodle-log-list__audit-link poodle-log-list__audit-resource"
                              tone="inherit"
                            >
                              {resourceContent}
                            </TextLink>
                          ) : (
                            <span className="poodle-log-list__audit-resource">{resourceContent}</span>
                          )}

                          <time className="poodle-log-list__audit-time" dateTime={entry.occurredAt}>
                            {formatRelativeTime(entry.occurredAt)}
                          </time>
                        </div>

                        {entryDetails ? <div className="poodle-log-list__audit-details">{entryDetails(entry)}</div> : null}
                      </div>
                    </li>
                  );
                })}
              </ul>
            )}
          </div>

          {showPagination ? (
            <div className="poodle-log-list__pagination">
              <span className="poodle-log-list__pagination-info">
                Showing {(page - 1) * pageSize + 1}-{Math.min(page * pageSize, total ?? 0)} of {total}
              </span>

              <div className="poodle-log-list__pagination-controls">
                <IconButton
                  icon="chevron-left"
                  variant="ghost"
                  size="sm"
                  ariaLabel="Previous page"
                  disabled={page <= 1 || loading}
                  onClick={() => requestPage(page - 1)}
                />
                <span className="poodle-log-list__pagination-page">
                  Page {page} of {totalPages}
                </span>
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
          ) : null}
        </section>
      ) : (
        <div
          className="poodle-log-list poodle-log-list--stream"
          data-size={resolvedSize}
          data-density={resolvedDensity}
          role="log"
          aria-label={ariaLabel}
        >
          <div className="poodle-log-list__toolbar">
            <div className="poodle-log-list__stream-filters">
              {levelFilterButton("All", null, entries.length)}
              {levelFilterButton("Info", "info", levelCounts.info, "info")}
              {levelFilterButton("Warn", "warn", levelCounts.warn, "warn")}
              {levelFilterButton("Error", "error", levelCounts.error, "error")}
            </div>

            <input
              type="text"
              className="poodle-log-list__search"
              placeholder="Filter logs..."
              value={currentFilterText}
              onChange={(event) => {
                if (filterText === undefined) setUncontrolledFilterText(event.currentTarget.value);
                onFilterTextChange?.(event.currentTarget.value);
              }}
              aria-label="Filter log messages"
            />
          </div>

          <div
            className="poodle-log-list__scroll"
            ref={scrollContainerRef}
            onScroll={() => {
              const container = scrollContainerRef.current;
              if (!container) return;
              const { scrollTop, scrollHeight, clientHeight } = container;
              setIsUserScrolled(scrollHeight - scrollTop - clientHeight > 32);
            }}
          >
            {displayEntries.length === 0 ? (
              <div className="poodle-log-list__empty">
                No log entries{currentFilterLevel || currentFilterText ? " match filters" : ""}
              </div>
            ) : (
              displayEntries.map((entry, index) => (
                <div key={entry.id ?? `${entry.timestamp}-${index}`} className="poodle-log-list__entry" data-level={entry.level}>
                  <time className="poodle-log-list__ts">{formatStreamTimestamp(entry.timestamp)}</time>
                  <span className="poodle-log-list__level">{entry.level.toUpperCase()}</span>
                  <span className="poodle-log-list__msg">{entry.message}</span>
                </div>
              ))
            )}
          </div>

          {isUserScrolled && autoScroll ? (
            <button
              type="button"
              className="poodle-log-list__scroll-btn"
              onClick={() => {
                setIsUserScrolled(false);
                queueMicrotask(() => {
                  const container = scrollContainerRef.current;
                  if (container) container.scrollTop = container.scrollHeight;
                });
              }}
              aria-label="Scroll to latest"
            >
              New entries
            </button>
          ) : null}
        </div>
      )}
    </UiPresentationProvider>
  );
}

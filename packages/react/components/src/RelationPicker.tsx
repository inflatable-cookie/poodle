import "@inflatable-cookie/poodle-styles/relation-picker.css";

import { Fragment, useEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type ReactNode } from "react";

import { Button } from "./Button";
import { Checkbox } from "./Checkbox";
import { FormActions } from "./FormActions";
import { Icon } from "./Icon";
import { PickerShell } from "./PickerShell";
import { Select } from "./Select";
import { SelectionSummary } from "./SelectionSummary";
import { TextInput } from "./TextInput";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  BrowseState,
  ControlDensity,
  ControlSize,
  DrillDownConfig,
  DrillDownContext,
  DrillDownItem,
  DrillDownLevel,
  PickerFilterConfig,
  PickerItem,
  PickerVariant,
  SelectionMode,
  SemanticControlSizeRole,
} from "./types";

export interface RelationPickerProps {
  title?: string;
  description?: string | null;
  items?: PickerItem[];
  selectedItems?: PickerItem[];
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  query?: string;
  defaultQuery?: string;
  selectionMode?: SelectionMode;
  variant?: PickerVariant;
  state?: BrowseState;
  ariaLabel?: string | null;
  searchPlaceholder?: string;
  filters?: PickerFilterConfig[];
  filterValues?: Record<string, string | undefined>;
  stateTitle?: string | null;
  stateMessage?: string | null;
  confirmLabel?: string;
  cancelLabel?: string;
  footerNote?: string | null;
  showFooter?: boolean;
  showSelectionSummary?: boolean;
  drillDown?: DrillDownConfig | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onQueryChange?: ((value: string) => void) | undefined;
  onSelectionChange?: ((selectedIds: string[]) => void) | undefined;
  onFilterChange?: ((key: string, value: string | undefined) => void) | undefined;
  onConfirm?: ((selectedIds: string[]) => void) | undefined;
  onCancel?: (() => void) | undefined;
  onDrillContext?: ((context: DrillDownContext) => void) | undefined;
  renderItem?: (item: PickerItem, selected: boolean) => ReactNode;
  stateContent?: ReactNode;
}

const statusId = "relation-picker-status";

function getFilterOptions(filter: PickerFilterConfig) {
  return [
    ...(filter.includeAll === false
      ? []
      : [
          {
            value: "__all__",
            label: filter.allLabel ?? "All",
          },
        ]),
    ...filter.options.map((option) => ({
      value: option.id,
      label: option.label,
    })),
  ];
}

export function RelationPicker({
  title = "Select items",
  description = null,
  items = [],
  selectedItems: providedSelectedItems = [],
  selectedIds: controlledSelectedIds,
  defaultSelectedIds = [],
  query: controlledQuery,
  defaultQuery = "",
  selectionMode = "multiple",
  variant = "inline",
  state: browseState = "ready",
  ariaLabel = null,
  searchPlaceholder = "Search picker results",
  filters = [],
  filterValues = {},
  stateTitle = null,
  stateMessage = null,
  confirmLabel = "Confirm selection",
  cancelLabel = "Cancel",
  footerNote = null,
  showFooter = true,
  showSelectionSummary = true,
  drillDown = null,
  size = null,
  sizeRole = "control",
  density = null,
  onQueryChange = undefined,
  onSelectionChange = undefined,
  onFilterChange = undefined,
  onConfirm = undefined,
  onCancel = undefined,
  onDrillContext = undefined,
  renderItem,
  stateContent,
}: RelationPickerProps) {
  const uiPresentation = useUiPresentation();

  const candidateButtons = useRef<Array<HTMLButtonElement | null>>([]);
  const [selectedItemLabels, setSelectedItemLabels] = useState<Record<string, string>>({});
  const [uncontrolledQuery, setUncontrolledQuery] = useState(controlledQuery ?? defaultQuery);
  const [uncontrolledSelectedIds, setUncontrolledSelectedIds] = useState<string[]>(
    controlledSelectedIds ? [...controlledSelectedIds] : defaultSelectedIds,
  );

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasQueryProp = controlledQuery !== undefined;
  const hasSelectedIdsProp = controlledSelectedIds !== undefined;
  const currentSelectedIds = hasSelectedIdsProp ? (controlledSelectedIds ?? []) : uncontrolledSelectedIds;
  const currentQuery = hasQueryProp ? (controlledQuery ?? "") : uncontrolledQuery;

  // Drill-down state
  const [drillDepth, setDrillDepth] = useState(0);
  const [drillSelections, setDrillSelections] = useState<Record<string, DrillDownItem>>({});
  const [drillSearchQuery, setDrillSearchQuery] = useState("");
  const [drillItems, setDrillItems] = useState<DrillDownItem[]>([]);
  const [drillLoading, setDrillLoading] = useState(false);
  const [finalItemsLoaded, setFinalItemsLoaded] = useState<PickerItem[] | null>(null);
  const [finalItemsLoading, setFinalItemsLoading] = useState(false);
  const lastDrillContextKey = useRef("");

  const isDrilling = drillDown !== null && drillDepth < (drillDown?.levels.length ?? 0);
  const hasDrillCompleted =
    drillDown !== null && drillDepth >= (drillDown?.levels.length ?? 0) && Object.keys(drillSelections).length > 0;
  const currentLevel: DrillDownLevel | null = drillDown?.levels[drillDepth] ?? null;

  const drillContext: DrillDownContext = Object.fromEntries(
    Object.entries(drillSelections).map(([key, item]) => [key, item.id]),
  ) as DrillDownContext;
  const drillContextKey = JSON.stringify(drillContext);

  useEffect(() => {
    if (drillContextKey === lastDrillContextKey.current) {
      return;
    }

    lastDrillContextKey.current = drillContextKey;
    onDrillContext?.(drillContext);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [drillContextKey]);

  // Load drill-down items when the level or its search changes.
  useEffect(() => {
    if (!currentLevel) return;
    let cancelled = false;

    async function load(level: DrillDownLevel): Promise<void> {
      if (typeof level.items === "function") {
        setDrillLoading(true);
        try {
          const result = await level.items(drillSearchQuery, drillContext);
          if (!cancelled) setDrillItems(result);
        } catch {
          if (!cancelled) setDrillItems([]);
        }
        if (!cancelled) setDrillLoading(false);
      } else {
        const q = drillSearchQuery.trim().toLowerCase();
        setDrillItems(
          q
            ? level.items.filter((item) =>
                [item.label, item.description ?? "", item.meta ?? ""].some((v) => v.toLowerCase().includes(q)),
              )
            : level.items,
        );
      }
    }

    void load(currentLevel);
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [drillDepth, drillSearchQuery, drillContextKey, drillDown]);

  // Load final items when drill-down completes.
  useEffect(() => {
    if (!hasDrillCompleted || !drillDown?.finalItems) return;
    let cancelled = false;
    const fn = drillDown.finalItems;

    async function load(): Promise<void> {
      setFinalItemsLoading(true);
      try {
        const result = await fn(currentQuery, drillContext);
        if (!cancelled) setFinalItemsLoaded(result);
      } catch {
        if (!cancelled) setFinalItemsLoaded([]);
      }
      if (!cancelled) setFinalItemsLoading(false);
    }

    void load();
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hasDrillCompleted, drillContextKey, currentQuery, drillDown]);

  function drillSelect(item: DrillDownItem): void {
    if (!drillDown || !currentLevel) return;
    setDrillSelections((prev) => ({ ...prev, [currentLevel.key]: item }));
    setDrillSearchQuery("");
    setDrillDepth((depth) => depth + 1);
  }

  function drillBack(): void {
    if (drillDepth <= 0 || !drillDown) return;
    const nextDepth = drillDepth - 1;
    const levelKey = drillDown.levels[nextDepth].key;
    setDrillSelections((prev) => {
      const next = { ...prev };
      delete next[levelKey];
      return next;
    });
    setDrillDepth(nextDepth);
    setDrillSearchQuery("");
    setFinalItemsLoaded(null);
  }

  function drillNavigateTo(depth: number): void {
    if (!drillDown) return;
    setDrillSelections((prev) => {
      const next = { ...prev };
      for (let i = depth; i < drillDown.levels.length; i++) {
        delete next[drillDown.levels[i].key];
      }
      return next;
    });
    setDrillDepth(depth);
    setDrillSearchQuery("");
    setFinalItemsLoaded(null);
  }

  function handleDrillSearchKeydown(keyboardEvent: KeyboardEvent | ReactKeyboardEvent): void {
    if (keyboardEvent.key === "Escape" || (keyboardEvent.key === "Backspace" && !drillSearchQuery)) {
      if (drillDepth > 0) {
        keyboardEvent.preventDefault();
        drillBack();
      }
    }
  }

  const drillBreadcrumbs = drillDown
    ? drillDown.levels.slice(0, drillDepth).map((level) => ({
        key: level.key,
        label: drillSelections[level.key]?.label ?? level.label,
        depth: drillDown.levels.indexOf(level),
      }))
    : [];

  // Flat picker logic — use finalItemsLoaded when drill-down provides items
  const activeItems = hasDrillCompleted && finalItemsLoaded !== null ? finalItemsLoaded : items;

  useEffect(() => {
    if (activeItems.length === 0 && providedSelectedItems.length === 0) {
      return;
    }

    setSelectedItemLabels((prev) => {
      const nextLabels = { ...prev };
      let changed = false;

      for (const item of [...activeItems, ...providedSelectedItems]) {
        if (nextLabels[item.id] !== item.label) {
          nextLabels[item.id] = item.label;
          changed = true;
        }
      }

      return changed ? nextLabels : prev;
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeItems, providedSelectedItems]);

  const filteredItems = activeItems.filter((item) =>
    currentQuery.trim().length === 0
      ? true
      : [item.label, item.description ?? "", item.meta ?? ""].some((value) =>
          value.toLowerCase().includes(currentQuery.trim().toLowerCase()),
        ),
  );
  const selectedItems = currentSelectedIds.map((id) => ({
    id,
    label: selectedItemLabels[id] ?? id,
  }));
  const shellState: BrowseState = isDrilling
    ? drillLoading
      ? "loading"
      : "ready"
    : finalItemsLoading
      ? "loading"
      : browseState;
  const pickerStatusText =
    shellState === "loading"
      ? "Picker results are loading."
      : shellState === "error"
        ? "Picker results are unavailable."
        : shellState === "empty"
          ? "No candidates are available."
          : shellState === "no-results"
            ? `No candidates match "${currentQuery}".`
            : `${filteredItems.length} candidate${filteredItems.length === 1 ? "" : "s"} available, ${currentSelectedIds.length} selected.`;

  function setSelection(nextIds: string[]): void {
    if (!hasSelectedIdsProp) {
      setUncontrolledSelectedIds(nextIds);
    }

    onSelectionChange?.(nextIds);
  }

  function setQuery(nextQuery: string): void {
    if (!hasQueryProp) {
      setUncontrolledQuery(nextQuery);
    }

    onQueryChange?.(nextQuery);
  }

  function toggleItem(id: string): void {
    const item = activeItems.find((candidate) => candidate.id === id);
    if (item?.disabled) {
      return;
    }

    if (selectionMode === "single") {
      setSelection([id]);
      return;
    }

    setSelection(
      currentSelectedIds.includes(id)
        ? currentSelectedIds.filter((selectedId) => selectedId !== id)
        : [...currentSelectedIds, id],
    );
  }

  function focusCandidate(index: number): void {
    candidateButtons.current[index]?.focus();
  }

  function handleCandidateKeydown(event: ReactKeyboardEvent, index: number): void {
    if (filteredItems.length === 0) return;

    if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      focusCandidate((index + 1) % filteredItems.length);
      return;
    }

    if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      focusCandidate((index - 1 + filteredItems.length) % filteredItems.length);
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      focusCandidate(0);
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      focusCandidate(filteredItems.length - 1);
    }
  }

  const breadcrumbsNode =
    drillBreadcrumbs.length > 0 ? (
      <div className="poodle-drill-breadcrumbs">
        <button type="button" className="poodle-drill-breadcrumbs__back" onClick={drillBack} aria-label="Go back">
          <Icon name="chevron-left" />
        </button>
        {drillBreadcrumbs.map((crumb, i) => (
          <Fragment key={crumb.key}>
            {i > 0 ? <span className="poodle-drill-breadcrumbs__sep">/</span> : null}
            <button type="button" className="poodle-drill-breadcrumbs__item" onClick={() => drillNavigateTo(crumb.depth)}>
              {crumb.label}
            </button>
          </Fragment>
        ))}
      </div>
    ) : null;

  const toolbarContent = isDrilling ? (
    <>
      {breadcrumbsNode}
      <div className="poodle-drill-level-label">{currentLevel?.label}</div>
      <TextInput
        id="drill-search"
        type="search"
        value={drillSearchQuery}
        ariaLabel={`Search ${currentLevel?.label ?? "items"}`}
        placeholder={currentLevel?.searchPlaceholder ?? `Search ${currentLevel?.label?.toLowerCase() ?? "items"}...`}
        onValueChange={(nextValue) => setDrillSearchQuery(nextValue)}
        onClear={() => setDrillSearchQuery("")}
        onKeyDown={handleDrillSearchKeydown}
      />
    </>
  ) : (
    <>
      {drillDown ? breadcrumbsNode : null}
      <TextInput
        id="relation-picker-search"
        type="search"
        value={currentQuery}
        ariaLabel="Search picker results"
        placeholder={searchPlaceholder}
        describedBy={statusId}
        onValueChange={setQuery}
        onClear={() => setQuery("")}
      />
      {filters.length > 0 ? (
        <div className="poodle-relation-picker__filters">
          {filters.map((filter) => (
            <Select
              key={filter.key}
              value={filterValues[filter.key] ?? "__all__"}
              options={getFilterOptions(filter)}
              ariaLabel={`${filter.label} filter`}
              size={resolvedSize}
              density={resolvedDensity}
              onValueChange={(value) => {
                onFilterChange?.(filter.key, value === "__all__" ? undefined : value);
              }}
            />
          ))}
        </div>
      ) : null}
    </>
  );

  const selectionContent =
    showSelectionSummary && currentSelectedIds.length > 0 ? (
      <SelectionSummary
        items={selectedItems}
        onRemove={(id) => setSelection(currentSelectedIds.filter((selectedId) => selectedId !== id))}
        onClear={() => setSelection([])}
      />
    ) : null;

  const footerContent = showFooter ? (
    <FormActions align="start">
      {footerNote !== null ? <p className="poodle-relation-picker__footer-note">{footerNote}</p> : null}
      <div className="poodle-relation-picker__footer-actions">
        <Button variant="ghost" size={resolvedSize} onClick={() => onCancel?.()}>
          {cancelLabel}
        </Button>
        <Button variant="primary" size={resolvedSize} onClick={() => onConfirm?.(currentSelectedIds)}>
          {confirmLabel}
        </Button>
      </div>
    </FormActions>
  ) : undefined;

  function candidateButton(item: PickerItem, index: number): ReactNode {
    const isSelected = currentSelectedIds.includes(item.id);

    return (
      <button
        ref={(el) => {
          candidateButtons.current[index] = el;
        }}
        type="button"
        className="poodle-relation-picker__item-button"
        aria-pressed={isSelected}
        aria-disabled={item.disabled}
        aria-describedby={item.description || item.meta ? `relation-picker-item-${item.id}` : undefined}
        onClick={() => toggleItem(item.id)}
        onKeyDown={(event) => handleCandidateKeydown(event, index)}
        disabled={item.disabled}
      >
        {renderItem ? (
          renderItem(item, isSelected)
        ) : (
          <span className="poodle-relation-picker__item-copy">
            <strong>{item.label}</strong>
            {item.description || item.meta ? (
              <small id={`relation-picker-item-${item.id}`}>
                {item.description ?? ""}
                {item.description && item.meta ? " · " : null}
                {item.meta ?? ""}
              </small>
            ) : null}
          </span>
        )}
      </button>
    );
  }

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <div className="poodle-relation-picker" data-size={resolvedSize} data-density={resolvedDensity} data-variant={variant}>
        <PickerShell
          title={title}
          description={description}
          variant={variant}
          state={shellState}
          ariaLabel={ariaLabel}
          resultCount={isDrilling ? drillItems.length : filteredItems.length}
          selectionCount={currentSelectedIds.length}
          statusText={
            isDrilling ? `${drillItems.length} item${drillItems.length === 1 ? "" : "s"}` : pickerStatusText
          }
          statusId={statusId}
          stateTitle={
            stateTitle ??
            (isDrilling
              ? "Loading"
              : browseState === "loading"
                ? "Loading candidates"
                : browseState === "error"
                  ? "Picker unavailable"
                  : browseState === "empty"
                    ? "No candidates available"
                    : "No matching candidates")
          }
          stateMessage={
            stateMessage ??
            (isDrilling
              ? "Loading items..."
              : browseState === "loading"
                ? "Picker results are loading while selection state stays host-owned."
                : browseState === "error"
                  ? "Error handling remains host-owned, but the picker preserves its structure."
                  : browseState === "empty"
                    ? "This relation has no available candidates yet."
                    : "Try widening the search query or clearing selection filters.")
          }
          toolbar={toolbarContent}
          selection={selectionContent}
          stateContent={stateContent}
          footer={footerContent}
        >
          {isDrilling ? (
            <ul className="poodle-drill-list" aria-label={currentLevel?.label ?? "Items"}>
              {drillItems.map((item) => (
                <li key={item.id} className="poodle-drill-list__item">
                  <button type="button" className="poodle-drill-list__button" onClick={() => drillSelect(item)}>
                    <span className="poodle-drill-list__copy">
                      <strong>{item.label}</strong>
                      {item.description ? <small>{item.description}</small> : null}
                    </span>
                    <span className="poodle-drill-list__meta">
                      {item.count !== undefined ? <span className="poodle-drill-list__count">{item.count}</span> : null}
                      <Icon name="chevron-right" />
                    </span>
                  </button>
                </li>
              ))}
              {drillItems.length === 0 && !drillLoading ? (
                <li className="poodle-drill-list__empty">No items found</li>
              ) : null}
            </ul>
          ) : (
            <ul className="poodle-relation-picker__list" aria-label="Available candidates">
              {filteredItems.map((item, index) => (
                <li
                  key={item.id}
                  className="poodle-relation-picker__item"
                  data-selection-mode={selectionMode}
                  data-selected={currentSelectedIds.includes(item.id)}
                  data-disabled={item.disabled}
                >
                  {selectionMode === "multiple" ? (
                    <>
                      <Checkbox
                        ariaLabel={`Select ${item.label}`}
                        checked={currentSelectedIds.includes(item.id)}
                        disabled={item.disabled}
                        onCheckedChange={() => toggleItem(item.id)}
                      />
                      {candidateButton(item, index)}
                    </>
                  ) : (
                    candidateButton(item, index)
                  )}
                </li>
              ))}
            </ul>
          )}
        </PickerShell>
      </div>
    </UiPresentationProvider>
  );
}

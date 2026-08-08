import {
  useEffect,
  useId,
  useRef,
  useState,
  type ChangeEvent,
  type KeyboardEvent,
  type MouseEvent,
  type ReactNode,
} from "react";
import {
  filterSelectGroups,
  flattenSelectOptions,
  isSelectOptionDisabled,
  layerContains,
  registerDismissLayer,
  selectOpenHighlightIndex,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/select.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  OverlayPlacement,
  SelectEmptyRenderState,
  SelectItems,
  SelectLoadOptions,
  SelectOption,
  SelectOptionGroup,
  SelectOptionRenderState,
  SelectTriggerRenderState,
  SemanticControlSizeRole,
  ValidationState,
} from "./types";

export interface SelectProps {
  id?: string;
  name?: string;
  value?: string | null;
  defaultValue?: string | null;
  options?: SelectItems;
  loadOptions?: SelectLoadOptions | null;
  loadKey?: string | null;
  valueLabel?: string | null;
  placeholder?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  disabled?: boolean;
  required?: boolean;
  validationState?: ValidationState;
  clearable?: boolean;
  searchable?: boolean;
  freeform?: boolean;
  native?: boolean;
  emptyMessage?: string;
  variant?: "default" | "ghost";
  menuMinWidth?: string | null;
  ariaLabel?: string | null;
  describedBy?: string | null;
  onValueChange?: (value: string) => void;
  onQueryChange?: (query: string) => void;
  onOpenChange?: (open: boolean) => void;
  trigger?: (state: SelectTriggerRenderState) => ReactNode;
  option?: (state: SelectOptionRenderState) => ReactNode;
  empty?: (state: SelectEmptyRenderState) => ReactNode;
}

export function Select({
  id,
  name,
  value,
  defaultValue = null,
  options = [],
  loadOptions = null,
  loadKey = null,
  valueLabel = null,
  placeholder = null,
  size = null,
  sizeRole = "control",
  density = null,
  disabled = false,
  required = false,
  validationState = "none",
  clearable = false,
  searchable = false,
  freeform = false,
  native,
  emptyMessage = "No matches",
  variant = "default",
  menuMinWidth = null,
  ariaLabel = null,
  describedBy = null,
  onValueChange,
  onQueryChange,
  onOpenChange,
  trigger: triggerRender,
  option: optionRender,
  empty: emptyRender,
}: SelectProps) {
  const generatedSelectId = useId();
  const uiPresentation = useUiPresentation();

  // The root is state, not a ref: the portalled listbox has to re-render once
  // it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const listboxRef = useRef<HTMLDivElement | null>(null);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const [open, setOpenState] = useState(false);
  const [query, setQuery] = useState("");
  const [highlightIndex, setHighlightIndex] = useState(0);
  // Reported by the anchored surface once the listbox is measured; the classes
  // below only need the side and the alignment, not the full placement.
  const [resolvedPlacement, setResolvedPlacement] = useState<OverlayPlacement>("bottom-start");
  const placement = resolvedPlacement.startsWith("top") ? "above" : "below";
  const alignEnd = resolvedPlacement.endsWith("-end");
  const [loadedOptions, setLoadedOptions] = useState<SelectItems | null>(null);
  const [loadState, setLoadState] = useState<"idle" | "loading" | "loaded" | "error">("idle");
  const [loadError, setLoadError] = useState<string | null>(null);
  const lastLoadKey = useRef<string | null>(null);
  const activeLoadRequestId = useRef(0);
  const [uncontrolledValue, setUncontrolledValue] = useState(defaultValue ?? "");

  const selectId = id ?? generatedSelectId;
  const listboxId = `${selectId}-listbox`;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== undefined;
  const currentValue = isControlled ? (value ?? "") : uncontrolledValue;
  const useCustom =
    native === true ? false : native === false ? true : searchable || Boolean(optionRender) || Boolean(triggerRender);
  const isLazy = Boolean(loadOptions);
  const clearValue = defaultValue ?? "";
  const placeholderValue = clearable ? clearValue : "";
  const placeholderLabel = placeholder ?? (clearable ? (valueLabel ?? "All") : null);
  const normalizedOptions = loadedOptions ?? options;
  const flatOptions = flattenSelectOptions(normalizedOptions as (SelectOption | SelectOptionGroup)[]) as SelectOption[];
  const hasPlaceholderOption = flatOptions.some((entry) => entry.value === placeholderValue);
  const hasCurrentOption = flatOptions.some((entry) => entry.value === currentValue);
  const hasSelection = currentValue !== "" && currentValue !== clearValue;
  const showClear = clearable && hasSelection && !disabled;
  const isGrouped = normalizedOptions.length > 0 && "options" in normalizedOptions[0];
  const normalizedGroups = isGrouped ? (normalizedOptions as SelectOptionGroup[]) : [];
  const selectedOption = flatOptions.find((entry) => entry.value === currentValue) ?? null;
  const filteredOptions =
    searchable && query.length > 0
      ? flatOptions.filter(
          (entry) => !isSelectOptionDisabled(entry) && entry.label.toLowerCase().includes(query.toLowerCase()),
        )
      : flatOptions.filter((entry) => !isSelectOptionDisabled(entry));
  const filteredGroups =
    isGrouped && searchable && query.length > 0
      ? (filterSelectGroups(normalizedOptions as SelectOptionGroup[], query) as SelectItems)
      : normalizedOptions;
  const visibleGroups = isGrouped ? (filteredGroups as SelectOptionGroup[]) : [];
  const highlightedOptionId =
    open && filteredOptions.length > 0 && highlightIndex >= 0 ? `${listboxId}-option-${highlightIndex}` : undefined;

  const stateRef = useRef({ filteredOptions, selectedOption, currentValue, open, highlightIndex, hasSelection });
  stateRef.current = { filteredOptions, selectedOption, currentValue, open, highlightIndex, hasSelection };

  async function startLoad(nextQuery = query): Promise<void> {
    const requestId = ++activeLoadRequestId.current;
    setLoadState("loading");
    setLoadError(null);
    try {
      const nextOptions = loadOptions
        ? await loadOptions({ query: nextQuery.trim() || undefined, value: stateRef.current.currentValue || null, loadKey })
        : [];
      if (requestId !== activeLoadRequestId.current) return;
      setLoadedOptions(nextOptions);
      setLoadState("loaded");
    } catch (error) {
      if (requestId !== activeLoadRequestId.current) return;
      setLoadState("error");
      setLoadError(error instanceof Error ? error.message : "Failed to load options");
    }
  }

  function commitValue(nextValue: string): void {
    if (!isControlled) setUncontrolledValue(nextValue);
    onValueChange?.(nextValue);
  }

  function setOpen(nextOpen: boolean): void {
    setOpenState(nextOpen);
    onOpenChange?.(nextOpen);
    if (nextOpen) {
      setHighlightIndex(
        selectOpenHighlightIndex(stateRef.current.filteredOptions, stateRef.current.selectedOption?.value ?? null),
      );
    }
  }

  function selectOption(option: SelectOption): void {
    if (isSelectOptionDisabled(option)) return;
    setQuery(option.label);
    commitValue(option.value);
    setOpen(false);
  }

  function handleInputInput(event: ChangeEvent<HTMLInputElement>): void {
    const next = event.currentTarget.value;
    setQuery(next);
    setHighlightIndex(0);
    if (!open) setOpen(true);
    onQueryChange?.(next);
    if (isLazy) void startLoad(next);
    if (freeform) commitValue(next);
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (!open) {
        setOpen(true);
        return;
      }
      setHighlightIndex((i) => Math.min(i + 1, filteredOptions.length - 1));
    }
    if (event.key === "ArrowUp") {
      event.preventDefault();
      if (!open) {
        setOpen(true);
        return;
      }
      setHighlightIndex((i) => Math.max(i - 1, 0));
    }
    if (event.key === "Enter" && open && filteredOptions[highlightIndex]) {
      event.preventDefault();
      selectOption(filteredOptions[highlightIndex]);
    }
    if (event.key === "Escape" && open) {
      event.preventDefault();
      setOpen(false);
    }
    if (event.key === "Home" && open) {
      event.preventDefault();
      setHighlightIndex(0);
    }
    if (event.key === "End" && open) {
      event.preventDefault();
      setHighlightIndex(Math.max(filteredOptions.length - 1, 0));
    }
  }

  function handleClear(event: MouseEvent): void {
    event.stopPropagation();
    setQuery("");
    commitValue(clearValue);
    if (isLazy) void startLoad("");
  }

  // sync query to selected label when closed (non-freeform)
  useEffect(() => {
    if (!open && !freeform) {
      setQuery(stateRef.current.hasSelection ? (stateRef.current.selectedOption?.label ?? "") : "");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open, freeform, currentValue, flatOptions.length]);

  // clamp highlight
  useEffect(() => {
    if (filteredOptions.length > 0 && highlightIndex >= filteredOptions.length) setHighlightIndex(0);
  }, [filteredOptions.length, highlightIndex]);

  // loadKey reset
  useEffect(() => {
    if (loadKey !== lastLoadKey.current) {
      activeLoadRequestId.current += 1;
      lastLoadKey.current = loadKey;
      setLoadedOptions(null);
      setLoadState("idle");
      setLoadError(null);
    }
  }, [loadKey]);

  // lazy initial load
  useEffect(() => {
    if (isLazy && loadState === "idle") void startLoad();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isLazy, loadState]);

  // dismiss layer while open
  useEffect(() => {
    if (!open) return;
    return registerDismissLayer({
      // The listbox is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target as Node, rootElement, listboxRef.current),
      dismissOnOutsideInteract: true,
      onDismiss: () => setOpen(false),
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open, rootElement]);

  function renderOption(option: SelectOption, flatIdx: number) {
    return (
      <button
        key={option.value}
        type="button"
        className="poodle-select__option"
        id={`${listboxId}-option-${flatIdx}`}
        role="option"
        aria-selected={currentValue === option.value}
        data-highlighted={highlightIndex === flatIdx}
        disabled={isSelectOptionDisabled(option)}
        onMouseEnter={() => setHighlightIndex(flatIdx)}
        onClick={() => selectOption(option)}
      >
        {optionRender ? (
          <span className="poodle-select__option-content">
            {optionRender({
              option,
              highlighted: highlightIndex === flatIdx,
              selected: currentValue === option.value,
              index: flatIdx,
            })}
          </span>
        ) : (
          <>
            {option.icon ? (
              <span className="poodle-select__option-icon">
                <Icon icon={option.icon} size="sm" />
              </span>
            ) : null}
            <span className="poodle-select__option-content">
              <span className="poodle-select__option-body">
                <span className="poodle-select__option-label">{option.label}</span>
                {option.description ? (
                  <span className="poodle-select__option-description">{option.description}</span>
                ) : null}
              </span>
            </span>
          </>
        )}
      </button>
    );
  }

  if (!useCustom) {
    // ═══ NATIVE MODE ═══
    return (
      <div
        className="poodle-select"
        data-placeholder={!hasSelection}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        data-validation-state={validationState}
      >
        <select
          id={selectId}
          name={name}
          className="poodle-select__control"
          value={currentValue}
          disabled={disabled}
          required={required}
          aria-label={ariaLabel ?? undefined}
          aria-describedby={describedBy ?? undefined}
          aria-invalid={validationState === "invalid" ? "true" : undefined}
          onChange={(event) => commitValue(event.currentTarget.value)}
        >
          {placeholderLabel && !hasPlaceholderOption ? (
            <option value={placeholderValue} disabled={!clearable && required}>
              {placeholderLabel}
            </option>
          ) : null}

          {isGrouped
            ? normalizedGroups.map((group, gi) =>
                group.label.trim().length === 0 ? (
                  group.options.map((option) => (
                    <option key={option.value} value={option.value} disabled={isSelectOptionDisabled(option)}>
                      {option.label}
                    </option>
                  ))
                ) : (
                  <optgroup key={`${group.label}:${gi}`} label={group.label}>
                    {group.options.map((option) => (
                      <option key={option.value} value={option.value} disabled={isSelectOptionDisabled(option)}>
                        {option.label}
                      </option>
                    ))}
                  </optgroup>
                ),
              )
            : flatOptions.length > 0
              ? flatOptions.map((option) => (
                  <option key={option.value} value={option.value} disabled={isSelectOptionDisabled(option)}>
                    {option.label}
                  </option>
                ))
              : isLazy && currentValue && valueLabel ? (
                  <option value={currentValue}>{valueLabel}</option>
                ) : isLazy && loadState === "loading" ? (
                  <option value={placeholderValue} disabled>
                    Loading…
                  </option>
                ) : isLazy && loadState === "error" ? (
                  <option value={placeholderValue} disabled>
                    {loadError ?? "Failed to load options"}
                  </option>
                ) : currentValue && !hasCurrentOption && valueLabel ? (
                  <option value={currentValue}>{valueLabel}</option>
                ) : currentValue && !hasCurrentOption ? (
                  <option value={currentValue}>{currentValue}</option>
                ) : null}
        </select>

        <span className="poodle-select__indicator" aria-hidden="true">
          <Icon name="chevron-down" />
        </span>
      </div>
    );
  }

  // ═══ CUSTOM MODE ═══
  return (
    <div
      ref={setRootElement}
      className="poodle-select poodle-select--custom"
      data-open={open}
      data-placeholder={!hasSelection}
      data-variant={variant}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-validation-state={validationState}
      data-has-clear={showClear}
      aria-invalid={validationState === "invalid" ? "true" : undefined}
    >
      {searchable ? (
        <div
          className="poodle-select__trigger-area"
          role="combobox"
          tabIndex={-1}
          aria-expanded={open}
          aria-haspopup="listbox"
          aria-controls={open ? listboxId : undefined}
          aria-label={ariaLabel ?? undefined}
        >
          <input
            id={selectId}
            ref={inputRef}
            className="poodle-select__input"
            type="text"
            value={query}
            disabled={disabled}
            placeholder={placeholder ?? undefined}
            aria-autocomplete="list"
            aria-activedescendant={highlightedOptionId}
            aria-describedby={describedBy ?? undefined}
            onFocus={() => {
              if (!open) setOpen(true);
            }}
            onChange={handleInputInput}
            onKeyDown={handleKeydown}
          />
          {showClear ? (
            <button type="button" className="poodle-select__clear" aria-label="Clear selection" onClick={handleClear}>
              <Icon name="x" size="xs" />
            </button>
          ) : null}
          <button
            type="button"
            className="poodle-select__indicator-button"
            aria-label={open ? "Close options" : "Open options"}
            onClick={(event) => {
              event.stopPropagation();
              if (disabled) return;
              setOpen(!open);
              inputRef.current?.focus();
            }}
          >
            <Icon name="chevron-down" />
          </button>
        </div>
      ) : (
        <div className="poodle-select__trigger-area">
          <button
            type="button"
            className="poodle-select__trigger"
            id={selectId}
            disabled={disabled}
            aria-expanded={open}
            aria-haspopup="listbox"
            aria-controls={open ? listboxId : undefined}
            aria-label={ariaLabel ?? undefined}
            aria-describedby={describedBy ?? undefined}
            onClick={() => {
              if (!disabled) setOpen(!open);
            }}
            onKeyDown={handleKeydown}
          >
            <span className="poodle-select__trigger-content">
              {triggerRender ? (
                triggerRender({ selectedOption, open, placeholder })
              ) : (
                <span className="poodle-select__value" data-placeholder={!hasSelection}>
                  {hasSelection ? (selectedOption?.label ?? "") : (placeholder ?? selectedOption?.label ?? "")}
                </span>
              )}
            </span>
          </button>
          {showClear ? (
            <button type="button" className="poodle-select__clear" aria-label="Clear selection" onClick={handleClear}>
              <Icon name="x" size="xs" />
            </button>
          ) : null}
          {variant !== "ghost" ? (
            <button
              type="button"
              className="poodle-select__indicator-button"
              aria-label={open ? "Close options" : "Open options"}
              onClick={() => {
                if (!disabled) setOpen(!open);
              }}
            >
              <Icon name="chevron-down" />
            </button>
          ) : null}
        </div>
      )}

      {name ? <input type="hidden" name={name} value={currentValue} /> : null}

      {open ? (
        <AnchoredSurface
          ref={listboxRef}
          anchor={rootElement}
          placement="bottom-start"
          // Ghost triggers sit tighter to their menu than bordered ones.
          offset={variant === "ghost" ? 6 : 4}
          // A fixed min-width means the listbox sizes to its content; without
          // one it tracks the trigger exactly, as the old absolute inset did.
          matchWidth={!menuMinWidth}
          onPlacement={setResolvedPlacement}
          id={listboxId}
          className={[
            "poodle-select__listbox",
            placement === "above" ? "poodle-select__listbox--above" : "",
            menuMinWidth ? "poodle-select__listbox--auto-width" : "",
            alignEnd ? "poodle-select__listbox--align-end" : "",
          ]
            .filter(Boolean)
            .join(" ")}
          data-variant={variant}
          data-size={resolvedSize}
          data-density={resolvedDensity}
          role="listbox"
          aria-label={ariaLabel ?? undefined}
          style={menuMinWidth ? { minWidth: menuMinWidth } : undefined}
        >
          {isGrouped && !searchable
            ? normalizedGroups.map((group, gi) =>
                group.options.length > 0 ? (
                  <div key={`${group.label}:${gi}`} className="poodle-select__group" role="group" aria-label={group.label || undefined}>
                    {group.label ? <div className="poodle-select__group-label">{group.label}</div> : null}
                    {group.options.map((option) => renderOption(option, flatOptions.indexOf(option)))}
                  </div>
                ) : null,
              )
            : isGrouped && searchable
              ? visibleGroups.map((group, gi) =>
                  group.options.length > 0 ? (
                    <div key={`${group.label}:${gi}`} className="poodle-select__group" role="group" aria-label={group.label || undefined}>
                      {group.label ? <div className="poodle-select__group-label">{group.label}</div> : null}
                      {group.options.map((option) => renderOption(option, filteredOptions.indexOf(option)))}
                    </div>
                  ) : null,
                )
              : filteredOptions.map((option, index) => renderOption(option, index))}

          {filteredOptions.length === 0 ? (
            emptyRender ? (
              emptyRender({ query })
            ) : (
              <div className="poodle-select__empty">{emptyMessage}</div>
            )
          ) : null}
        </AnchoredSurface>
      ) : null}
    </div>
  );
}

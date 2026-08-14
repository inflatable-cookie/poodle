import "@inflatable-cookie/poodle-core/styles/model-connection.css";

import {
  useId,
  useMemo,
  useRef,
  useState,
  type KeyboardEvent,
  type ReactNode,
} from "react";

import {
  filterModelConnectionOptions,
  groupModelConnectionOptions,
  menuListNavigate,
  modelConnectionAvailabilityTone,
  modelConnectionOptionSelectable,
  modelConnectionPickerResultAnnouncement,
  resolveModelConnectionPickerShellState,
  singleSelectTransition,
  type ModelConnectionOption,
  type ModelConnectionPickerState,
} from "@inflatable-cookie/poodle-core";

import { Icon } from "./Icon";
import { PickerShell } from "./PickerShell";
import { Pill } from "./Pill";
import { StatusIndicator } from "./StatusIndicator";
import { TextInput } from "./TextInput";
import type { PickerVariant } from "./types";

export interface ModelConnectionPickerLeadingProps {
  option: ModelConnectionOption;
}

export interface ModelConnectionPickerProps {
  options?: ModelConnectionOption[];
  value?: string | null | undefined;
  defaultValue?: string | null;
  query?: string | undefined;
  defaultQuery?: string;
  state?: ModelConnectionPickerState;
  title?: string;
  description?: string | null;
  searchPlaceholder?: string;
  ariaLabel?: string | null;
  isDisabled?: boolean;
  variant?: PickerVariant;
  onValueChange?: ((id: string) => void) | null;
  onQueryChange?: ((query: string) => void) | null;
  leading?: (props: ModelConnectionPickerLeadingProps) => ReactNode;
  footer?: () => ReactNode;
}

export function ModelConnectionPicker({
  options = [],
  value = undefined,
  defaultValue = null,
  query = undefined,
  defaultQuery = "",
  state = "ready",
  title = "Choose a connection",
  description = null,
  searchPlaceholder = "Search connections",
  ariaLabel = null,
  isDisabled = false,
  variant = "inline",
  onValueChange = null,
  onQueryChange = null,
  leading,
  footer,
}: ModelConnectionPickerProps) {
  const instanceId = useId();
  const rootRef = useRef<HTMLElement | null>(null);

  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const [uncontrolledQuery, setUncontrolledQuery] = useState(defaultQuery);

  const isValueControlled = value !== undefined;
  const isQueryControlled = query !== undefined;
  const currentValue = isValueControlled ? (value ?? null) : uncontrolledValue;
  const currentQuery = isQueryControlled ? (query ?? "") : uncontrolledQuery;

  const filtered = useMemo(
    () => filterModelConnectionOptions(options, currentQuery),
    [options, currentQuery],
  );
  const groups = useMemo(() => groupModelConnectionOptions(filtered), [filtered]);
  const flatEnabled = useMemo(
    () => filtered.filter(modelConnectionOptionSelectable),
    [filtered],
  );
  const shellState = useMemo(
    () =>
      resolveModelConnectionPickerShellState(state, options.length, filtered.length, currentQuery),
    [state, options.length, filtered.length, currentQuery],
  );
  const statusText =
    shellState === "ready" || shellState === "no-results"
      ? modelConnectionPickerResultAnnouncement(filtered.length, currentQuery)
      : null;

  function groupDomId(group: string): string {
    return `${instanceId}-${group.replace(/[^a-zA-Z0-9_-]+/g, "-").toLowerCase()}`;
  }

  function setQuery(next: string): void {
    if (!isQueryControlled) setUncontrolledQuery(next);
    onQueryChange?.(next);
  }

  function select(id: string): void {
    const result = singleSelectTransition(
      {
        value: currentValue,
        options: options.map((option) => ({
          value: option.id,
          disabled: !modelConnectionOptionSelectable(option),
        })),
        disabled: isDisabled,
      },
      { type: "SELECT", value: id },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isValueControlled) setUncontrolledValue(effect.value);
        onValueChange?.(effect.value);
      }
    }
  }

  function focusOption(id: string): void {
    rootRef.current
      ?.querySelector<HTMLElement>(`[data-model-connection-option="${CSS.escape(id)}"]`)
      ?.focus();
  }

  function handleOptionKeydown(event: KeyboardEvent<HTMLButtonElement>, optionId: string): void {
    if (isDisabled) return;

    const enabled = flatEnabled;
    const currentIndex = enabled.findIndex((option) => option.id === optionId);
    if (currentIndex < 0) return;

    const navItems = enabled.map((option) => ({ value: option.id, disabled: false }));
    let next: ModelConnectionOption | undefined;

    if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      next = enabled[menuListNavigate(navItems, currentIndex, "next")];
    } else if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      next = enabled[menuListNavigate(navItems, currentIndex, "prev")];
    } else if (event.key === "Home") {
      event.preventDefault();
      next = enabled[0];
    } else if (event.key === "End") {
      event.preventDefault();
      next = enabled[enabled.length - 1];
    } else if (event.key === " " || event.key === "Enter") {
      event.preventDefault();
      select(optionId);
      return;
    }

    if (!next) return;
    select(next.id);
    focusOption(next.id);
  }

  function tabIndexFor(option: ModelConnectionOption): number {
    if (isDisabled || !modelConnectionOptionSelectable(option)) return -1;
    if (currentValue === option.id) return 0;
    if (currentValue === null && flatEnabled[0]?.id === option.id) return 0;
    return -1;
  }

  return (
    <section
      ref={rootRef}
      className="poodle-model-connection-picker"
      aria-label={ariaLabel ?? title}
      data-disabled={isDisabled ? "true" : "false"}
    >
      <PickerShell
        title={title}
        description={description}
        variant={variant}
        state={shellState}
        ariaLabel={ariaLabel ?? title}
        resultCount={shellState === "ready" ? filtered.length : null}
        selectionCount={currentValue ? 1 : 0}
        statusText={statusText}
        statusId={`${instanceId}-status`}
        toolbar={
          <TextInput
            type="search"
            value={currentQuery}
            placeholder={searchPlaceholder}
            disabled={isDisabled}
            ariaLabel={searchPlaceholder}
            describedBy={`${instanceId}-status`}
            onValueChange={(next) => setQuery(next)}
          />
        }
        footer={
          footer ? (
            <div className="poodle-model-connection-picker__footer">{footer()}</div>
          ) : undefined
        }
      >
        <div className="poodle-model-connection-picker__groups">
          {groups.map((group) => {
            const groupId = groupDomId(group.group);
            return (
              <section
                key={group.group}
                className="poodle-model-connection-picker__group"
                aria-labelledby={groupId}
              >
                <h3 className="poodle-model-connection-picker__group-title" id={groupId}>
                  {group.group}
                </h3>
                <div
                  className="poodle-model-connection-picker__options"
                  role="radiogroup"
                  aria-labelledby={groupId}
                >
                  {group.options.map((option) => {
                    const selectable = modelConnectionOptionSelectable(option);
                    const optionDisabled = isDisabled || !selectable;
                    return (
                      <button
                        key={option.id}
                        type="button"
                        className="poodle-model-connection-picker__option"
                        role="radio"
                        data-model-connection-option={option.id}
                        data-availability={option.availability}
                        tabIndex={tabIndexFor(option)}
                        aria-checked={currentValue === option.id ? "true" : "false"}
                        aria-disabled={optionDisabled ? "true" : undefined}
                        disabled={optionDisabled}
                        onClick={() => select(option.id)}
                        onKeyDown={(event) => handleOptionKeydown(event, option.id)}
                      >
                        <span className="poodle-model-connection-picker__leading" aria-hidden="true">
                          {leading ? leading({ option }) : <Icon name="package" />}
                        </span>
                        <span className="poodle-model-connection-picker__copy">
                          <span className="poodle-model-connection-picker__title-row">
                            <span className="poodle-model-connection-picker__provider">
                              {option.providerLabel}
                            </span>
                            {option.badges.length > 0 ? (
                              <span className="poodle-model-connection-picker__badges">
                                {option.badges.map((badge) => (
                                  <Pill
                                    key={badge.label}
                                    tone={badge.tone ?? "neutral"}
                                    appearance="subtle"
                                  >
                                    {badge.label}
                                  </Pill>
                                ))}
                              </span>
                            ) : null}
                          </span>
                          {option.routeLabel ? (
                            <span className="poodle-model-connection-picker__route">
                              {option.routeLabel}
                            </span>
                          ) : null}
                          {option.description ? (
                            <span className="poodle-model-connection-picker__description">
                              {option.description}
                            </span>
                          ) : null}
                        </span>
                        <span className="poodle-model-connection-picker__availability">
                          <StatusIndicator
                            status={modelConnectionAvailabilityTone(option.availability)}
                            label={option.availabilityLabel}
                          />
                        </span>
                      </button>
                    );
                  })}
                </div>
              </section>
            );
          })}
        </div>
      </PickerShell>
    </section>
  );
}

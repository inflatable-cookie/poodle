import { useRef, useState, type KeyboardEvent, type PointerEvent } from "react";
import { mergeTokens, splitTokenInput, tokenBackspaceRemoves } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/token-input.css";

import { Icon } from "./Icon";
import { Pill } from "./Pill";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface TokenInputProps {
  id?: string;
  values?: string[];
  defaultValues?: string[];
  name?: string;
  placeholder?: string | null;
  disabled?: boolean;
  readOnly?: boolean;
  required?: boolean;
  spellCheck?: boolean;
  autoCapitalize?: string;
  autoComplete?: string;
  ariaLabel?: string | null;
  describedBy?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  separators?: string[];
  dedupe?: boolean;
  commitOnBlur?: boolean;
  maxLength?: number | null;
  resolveToken?: (value: string, values: string[]) => string | null | undefined;
  onValuesChange?: (values: string[]) => void;
  onTokenReject?: (value: string) => void;
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

export function TokenInput({
  id = "",
  values,
  defaultValues = [],
  name,
  placeholder = null,
  disabled = false,
  readOnly = false,
  required = false,
  spellCheck = false,
  autoCapitalize = "none",
  autoComplete = "off",
  ariaLabel = null,
  describedBy = null,
  size = null,
  sizeRole = "control",
  density = null,
  separators = [","],
  dedupe = true,
  commitOnBlur = true,
  maxLength = null,
  resolveToken,
  onValuesChange,
  onTokenReject,
}: TokenInputProps) {
  const uiPresentation = useUiPresentation();

  const [inputValue, setInputValue] = useState("");
  const [uncontrolledValues, setUncontrolledValues] = useState<string[]>(defaultValues);
  const inputRef = useRef<HTMLInputElement | null>(null);

  const isControlled = values !== undefined;
  const currentValues = isControlled ? values : uncontrolledValues;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const canEdit = !disabled && !readOnly;
  const separatorChars = Array.from(new Set(separators.filter((separator) => separator.length > 0))).join("");
  const splitPattern = separatorChars.length > 0 ? new RegExp(`[${escapeRegExp(separatorChars)}]+`) : null;

  function normalizeToken(value: string, current: string[]): string | null {
    const trimmed = value.trim();
    if (!trimmed) return null;

    const resolved = resolveToken ? resolveToken(trimmed, current) : trimmed;
    if (typeof resolved !== "string") {
      onTokenReject?.(trimmed);
      return null;
    }

    const normalized = resolved.trim();
    if (!normalized) {
      onTokenReject?.(trimmed);
      return null;
    }
    return normalized;
  }

  function applyValues(nextValues: string[]): void {
    if (!isControlled) setUncontrolledValues(nextValues);
    onValuesChange?.(nextValues);
  }

  function addTokens(rawTokens: string[]): void {
    const current = currentValues ?? [];
    const nextTokens = rawTokens
      .map((token) => normalizeToken(token, current))
      .filter((token): token is string => Boolean(token));
    if (nextTokens.length === 0) return;
    applyValues(mergeTokens(current, nextTokens, dedupe));
  }

  function commitInput(): void {
    const trimmed = normalizeToken(inputValue, currentValues ?? []);
    if (!trimmed) {
      setInputValue("");
      return;
    }
    addTokens([trimmed]);
    setInputValue("");
  }

  function removeToken(index: number): void {
    if (!canEdit) return;
    applyValues(currentValues.filter((_, currentIndex) => currentIndex !== index));
  }

  function handleInput(nextValue: string): void {
    const split = splitTokenInput(nextValue, splitPattern, separatorChars);
    if (!split) {
      setInputValue(nextValue);
      return;
    }
    addTokens(split.committed);
    setInputValue(split.remainder);
  }

  function handleKeyDown(event: KeyboardEvent<HTMLInputElement>): void {
    if (!canEdit) return;

    if (event.key === "Enter" || event.key === "Tab") {
      if (inputValue.trim().length > 0) {
        event.preventDefault();
        commitInput();
      }
      return;
    }

    if (event.key === "Backspace" && tokenBackspaceRemoves(inputValue, currentValues.length)) {
      event.preventDefault();
      applyValues(currentValues.slice(0, -1));
    }
  }

  function handlePointerDown(event: PointerEvent<HTMLDivElement>): void {
    if (!canEdit || event.target === inputRef.current) return;
    if (event.target instanceof Element && event.target.closest("button")) return;
    event.preventDefault();
    inputRef.current?.focus();
  }

  return (
    <div
      className="poodle-token-input"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-disabled={disabled || undefined}
      data-read-only={readOnly || undefined}
      onPointerDown={handlePointerDown}
    >
      {name
        ? currentValues.map((token, index) => (
            <input key={`${token}:${index}`} type="hidden" name={name} value={token} />
          ))
        : null}

      <div className="poodle-token-input__tokens">
        {currentValues.map((token, index) => (
          <span key={`${token}:${index}`} className="poodle-token-input__token">
            <Pill tone="neutral" appearance="subtle" size={resolvedSize} adaptiveWidth>
              <span className="poodle-token-input__token-label">{token}</span>
              {canEdit ? (
                <button
                  type="button"
                  className="poodle-token-input__remove"
                  aria-label={`Remove ${token}`}
                  onClick={() => removeToken(index)}
                >
                  <Icon name="x" size="xs" />
                </button>
              ) : null}
            </Pill>
          </span>
        ))}

        <input
          id={id}
          ref={inputRef}
          className="poodle-token-input__control"
          type="text"
          value={inputValue}
          disabled={disabled}
          readOnly={readOnly}
          required={required}
          spellCheck={spellCheck}
          autoCapitalize={autoCapitalize}
          autoComplete={autoComplete}
          aria-label={ariaLabel ?? undefined}
          aria-describedby={describedBy ?? undefined}
          placeholder={currentValues.length === 0 ? (placeholder ?? undefined) : undefined}
          maxLength={maxLength ?? undefined}
          onChange={(event) => handleInput(event.currentTarget.value)}
          onKeyDown={handleKeyDown}
          onBlur={() => {
            if (commitOnBlur) commitInput();
          }}
        />
      </div>
    </div>
  );
}

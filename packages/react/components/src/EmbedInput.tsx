import "@inflatable-cookie/poodle-styles/embed-input.css";

import { useEffect, useRef, useState } from "react";

import { Pill } from "./Pill";
import { TextInput } from "./TextInput";
import { resolveEmbedParseState, type EmbedParseState } from "./embed-input";
import type { ControlDensity, ControlSize, ParsedEmbed, SemanticControlSizeRole } from "./types";

export interface EmbedInputProps {
  id?: string;
  value?: string;
  defaultValue?: string;
  parsed?: ParsedEmbed | null;
  placeholder?: string;
  parseDebounce?: number;
  providers?: string[];
  disabled?: boolean;
  error?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onParse?: ((parsed: ParsedEmbed | null, error: string | null) => void) | null;
  onValueChange?: ((value: string) => void) | null;
  resolveParseState?: ((value: string, providers: string[]) => EmbedParseState) | undefined;
}

export function EmbedInput({
  id = "embed-input",
  value: controlledValue,
  defaultValue = "",
  parsed: controlledParsed,
  placeholder = "Paste a URL or embed code...",
  parseDebounce = 300,
  providers = [],
  disabled = false,
  error: controlledError,
  size = null,
  sizeRole = "control",
  density = null,
  onParse = null,
  onValueChange = null,
  resolveParseState = undefined,
}: EmbedInputProps) {
  const [uncontrolledValue, setUncontrolledValue] = useState(defaultValue);
  const [uncontrolledParsed, setUncontrolledParsed] = useState<ParsedEmbed | null>(null);
  const [uncontrolledError, setUncontrolledError] = useState<string | null>(null);

  const value = controlledValue !== undefined ? controlledValue : uncontrolledValue;
  const parsed = controlledParsed !== undefined ? controlledParsed : uncontrolledParsed;
  const error = controlledError !== undefined ? controlledError : uncontrolledError;

  const parseTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const valueRef = useRef(value);
  valueRef.current = value;

  function doParse(): void {
    const nextState = (resolveParseState ?? resolveEmbedParseState)(valueRef.current, providers);
    setUncontrolledParsed(nextState.parsed);
    setUncontrolledError(nextState.error);

    onParse?.(nextState.parsed, nextState.error);
  }

  function handleValueChange(nextValue: string): void {
    if (controlledValue === undefined) {
      setUncontrolledValue(nextValue);
    }

    onValueChange?.(nextValue);

    if (parseTimer.current) {
      clearTimeout(parseTimer.current);
    }

    parseTimer.current = setTimeout(doParse, parseDebounce);
  }

  useEffect(() => {
    return () => {
      if (parseTimer.current) {
        clearTimeout(parseTimer.current);
      }
    };
  }, []);

  return (
    <div className="poodle-embed-input">
      <TextInput
        id={id}
        type="multiline"
        value={value}
        placeholder={placeholder}
        disabled={disabled}
        size={size}
        sizeRole={sizeRole}
        density={density}
        rows={3}
        onValueChange={handleValueChange}
      />

      <div className="poodle-embed-input__status">
        {error ? (
          <span className="poodle-embed-input__error">{error}</span>
        ) : parsed ? (
          <>
            <Pill tone="success" sizeRole="chrome">
              {parsed.provider}
            </Pill>
            <span className="poodle-embed-input__success">Embed detected</span>
          </>
        ) : null}
      </div>
    </div>
  );
}

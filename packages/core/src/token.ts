/**
 * Token/chip entry machinery (TokenInput).
 * Contract: docs/contracts/components/token-input.md, "Behavior Machine".
 *
 * Pure token-list math: merge-with-dedupe and separator splitting. Token
 * resolution/rejection callbacks stay adapter-side (app-defined hooks).
 */

export function mergeTokens(current: readonly string[], next: readonly string[], dedupe: boolean): string[] {
  return dedupe ? Array.from(new Set([...current, ...next])) : [...current, ...next];
}

export interface TokenSplit {
  /** Parts committed as tokens by this input. */
  committed: string[];
  /** Text left in the input field. */
  remainder: string;
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/**
 * Separator-driven splitting of raw input. Returns null when the input
 * contains no completed tokens yet (no separator hit).
 *
 * Each non-empty character in `separators` becomes part of the split set.
 */
export function splitTokenInput(
  rawValue: string,
  separators: readonly string[],
): TokenSplit | null {
  const separatorChars = Array.from(
    new Set(separators.filter((separator) => separator.length > 0).join("")),
  );
  if (separatorChars.length === 0) {
    return null;
  }

  const splitPattern = new RegExp(`[${escapeRegExp(separatorChars.join(""))}]+`);
  const rawParts = rawValue.split(splitPattern);
  const endsWithSeparator = separatorChars.some((separator) => rawValue.endsWith(separator));

  if (rawParts.length <= 1 && !endsWithSeparator) {
    return null;
  }

  return {
    committed: endsWithSeparator ? rawParts : rawParts.slice(0, -1),
    remainder: endsWithSeparator ? "" : rawParts.at(-1) ?? "",
  };
}

/** Backspace on an empty input removes the last chip. */
export function tokenBackspaceRemoves(inputValue: string, tokenCount: number): boolean {
  return inputValue.length === 0 && tokenCount > 0;
}

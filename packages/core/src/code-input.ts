/**
 * Segmented code-entry machinery (CodeInput).
 * Contract: docs/contracts/components/code-input.md, "Behavior Machine".
 *
 * Pure value/caret math over the single hidden input that backs the visual
 * segments: sanitization, position clamping, slot-click selection, and
 * insert-replacement. Selection-range DOM calls stay adapter-side.
 */

export function sanitizeCodeValue(input: string, length: number, numbersOnly: boolean): string {
  const normalized = numbersOnly ? input.replace(/\D/g, "") : input;

  return normalized.slice(0, length);
}

/**
 * Slot indices that end a visual group.
 *
 * Group lengths are accepted only as one complete, positive-integer
 * partition of `length`. Invalid partial patterns produce no breaks, keeping
 * grouping presentation-only and leaving value/caret behaviour untouched.
 */
export function codeGroupEndIndices(
  length: number,
  groups: readonly number[] | null | undefined,
): number[] {
  if (
    !Number.isInteger(length) ||
    length < 1 ||
    !groups ||
    groups.length < 2 ||
    groups.some((group) => !Number.isInteger(group) || group < 1) ||
    groups.reduce((total, group) => total + group, 0) !== length
  ) {
    return [];
  }

  let consumed = 0;
  return groups.slice(0, -1).map((group) => {
    consumed += group;
    return consumed - 1;
  });
}

/** Active caret position clamped into the filled prefix and the segment count. */
export function clampCodePosition(index: number, valueLength: number, length: number): number {
  const maxPosition = Math.max(Math.min(valueLength, length - 1), 0);

  return Math.min(Math.max(index, 0), maxPosition);
}

/** Selection range for an active position; selects the filled digit when requested. */
export function codeSelectionRange(
  position: number,
  valueLength: number,
  selectFilled: boolean,
): { start: number; end: number } {
  const end = selectFilled && position < valueLength ? position + 1 : position;

  return { start: position, end };
}

/** Selection for clicking the visual slot at `index`. */
export function codeSlotSelection(index: number, valueLength: number): { start: number; end: number } {
  const start = Math.min(index, valueLength);
  const end = index < valueLength ? index + 1 : start;

  return { start, end };
}

/**
 * Replacement math for an insert beforeinput: overwrites from the selection
 * start, extending the replaced span to cover the inserted data, capped to
 * `length`. Returns null when the (sanitized) data is empty.
 */
export function codeInsertReplacement(
  currentValue: string,
  data: string,
  selectionStart: number,
  selectionEnd: number,
  length: number,
  numbersOnly: boolean,
): { value: string; caret: number } | null {
  const nextData = numbersOnly ? data.replace(/\D/g, "") : data;

  if (nextData.length === 0) {
    return null;
  }

  const replacementEnd = Math.max(selectionEnd, Math.min(selectionStart + nextData.length, currentValue.length));
  const value = `${currentValue.slice(0, selectionStart)}${nextData}${currentValue.slice(replacementEnd)}`.slice(
    0,
    length,
  );

  return { value, caret: Math.min(selectionStart + nextData.length, length - 1) };
}

/**
 * Text/number entry machinery (TextInput, NumberInput).
 * Contracts: docs/contracts/components/text-input.md, number-input.md,
 * "Behavior Machine" sections.
 *
 * Pure value semantics shared by the entry components; debounce, DOM events,
 * and async validation orchestration stay adapter-side.
 */

export type InputValidationStatus = "idle" | "validating" | "valid" | "invalid";

export type ValidationState = "none" | "pending" | "valid" | "invalid";

export function validationStatusToState(
  status: InputValidationStatus,
  fallback: ValidationState,
): ValidationState {
  if (status === "validating") return "pending";
  if (status === "valid") return "valid";
  if (status === "invalid") return "invalid";
  return fallback;
}

/** Numeric coercion: empty/null/non-finite become null. */
export function parseNumberish(input: number | string | null | undefined): number | null {
  if (input === null || input === undefined || input === "") {
    return null;
  }

  const value = Number(input);

  return Number.isFinite(value) ? value : null;
}

/** Step parsing: invalid or non-positive steps fall back to 1. */
export function parseStep(input: number | string | null): number {
  if (input === null || input === "") {
    return 1;
  }

  const value = Number(input);

  return Number.isFinite(value) && value > 0 ? value : 1;
}

/** Clamp with optional bounds (null bound = unbounded on that side). */
export function clampNullable(value: number, min: number | null, max: number | null): number {
  let result = value;

  if (min !== null) {
    result = Math.max(result, min);
  }

  if (max !== null) {
    result = Math.min(result, max);
  }

  return result;
}

/** URL-slug normalization: strip diacritics, kebab-case, collapse dashes. */
export function slugify(input: string): string {
  return input
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9\s-]/g, "")
    .replace(/[\s_]+/g, "-")
    .replace(/-+/g, "-")
    .replace(/^-|-$/g, "");
}

export function isValidSlugFormat(slug: string, limit = 100): boolean {
  return /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug) && slug.length >= 2 && slug.length <= limit;
}

import { getContext, setContext } from "svelte";

import type { PillSize } from "./types";

export type PillTypography = "label" | "inherit";

export interface PillContextValue {
  size?: PillSize;
  typography?: PillTypography;
}

const POODLE_PILL_CONTEXT = Symbol("poodle-pill-context");

export function setPillContext(value: PillContextValue): void {
  setContext(POODLE_PILL_CONTEXT, value);
}

export function getPillContext(): PillContextValue | null {
  return getContext<PillContextValue>(POODLE_PILL_CONTEXT) ?? null;
}

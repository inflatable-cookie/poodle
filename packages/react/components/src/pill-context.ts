import { createContext, useContext } from "react";

import type { PillSize, PillTypography } from "./types";

/** Parent scope (e.g. pill groups) can pin pill size/typography. */
export interface PillContextValue {
  size?: PillSize;
  typography?: PillTypography;
}

export const PillContext = createContext<PillContextValue | null>(null);

export function usePillContext(): PillContextValue | null {
  return useContext(PillContext);
}

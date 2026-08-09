import type { ReactNode } from "react";

import { IconSetContext } from "./icon-registry";
import type { IconSet } from "./types";

// An icon set mapping kebab-case names to SVG node arrays.
// Any icon set in this format works — a generated Lucide set, a Phosphor
// equivalent, or a custom set.
// String lookups resolve from this set first, then Poodle's scoped default
// Lucide set.
export interface IconProviderProps {
  icons: IconSet;
  children?: ReactNode;
}

export function IconProvider({ icons, children }: IconProviderProps) {
  return <IconSetContext.Provider value={icons}>{children}</IconSetContext.Provider>;
}

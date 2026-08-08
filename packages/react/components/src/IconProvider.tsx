import type { ReactNode } from "react";

import { IconSetContext } from "./icon-registry";
import type { IconSet } from "./types";

// A complete icon set mapping kebab-case names to SVG node arrays.
// Any icon set in this format works — lucide-static/icon-nodes.json,
// a Phosphor equivalent, or a custom set.
// String-based icon lookups resolve from this set first. If not found
// and @inflatable-cookie/poodle-icons-lucide is installed, icons are lazily auto-imported.
export interface IconProviderProps {
  icons: IconSet;
  children?: ReactNode;
}

export function IconProvider({ icons, children }: IconProviderProps) {
  return <IconSetContext.Provider value={icons}>{children}</IconSetContext.Provider>;
}

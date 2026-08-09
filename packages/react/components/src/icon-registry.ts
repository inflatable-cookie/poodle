import { createContext, useContext } from "react";

export { resolveIconNodes } from "@inflatable-cookie/poodle-core/icons";

import type { IconSet } from "./types";

/** Icon-set context set by `IconProvider`; string lookups resolve here first. */
export const IconSetContext = createContext<IconSet>({});

export function useIconSet(): IconSet {
  return useContext(IconSetContext);
}

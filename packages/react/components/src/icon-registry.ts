import { createContext, useContext } from "react";

import { defaultLucideIconSet } from "@inflatable-cookie/poodle-core/icons";

import type { IconNodeElement, IconNodes, IconSet } from "./types";

/** Icon-set context set by `IconProvider`; string lookups resolve here first. */
export const IconSetContext = createContext<IconSet>({});

export function useIconSet(): IconSet {
  return useContext(IconSetContext);
}

/**
 * Icon resolution for `@inflatable-cookie/poodle-react`. Mirrors the Svelte package's
 * `icon-registry.ts` — the alias table must stay identical (edit both).
 */

const aliases: Record<string, string> = {
  "alert-circle": "circle-alert",
  "check-square": "square-check",
  "check-circle": "circle-check",
  "edit": "pencil",
  "filter": "list-filter",
  "more-horizontal": "ellipsis",
  "more-vertical": "ellipsis-vertical",
  "package": "package-icon",
  "unlock": "lock-open",
};

const reportedMissingIcons = new Set<string>();

function reportMissingIcon(name: string): IconNodes {
  if (!reportedMissingIcons.has(name)) {
    reportedMissingIcons.add(name);
    console.error(
      `[Poodle] Unresolved icon "${name}". Add it to the nearest IconProvider set or pass IconNodes directly.`,
    );
  }
  return defaultLucideIconSet["circle-x"];
}

export function resolveIconNodes(
  ref: IconNodes | string | null | undefined,
  iconSet?: IconSet | null,
): IconNodeElement[] {
  if (!ref) return [];
  if (Array.isArray(ref)) return ref;

  const canonical = aliases[ref] ?? ref;
  if (iconSet && canonical in iconSet) return iconSet[canonical];
  if (iconSet && ref in iconSet) return iconSet[ref];

  if (canonical in defaultLucideIconSet) return defaultLucideIconSet[canonical];
  if (ref in defaultLucideIconSet) return defaultLucideIconSet[ref];

  return reportMissingIcon(ref);
}

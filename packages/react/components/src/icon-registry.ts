import { createContext, useContext } from "react";

import * as lucideIcons from "@poodle/icons-lucide";

import type { IconNodeElement, IconNodes, IconSet } from "./types";

/** Icon-set context set by `IconProvider`; string lookups resolve here first. */
export const IconSetContext = createContext<IconSet>({});

export function useIconSet(): IconSet {
  return useContext(IconSetContext);
}

/**
 * Icon resolution for `@poodle/react`. Mirrors the Svelte package's
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

function kebabToCamel(name: string): string {
  return name.replace(/-([a-z0-9])/g, (_, c: string) => c.toUpperCase());
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

  const iconModule = lucideIcons as unknown as Record<string, IconNodes>;
  return iconModule[kebabToCamel(canonical)] ?? [];
}

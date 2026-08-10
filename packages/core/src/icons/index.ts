import { iconAliases } from "./aliases.generated";
import { defaultLucideIconSet } from "./generated";
import type { IconNodeElement, IconNodes, IconSet } from "./types";

export * from "./generated";
export type { IconNodeElement, IconNodes, IconSet } from "./types";
export { createIconSet } from "./types";

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

/** Resolve direct icon nodes or a name against an operator set and Poodle's
 * scoped Lucide defaults. */
export function resolveIconNodes(
  ref: IconNodes | string | null | undefined,
  iconSet?: IconSet | null,
): IconNodeElement[] {
  if (!ref) return [];
  if (Array.isArray(ref)) return ref;

  const canonical = iconAliases[ref] ?? ref;
  if (iconSet && canonical in iconSet) return iconSet[canonical];
  if (iconSet && ref in iconSet) return iconSet[ref];
  if (canonical in defaultLucideIconSet) return defaultLucideIconSet[canonical];
  if (ref in defaultLucideIconSet) return defaultLucideIconSet[ref];

  return reportMissingIcon(ref);
}

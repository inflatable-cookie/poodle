import { iconAliases } from "./aliases.generated";
import { defaultLucideIconSet } from "./generated";
import type { IconNodeElement, IconNodes, IconSet } from "./types";

export * from "./generated";
export type { IconNodeElement, IconNodes, IconSet } from "./types";
export { createIconSet } from "./types";

const reportedMissingIcons = new Set<string>();

function lookupIcon(set: IconSet, name: string): IconNodes | undefined {
  if (!(name in set)) return undefined;
  return set[name];
}

function reportMissingIcon(name: string): IconNodes {
  if (!reportedMissingIcons.has(name)) {
    reportedMissingIcons.add(name);
    console.error(
      `[Poodle] Unresolved icon "${name}". Add it to the nearest IconProvider set or pass IconNodes directly.`,
    );
  }
  const fallback = lookupIcon(defaultLucideIconSet, "circle-x");
  if (!fallback) {
    throw new Error(`[Poodle] Missing default fallback icon "circle-x".`);
  }
  return fallback;
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
  const fromSet =
    iconSet ? lookupIcon(iconSet, canonical) ?? lookupIcon(iconSet, ref) : undefined;
  if (fromSet) return fromSet;
  const fromDefault =
    lookupIcon(defaultLucideIconSet, canonical) ?? lookupIcon(defaultLucideIconSet, ref);
  if (fromDefault) return fromDefault;

  return reportMissingIcon(ref);
}

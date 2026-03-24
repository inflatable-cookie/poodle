import { getContext, setContext } from "svelte";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** A single SVG child element: [tagName, attributes]. */
export type IconNodeElement = [string, Record<string, string>];

/**
 * The SVG node data for a single icon — an array of `[tagName, attributes]`
 * tuples describing the SVG children.
 *
 * This is the raw data format used by `lucide-static/icon-nodes.json` and
 * compatible icon set packages. Pass directly to the `Icon` component.
 */
export type IconNodes = IconNodeElement[];

/** A complete icon set: a map of kebab-case names to SVG node arrays. */
export type IconSet = Record<string, IconNodes>;

// ---------------------------------------------------------------------------
// Context (for providing / overriding icon sets)
// ---------------------------------------------------------------------------

const POODLE_ICON_SET = Symbol("poodle-icon-set");

/** @internal Set an icon set via Svelte context. Used by `IconProvider`. */
export function setIconSet(icons: IconSet): void {
  setContext(POODLE_ICON_SET, icons);
}

/** @internal Read the icon set from Svelte context. */
export function getIconSet(): IconSet | null {
  return getContext<IconSet>(POODLE_ICON_SET) ?? null;
}

// ---------------------------------------------------------------------------
// Aliases (legacy / shorthand names → canonical Lucide names)
// ---------------------------------------------------------------------------

const aliases: Record<string, string> = {
  "alert-circle": "circle-alert",
  "check-circle": "circle-check",
  "edit": "pencil",
  "filter": "list-filter",
  "more-horizontal": "ellipsis",
  "more-vertical": "ellipsis-vertical",
  "unlock": "lock-open",
};

// ---------------------------------------------------------------------------
// Lazy icon cache
// ---------------------------------------------------------------------------

/** Cache of lazily-loaded icons. Once resolved, lookups are synchronous. */
const lazyCache: IconSet = {};

/** In-flight import promises, keyed by canonical kebab-case name. */
const inflight: Record<string, Promise<IconNodes>> = {};

/**
 * Convert kebab-case icon name to camelCase export name.
 * e.g. "circle-check" → "circleCheck", "arrow-up" → "arrowUp"
 */
function kebabToCamel(name: string): string {
  return name.replace(/-([a-z0-9])/g, (_, c) => c.toUpperCase());
}

/**
 * Lazily import a single icon from `@poodle/icons-lucide`.
 * Returns cached data immediately if already loaded, otherwise kicks off
 * an async import and caches the result for future sync access.
 */
export function lazyResolveIcon(
  name: string,
  onLoaded?: () => void,
): IconNodeElement[] {
  // Resolve aliases
  const canonical = aliases[name] ?? name;

  // Already cached — return immediately
  if (canonical in lazyCache) return lazyCache[canonical];

  // Already in-flight — attach callback
  if (canonical in inflight) {
    if (onLoaded) {
      inflight[canonical].then(onLoaded);
    }
    return [];
  }

  // Kick off dynamic import
  const exportName = kebabToCamel(canonical);
  inflight[canonical] = import(`@poodle/icons-lucide`)
    .then((mod) => {
      const iconModule = mod as unknown as Record<string, IconNodes>;
      const nodes: IconNodes = iconModule[exportName] ?? [];
      lazyCache[canonical] = nodes;
      // Also cache under the alias if used
      if (name !== canonical) lazyCache[name] = nodes;
      delete inflight[canonical];
      if (onLoaded) onLoaded();
      return nodes;
    })
    .catch(() => {
      // Icon not found — cache empty to avoid re-fetching
      lazyCache[canonical] = [];
      delete inflight[canonical];
      if (onLoaded) onLoaded();
      return [] as IconNodes;
    });

  return [];
}

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

/**
 * Resolve an icon reference to SVG nodes (synchronous path).
 *
 * - If `ref` is an `IconNodes` array (array of arrays), returns it directly.
 * - If `ref` is a string, checks the context icon set first, then the
 *   lazy cache of previously loaded icons.
 * - Returns an empty array if the icon is not yet loaded.
 */
export function resolveIconNodes(
  ref: IconNodes | string | null | undefined,
  iconSet?: IconSet | null,
): IconNodeElement[] {
  if (!ref) return [];

  // Direct icon data — array of [tag, attrs] tuples
  if (Array.isArray(ref)) return ref;

  // Resolve aliases
  const canonical = aliases[ref] ?? ref;

  // String name — check provided icon set first
  if (iconSet && canonical in iconSet) return iconSet[canonical];
  // Also check under the original name (icon sets may use either form)
  if (iconSet && ref in iconSet) return iconSet[ref];

  // Check lazy cache
  if (canonical in lazyCache) return lazyCache[canonical];

  return [];
}

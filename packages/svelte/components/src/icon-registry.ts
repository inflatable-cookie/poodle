import { getContext, setContext } from "svelte";
import { writable, type Readable, type Writable } from "svelte/store";
import * as lucideIcons from "@inflatable-cookie/poodle-icons-lucide";

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
const DEFAULT_ICON_SET_STORE = writable<IconSet>({});

/** @internal Set an icon set via Svelte context. Used by `IconProvider`. */
export function setIconSet(icons: IconSet): Writable<IconSet> {
  const store = writable(icons);
  setContext(POODLE_ICON_SET, store);
  return store;
}

/** @internal Read the icon set store from Svelte context. */
export function getIconSetStore(): Readable<IconSet> {
  return getContext<Readable<IconSet>>(POODLE_ICON_SET) ?? DEFAULT_ICON_SET_STORE;
}

// ---------------------------------------------------------------------------
// Aliases (legacy / shorthand names → canonical Lucide names)
// ---------------------------------------------------------------------------

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

/**
 * Convert kebab-case icon name to camelCase export name.
 * e.g. "circle-check" → "circleCheck", "arrow-up" → "arrowUp"
 */
function kebabToCamel(name: string): string {
  return name.replace(/-([a-z0-9])/g, (_, c) => c.toUpperCase());
}

/**
 * Resolve a single built-in icon from `@inflatable-cookie/poodle-icons-lucide`.
 *
 * This remains a public helper for compatibility with existing primitive
 * internals, but resolution is now synchronous so source-linked consumers do
 * not rely on package-boundary async imports during rendering.
 */
export function lazyResolveIcon(
  name: string,
  onLoaded?: () => void,
): IconNodeElement[] {
  const canonical = aliases[name] ?? name;
  const exportName = kebabToCamel(canonical);
  const iconModule = lucideIcons as unknown as Record<string, IconNodes>;
  const nodes = iconModule[exportName] ?? [];
  if (onLoaded) onLoaded();
  return nodes;
}

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

/**
 * Resolve an icon reference to SVG nodes (synchronous path).
 *
 * - If `ref` is an `IconNodes` array (array of arrays), returns it directly.
 * - If `ref` is a string, checks the context icon set first, then the
 *   built-in Poodle icon set.
 * - Returns an empty array if the icon is unknown.
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

  return lazyResolveIcon(canonical);
}

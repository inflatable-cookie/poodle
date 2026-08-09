import { getContext, setContext } from "svelte";
import { writable, type Readable, type Writable } from "svelte/store";
import { defaultLucideIconSet } from "@inflatable-cookie/poodle-core/icons";

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

/** An icon set: a map of kebab-case names to SVG node arrays. */
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
  "alert-triangle": "triangle-alert",
  "check-square": "square-check",
  "check-circle": "circle-check",
  "circle-help": "circle-question-mark",
  "edit": "pencil",
  "file-question": "file-question-mark",
  "filter": "list-filter",
  "more-horizontal": "ellipsis",
  "more-vertical": "ellipsis-vertical",
  "help-circle": "circle-question-mark",
  "package": "package-icon",
  "pause-circle": "circle-pause",
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

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

/**
 * Resolve an icon reference to SVG nodes (synchronous path).
 *
 * - If `ref` is an `IconNodes` array (array of arrays), returns it directly.
 * - If `ref` is a string, checks the context icon set first, then the
 *   scoped default Lucide set required by Poodle components.
 * - Reports an unknown string and renders the default error glyph.
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

  if (canonical in defaultLucideIconSet) return defaultLucideIconSet[canonical];
  if (ref in defaultLucideIconSet) return defaultLucideIconSet[ref];

  return reportMissingIcon(ref);
}

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
 *
 * @example
 * ```svelte
 * <script>
 *   import { Icon } from "@pug/svelte-primitives";
 *   import iconNodes from "lucide-static/icon-nodes.json";
 * </script>
 *
 * <Icon icon={iconNodes["search"]} />
 * ```
 */
export type IconNodes = IconNodeElement[];

/** A complete icon set: a map of kebab-case names to SVG node arrays. */
export type IconSet = Record<string, IconNodes>;

// ---------------------------------------------------------------------------
// Context (for providing / overriding icon sets)
// ---------------------------------------------------------------------------

const PUG_ICON_SET = Symbol("pug-icon-set");

/** @internal Set an icon set via Svelte context. Used by `IconProvider`. */
export function setIconSet(icons: IconSet): void {
  setContext(PUG_ICON_SET, icons);
}

/** @internal Read the icon set from Svelte context. */
export function getIconSet(): IconSet | null {
  return getContext<IconSet>(PUG_ICON_SET) ?? null;
}

// ---------------------------------------------------------------------------
// Internal (built-in) icons
// ---------------------------------------------------------------------------

/**
 * Icons used internally by Pug component chrome. This set is intentionally
 * minimal — it covers only the icons referenced by primitives and composites
 * (Select chevrons, Callout dismiss, Breadcrumb separators, etc.) so that
 * the framework works without any user-supplied icons.
 */

/** Aliases that map legacy / shorthand names to canonical Lucide names. */
const aliases: Record<string, string> = {
  "alert-circle": "circle-alert",
  "check-circle": "circle-check",
  "edit": "pencil",
  "filter": "list-filter",
  "more-horizontal": "ellipsis",
  "more-vertical": "ellipsis-vertical",
  "unlock": "lock-open",
};

/* prettier-ignore */
const internalEntries: Array<[string, IconNodes]> = [
  ["arrow-down", [["path",{"d":"M12 5v14"}],["path",{"d":"m19 12-7 7-7-7"}]]],
  ["arrow-right", [["path",{"d":"M5 12h14"}],["path",{"d":"m12 5 7 7-7 7"}]]],
  ["arrow-up", [["path",{"d":"m5 12 7-7 7 7"}],["path",{"d":"M12 19V5"}]]],
  ["check", [["path",{"d":"M20 6 9 17l-5-5"}]]],
  ["chevron-down", [["path",{"d":"m6 9 6 6 6-6"}]]],
  ["chevron-left", [["path",{"d":"m15 18-6-6 6-6"}]]],
  ["chevron-right", [["path",{"d":"m9 18 6-6-6-6"}]]],
  ["chevron-up", [["path",{"d":"m18 15-6-6-6 6"}]]],
  ["circle-alert", [["circle",{"cx":"12","cy":"12","r":"10"}],["line",{"x1":"12","x2":"12","y1":"8","y2":"12"}],["line",{"x1":"12","x2":"12.01","y1":"16","y2":"16"}]]],
  ["circle-check", [["circle",{"cx":"12","cy":"12","r":"10"}],["path",{"d":"m9 12 2 2 4-4"}]]],
  ["circle-x", [["circle",{"cx":"12","cy":"12","r":"10"}],["path",{"d":"m15 9-6 6"}],["path",{"d":"m9 9 6 6"}]]],
  ["columns-3", [["rect",{"width":"18","height":"18","x":"3","y":"3","rx":"2"}],["path",{"d":"M9 3v18"}],["path",{"d":"M15 3v18"}]]],
  ["download", [["path",{"d":"M12 15V3"}],["path",{"d":"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"}],["path",{"d":"m7 10 5 5 5-5"}]]],
  ["ellipsis", [["circle",{"cx":"12","cy":"12","r":"1"}],["circle",{"cx":"19","cy":"12","r":"1"}],["circle",{"cx":"5","cy":"12","r":"1"}]]],
  ["ellipsis-vertical", [["circle",{"cx":"12","cy":"12","r":"1"}],["circle",{"cx":"12","cy":"5","r":"1"}],["circle",{"cx":"12","cy":"19","r":"1"}]]],
  ["external-link", [["path",{"d":"M15 3h6v6"}],["path",{"d":"M10 14 21 3"}],["path",{"d":"M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"}]]],
  ["file-text", [["path",{"d":"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"}],["path",{"d":"M14 2v5a1 1 0 0 0 1 1h5"}],["path",{"d":"M10 9H8"}],["path",{"d":"M16 13H8"}],["path",{"d":"M16 17H8"}]]],
  ["grip-vertical", [["circle",{"cx":"9","cy":"12","r":"1"}],["circle",{"cx":"9","cy":"5","r":"1"}],["circle",{"cx":"9","cy":"19","r":"1"}],["circle",{"cx":"15","cy":"12","r":"1"}],["circle",{"cx":"15","cy":"5","r":"1"}],["circle",{"cx":"15","cy":"19","r":"1"}]]],
  ["image", [["rect",{"width":"18","height":"18","x":"3","y":"3","rx":"2","ry":"2"}],["circle",{"cx":"9","cy":"9","r":"2"}],["path",{"d":"m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"}]]],
  ["inbox", [["polyline",{"points":"22 12 16 12 14 15 10 15 8 12 2 12"}],["path",{"d":"M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"}]]],
  ["info", [["circle",{"cx":"12","cy":"12","r":"10"}],["path",{"d":"M12 16v-4"}],["path",{"d":"M12 8h.01"}]]],
  ["list-filter", [["path",{"d":"M2 5h20"}],["path",{"d":"M6 12h12"}],["path",{"d":"M9 19h6"}]]],
  ["loader", [["path",{"d":"M12 2v4"}],["path",{"d":"m16.2 7.8 2.9-2.9"}],["path",{"d":"M18 12h4"}],["path",{"d":"m16.2 16.2 2.9 2.9"}],["path",{"d":"M12 18v4"}],["path",{"d":"m4.9 19.1 2.9-2.9"}],["path",{"d":"M2 12h4"}],["path",{"d":"m4.9 4.9 2.9 2.9"}]]],
  ["lock-open", [["rect",{"width":"18","height":"11","x":"3","y":"11","rx":"2","ry":"2"}],["path",{"d":"M7 11V7a5 5 0 0 1 9.9-1"}]]],
  ["minus", [["path",{"d":"M5 12h14"}]]],
  ["music", [["path",{"d":"M9 18V5l12-2v13"}],["circle",{"cx":"6","cy":"18","r":"3"}],["circle",{"cx":"18","cy":"16","r":"3"}]]],
  ["pencil", [["path",{"d":"M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"}],["path",{"d":"m15 5 4 4"}]]],
  ["play", [["path",{"d":"M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z"}]]],
  ["plus", [["path",{"d":"M5 12h14"}],["path",{"d":"M12 5v14"}]]],
  ["search", [["path",{"d":"m21 21-4.34-4.34"}],["circle",{"cx":"11","cy":"11","r":"8"}]]],
  ["star", [["path",{"d":"M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"}]]],
  ["trending-down", [["path",{"d":"M16 17h6v-6"}],["path",{"d":"m22 17-8.5-8.5-5 5L2 7"}]]],
  ["trending-up", [["path",{"d":"M16 7h6v6"}],["path",{"d":"m22 7-8.5 8.5-5-5L2 17"}]]],
  ["triangle-alert", [["path",{"d":"m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"}],["path",{"d":"M12 9v4"}],["path",{"d":"M12 17h.01"}]]],
  ["x", [["path",{"d":"M18 6 6 18"}],["path",{"d":"m6 6 12 12"}]]],
];

/** @internal Built-in icon lookup table. */
const internalIcons: IconSet = Object.fromEntries([
  ...internalEntries,
  ...Object.entries(aliases).map(([alias, canonical]) => {
    const entry = internalEntries.find(([name]) => name === canonical);
    return [alias, entry ? entry[1] : []];
  }),
]);

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

/**
 * Resolve an icon reference to SVG nodes.
 *
 * - If `ref` is an `IconNodes` array (array of arrays), returns it directly.
 * - If `ref` is a string, checks the context icon set first, then the
 *   built-in internal icons.
 * - Returns an empty array if the icon is not found.
 */
export function resolveIconNodes(
  ref: IconNodes | string | null | undefined,
  iconSet?: IconSet | null,
): IconNodeElement[] {
  if (!ref) return [];

  // Direct icon data — array of [tag, attrs] tuples
  if (Array.isArray(ref)) return ref;

  // String name — check provided icon set first, then built-in internals
  if (iconSet && ref in iconSet) return iconSet[ref];

  return internalIcons[ref] ?? [];
}

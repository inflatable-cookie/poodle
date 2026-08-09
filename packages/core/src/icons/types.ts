/** A single SVG child element: [tagName, attributes]. */
export type IconNodeElement = [string, Record<string, string>];

/** The SVG node data for a single icon. */
export type IconNodes = IconNodeElement[];

/** A name-to-node set accepted by Poodle icon providers. */
export type IconSet = Record<string, IconNodes>;

/**
 * Type a tree-shakeable set assembled from named icon-node imports.
 *
 * Do not pass a default or namespace import of a full icon catalogue: a
 * runtime property pick cannot be tree-shaken.
 */
export function createIconSet(icons: Record<string, unknown>): IconSet {
  return icons as IconSet;
}

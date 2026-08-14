/** Generic computed DOM channels shared by component and primitive probes. */

export const geometryFields = [
  "height",
  "minWidth",
  "paddingLeft",
  "paddingRight",
  "radius",
  "borderWidth",
] as const;

/** Parse a computed CSS length; calc()/var() residues are not observable. */
export function parseLength(value: string | null): number | null {
  if (!value || value.includes("calc(")) return null;
  const parsed = Number.parseFloat(value);
  return Number.isFinite(parsed) ? parsed : null;
}

export function geometryOf(root: HTMLElement): Record<string, number | null> {
  const style = root.ownerDocument.defaultView?.getComputedStyle(root);
  if (!style) return Object.fromEntries(geometryFields.map((field) => [field, null]));
  return {
    height: parseLength(style.height),
    minWidth: parseLength(style.minWidth),
    paddingLeft: parseLength(style.paddingLeft),
    paddingRight: parseLength(style.paddingRight),
    radius: parseLength(style.borderRadius),
    borderWidth: parseLength(style.borderWidth),
  };
}

export function channelsOf(root: HTMLElement): Record<string, string | null> {
  const style = root.ownerDocument.defaultView?.getComputedStyle(root);
  if (!style) return { background: null, borderColor: null, color: null, opacity: null };
  const clean = (value: string | null): string | null =>
    value && !value.includes("color-mix") && !value.includes("calc(") ? value : null;
  return {
    background: clean(style.backgroundColor),
    borderColor: clean(style.borderColor),
    color: clean(style.color),
    opacity: clean(style.opacity),
  };
}

export function computedStyleOf(root: HTMLElement): CSSStyleDeclaration {
  const style = root.ownerDocument.defaultView?.getComputedStyle(root);
  if (!style) throw new Error("computed style is unavailable for the mounted element");
  return style;
}

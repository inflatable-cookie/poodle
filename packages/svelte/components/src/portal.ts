export function portal(node: HTMLElement): { destroy(): void } {
  if (typeof document === "undefined") {
    return {
      destroy() {},
    };
  }

  const explicitThemeRoot = node.parentElement?.closest("[data-poodle-theme-root]") as HTMLElement | null;
  const nearestThemeAncestor = node.parentElement?.closest("[data-theme]") as HTMLElement | null;

  // Portal into an explicit theme root when present. Otherwise use the nearest
  // themed ancestor, but never mount overlays directly under <html>.
  const target = explicitThemeRoot
    ?? (nearestThemeAncestor && nearestThemeAncestor !== document.documentElement
      ? nearestThemeAncestor
      : document.body);
  target.appendChild(node);

  return {
    destroy() {
      if (node.parentNode === target) {
        target.removeChild(node);
      }
    },
  };
}

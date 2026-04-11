export function portal(node: HTMLElement): { destroy(): void } {
  if (typeof document === "undefined") {
    return {
      destroy() {},
    };
  }

  // Portal into the nearest themed ancestor so overlays inherit
  // CSS custom properties (colors, typography, etc.) from the
  // active theme. Falls back to document.body.
  const target =
    node.parentElement?.closest("[data-theme]") as HTMLElement | null
    ?? document.body;
  target.appendChild(node);

  return {
    destroy() {
      if (node.parentNode === target) {
        target.removeChild(node);
      }
    },
  };
}

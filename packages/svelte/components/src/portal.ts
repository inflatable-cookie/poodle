import { resolvePortalTarget } from "@inflatable-cookie/poodle-core";

export function portal(node: HTMLElement): { destroy(): void } {
  if (typeof document === "undefined") {
    return {
      destroy() {},
    };
  }

  const target = resolvePortalTarget(node);
  target.appendChild(node);

  return {
    destroy() {
      if (node.parentNode === target) {
        target.removeChild(node);
      }
    },
  };
}

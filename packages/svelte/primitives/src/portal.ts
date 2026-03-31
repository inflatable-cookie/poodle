export function portal(node: HTMLElement): { destroy(): void } {
  if (typeof document === "undefined") {
    return {
      destroy() {},
    };
  }

  const target = document.body;
  target.appendChild(node);

  return {
    destroy() {
      if (node.parentNode === target) {
        target.removeChild(node);
      }
    },
  };
}

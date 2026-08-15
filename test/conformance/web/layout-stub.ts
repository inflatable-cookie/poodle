/**
 * Minimal box-model surrogate for happy-dom, which implements no layout at
 * all (every rect is zero). Elements with an explicit box — an inline
 * positioned origin or a width/height — report that box; everything else
 * reports the degenerate zero rect happy-dom itself would.
 *
 * Same posture as the harness's browser-default simulation: happy-dom cannot
 * lay out, so the harness supplies the box a real browser would, and anchored
 * overlay placement resolves against a real anchor origin. Test-environment
 * only; installed by the popover web test.
 */

export function installLayoutStub(): () => void {
  const original = Element.prototype.getBoundingClientRect;
  Element.prototype.getBoundingClientRect = function getBoundingClientRect(this: Element): DOMRect {
    if (this instanceof HTMLElement) {
      const style = this.ownerDocument.defaultView?.getComputedStyle(this);
      if (style) {
        const top = Number.parseFloat(style.top) || 0;
        const left = Number.parseFloat(style.left) || 0;
        const width = Number.parseFloat(style.width) || 0;
        const height = Number.parseFloat(style.height) || 0;
        if (top !== 0 || left !== 0 || width !== 0 || height !== 0) {
          return {
            top,
            left,
            width,
            height,
            right: left + width,
            bottom: top + height,
            x: left,
            y: top,
          } as DOMRect;
        }
      }
    }
    return original.call(this);
  };
  return () => {
    Element.prototype.getBoundingClientRect = original;
  };
}

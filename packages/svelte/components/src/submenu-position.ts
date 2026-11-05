import {
  isAnchorClipped,
  observeAnchorMovement,
  resolveClipRect,
} from "@inflatable-cookie/poodle-core";

/** A fixed descendant escapes the scrolling menu while retaining dismiss ownership. */
export function submenuPosition(node: HTMLElement, nested: boolean) {
  if (!nested) return {};
  const anchor = node.parentElement;
  if (!anchor) return {};
  let previous = "";
  function reposition() {
    const row = anchor!.getBoundingClientRect();
    const viewport = { width: window.innerWidth, height: window.innerHeight };
    const parentSurface = anchor!.closest<HTMLElement>('[role="menu"]');
    const hidden = parentSurface?.dataset.anchorHidden === "true" ||
      isAnchorClipped(row, resolveClipRect(anchor, viewport));
    if (node.dataset.anchorHidden !== String(hidden)) node.dataset.anchorHidden = String(hidden);
    const maxHeight = Math.max(0, viewport.height - 16);
    const maxWidth = Math.max(0, viewport.width - 16);
    if (node.style.maxHeight !== `${maxHeight}px`) node.style.maxHeight = `${maxHeight}px`;
    if (node.style.maxWidth !== `${maxWidth}px`) node.style.maxWidth = `${maxWidth}px`;
    const rect = node.getBoundingClientRect();
    const padding = Number.parseFloat(getComputedStyle(node).paddingTop) || 0;
    const flipped = row.right + rect.width > viewport.width - 8;
    const left = Math.max(8, Math.min(flipped ? row.left - rect.width : row.right, viewport.width - rect.width - 8));
    const top = Math.max(8, Math.min(row.top - padding, viewport.height - rect.height - 8));
    const next = `${left},${top},${flipped}`;
    if (next !== previous) {
      node.style.left = `${Math.round(left)}px`;
      node.style.top = `${Math.round(top)}px`;
      node.classList.toggle("poodle-menu-surface--flipped", flipped);
      previous = next;
    }
  }
  reposition();
  const stop = observeAnchorMovement(anchor, node, reposition);
  // Layout and transformed ancestors may move without scroll/resize events.
  let frame = 0;
  function track() {
    reposition();
    frame = requestAnimationFrame(track);
  }
  frame = requestAnimationFrame(track);
  return { destroy() { stop(); cancelAnimationFrame(frame); } };
}

/** Read indent and label-gutter from a tree row for outline-drop X mapping. */
export function readTreeDropMetrics(row: HTMLElement): { indentPx: number; gutterPx: number } {
  const tree = row.closest(".poodle-tree");
  const treeStyles = getComputedStyle(tree ?? row);
  const indentVar = cssLength(treeStyles, "--poodle-tree-indent", 16);
  const padVar = cssLength(treeStyles, "--poodle-tree-row-pad-inline", 6);
  const twistyVar = cssLength(treeStyles, "--poodle-tree-twisty-size", 20);
  const indentEl = row.querySelector(".poodle-tree__indent");
  const twistyEl = row.querySelector(".poodle-tree__twisty");
  const indentPx = indentEl?.getBoundingClientRect().width || indentVar;
  const padPx = parseFloat(getComputedStyle(row).paddingLeft) || padVar;
  const twistyPx = twistyEl?.getBoundingClientRect().width || twistyVar;
  return { indentPx, gutterPx: padPx + twistyPx };
}

function cssLength(styles: CSSStyleDeclaration, property: string, fallback: number): number {
  const raw = styles.getPropertyValue(property).trim();
  if (!raw) return fallback;
  const value = parseFloat(raw);
  if (!Number.isFinite(value)) return fallback;
  if (raw.endsWith("rem")) {
    const root = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
    return value * root;
  }
  return value;
}

import { arrowDown } from "./icons/arrow-down";
import { arrowLeft } from "./icons/arrow-left";
import { arrowRight } from "./icons/arrow-right";
import { arrowUp } from "./icons/arrow-up";
import { arrowUpDown } from "./icons/arrow-up-down";
import { bold } from "./icons/bold";
import { check } from "./icons/check";
import { checkCheck } from "./icons/check-check";
import { chevronDown } from "./icons/chevron-down";
import { chevronLeft } from "./icons/chevron-left";
import { chevronRight } from "./icons/chevron-right";
import { chevronUp } from "./icons/chevron-up";
import { circle } from "./icons/circle";
import { circleX } from "./icons/circle-x";
import { code } from "./icons/code";
import { columns2 } from "./icons/columns-2";
import { columns3 } from "./icons/columns-3";
import { diff } from "./icons/diff";
import { dot } from "./icons/dot";
import { download } from "./icons/download";
import { ellipsis } from "./icons/ellipsis";
import { externalLink } from "./icons/external-link";
import { eye } from "./icons/eye";
import { file } from "./icons/file";
import { filePen } from "./icons/file-pen";
import { fileText } from "./icons/file-text";
import { gitBranch } from "./icons/git-branch";
import { gitCommitHorizontal } from "./icons/git-commit-horizontal";
import { gripVertical } from "./icons/grip-vertical";
import { heading } from "./icons/heading";
import { image } from "./icons/image";
import { inbox } from "./icons/inbox";
import { info } from "./icons/info";
import { italic } from "./icons/italic";
import { link } from "./icons/link";
import { list } from "./icons/list";
import { menu } from "./icons/menu";
import { minus } from "./icons/minus";
import { music } from "./icons/music";
import { pencil } from "./icons/pencil";
import { play } from "./icons/play";
import { plus } from "./icons/plus";
import { quote } from "./icons/quote";
import { refreshCw } from "./icons/refresh-cw";
import { search } from "./icons/search";
import { square } from "./icons/square";
import { star } from "./icons/star";
import { tag } from "./icons/tag";
import { terminal } from "./icons/terminal";
import { trash2 } from "./icons/trash-2";
import { trendingDown } from "./icons/trending-down";
import { trendingUp } from "./icons/trending-up";
import { triangleAlert } from "./icons/triangle-alert";
import { x } from "./icons/x";
import type { IconNodeElement, IconNodes, IconSet } from "./types";

export type { IconNodeElement, IconNodes, IconSet } from "./types";
export { createIconSet } from "./types";

export {
  arrowDown,
  arrowLeft,
  arrowRight,
  arrowUp,
  arrowUpDown,
  bold,
  check,
  checkCheck,
  chevronDown,
  chevronLeft,
  chevronRight,
  chevronUp,
  circle,
  circleX,
  code,
  columns2,
  columns3,
  diff,
  dot,
  download,
  ellipsis,
  externalLink,
  eye,
  file,
  filePen,
  fileText,
  gitBranch,
  gitCommitHorizontal,
  gripVertical,
  heading,
  image,
  inbox,
  info,
  italic,
  link,
  list,
  menu,
  minus,
  music,
  pencil,
  play,
  plus,
  quote,
  refreshCw,
  search,
  square,
  star,
  tag,
  terminal,
  trash2,
  trendingDown,
  trendingUp,
  triangleAlert,
  x,
};

/**
 * The scoped Lucide set required by Poodle's own component chrome.
 * Application icons belong in an `IconProvider` set.
 */
export const defaultLucideIconSet: IconSet = {
  "arrow-down": arrowDown,
  "arrow-left": arrowLeft,
  "arrow-right": arrowRight,
  "arrow-up": arrowUp,
  "arrow-up-down": arrowUpDown,
  bold,
  check,
  "check-check": checkCheck,
  "chevron-down": chevronDown,
  "chevron-left": chevronLeft,
  "chevron-right": chevronRight,
  "chevron-up": chevronUp,
  circle,
  "circle-x": circleX,
  code,
  "columns-2": columns2,
  "columns-3": columns3,
  diff,
  dot,
  download,
  ellipsis,
  "external-link": externalLink,
  eye,
  file,
  "file-pen": filePen,
  "file-text": fileText,
  "git-branch": gitBranch,
  "git-commit-horizontal": gitCommitHorizontal,
  "grip-vertical": gripVertical,
  heading,
  image,
  inbox,
  info,
  italic,
  link,
  list,
  menu,
  minus,
  music,
  pencil,
  play,
  plus,
  quote,
  "refresh-cw": refreshCw,
  search,
  square,
  star,
  tag,
  terminal,
  "trash-2": trash2,
  "trending-down": trendingDown,
  "trending-up": trendingUp,
  "triangle-alert": triangleAlert,
  x,
};

/** Legacy and shorthand names mapped to their canonical Lucide names. */
const iconAliases: Readonly<Record<string, string>> = {
  "alert-circle": "circle-alert",
  "alert-triangle": "triangle-alert",
  "check-square": "square-check",
  "check-circle": "circle-check",
  "circle-help": "circle-question-mark",
  edit: "pencil",
  "file-question": "file-question-mark",
  filter: "list-filter",
  "more-horizontal": "ellipsis",
  "more-vertical": "ellipsis-vertical",
  "help-circle": "circle-question-mark",
  package: "package-icon",
  "pause-circle": "circle-pause",
  unlock: "lock-open",
};

const reportedMissingIcons = new Set<string>();

function reportMissingIcon(name: string): IconNodes {
  if (!reportedMissingIcons.has(name)) {
    reportedMissingIcons.add(name);
    console.error(
      `[Poodle] Unresolved icon "${name}". Add it to the nearest IconProvider set or pass IconNodes directly.`,
    );
  }
  return defaultLucideIconSet["circle-x"];
}

/** Resolve direct icon nodes or a name against an operator set and Poodle's
 * scoped Lucide defaults. */
export function resolveIconNodes(
  ref: IconNodes | string | null | undefined,
  iconSet?: IconSet | null,
): IconNodeElement[] {
  if (!ref) return [];
  if (Array.isArray(ref)) return ref;

  const canonical = iconAliases[ref] ?? ref;
  if (iconSet && canonical in iconSet) return iconSet[canonical];
  if (iconSet && ref in iconSet) return iconSet[ref];
  if (canonical in defaultLucideIconSet) return defaultLucideIconSet[canonical];
  if (ref in defaultLucideIconSet) return defaultLucideIconSet[ref];

  return reportMissingIcon(ref);
}

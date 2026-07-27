/**
 * Contract ARIA roles vs the roles Jetstream actually projects.
 *
 *   bun scripts/contract-role-drift.ts
 *
 * `contract-spec-drift` checks that a documented *prop* reaches `poodle-specs`.
 * This checks the equivalent for accessibility: that a role a contract names in
 * prose actually appears in the accessibility tree the component renders.
 *
 * The role census comes from `cargo run --bin a11y -- --json` in the Jetstream
 * preview, which projects every specimen headlessly. So this needs the sibling
 * jetstream repo, and lives in `ci:native` with the rest of that constraint.
 *
 * **What this can and cannot see.** It compares sets, not placement: a contract
 * asking for `role="option"` is satisfied by a `ListBoxOption` appearing
 * anywhere in that component's tree, not necessarily on the right element. That
 * is deliberate — checking placement needs an anatomy model the contracts do
 * not have — and it still catches the case that matters most, which is a role
 * the component never emits at all.
 */

import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";

const repoRoot = path.resolve(import.meta.dir, "../../../..");
const CONTRACTS = path.join(repoRoot, "docs/contracts/components");

/** ARIA role name → the `accesskit::Role` that represents it. */
const ARIA_TO_ACCESSKIT: Record<string, string[]> = {
  alert: ["Alert"],
  alertdialog: ["AlertDialog"],
  button: ["Button", "DefaultButton"],
  checkbox: ["CheckBox"],
  combobox: ["ComboBox"],
  dialog: ["Dialog", "AlertDialog"],
  grid: ["Grid"],
  gridcell: ["Cell"],
  group: ["Group", "GenericContainer"],
  img: ["Image"],
  link: ["Link"],
  list: ["List"],
  listbox: ["ListBox"],
  listitem: ["ListItem"],
  log: ["Log"],
  menu: ["Menu"],
  menubar: ["MenuBar"],
  menuitem: ["MenuItem"],
  menuitemcheckbox: ["MenuItemCheckBox"],
  menuitemradio: ["MenuItemRadio"],
  option: ["ListBoxOption", "MenuListOption"],
  presentation: [],
  progressbar: ["ProgressIndicator"],
  radio: ["RadioButton"],
  radiogroup: ["RadioGroup"],
  region: ["Region"],
  row: ["Row"],
  separator: ["Splitter"],
  slider: ["Slider"],
  spinbutton: ["SpinButton"],
  status: ["Status"],
  switch: ["Switch"],
  tab: ["Tab"],
  tablist: ["TabList"],
  tabpanel: ["TabPanel"],
  toolbar: ["Toolbar"],
  tooltip: ["Tooltip"],
  tree: ["Tree"],
  treeitem: ["TreeItem"],
  none: [],
};

/**
 * Roles a contract mentions only to say the target does *not* have them, or
 * which no renderer can satisfy.
 *
 * `presentation` and `none` are assertions of absence — a contract saying
 * `role="presentation"` is asking for the element to be *skipped*, which is
 * `aria_hidden`, not a role to look for.
 */
const NOT_A_TARGET = new Set(["presentation", "none"]);

function contractRoles(slug: string): Set<string> {
  const file = path.join(CONTRACTS, `${slug}.md`);
  const text = readFileSync(file, "utf8");
  const roles = new Set<string>();
  for (const match of text.matchAll(/role=["`]?([a-z]+)["`]?/g)) {
    const role = match[1];
    if (!NOT_A_TARGET.has(role)) roles.add(role);
  }
  return roles;
}

const censusRaw = Bun.spawnSync(
  ["cargo", "run", "--quiet", "--bin", "a11y", "--", "--json"],
  { cwd: path.join(repoRoot, "packages/jetstream/preview"), stdout: "pipe", stderr: "pipe" },
);
if (!censusRaw.success) {
  console.error(`role census failed:\n${censusRaw.stderr.toString().split("\n").slice(-15).join("\n")}`);
  process.exit(1);
}
const census: Record<string, string[]> = JSON.parse(censusRaw.stdout.toString());

const known = new Set(readdirSync(CONTRACTS).filter((f) => f.endsWith(".md")).map((f) => f.replace(".md", "")));

/**
 * Roles that exist only in a state no specimen renders.
 *
 * A date picker's `dialog` exists once it is opened; a menu's `menuitem`s
 * exist once the menu is. The specimens render resting states, so these are
 * invisible to a headless projection — not absent from the component.
 *
 * They are separated rather than deleted because the distinction is the whole
 * point: a role that is *never* emitted is a defect, and a role that needs an
 * open overlay to observe is a coverage gap in the specimens. Conflating them
 * turns the count into a number nobody trusts.
 *
 * Confirming these needs specimens that render open states, which is the
 * follow-on work.
 */
const OVERLAY_ONLY: Record<string, string[]> = {
  "color-picker": ["dialog", "listbox", "option", "slider"],
  "command-palette": ["dialog", "status"],
  "context-menu": ["menu", "menuitem", "menuitemcheckbox", "menuitemradio"],
  "data-table": ["menu"],
  "date-picker": ["dialog"],
  "date-range-picker": ["dialog"],
  "date-time-picker": ["dialog"],
  "date-time-range-picker": ["dialog"],
  "date-time-zone-picker": ["dialog"],
  drawer: ["dialog"],
  field: ["dialog"],
  "filter-builder": ["dialog"],
  "hover-card": ["button", "dialog"],
  "icon-button": ["tooltip"],
  menu: ["button", "menuitem", "menuitemcheckbox", "menuitemradio"],
  menubar: ["menu", "menuitem", "menuitemcheckbox", "menuitemradio", "separator"],
  "media-picker": ["listbox", "option"],
  "model-picker": ["dialog"],
  "order-by": ["dialog", "list", "listitem"],
  popover: ["dialog"],
  "ref-select": ["dialog", "listbox", "option", "status"],
  select: ["listbox", "option"],
  "split-button": ["menu", "menuitem", "separator"],
  tooltip: ["button"],
};

function isOverlayOnly(slug: string, aria: string): boolean {
  return (OVERLAY_ONLY[slug] ?? []).includes(aria);
}

/**
 * Roles a component genuinely does not emit, with the reason it is not a
 * defect. Each needs a justification, not just an entry.
 *
 * These are the residue after the real gaps were closed, and they are
 * qualitatively different from `OVERLAY_ONLY`: those are observable with a
 * better specimen, these are not observable at all in this implementation.
 */
const NOT_APPLICABLE: Record<string, Record<string, string>> = {
  "editable-list": {
    alert: "validation message; only rendered when a row is invalid",
    status: "status line; only rendered while saving or empty",
  },
  rating: {
    slider: "the contract lists radio *or* slider by variant; this renders the radio group",
  },
  tabs: {
    tabpanel: "TabDefinition carries no content in this API, so no panel exists to label",
  },
};

function notApplicable(slug: string, aria: string): string | undefined {
  return NOT_APPLICABLE[slug]?.[aria];
}

type Gap = { slug: string; aria: string; expected: string[] };
const gaps: Gap[] = [];
let checked = 0;
let overlayOnly = 0;
const exempt: string[] = [];

for (const [slug, projected] of Object.entries(census)) {
  if (!known.has(slug)) continue;
  const present = new Set(projected);
  for (const aria of contractRoles(slug)) {
    const accepted = ARIA_TO_ACCESSKIT[aria];
    // An unmapped ARIA role is a hole in the table above, not a component
    // defect — say so rather than reporting it as drift.
    if (accepted === undefined) {
      console.warn(`  ? ${slug}: no accesskit mapping for role="${aria}"`);
      continue;
    }
    if (accepted.length === 0) continue;
    checked += 1;
    if (!accepted.some((role) => present.has(role))) {
      if (isOverlayOnly(slug, aria)) {
        overlayOnly += 1;
        continue;
      }
      const reason = notApplicable(slug, aria);
      if (reason) {
        exempt.push(`${slug} ${aria} — ${reason}`);
        continue;
      }
      gaps.push({ slug, aria, expected: accepted });
    }
  }
}

console.log(`\nchecked ${checked} contract role requirements across ${Object.keys(census).length} specimens`);
console.log(`${overlayOnly} need an open overlay to observe and are set aside — see OVERLAY_ONLY`);
if (exempt.length > 0) {
  console.log(`${exempt.length} exempt with a recorded reason:`);
  for (const line of exempt) console.log(`  ${line}`);
}

if (gaps.length === 0) {
  console.log("every ARIA role a contract names is projected by its component.");
  process.exit(0);
}

const bySlug = new Map<string, Gap[]>();
for (const gap of gaps) bySlug.set(gap.slug, [...(bySlug.get(gap.slug) ?? []), gap]);

console.log(`\n${gaps.length} contract role(s) never projected, across ${bySlug.size} components:`);
for (const [slug, list] of [...bySlug].sort((a, b) => b[1].length - a[1].length)) {
  console.log(`  ${slug.padEnd(28)} ${list.map((g) => g.aria).join(", ")}`);
}
process.exit(1);

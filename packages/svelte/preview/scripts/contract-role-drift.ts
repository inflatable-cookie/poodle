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

type Gap = { slug: string; aria: string; expected: string[] };
const gaps: Gap[] = [];
let checked = 0;

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
      gaps.push({ slug, aria, expected: accepted });
    }
  }
}

console.log(`\nchecked ${checked} contract role requirements across ${Object.keys(census).length} specimens`);

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

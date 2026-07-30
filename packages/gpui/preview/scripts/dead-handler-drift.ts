/**
 * GPUI handler drift: a component that accepts a handler must use it.
 *
 * `Stepper` stored `on_change` and `on_rerun` in eleven places and attached
 * neither. The builders type-checked, the pointing-hand cursor promised a
 * click, and nothing happened when you made one — an API that lies is worse
 * than one that is missing, and nothing in the repo could catch it.
 *
 * The rule is deliberately narrow: a declared handler field has to be *read*
 * somewhere other than its own assignment. Reading it is what forwarding to a
 * child or attaching an `on_click` both look like, so this stays quiet about
 * how a component chooses to use one, and loud about never using it at all.
 */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const ROOT = new URL("../../components/src", import.meta.url).pathname;

function walk(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) return walk(path);
    return path.endsWith(".rs") ? [path] : [];
  });
}

/**
 * Handlers already accepted and unused when this gate was written.
 *
 * A baseline, not an allowlist. Fixing one means deleting its entry; adding one
 * means a component shipped a builder that does nothing, which is exactly what
 * this exists to stop. `Stepper` is deliberately absent — it was the first one
 * fixed, and it is why the gate exists.
 */
const BASELINE: Record<string, string[]> = {
  "composites/action_discovery_panel.rs": ["on_select"],
  "composites/command_palette.rs": ["on_query_change", "on_select"],
  "composites/data_table/mod.rs": ["on_page_change", "on_row_click", "on_row_expand", "on_row_select", "on_select_all", "on_sort"],
  "composites/dock_region.rs": ["on_collapse_toggle", "on_tab_change"],
  "composites/log_list/mod.rs": ["on_filter_change", "on_search"],
  "composites/split_view.rs": ["on_ratio_change"],
  "composites/tree/mod.rs": ["on_check", "on_context_menu", "on_drag_over", "on_rename_cancel", "on_rename_change", "on_rename_commit", "on_rename_start", "on_select"],
  "primitives/calendar/mod.rs": ["on_navigate", "on_range_select", "on_select"],
  "primitives/date_picker.rs": ["on_select"],
  "primitives/editable_label.rs": ["on_change"],
  "primitives/list_card.rs": ["on_click"],
  "primitives/number_input.rs": ["on_change"],
  "primitives/pagination/mod.rs": ["on_goto_input_change"],
  "primitives/select/mod.rs": ["on_change", "on_search_change", "on_toggle"],
  "primitives/tabs/mod.rs": ["on_change"],
};

const dead: string[] = [];
const fixed: string[] = [];

for (const file of walk(ROOT)) {
  const source = readFileSync(file, "utf8");
  const fields = [...source.matchAll(/^\s+(on_[a-z_]+):\s*Option</gm)].map((m) => m[1]);

  for (const field of new Set(fields)) {
    // Every read of the field, minus the assignment inside its own builder.
    const reads = [...source.matchAll(new RegExp(`self\\.${field}\\b`, "g"))].length;
    const writes = [...source.matchAll(new RegExp(`self\\.${field}\\s*=`, "g"))].length;

    const relative = file.replace(`${ROOT}/`, "");
    const known = BASELINE[relative]?.includes(field) ?? false;

    if (reads - writes <= 0) {
      if (!known) {
        dead.push(`${relative}: \`${field}\` is accepted and never used`);
      }
    } else if (known) {
      fixed.push(`${relative}: \`${field}\``);
    }
  }
}

if (dead.length > 0) {
  console.error("GPUI components accept handlers they never use:\n");
  for (const entry of dead) console.error(`  ${entry}`);
  console.error(
    `\n${dead.length} dead handler(s). Either wire the handler or stop accepting it —` +
      " a builder that type-checks and does nothing is worse than a missing one.",
  );
  process.exit(1);
}

if (fixed.length > 0) {
  // Keeping a fixed entry in the baseline would let it rot back to dead
  // without the gate noticing.
  console.error("these handlers are wired now — delete them from BASELINE:\n");
  for (const entry of fixed) console.error(`  ${entry}`);
  process.exit(1);
}

const remaining = Object.values(BASELINE).reduce((sum, list) => sum + list.length, 0);
console.log(
  `every GPUI handler a component accepts is used, or is one of ${remaining} tracked in the baseline.`,
);

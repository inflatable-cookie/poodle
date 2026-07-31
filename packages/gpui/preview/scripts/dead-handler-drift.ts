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
import { dirname, join } from "node:path";

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
  // 23 of the original 34 entries were never dead: their handlers were wired
  // in sibling files of the same module, and the gate only read the declaring
  // file. Of the true 11, seven are wired now. These four need input routes
  // this pass did not build: a divider drag (split_view) and live text editing
  // (the other three).
  "composites/command_palette.rs": ["on_query_change"],
  "composites/split_view.rs": ["on_ratio_change"],
  "primitives/editable_label.rs": ["on_change"],
  "primitives/pagination/mod.rs": ["on_goto_input_change"],
};

const dead: string[] = [];
const fixed: string[] = [];

/**
 * The whole module's source, not one file's.
 *
 * A component split across a directory declares its handlers in `mod.rs` and
 * uses them wherever the render lives — `calendar/` reads all three of its
 * handlers in `render.rs`. Counting reads only in the declaring file reported
 * them dead when they were wired all along, and the baseline preserved that
 * false accusation for weeks. Same blind spot the Jetstream gate had; same fix.
 */
function moduleSource(file: string): string {
  if (!file.endsWith("/mod.rs")) return readFileSync(file, "utf8");
  const dir = dirname(file);
  return readdirSync(dir)
    .filter((entry) => entry.endsWith(".rs"))
    .map((entry) => readFileSync(join(dir, entry), "utf8"))
    .join("\n");
}

for (const file of walk(ROOT)) {
  const source = readFileSync(file, "utf8");
  const fields = [...source.matchAll(/^\s+(on_[a-z_]+):\s*Option</gm)].map((m) => m[1]);
  const scope = moduleSource(file);

  for (const field of new Set(fields)) {
    // Every read of the field, minus the assignment inside its own builder.
    const reads = [...scope.matchAll(new RegExp(`self\\.${field}\\b`, "g"))].length;
    const writes = [...scope.matchAll(new RegExp(`self\\.${field}\\s*=`, "g"))].length;

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

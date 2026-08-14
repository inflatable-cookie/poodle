/**
 * docs:capability-drift — legacy static capability gate (g13.018).
 *
 * g14.002 disposition: **adapt**. This script remains as cheap debt detection
 * against `capabilities.json` source traces. It cannot mark a primitive
 * capability passing. Execution authority is
 * `primitive-capability-report.v1` (`effigy conformance:primitives-report`).
 * The `timers` row is retired from the primitive roster (host timing, not
 * render vocabulary) but may still appear here as legacy debt.
 *
 * The two expressiveness gaps the g13.008 revise verdict named, fixed, and
 * now pinned: per-runtime capability provision *including absence*, and
 * the drift that would silently undo either half.
 *
 * The definition declares, per capability, which of the four runtimes
 * provide it, delegate it, or lack it — each with a reason (g13.018 R3;
 * `CapabilityRequirement::runtimes`). A type nobody checks changes
 * nothing (R4), so this gate checks the *truth* of every declared row
 * against the runtime sources, in both directions independently:
 *
 *   - a declared **absence** must stay true — if the runtime gains a trace
 *     of the capability while still declared as lacking it, the gate
 *     fails. This is the b049 finding made checkable: Jetstream renders a
 *     text field nobody can type into and is declared identically to
 *     GPUI; now "Jetstream lacks text editing" is declared, and the gate
 *     fails the moment Jetstream gains edit handlers while the
 *     declaration still says absent.
 *   - a declared **provision** (provided or delegated) must have a trace —
 *     if a runtime claims a capability it has no trace of, the gate fails.
 *     A runtime may not declare a capability it does not implement.
 *
 * The trace vocabulary is the measured evidence from b049 (the TextInput
 * native slice): the edit-handler names (`on_edit_key`/`on_edit_insert`/
 * `on_select_range`), the file names (`ime.rs`, `input_text.rs`), the web
 * components' native-input wiring, and the adapter surfaces each runtime
 * actually ships. A probe table entry is required for every declared
 * (capability, runtime) row — a declaration without a probe is a gate
 * misconfiguration and fails, because a gate that cannot check a claim
 * must not pass it.
 *
 * This is static checking in the shape of `docs:react-specimen-drift`:
 * build-time, no browser, and it exists for the same reason — the
 * runtime-only guard shipped a fatally broken preview with every other
 * gate green. Runs standalone via `effigy docs:capability-drift` and as a
 * member of `effigy ci:web`.
 */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
// packages/svelte/preview/scripts -> repo root
const ROOT = resolve(here, "../../../..");
const CAPABILITIES_PATH = join(
  ROOT,
  "packages/contracts/headless/capabilities/capabilities.json",
);

/** The serialized runtime names the fixtures carry (the four RuntimeTargets). */
type RuntimeName = "svelte" | "react" | "gpui" | "jetstream";
/** The serialized provision names the fixtures carry (CapabilityProvision). */
type ProvisionName = "provided" | "delegated" | "absent";

interface Probe {
  /** Directories under the repo root that constitute the runtime's sources. */
  scopes: string[];
  /** Regexes (no flags) that are evidence the runtime implements the capability. */
  patterns: string[];
}

/**
 * The trace table — one entry per (component, capability, runtime) the
 * authored models declare. Patterns are the measured implementation
 * vocabulary (b049 for TextInput; b042/b046 for the natives), scoped to
 * the runtime surface the component actually consumes: the component's own
 * file on web, the node-backend for GPUI, the Jetstream package for
 * Jetstream. Per-component keys are required — the same capability can
 * have a different relationship per component (e.g. Jetstream focus is
 * absent for TextInput fields — no `on_focus_change` route, measured —
 * but provided for RangeSlider's focusable root). Absence probes reuse
 * the same vocabulary: a trace appearing where absence is declared is the
 * failure.
 */
const PROBES: Record<
  string,
  Record<string, Record<RuntimeName, Probe>>
> = {
  "text-input": {
    "text-editing": {
      svelte: {
        scopes: ["packages/svelte/components/src/TextInput.svelte"],
        patterns: ["on:input|oninput"],
      },
      react: {
        scopes: ["packages/react/components/src/TextInput.tsx"],
        patterns: ["onChange|onInput"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["on_edit_insert|on_edit_key|on_select_range"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["on_edit_key|on_edit_insert|on_select_range"],
      },
    },
    ime: {
      svelte: {
        scopes: ["packages/svelte/components/src/TextInput.svelte"],
        patterns: ["oncomposition|isComposing"],
      },
      react: {
        scopes: ["packages/react/components/src/TextInput.tsx"],
        patterns: ["isComposing|onComposition"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["\\bInputHandler\\b"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["\\bime\\.rs\\b|\\bInputHandler\\b"],
      },
    },
    clipboard: {
      svelte: {
        scopes: ["packages/svelte/components/src/TextInput.svelte"],
        patterns: ["<input|<textarea"],
      },
      react: {
        scopes: ["packages/react/components/src/TextInput.tsx"],
        patterns: ["<input|<textarea"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["clipboard|on_edit_insert"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["clipboard|on_edit_insert"],
      },
    },
    measurement: {
      svelte: {
        scopes: ["packages/svelte/components/src/TextInput.svelte"],
        patterns: ["<input|<textarea"],
      },
      react: {
        scopes: ["packages/react/components/src/TextInput.tsx"],
        patterns: ["<input|<textarea"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["ShapedLine|x_for_index|closest_index_for_x"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["ShapedLine|x_for_index|closest_index_for_x"],
      },
    },
    focus: {
      svelte: {
        scopes: ["packages/svelte/components/src/TextInput.svelte"],
        patterns: ["focus\\(|autofocus|tabindex"],
      },
      react: {
        scopes: ["packages/react/components/src/TextInput.tsx"],
        patterns: ["focus|tabIndex|autoFocus"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["on_focus_change|focusable"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        // The b049 measurement: Jetstream never observes focus for poodle
        // *fields* — no `on_focus_change` route. The preview's general
        // focus system (set_focus/focusable) serves other controls, not
        // the text field, so the field's absence probe is the poodle
        // focus-observation channel itself.
        patterns: ["on_focus_change"],
      },
    },
    timers: {
      svelte: {
        scopes: ["packages/svelte/components/src/TextInput.svelte"],
        patterns: ["setTimeout"],
      },
      react: {
        scopes: ["packages/react/components/src/TextInput.tsx"],
        patterns: ["setTimeout"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["setTimeout|schedule_timer"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["setTimeout"],
      },
    },
  },
  "range-slider": {
    "pointer-capture": {
      svelte: {
        scopes: ["packages/svelte/components/src/RangeSlider.svelte"],
        patterns: ["setPointerCapture"],
      },
      react: {
        scopes: ["packages/react/components/src/RangeSlider.tsx"],
        patterns: ["setPointerCapture"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["on_scrub|on_drag"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["setPointerCapture|pointer_capture|PointerCapture"],
      },
    },
    focus: {
      svelte: {
        scopes: ["packages/svelte/components/src/RangeSlider.svelte"],
        patterns: ["focus\\(|tabindex|autofocus"],
      },
      react: {
        scopes: ["packages/react/components/src/RangeSlider.tsx"],
        patterns: ["focus|tabIndex|autoFocus"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["on_focus_change|focusable"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["set_focus|focusable"],
      },
    },
    "scrub-fraction": {
      svelte: {
        scopes: ["packages/svelte/components/src/RangeSlider.svelte"],
        patterns: ["runControl|pointNorm"],
      },
      react: {
        scopes: ["packages/react/components/src/RangeSlider.tsx"],
        patterns: ["runControl|pointNorm"],
      },
      gpui: {
        scopes: ["packages/gpui/node-backend/src"],
        patterns: ["on_scrub"],
      },
      jetstream: {
        scopes: ["packages/jetstream"],
        patterns: ["on_scrub|\\bscrub\\b"],
      },
    },
  },
};

const SOURCE_EXTENSIONS = new Set([".rs", ".ts", ".tsx", ".svelte"]);

/** All source files under a scope (relative to the repo root). A scope may
 * be a single component file or a directory. */
function sourceFiles(scope: string): string[] {
  const root = join(ROOT, scope);
  if (!statSync(root).isDirectory()) {
    return [relative(ROOT, root)];
  }
  const out: string[] = [];
  const walk = (dir: string) => {
    for (const entry of readdirSync(dir)) {
      if (entry === "node_modules" || entry === ".git" || entry.startsWith(".")) continue;
      const full = join(dir, entry);
      if (statSync(full).isDirectory()) {
        walk(full);
      } else if (SOURCE_EXTENSIONS.has(entry.slice(entry.lastIndexOf(".")))) {
        out.push(relative(ROOT, full));
      }
    }
  };
  walk(root);
  return out;
}

/** Total pattern matches across a probe's scopes. */
function traceCount(probe: Probe): number {
  const regexes = probe.patterns.map((pattern) => new RegExp(pattern));
  let count = 0;
  for (const file of sourceFiles(probe.scopes[0]).concat(
    probe.scopes.slice(1).flatMap((scope) => sourceFiles(scope)),
  )) {
    const contents = readFileSync(join(ROOT, file), "utf8");
    for (const regex of regexes) {
      count += (contents.match(regex) ?? []).length;
    }
  }
  return count;
}

interface Failure {
  component: string;
  capability: string;
  runtime: RuntimeName;
  provision: ProvisionName;
  detail: string;
}

const failures: Failure[] = [];
let checked = 0;

function checkRows(component: string, capability: string, rows: Array<Record<string, unknown>>) {
  for (const row of rows) {
    const runtime = row["runtime"] as RuntimeName;
    const provision = row["provision"] as ProvisionName;
    const probe = PROBES[component]?.[capability]?.[runtime];
    if (!probe) {
      failures.push({
        component,
        capability,
        runtime,
        provision,
        detail:
          `no probe table entry for (${capability}, ${runtime}) — a declared row without ` +
          "evidence cannot be checked, and a gate that cannot check a claim must not pass it",
      });
      continue;
    }
    const traces = traceCount(probe);
    checked += 1;
    if (provision === "absent") {
      if (traces > 0) {
        failures.push({
          component,
          capability,
          runtime,
          provision,
          detail:
            `declared absent but the runtime has ${traces} trace(s) of it — a declared ` +
            "absence must stay true (g13.018 R4 direction A)",
        });
      }
    } else if (traces === 0) {
      failures.push({
        component,
        capability,
        runtime,
        provision,
        detail:
          `declared ${provision} but the runtime has no trace of it — a runtime may not ` +
          "claim a capability it does not implement (g13.018 R4 direction B)",
      });
    }
  }
}

const model = JSON.parse(readFileSync(CAPABILITIES_PATH, "utf8")) as {
  components: Array<{
    id: string;
    capabilities?: Array<{
      capability: string;
      runtimes?: Array<Record<string, unknown>>;
    }>;
  }>;
};
for (const component of model.components ?? []) {
  for (const requirement of component.capabilities ?? []) {
    const rows = requirement.runtimes ?? [];
    if (rows.length === 0) continue; // pre-g13.018 shape: nothing declared, nothing checked
    checkRows(component.id, requirement.capability, rows);
  }
}

if (failures.length > 0) {
  console.error("capability-drift: declared capability rows contradicted by the runtime sources.");
  for (const failure of failures) {
    console.error(
      `  [${failure.component}] ${failure.capability} @ ${failure.runtime} ` +
        `(declared ${failure.provision}): ${failure.detail}`,
    );
  }
  console.error(
    "\nEvery declared provision must have a trace in the runtime, and every declared\n" +
      "absence must stay true. Fix the implementation or the declaration — never silence\n" +
      "the gate. The trace vocabulary lives in this script's PROBES table.",
  );
  process.exit(1);
}

console.log(`capability-drift: ${checked} declared capability rows verified against runtime traces.`);

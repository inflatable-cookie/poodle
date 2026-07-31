/**
 * Jetstream interaction drift: a component that offers a handler must prove a
 * click reaches it.
 *
 * This is the sibling of `drift:handlers` on the GPUI side, and it is stronger.
 * GPUI can only check that a handler field is *read* somewhere, because a real
 * click there needs a live window and nothing in the repo can drive one.
 * Jetstream dispatches clicks through `GameUi` with no window, so here the
 * question "does this actually work?" is answerable, and a gate that can ask it
 * should.
 *
 * Two rules:
 *
 *   1. Every `pub fn on_x` builder method needs a test in the same file that
 *      passes a handler to it *and* drives a click through `click_probe`.
 *      Attaching a handler and never testing it is how `Stepper` on GPUI kept
 *      two dead builders for weeks.
 *   2. A component whose contract has events but which has no builder yet is
 *      unconverted, and has to be in `BASELINE`. Converting one means deleting
 *      its entry — a stale entry fails, so a conversion cannot leave the gate
 *      believing there is still work to do.
 *
 * The baseline shrinks as the sweep in `docs/roadmaps/g12/017` proceeds. When
 * it is empty, delete it and this comment with it.
 */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import path from "node:path";

import { allComponents } from "../../../svelte/preview/src/component-registry.ts";

const repoRoot = path.resolve(import.meta.dir, "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const componentsDir = path.join(repoRoot, "packages/jetstream/components/src");

/** Components whose Jetstream file is not `snake(displayName).rs`. */
const ALIASES: Record<string, string> = {
  Breadcrumbs: "breadcrumbs_comp",
  Pagination: "pagination_comp",
  TimeInput: "time_field",
};

/**
 * Contract events with no Jetstream handler, by design rather than by omission.
 *
 * Each needs a reason that says what is missing on the *render* side, because
 * that is the only good reason not to have a handler: there is nothing drawn
 * for a click to land on. "Not done yet" belongs in BASELINE, not here.
 */
const EXEMPT: Record<string, string> = {
  AgentMessage:
    "inline nodes flatten to text, so there is no link element for onLinkClick to attach to",
  Radio: "no standalone Jetstream component; RadioGroup renders the options",
  Popover:
    "renders the panel only — the trigger and the open state are the consumer's, so nothing here can raise onOpenChange",
  Tooltip:
    "renders the bubble only; it is summoned by hover on a trigger the consumer owns",
  HoverCard:
    "as Tooltip — the card is the panel, and the hover that opens it belongs to the trigger",
};

/**
 * Components with contract events and no builder yet — the sweep's worklist.
 *
 * A baseline, not an allowlist. Adding one means a component with events
 * shipped without a way to receive them; removing one is what converting it
 * looks like.
 */
const BASELINE = new Set([
  "ActionDiscoveryPanel",
  "AgentChatInput",
  "AudioPlayer",
  "BlockEditor",
  "BulkActionBar",
  "Calendar",
  "Callout",
  "CardRadioGroup",
  "CardToggleGroup",
  "CodeInput",
  "ColorPicker",
  "CommandPalette",
  "ConfirmAction",
  "DatePicker",
  "DateRangePicker",
  "DateTimePicker",
  "DateTimeRangePicker",
  "DateTimeZonePicker",
  "DockRegion",
  "DurationInput",
  "EditableList",
  "EmbedInput",
  "FileUpload",
  "FilterBuilder",
  "FilterToolbar",
  "FormDialog",
  "ListCard",
  "ListCardCounter",
  "ListContainer",
  "LogList",
  "MarkdownEditor",
  "MediaBrowsePanel",
  "MediaPicker",
  "Menubar",
  "ModelPicker",
  "OrderBy",
  "PageLoading",
  "Pagination",
  "RadioGroup",
  "RangeSlider",
  "RefSelect",
  "RelationPicker",
  "ResizeHandle",
  "ScrollShell",
  "Select",
  "SelectionSummary",
  "Slider",
  "SplitButton",
  "SplitView",
  "Stepper",
  "ThemeSelect",
  "TimeInput",
  "TimeZoneSelect",
  "ToastHost",
  "ToastStack",
  "VideoPlayer",
]);

function snake(name: string): string {
  return name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

function walk(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const full = path.join(dir, entry);
    if (statSync(full).isDirectory()) return walk(full);
    return full.endsWith(".rs") ? [full] : [];
  });
}

const sources = walk(componentsDir);

/** The `#[cfg(test)]` module, where a click test has to live to count. */
function testModule(source: string): string {
  const start = source.indexOf("#[cfg(test)]");
  return start === -1 ? "" : source.slice(start);
}

const untested: string[] = [];
const unconverted: string[] = [];
const stale: string[] = [];
let checked = 0;
let handlers = 0;

for (const entry of allComponents) {
  const contractPath = path.join(contractsDir, `${entry.slug}.md`);
  if (!existsSync(contractPath)) continue;

  // Any `onSomething` the contract names, wherever it names it: some contracts
  // carry them in the props table, some in an Events section, some in both.
  const contract = readFileSync(contractPath, "utf8");
  const events = new Set([...contract.matchAll(/`(on[A-Z][A-Za-z]*)`/g)].map((m) => m[1]));
  if (events.size === 0) continue;

  const base = ALIASES[entry.displayName] ?? snake(entry.displayName);
  const file = sources.find(
    (p) => p.endsWith(`/${base}.rs`) || p.endsWith(`/${base}/mod.rs`),
  );

  const exemption = EXEMPT[entry.displayName];
  const source = file ? readFileSync(file, "utf8") : "";
  const methods = [...source.matchAll(/pub fn (on_[a-z_]+)\(/g)].map((m) => m[1]);

  if (methods.length === 0) {
    if (exemption) continue;
    if (!BASELINE.has(entry.displayName)) {
      unconverted.push(
        `${entry.displayName}: contract has events (${[...events].join(", ")}) and the component takes no handler`,
      );
    }
    continue;
  }

  if (BASELINE.has(entry.displayName)) {
    stale.push(`${entry.displayName}`);
  }

  const tests = testModule(source);
  const relative = file!.replace(`${componentsDir}/`, "");
  checked++;

  for (const method of new Set(methods)) {
    handlers++;
    const passed = tests.includes(`.${method}(`);
    const clicked = /click_probe::click/.test(tests);

    if (!passed || !clicked) {
      untested.push(
        `${relative}: \`${method}\` ${
          passed
            ? "is passed a handler in tests, but nothing drives a click"
            : "is never given a handler in a test"
        }`,
      );
    }
  }
}

if (untested.length > 0) {
  console.error("Jetstream handlers with no click test:\n");
  for (const line of untested) console.error(`  ${line}`);
  console.error(
    `\n${untested.length} untested handler(s). Jetstream can drive a real click in a unit` +
      " test — `element::click_probe::click_text` — so a handler that is never clicked is" +
      " a handler nobody has checked works.",
  );
  process.exit(1);
}

if (unconverted.length > 0) {
  console.error("Jetstream components with contract events and no handler:\n");
  for (const line of unconverted) console.error(`  ${line}`);
  console.error(
    "\nGive the component a builder with the handlers its contract names, or add it to" +
      " BASELINE if it is waiting on the sweep.",
  );
  process.exit(1);
}

if (stale.length > 0) {
  // A converted component left in the baseline would let the gate go on
  // believing the work is outstanding, and hide a regression that removed it.
  console.error("these components are converted now — delete them from BASELINE:\n");
  for (const line of stale) console.error(`  ${line}`);
  process.exit(1);
}

console.log(
  `every handler on ${checked} converted Jetstream component(s) is proved by a click test` +
    ` (${handlers} handlers). ${BASELINE.size} component(s) still awaiting conversion.`,
);

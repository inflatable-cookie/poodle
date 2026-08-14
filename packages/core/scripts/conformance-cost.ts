/**
 * Conformance cost report (spec 066, g14.001): exhaustive source LOC by
 * ownership, generated data in bytes, and replaced source against the branch
 * merge base. LOC excludes blank and comment-only lines.
 */

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

const ROOT = execFileSync("git", ["rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
/** Fixed pre-proof baseline; override only when measuring a later pilot. */
const BASE = process.env.POODLE_CONFORMANCE_COST_BASE ?? "5180ac16ad276988eb9f235a6d6957a283aea3b8";

function countLines(text: string): number {
  return text.split("\n").filter((line) => {
    const trimmed = line.trim();
    if (!trimmed) return false;
    if (trimmed.startsWith("//") || trimmed.startsWith("*") || trimmed.startsWith("/*")) return false;
    if (trimmed.startsWith("#") && !trimmed.startsWith("#!")) return false;
    return true;
  }).length;
}

function workingText(path: string): string {
  try {
    return readFileSync(`${ROOT}/${path}`, "utf8");
  } catch {
    return "";
  }
}

function baseText(path: string): string {
  try {
    return execFileSync("git", ["show", `${BASE}:${path}`], {
      cwd: ROOT,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    });
  } catch {
    return "";
  }
}

function workingLoc(path: string): number {
  return countLines(workingText(path));
}

function baseLoc(path: string): number {
  return countLines(baseText(path));
}

const REUSABLE_AUTHORITY: Array<[string, string]> = [
  ["Interface and case schema", "packages/core/src/conformance/define.ts"],
  ["Primitive capability roster", "packages/core/src/conformance/primitives.ts"],
  ["Specimen projection", "packages/core/src/conformance/project.ts"],
  ["Serializer", "packages/core/scripts/conformance-serialize.ts"],
  ["Primitive report gate", "packages/core/scripts/primitive-capability-report.ts"],
  ["Authority validation tests", "packages/core/test/component-case-authority.test.ts"],
];

const BUTTON_AUTHORITY: Array<[string, string]> = [
  ["Button interface", "packages/core/src/conformance/button.ts"],
  ["Button corpus", "packages/core/src/conformance/button-cases.ts"],
];

const RANGE_SLIDER_AUTHORITY: Array<[string, string]> = [
  ["RangeSlider interface", "packages/core/src/conformance/range-slider.ts"],
  ["RangeSlider corpus", "packages/core/src/conformance/range-slider-cases.ts"],
];

const TABS_AUTHORITY: Array<[string, string]> = [
  ["Tabs interface", "packages/core/src/conformance/tabs.ts"],
  ["Tabs corpus", "packages/core/src/conformance/tabs-cases.ts"],
];

const CODEGEN: Array<[string, string]> = [
  ["Conformance parsing and validation", "packages/codegen/src/conformance.rs"],
  ["Rust declaration target", "packages/codegen/src/targets/conformance_rust.rs"],
  ["CLI integration", "packages/codegen/src/bin/poodle-codegen.rs"],
];

const GENERATED_SOURCE: Array<[string, string]> = [
  ["Generated Button Rust declaration", "packages/contracts/components/src/generated/button.rs"],
];

const GENERATED_DATA: Array<[string, string]> = [
  ["Interface fixture JSON", "packages/codegen/fixtures/conformance/button-interface.json"],
  ["Case fixture JSON", "packages/codegen/fixtures/conformance/button-cases.json"],
  ["RangeSlider interface fixture JSON", "packages/codegen/fixtures/conformance/range-slider-interface.json"],
  ["RangeSlider case fixture JSON", "packages/codegen/fixtures/conformance/range-slider-cases.json"],
  ["Tabs interface fixture JSON", "packages/codegen/fixtures/conformance/tabs-interface.json"],
  ["Tabs case fixture JSON", "packages/codegen/fixtures/conformance/tabs-cases.json"],
  ["Primitive roster JSON", "packages/codegen/fixtures/conformance/primitive-capability-roster.json"],
];

const GENERIC_RUNTIME: Array<[string, string]> = [
  ["Web runner and observer", "test/conformance/web/runner.ts"],
  ["Native observer and assertion runner", "packages/render/src/conformance.rs"],
  ["Renderer-neutral primitive probes", "packages/render/src/primitive_probes.rs"],
  ["Cross-runtime comparator", "test/conformance/compare.ts"],
  ["Web primitive probes", "test/conformance/web/primitives.test.ts"],
  ["GPUI generic driver", "packages/gpui/preview/src/conformance_driver.rs"],
  ["GPUI primitive probes", "packages/gpui/preview/src/primitive_probes_gpui.rs"],
];

const BUTTON_HARNESS: Array<[string, string]> = [
  ["Svelte adapter", "test/conformance/web/svelte-adapter.ts"],
  ["React adapter", "test/conformance/web/react-adapter.tsx"],
  ["Svelte host", "test/conformance/web/hosts/ButtonHost.svelte"],
  ["Web execution tests", "test/conformance/web/button.test.ts"],
  ["GPUI conformance CLI", "packages/gpui/preview/src/bin/conformance.rs"],
  ["GPUI Button adapter", "packages/gpui/preview/src/conformance_button.rs"],
  ["GPUI fixture adapter", "packages/gpui/preview/src/conformance_support.rs"],
];

const RANGE_SLIDER_HARNESS: Array<[string, string]> = [
  ["Svelte RangeSlider adapter", "test/conformance/web/svelte-range-slider-adapter.ts"],
  ["React RangeSlider adapter", "test/conformance/web/react-range-slider-adapter.tsx"],
  ["Svelte RangeSlider host", "test/conformance/web/hosts/RangeSliderHost.svelte"],
  ["Web RangeSlider tests", "test/conformance/web/range-slider.test.ts"],
  ["GPUI RangeSlider adapter", "packages/gpui/preview/src/conformance_range_slider.rs"],
];

const TABS_HARNESS: Array<[string, string]> = [
  ["Svelte Tabs adapter", "test/conformance/web/svelte-tabs-adapter.ts"],
  ["React Tabs adapter", "test/conformance/web/react-tabs-adapter.tsx"],
  ["Svelte Tabs host", "test/conformance/web/hosts/TabsHost.svelte"],
  ["Web Tabs tests", "test/conformance/web/tabs.test.ts"],
  ["GPUI Tabs adapter", "packages/gpui/preview/src/conformance_tabs.rs"],
];

const CAPTURE_REPAIR: Array<[string, string]> = [
  ["Native visual runner", "test/native-visual/run.ts"],
  ["Native visual capture", "test/native-visual/capture.ts"],
  ["Native visual README", "test/native-visual/README.md"],
];

const GENERIC_SUPPORTING_DELTAS: Array<[string, string]> = [
  ["poodle-node observation vocabulary", "packages/contracts/node/src/lib.rs"],
  ["GPUI focus and activation backend", "packages/gpui/node-backend/src/lib.rs"],
  ["GPUI interaction backend", "packages/gpui/node-backend/src/interaction.rs"],
];

const BUTTON_SUPPORTING_DELTAS: Array<[string, string]> = [
  ["Button renderer semantics", "packages/render/src/button.rs"],
  ["Svelte Button shell", "packages/svelte/components/src/Button.svelte"],
  ["React Button shell", "packages/react/components/src/Button.tsx"],
];

const RANGE_SLIDER_SUPPORTING_DELTAS: Array<[string, string]> = [
  ["RangeSlider renderer semantics", "packages/render/src/range_slider.rs"],
  ["Svelte RangeSlider shell", "packages/svelte/components/src/RangeSlider.svelte"],
  ["React RangeSlider shell", "packages/react/components/src/RangeSlider.tsx"],
];

const TABS_SUPPORTING_DELTAS: Array<[string, string]> = [
  ["Tabs renderer semantics", "packages/render/src/tabs.rs"],
  ["Svelte Tabs shell", "packages/svelte/components/src/Tabs.svelte"],
  ["React Tabs shell", "packages/react/components/src/Tabs.tsx"],
];

const WIRING: Array<[string, string]> = [
  ["Effigy selector section", "tasks/effigy.tasks.toml#conformance"],
  ["Cost report", "packages/core/scripts/conformance-cost.ts"],
  ["macOS conformance workflow", ".github/workflows/ci-conformance.yml"],
];

const REPLACED: Array<[string, string]> = [
  ["Hand-written ButtonSpec declaration", "packages/contracts/components/src/button.rs"],
  ["Svelte Button specimen fixtures", "packages/svelte/preview/src/specimens/ButtonSpecimen.svelte"],
  ["React Button specimen fixtures", "packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx"],
  ["GPUI Button specimen fixtures", "packages/gpui/preview/src/specimens/button.rs"],
  ["Svelte RangeSlider specimen fixtures", "packages/svelte/preview/src/specimens/RangeSliderSpecimen.svelte"],
  ["React RangeSlider specimen fixtures", "packages/react/preview/src/gallery/specimens/RangeSliderSpecimen.tsx"],
  ["GPUI RangeSlider specimen fixtures", "packages/gpui/preview/src/specimens/range_slider.rs"],
  ["Svelte Tabs specimen fixtures", "packages/svelte/preview/src/specimens/TabsSpecimen.svelte"],
  ["React Tabs specimen fixtures", "packages/react/preview/src/gallery/specimens/TabsSpecimen.tsx"],
  ["GPUI Tabs specimen fixtures", "packages/gpui/preview/src/specimens/tabs.rs"],
];

function sectionLoc(path: string, startMarker: string, endMarker: string): number {
  const text = workingText(path);
  const start = text.indexOf(startMarker);
  const end = text.indexOf(endMarker, start + startMarker.length);
  if (start < 0 || end < 0) return 0;
  return countLines(text.slice(start, end));
}

function sourceTable(
  title: string,
  rows: Array<[string, string]>,
  mode: "working" | "delta" | "replaced",
): number {
  console.log(`\n${title} (LOC)`);
  let total = 0;
  for (const [label, path] of rows) {
    let count: number;
    if (mode === "replaced") {
      count = Math.max(0, baseLoc(path) - workingLoc(path));
    } else if (mode === "delta") {
      count = Math.max(0, workingLoc(path) - baseLoc(path));
    } else if (path.endsWith("#conformance")) {
      count = sectionLoc(path.split("#")[0], "# Conformance kernel (g14.001", "# Preview (documentation site)");
    } else {
      count = workingLoc(path);
    }
    total += count;
    console.log(`  ${String(count).padStart(5)}  ${label}`);
  }
  console.log(`  ${String(total).padStart(5)}  total`);
  return total;
}

function dataTable(rows: Array<[string, string]>): number {
  console.log("\nGenerated data artifacts (bytes)");
  let total = 0;
  for (const [label, path] of rows) {
    const bytes = Buffer.byteLength(workingText(path));
    total += bytes;
    console.log(`  ${String(bytes).padStart(5)}  ${label}`);
  }
  console.log(`  ${String(total).padStart(5)}  total`);
  return total;
}

console.log(`Conformance cost base: ${BASE}`);
const reusableAuthority = sourceTable("Reusable authority", REUSABLE_AUTHORITY, "working");
const buttonAuthority = sourceTable("Button authored authority", BUTTON_AUTHORITY, "working");
const rangeSliderAuthority = sourceTable(
  "RangeSlider authored authority",
  RANGE_SLIDER_AUTHORITY,
  "working",
);
const tabsAuthority = sourceTable("Tabs authored authority", TABS_AUTHORITY, "working");
const codegen = sourceTable("Codegen", CODEGEN, "delta");
const generatedSource = sourceTable("Generated source", GENERATED_SOURCE, "working");
const generatedBytes = dataTable(GENERATED_DATA);
const genericRuntime = sourceTable("Generic observers and runners", GENERIC_RUNTIME, "working");
const buttonHarness = sourceTable("Button pilot harness", BUTTON_HARNESS, "working");
const rangeSliderHarness = sourceTable("RangeSlider pilot harness", RANGE_SLIDER_HARNESS, "working");
const tabsHarness = sourceTable("Tabs pilot harness", TABS_HARNESS, "working");
const captureRepair = sourceTable("GPUI capture repair", CAPTURE_REPAIR, "working");
const genericSupporting = sourceTable("Generic runtime deltas", GENERIC_SUPPORTING_DELTAS, "delta");
const buttonSupporting = sourceTable("Button runtime deltas", BUTTON_SUPPORTING_DELTAS, "delta");
const rangeSliderSupporting = sourceTable(
  "RangeSlider runtime deltas",
  RANGE_SLIDER_SUPPORTING_DELTAS,
  "delta",
);
const tabsSupporting = sourceTable("Tabs runtime deltas", TABS_SUPPORTING_DELTAS, "delta");
const wiring = sourceTable("Wiring", WIRING, "working");
const replaced = sourceTable("Replaced hand-written source", REPLACED, "replaced");

const genericKernel = reusableAuthority + codegen + genericRuntime + genericSupporting + wiring;
const buttonPilot = buttonAuthority + generatedSource + buttonHarness + buttonSupporting;
const rangeSliderPilot = rangeSliderAuthority + rangeSliderHarness + rangeSliderSupporting;
const tabsPilot = tabsAuthority + tabsHarness + tabsSupporting;
const sourceMechanism = genericKernel + buttonPilot + rangeSliderPilot + tabsPilot + captureRepair;

console.log("\n=== Summary ===");
console.log(`source mechanism: ${sourceMechanism} LOC`);
console.log(`  generic kernel: ${genericKernel} LOC`);
console.log(`  Button pilot increment: ${buttonPilot} LOC`);
console.log(`    authored authority: ${buttonAuthority} LOC`);
console.log(`    generated source: ${generatedSource} LOC`);
console.log(`    harness and runtime deltas: ${buttonHarness + buttonSupporting} LOC`);
console.log(`  RangeSlider pilot increment: ${rangeSliderPilot} LOC`);
console.log(`  Tabs pilot increment: ${tabsPilot} LOC`);
console.log(`  GPUI capture repair: ${captureRepair} LOC`);
console.log(`generated data: ${generatedBytes} bytes`);
console.log(`replaced hand-written source: ${replaced} LOC`);
console.log(
  `stop-condition evidence: Button pilot increment ${buttonPilot} vs replaced ${replaced} — ` +
    "triggered at g14.001. g14.003 and g14.004 reuse the generic kernel for RangeSlider and Tabs.",
);

/**
 * Conformance cost report (spec 066 "Cost And Replacement Rules", g14.001):
 * an exhaustive inventory of every mechanism line — all four committed JSON
 * artifacts included — against what the mechanism replaced. Counts
 * non-blank, non-comment lines; replaced lines are measured against
 * `origin/main` (the pre-card baseline).
 */

import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";

const ROOT = execSync("git rev-parse --show-toplevel").toString().trim();

function countLines(text: string): number {
  return text
    .split("\n")
    .filter((line) => {
      const trimmed = line.trim();
      if (!trimmed) return false;
      if (trimmed.startsWith("//") || trimmed.startsWith("*") || trimmed.startsWith("/*")) return false;
      if (trimmed.startsWith("#") && !trimmed.startsWith("#!")) return false;
      return true;
    }).length;
}

function workingLoc(path: string): number {
  return countLines(readFileSync(`${ROOT}/${path}`, "utf8"));
}

function mainLoc(path: string): number {
  try {
    return countLines(execSync(`git show origin/main:${path}`, { cwd: ROOT }).toString());
  } catch {
    return 0;
  }
}

const AUTHORED: Array<[string, string]> = [
  ["Interface schema (define.ts)", "packages/core/src/conformance/define.ts"],
  ["Button interface (button.ts)", "packages/core/src/conformance/button.ts"],
  ["Button corpus (button-cases.ts)", "packages/core/src/conformance/button-cases.ts"],
  ["Specimen projection (project.ts)", "packages/core/src/conformance/project.ts"],
  ["Serializer script", "packages/core/scripts/conformance-serialize.ts"],
];

const CODEGEN: Array<[string, string]> = [
  ["Codegen conformance parsing + case validation", "packages/codegen/src/conformance.rs"],
  ["Codegen Rust declaration target", "packages/codegen/src/targets/conformance_rust.rs"],
  ["Codegen cases-copy target", "packages/codegen/src/targets/conformance_cases.rs"],
  ["Codegen CLI mode (delta)", "packages/codegen/src/bin/poodle-codegen.rs"],
];

const GENERATED: Array<[string, string]> = [
  ["Rust declaration (generated/button.rs)", "packages/contracts/components/src/generated/button.rs"],
  ["Interface fixture JSON", "packages/codegen/fixtures/conformance/button-interface.json"],
  ["Case fixture JSON", "packages/codegen/fixtures/conformance/button-cases.json"],
  ["Interface JSON copy (gpui preview)", "packages/gpui/preview/src/generated/conformance/button-interface.json"],
  ["Case JSON copy (gpui preview)", "packages/gpui/preview/src/generated/conformance/button-cases.json"],
];

const OBSERVERS_AND_RUNNERS: Array<[string, string]> = [
  ["Web runner core (data-driven observer)", "test/conformance/web/runner.ts"],
  ["Svelte adapter", "test/conformance/web/svelte-adapter.ts"],
  ["React adapter", "test/conformance/web/react-adapter.tsx"],
  ["Svelte host component", "test/conformance/web/hosts/ButtonHost.svelte"],
  ["Web test entry", "test/conformance/web/button.test.ts"],
  ["Native observer (render::conformance)", "packages/render/src/conformance.rs"],
  ["GPUI runner bin (real window + driver)", "packages/gpui/preview/src/bin/conformance.rs"],
  ["GPUI support module", "packages/gpui/preview/src/conformance_support.rs"],
  ["Orchestrator (normalized comparison)", "test/conformance/compare.ts"],
];

const SUPPORTING_DELTAS: Array<[string, string]> = [
  ["poodle-node roles + intrinsic_text", "packages/contracts/node/src/lib.rs"],
  ["render::button roles/focus/a11y/metrics fixes", "packages/render/src/button.rs"],
  ["node-backend focus query + single activation path", "packages/gpui/node-backend/src/lib.rs"],
  ["node-backend interaction fix", "packages/gpui/node-backend/src/interaction.rs"],
  ["Svelte Button shell + identity channels", "packages/svelte/components/src/Button.svelte"],
  ["React Button shell + identity channels", "packages/react/components/src/Button.tsx"],
];

const WIRING: Array<[string, string]> = [
  ["Effigy selectors + gate wiring", "tasks/effigy.tasks.toml#conformance"],
  ["Cost report (this script)", "packages/core/scripts/conformance-cost.ts"],
];

const REPLACED: Array<[string, string]> = [
  ["Hand-written ButtonSpec declaration surface", "packages/contracts/components/src/button.rs"],
  ["Svelte specimen fixture content", "packages/svelte/preview/src/specimens/ButtonSpecimen.svelte"],
  ["React specimen fixture content", "packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx"],
  ["GPUI specimen fixture content", "packages/gpui/preview/src/specimens/button.rs"],
];

function sectionLoc(path: string, startMarker: string, endMarker: string): number {
  const text = readFileSync(`${ROOT}/${path}`, "utf8");
  const start = text.indexOf(startMarker);
  const end = text.indexOf(endMarker, start + 1);
  if (start < 0 || end < 0) return 0;
  return countLines(text.slice(start, end));
}

function table(title: string, rows: Array<[string, string]>, mode: "working" | "delta" | "replaced"): number {
  console.log(`\n${title}`);
  let total = 0;
  for (const [label, path] of rows) {
    let count: number;
    if (mode === "replaced") {
      count = Math.max(0, mainLoc(path) - workingLoc(path));
    } else if (mode === "delta") {
      count = Math.max(0, workingLoc(path) - mainLoc(path));
    } else if (path.includes("#conformance")) {
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

const authored = table("Authored (TS authority + serializer)", AUTHORED, "working");
const codegen = table("Codegen (parsing, validation, targets)", CODEGEN, "delta");
const generated = table("Generated artifacts (all four committed JSON artifacts included)", GENERATED, "working");
const observers = table("Observers and runners", OBSERVERS_AND_RUNNERS, "working");
const supporting = table("Supporting deltas (vocabulary, renderer, backends, shells)", SUPPORTING_DELTAS, "delta");
const wiring = table("Wiring (selectors + cost script)", WIRING, "working");
const replaced = table("Replaced (deleted hand-written surfaces, measured against main)", REPLACED, "replaced");

const mechanism = authored + codegen + generated + observers + supporting + wiring;
const reusable = codegen + observers + supporting + wiring;
const perComponent = authored;

console.log("\n=== Summary ===");
console.log(`mechanism total: ${mechanism}`);
console.log(`  reusable kernel (codegen + observers/runners + supporting + wiring): ${reusable}`);
console.log(`  per-component authority (interface + corpus + projection + serializer): ${perComponent}`);
console.log(`  generated artifacts (declaration + four JSON artifacts): ${generated}`);
console.log(`replaced: ${replaced}`);
console.log(`net (mechanism minus replaced): ${mechanism - replaced}`);
console.log(
  `stop-condition check: mechanism ${mechanism} vs replaced ${replaced} on Button alone; ` +
    `the reusable kernel (${reusable}) is a one-time investment the remaining profile pilots ` +
    `consume without growth — the amortization claim is tested again at the RangeSlider pilot.`,
);

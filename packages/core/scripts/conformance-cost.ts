/**
 * Conformance cost report (spec 066 "Cost And Replacement Rules", g14.001):
 * every mechanism line vs what it replaced. Counts non-blank, non-comment
 * lines; replaced lines are measured from git against the pre-pilot state.
 */

import { execSync } from "node:child_process";

const ROOT = execSync("git rev-parse --show-toplevel").toString().trim();

function loc(path: string): number {
  try {
    const source = execSync(`git show HEAD:${path} 2>/dev/null || true`, {
      cwd: ROOT,
    })
      .toString();
    const text = require("node:fs").readFileSync(
      `${ROOT}/${path}`,
      "utf8",
    );
    void source;
    const lines = text.split("\n").filter((line) => {
      const trimmed = line.trim();
      if (!trimmed) return false;
      if (trimmed.startsWith("//") || trimmed.startsWith("*") || trimmed.startsWith("/*")) return false;
      return true;
    });
    return lines.length;
  } catch {
    return 0;
  }
}

function locFrom(original: string): number {
  const text = original;
  return text
    .split("\n")
    .filter((line) => {
      const trimmed = line.trim();
      if (!trimmed) return false;
      if (trimmed.startsWith("//") || trimmed.startsWith("*") || trimmed.startsWith("/*")) return false;
      return true;
    }).length;
}

function blob(path: string): string {
  try {
    return execSync(`git show HEAD:${path}`, { cwd: ROOT }).toString();
  } catch {
    return "";
  }
}

const authored: Array<[string, string]> = [
  ["Interface schema (define.ts)", "packages/core/src/conformance/define.ts"],
  ["Button interface (button.ts)", "packages/core/src/conformance/button.ts"],
  ["Button corpus (button-cases.ts)", "packages/core/src/conformance/button-cases.ts"],
  ["Specimen projection (project.ts)", "packages/core/src/conformance/project.ts"],
  ["Serializer script", "packages/core/scripts/conformance-serialize.ts"],
];

const generated: Array<[string, string]> = [
  ["Rust declaration (generated/button.rs)", "packages/contracts/components/src/generated/button.rs"],
  ["Case JSON copies (gpui)", "packages/gpui/preview/src/generated/conformance/button-cases.json"],
  ["Case JSON copies (jetstream)", "packages/jetstream/preview/src/generated/conformance/button-cases.json"],
];

const adapters: Array<[string, string]> = [
  ["Web runner core", "test/conformance/web/runner.ts"],
  ["Svelte adapter + host", "test/conformance/web/svelte-adapter.ts"],
  ["React adapter", "test/conformance/web/react-adapter.tsx"],
  ["Web test entry", "test/conformance/web/button.test.ts"],
  ["Native observer (render::conformance)", "packages/render/src/conformance.rs"],
  ["Jetstream runner bin", "packages/jetstream/preview/src/bin/conformance.rs"],
  ["Jetstream support module", "packages/jetstream/preview/src/conformance_support.rs"],
  ["GPUI runner bin", "packages/gpui/preview/src/bin/conformance.rs"],
  ["GPUI support module", "packages/gpui/preview/src/conformance_support.rs"],
  ["Orchestrator (compare)", "test/conformance/compare.ts"],
];

const wiring: Array<[string, string]> = [
  ["Effigy selectors + gate wiring (conformance section)", "tasks/effigy.tasks.toml#conformance"],
  ["Cost report (this script)", "packages/core/scripts/conformance-cost.ts"],
];

function locOfSection(path: string, startMarker: string, endMarker: string): number {
  const text = require("node:fs").readFileSync(`${ROOT}/${path}`, "utf8");
  const start = text.indexOf(startMarker);
  const end = text.indexOf(endMarker, start + 1);
  if (start < 0 || end < 0) return 0;
  return text
    .slice(start, end)
    .split("\n")
    .filter((line) => line.trim().length > 0).length;
}

/** Replaced surfaces, measured against HEAD (the pre-pilot state). */
const replaced: Array<[string, string, number]> = [
  [
    "Hand-written ButtonSpec declaration surface",
    "packages/contracts/components/src/button.rs",
    0, // measured below as the delta vs the generated+extension split
  ],
  ["Svelte specimen fixture content", "packages/svelte/preview/src/specimens/ButtonSpecimen.svelte", 0],
  ["React specimen fixture content", "packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx", 0],
  ["Jetstream specimen fixture content", "packages/jetstream/preview/src/specimens/button.rs", 0],
  ["GPUI specimen fixture content", "packages/gpui/preview/src/specimens/button.rs", 0],
];

function replacedDelta(): void {
  for (const entry of replaced) {
    const [label, path] = entry;
    const before = blob(path);
    const beforeLoc = locFrom(before);
    const after = require("node:fs").readFileSync(`${ROOT}/${path}`, "utf8");
    const afterLoc = locFrom(after);
    const delta = beforeLoc - afterLoc;
    entry[2] = delta > 0 ? delta : 0;
  }
}

function table(title: string, rows: Array<[string, string] | [string, string, number]>): number {
  console.log(`\n${title}`);
  let total = 0;
  for (const [label, path, replacedCount] of rows as Array<[string, string, number?]>) {
    let count: number;
    if (replacedCount !== undefined) {
      count = replacedCount;
    } else if (path.includes("#conformance")) {
      count = locOfSection(
        path.split("#")[0],
        "# Conformance kernel (g14.001",
        "# Preview (documentation site)",
      );
    } else {
      count = loc(path);
    }
    total += count;
    console.log(`  ${String(count).padStart(5)}  ${label}`);
  }
  console.log(`  ${String(total).padStart(5)}  total`);
  return total;
}

replacedDelta();

const authoredTotal = table("Authored (TS authority + serializer)", authored);
const generatedTotal = table("Generated (Rust declaration + JSON copies)", generated);
const adapterTotal = table("Adapters (runners + observers + orchestrator)", adapters);
const wiringTotal = table("Wiring (selectors + cost script)", wiring);
const replacedTotal = table("Replaced (deleted hand-written surfaces, from git)", replaced);

console.log("\n=== Summary ===");
console.log(`mechanism (authored + generated + adapters + wiring): ${authoredTotal + generatedTotal + adapterTotal + wiringTotal}`);
console.log(`replaced: ${replacedTotal}`);
console.log(
  `net: ${authoredTotal + generatedTotal + adapterTotal + wiringTotal - replacedTotal} lines (positive = mechanism grew; the Button proof's per-component cost is authored + adapters)`,
);
console.log(
  `ongoing per-component authoring cost (interface + corpus, per component): ~${Math.round((authoredTotal / 1))} lines today, before profile reuse`,
);

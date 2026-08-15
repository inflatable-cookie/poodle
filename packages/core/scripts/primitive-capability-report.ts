/**
 * Build and gate `primitive-capability-report.v1` (g14.002).
 *
 * Merges executed probe evidence from Svelte, React, render-neutral Rust, and
 * GPUI. Owned rows must pass on every required layer. Deferred rows are listed
 * with ownership only. Jetstream is program-deferred outside capability rows.
 *
 *   bun packages/core/scripts/primitive-capability-report.ts
 *   bun packages/core/scripts/primitive-capability-report.ts --check
 */

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import {
  PRIMITIVE_CAPABILITIES,
  ownedPrimitiveCapabilities,
  type PrimitiveCapability,
} from "../src/conformance/primitives";

const CHECK = process.argv.includes("--check");
const ROOT = join(fileURLToPath(new URL("..", import.meta.url)), "..", "..");
const OUT_DIR = join(ROOT, "test/conformance/web/out");
const REPORT_PATH = join(OUT_DIR, "primitive-capability-report.json");
const RUST_EVIDENCE = join(OUT_DIR, "primitive-render-neutral.json");
const MARKDOWN_PATH = join(OUT_DIR, "primitive-capability-report.md");

type Layer = "svelte" | "react" | "render-neutral" | "gpui";
type Verdict = "passing" | "failing" | "missing";

type ProbeEvidenceFile = {
  schema: string;
  runtime: string;
  probes: Array<{
    capabilityId: string;
    probeId: string;
    verdict: string;
    observations?: string[];
    fields?: unknown;
    reason?: string | null;
  }>;
};

type LayerEvidence = {
  status: Verdict;
  probeIds: string[];
  observations: string[];
  reason?: string;
};

const capabilityIds = new Set(PRIMITIVE_CAPABILITIES.map((row) => row.id));

function readEvidence(path: string, expectedRuntime: string): ProbeEvidenceFile | null {
  if (!existsSync(path)) return null;
  const file = JSON.parse(readFileSync(path, "utf8")) as ProbeEvidenceFile;
  if (file.schema !== "primitive-probe-evidence.v1") {
    throw new Error(`${path}: expected primitive-probe-evidence.v1`);
  }
  if (file.runtime !== expectedRuntime) {
    throw new Error(`${path}: runtime '${file.runtime}', expected '${expectedRuntime}'`);
  }
  for (const probe of file.probes) {
    if (!capabilityIds.has(probe.capabilityId)) {
      throw new Error(`${path}: unknown capability '${probe.capabilityId}' in probe '${probe.probeId}'`);
    }
    if (!Array.isArray(probe.observations) || probe.observations.some((item) => typeof item !== "string")) {
      throw new Error(`${path}: probe '${probe.probeId}' must declare its executed observations`);
    }
  }
  return file;
}

function ensureRustEvidence(): ProbeEvidenceFile {
  execFileSync(
    "cargo",
    [
      "test",
      "--quiet",
      "--manifest-path",
      "packages/render/Cargo.toml",
      "primitive_probes::tests::emit_neutral_primitive_evidence",
    ],
    { cwd: ROOT, encoding: "utf8", stdio: "inherit" },
  );
  const existing = readEvidence(RUST_EVIDENCE, "render-neutral");
  if (!existing) {
    throw new Error(`render-neutral evidence missing at ${RUST_EVIDENCE}`);
  }
  return existing;
}

function layerFromFile(file: ProbeEvidenceFile | null, capabilityId: string): LayerEvidence {
  if (!file) return { status: "missing", probeIds: [], observations: [], reason: "evidence file missing" };
  const probes = file.probes.filter((p) => p.capabilityId === capabilityId);
  if (probes.length === 0) {
    return { status: "missing", probeIds: [], observations: [], reason: "no executed probe" };
  }
  const observations = [...new Set(probes.flatMap((probe) => probe.observations ?? []))].sort();
  const failed = probes.find((p) => p.verdict !== "pass");
  if (failed) {
    return {
      status: "failing",
      probeIds: probes.map((p) => p.probeId),
      observations,
      reason: failed.reason ?? `${failed.probeId} failed`,
    };
  }
  return { status: "passing", probeIds: probes.map((p) => p.probeId), observations };
}

function rowReport(
  row: PrimitiveCapability,
  layers: Record<Layer, ProbeEvidenceFile | null>,
): Record<string, unknown> {
  const owned = row.owner === "g14.002" || row.owner === "g14.005";
  const evidence: Record<string, LayerEvidence | { status: "deferred"; owner: string }> = {};
  if (owned) {
    evidence.svelte = layerFromFile(layers.svelte, row.id);
    evidence.react = layerFromFile(layers.react, row.id);
    evidence["render-neutral"] = layerFromFile(layers["render-neutral"], row.id);
    evidence.gpui = layerFromFile(layers.gpui, row.id);
  } else {
    evidence.semanticProof = { status: "deferred", owner: row.owner };
  }

  let status: Verdict | "deferred" = "deferred";
  let missingObservations: string[] = [];
  if (owned) {
    const statuses = ["svelte", "react", "render-neutral", "gpui"].map(
      (layer) => (evidence[layer as Layer] as LayerEvidence).status,
    );
    const observed = new Set(
      ["svelte", "react", "render-neutral", "gpui"].flatMap(
        (layer) => (evidence[layer as Layer] as LayerEvidence).observations,
      ),
    );
    missingObservations = row.requiredObservations.filter((field) => !observed.has(field));
    if (statuses.some((s) => s === "failing")) status = "failing";
    else if (statuses.some((s) => s === "missing")) status = "missing";
    else if (missingObservations.length > 0) status = "missing";
    else status = "passing";
  }

  return {
    id: row.id,
    family: row.family,
    covers: row.covers,
    owner: row.owner,
    requiredObservations: row.requiredObservations,
    governingContract: row.governingContract ?? null,
    notes: row.notes ?? null,
    missingObservations,
    status,
    evidence,
  };
}

const layers: Record<Layer, ProbeEvidenceFile | null> = {
  svelte: readEvidence(join(OUT_DIR, "primitive-svelte.json"), "svelte"),
  react: readEvidence(join(OUT_DIR, "primitive-react.json"), "react"),
  "render-neutral": ensureRustEvidence(),
  gpui: readEvidence(join(OUT_DIR, "primitive-gpui.json"), "gpui"),
};

const rows = PRIMITIVE_CAPABILITIES.map((row) => rowReport(row, layers));
const owned = rows.filter((row) => row.owner === "g14.002" || row.owner === "g14.005");
const failingOwned = owned.filter((row) => row.status !== "passing");

const report = {
  schema: "primitive-capability-report.v1",
  generatedAt: new Date().toISOString(),
  jetstream: "program-deferred",
  gpuiMountedAccessibility: {
    status: "forced-acceptance",
    contract: "docs/contracts/003-native-accessibility.md",
    note: "GPUI 0.2.2 has no mounted accessibility tree for Poodle content. Omission is deliberate and observable; do not schedule a parallel tree.",
  },
  legacyCapabilityTooling: {
    capabilitiesJson: "packages/contracts/headless/capabilities/capabilities.json",
    driftScript: "packages/svelte/preview/scripts/capability-drift.ts",
    disposition: "adapt",
    note: "Static source traces remain as non-passing debt evidence. Execution authority is this report. timers is retired from the primitive roster (host timing, not render vocabulary).",
  },
  summary: {
    total: rows.length,
    owned: owned.length,
    ownedPassing: owned.length - failingOwned.length,
    deferred: rows.length - owned.length,
  },
  capabilities: rows,
};

const markdownLines = [
  "# Primitive capability report",
  "",
  `Generated: ${report.generatedAt}`,
  "",
  "Jetstream: program-deferred (outside capability rows).",
  "",
  "GPUI mounted accessibility: forced-acceptance per contract 003.",
  "",
  `| Capability | Family | Owner | Svelte | React | Rust | GPUI | Status |`,
  `| --- | --- | --- | --- | --- | --- | --- | --- |`,
];

for (const row of rows) {
  if (row.owner !== "g14.002" && row.owner !== "g14.005") {
    markdownLines.push(
      `| \`${row.id}\` | ${row.family} | ${row.owner} | — | — | — | — | deferred |`,
    );
    continue;
  }
  const ev = row.evidence as Record<Layer, LayerEvidence>;
  markdownLines.push(
    `| \`${row.id}\` | ${row.family} | ${row.owner} | ${ev.svelte.status} | ${ev.react.status} | ${ev["render-neutral"].status} | ${ev.gpui.status} | ${row.status} |`,
  );
}
markdownLines.push("");

const document = `${JSON.stringify(report, null, 2)}\n`;
const markdown = `${markdownLines.join("\n")}\n`;

mkdirSync(OUT_DIR, { recursive: true });

if (CHECK) {
  if (!existsSync(REPORT_PATH)) {
    console.error(`primitive report missing: ${REPORT_PATH}`);
    process.exit(1);
  }
  const committed = readFileSync(REPORT_PATH, "utf8");
  // Compare capability statuses only — timestamps drift.
  const committedJson = JSON.parse(committed);
  const normalize = (value: typeof report) => {
    const clone = structuredClone(value);
    delete (clone as { generatedAt?: string }).generatedAt;
    return clone;
  };
  if (JSON.stringify(normalize(committedJson)) !== JSON.stringify(normalize(report))) {
    console.error("primitive-capability-report.json is stale — run primitive report without --check");
    process.exit(1);
  }
  console.log("primitive-capability-report.v1 is current.");
} else {
  writeFileSync(REPORT_PATH, document);
  writeFileSync(MARKDOWN_PATH, markdown);
  console.log(`Wrote ${REPORT_PATH}`);
  console.log(`Wrote ${MARKDOWN_PATH}`);
}

if (failingOwned.length > 0) {
  console.error(
    `primitive completion failed: ${failingOwned.length} owned row(s) not passing:\n` +
      failingOwned.map((row) => `  - ${row.id} (${row.status})`).join("\n"),
  );
  process.exit(1);
}

console.log(
  `primitive-capability-report: ${owned.length - failingOwned.length}/${owned.length} owned rows passing; ${rows.length - owned.length} deferred.`,
);

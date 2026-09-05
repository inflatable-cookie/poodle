/**
 * g15.047 — the Button visual comparison batch: one command captures all 18
 * accepted fixtures through Svelte, React, and GPUI, verifies every
 * receipt/PNG pair, proves repeat-capture determinism, compares Svelte↔React
 * exactly and Svelte↔GPUI under the fixed renderer-aware policy, and writes
 * the machine-readable summary, 36 diffs, and the operator contact sheet.
 *
 *   bun test/visual/button-comparison/run.ts [--out=<dir>]
 *
 * Default output is the disposable `test/visual/button-comparison/out`. An
 * explicit `--out` directory is only cleaned when it carries this tool's own
 * marker file. Committed evidence is point-in-time review material; nothing
 * here ever reads a previous run as expected output, and there is no
 * update/refresh mode.
 */

import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";

import {
  loadButtonVisualInventory,
  type ButtonFixture,
} from "../fixtures/button-visual-inventory.ts";
import { captureGpuiBatch, fixtureFileStem } from "./capture-gpui.ts";
import { captureWebBatch, type WebCaptureRecord } from "./capture-web.ts";
import { captureSetProblems } from "./capture-set.ts";
import {
  compareExactPair,
  compareRendererAwarePair,
  type CaptureEvidence,
} from "./compare.ts";
import { renderContactSheet, type FixtureReport } from "./contact-sheet.ts";
import {
  GEOMETRY,
  KNOWN_RENDERER_DELTAS,
  PIXELS,
  ROLES,
  classifyKnownDelta,
  type Channel,
  type Finding,
  type PairVerdict,
} from "./policy.ts";
import {
  parseButtonCaptureReceipt,
  verifyReceiptAgainstPng,
  type RuntimeName,
} from "./receipt.ts";

const DEFAULT_OUT = "test/visual/button-comparison/out";
const MARKER = ".g15-047-output";

function args(): { outDir: string } {
  const out = process.argv.find((value) => value.startsWith("--out="));
  const unknown = process.argv.slice(2).filter((value) => !value.startsWith("--out="));
  if (unknown.length > 0) {
    throw new Error(`unknown arguments: ${unknown.join(" ")} — the batch is the fixed 18 fixtures, no subset mode`);
  }
  return { outDir: out ? out.slice("--out=".length) : DEFAULT_OUT };
}

function prepareOutDir(outDir: string): void {
  if (!existsSync(outDir)) {
    mkdirSync(outDir, { recursive: true });
  } else if (resolve(outDir) !== resolve(DEFAULT_OUT)) {
    // An explicit directory is only ever cleaned when it is provably ours.
    if (!existsSync(join(outDir, MARKER))) {
      throw new Error(
        `refusing to reuse ${outDir}: it has no ${MARKER} marker — choose an empty directory or one this tool created`,
      );
    }
    rmSync(outDir, { recursive: true });
    mkdirSync(outDir, { recursive: true });
  } else {
    rmSync(outDir, { recursive: true });
    mkdirSync(outDir, { recursive: true });
  }
  writeFileSync(join(outDir, MARKER), "g15.047 button visual comparison output\n");
}

function loadEvidence(
  fixture: ButtonFixture,
  runtime: RuntimeName,
  pngPath: string,
  receiptPath: string,
): CaptureEvidence {
  const png = readFileSync(pngPath);
  const receipt = parseButtonCaptureReceipt(JSON.parse(readFileSync(receiptPath, "utf8")), {
    fixture,
    runtime,
  });
  const problems = verifyReceiptAgainstPng(receipt, png);
  if (problems.length > 0) {
    throw new Error(`capture does not verify for ${fixture.name} [${runtime}]:\n  - ${problems.join("\n  - ")}`);
  }
  return { receipt, png };
}

/** The diff artifact for a passing pair: pixelmatch's rendering, no findings. */
function renderDiffPng(a: Buffer, b: Buffer): Buffer {
  const pa = PNG.sync.read(a);
  const pb = PNG.sync.read(b);
  const diff = new PNG({ width: pa.width, height: pa.height });
  pixelmatch(pa.data, pb.data, diff.data, pa.width, pa.height, {
    threshold: PIXELS.threshold,
    includeAA: PIXELS.includeAA,
  });
  return PNG.sync.write(diff);
}

type AnnotatedPair = {
  verdict: PairVerdict;
  knownDeltas: { pair: string; finding: Finding; deltaId: string }[];
};

function annotate(verdict: PairVerdict, web: CaptureEvidence, gpui: CaptureEvidence | null): AnnotatedPair {
  const knownDeltas: AnnotatedPair["knownDeltas"] = [];
  if (verdict.pair === "svelte-gpui" && gpui !== null) {
    const context = {
      webShadowLayers: web.receipt.roles.shadow.layers.length,
      gpuiShadowLayers: gpui.receipt.roles.shadow.layers.length,
      fixture: verdict.fixture,
    };
    for (const channelVerdict of Object.values(verdict.channels)) {
      for (const finding of channelVerdict.findings) {
        const deltaId = classifyKnownDelta(finding, context);
        if (deltaId !== null) knownDeltas.push({ pair: verdict.pair, finding, deltaId });
      }
    }
  }
  return { verdict, knownDeltas };
}

function pairBlockingFailures(annotated: AnnotatedPair): Finding[] {
  // Known-delta classification annotates a finding with its contract
  // citation; it never excuses it. Every fixed-policy finding blocks the run
  // — changing exit semantics is an orchestrator card decision, not something
  // this runner may do.
  return Object.values(annotated.verdict.channels).flatMap(
    (channelVerdict) => channelVerdict.findings,
  );
}

async function main(): Promise<void> {
  const { outDir } = args();
  const inventory = loadButtonVisualInventory();
  // The denominator is fixed: the accepted 18 fixtures, no subset mode. A
  // partial run could otherwise masquerade as a complete closed batch.
  const fixtures = inventory.fixtures;

  prepareOutDir(outDir);
  const capturesDir = join(outDir, "captures");
  const diffsDir = join(outDir, "diffs");
  mkdirSync(diffsDir, { recursive: true });

  console.log(`g15.047 button visual comparison: ${fixtures.length} fixtures × 3 runtimes → ${outDir}`);

  console.log("## capture: gpui (non-activating window batches, twice)");
  const gpuiRecords = captureGpuiBatch(fixtures, capturesDir);

  console.log("## capture: svelte + react (pinned headless Chromium, twice each)");
  const webRecords: WebCaptureRecord[] = await captureWebBatch(fixtures, capturesDir);

  const records = [...webRecords, ...gpuiRecords];
  const completeness = captureSetProblems(fixtures, records);
  if (completeness.length > 0) {
    throw new Error(`capture set is not the closed batch:\n  - ${completeness.join("\n  - ")}`);
  }

  console.log("## verify + compare");
  const reports: FixtureReport[] = [];
  const summaryFixtures: unknown[] = [];
  let blocking = 0;
  let knownDeltaCount = 0;

  for (const fixture of fixtures) {
    const stem = fixtureFileStem(fixture.name);
    const evidence: Record<RuntimeName, CaptureEvidence> = {
      svelte: loadEvidence(fixture, "svelte", join(capturesDir, "svelte", `${stem}.png`), join(capturesDir, "svelte", `${stem}.json`)),
      react: loadEvidence(fixture, "react", join(capturesDir, "react", `${stem}.png`), join(capturesDir, "react", `${stem}.json`)),
      gpui: loadEvidence(fixture, "gpui", join(capturesDir, "gpui", `${stem}.png`), join(capturesDir, "gpui", `${stem}.json`)),
    };

    const exact = compareExactPair(fixture, evidence.svelte, evidence.react);
    const aware = compareRendererAwarePair(fixture, evidence.svelte, evidence.gpui);

    writeFileSync(join(diffsDir, `${stem}--svelte-react.png`), exact.diffPng ?? renderDiffPng(evidence.svelte.png, evidence.react.png));
    writeFileSync(join(diffsDir, `${stem}--svelte-gpui.png`), aware.diffPng ?? renderDiffPng(evidence.svelte.png, evidence.gpui.png));

    const annotatedExact = annotate(exact.verdict, evidence.svelte, null);
    const annotatedAware = annotate(aware.verdict, evidence.svelte, evidence.gpui);
    const pairs = [annotatedExact, annotatedAware];
    knownDeltaCount += pairs.reduce((count, pair) => count + pair.knownDeltas.length, 0);

    const failures = pairs.flatMap(pairBlockingFailures);
    blocking += failures.length;

    reports.push({
      fixture,
      pairs: pairs.map((pair) => pair.verdict),
      knownDeltas: pairs.flatMap((pair) => pair.knownDeltas),
    });

    const line = (pair: AnnotatedPair): string => {
      const channels = (Object.entries(pair.verdict.channels) as [Channel, PairVerdict["channels"][Channel]][])
        .map(([name, channelVerdict]) => {
          if (channelVerdict.status === "pass") return `${name}:pass`;
          const excused = new Set(pair.knownDeltas.map((entry) => entry.finding));
          return channelVerdict.findings.every((finding) => excused.has(finding))
            ? `${name}:known-delta`
            : `${name}:FAIL`;
        })
        .join(" ");
      return `    ${pair.verdict.pair}: ${channels}`;
    };
    console.log(`  ${fixture.name}`);
    console.log(line(annotatedExact));
    console.log(line(annotatedAware));
    for (const finding of failures) {
      console.log(`      FAIL [${finding.channel}] ${finding.detail}`);
    }
    for (const pair of pairs) {
      for (const delta of pair.knownDeltas) {
        console.log(`      known-delta [${delta.finding.channel}] ${delta.finding.detail}`);
      }
    }

    summaryFixtures.push({
      fixture: fixture.name,
      group: fixture.group,
      captures: Object.fromEntries(
        records
          .filter((record) => record.fixture === fixture.name)
          .map((record) => [
            record.runtime,
            {
              png: `captures/${record.runtime}/${stem}.png`,
              receipt: `captures/${record.runtime}/${stem}.json`,
              sha256: record.sha256,
              repeatSha256: record.repeatSha256,
              repeatIdentical: record.sha256 === record.repeatSha256,
            },
          ]),
      ),
      pairs: pairs.map((pair) => ({
        pair: pair.verdict.pair,
        ok: pair.verdict.ok,
        channels: Object.fromEntries(
          (Object.entries(pair.verdict.channels) as [Channel, PairVerdict["channels"][Channel]][]).map(
            ([name, channelVerdict]) => [
              name,
              {
                status: channelVerdict.status,
                metrics: channelVerdict.metrics ?? null,
                findings: channelVerdict.findings.map((finding) => ({
                  ...finding,
                  knownDelta:
                    pair.knownDeltas.find((entry) => entry.finding === finding)?.deltaId ?? null,
                })),
              },
            ],
          ),
        ),
      })),
    });
  }

  const environments = [
    `chromium ${(JSON.parse(readFileSync(join(capturesDir, "svelte", `${fixtureFileStem(fixtures[0].name)}.json`), "utf8")) as { environment: { version: string } }).environment.version}`,
    (() => {
      const env = (JSON.parse(readFileSync(join(capturesDir, "gpui", `${fixtureFileStem(fixtures[0].name)}.json`), "utf8")) as { environment: { os: string; arch: string; gpuiSource: string; gpuiVersion: string; kind: string } }).environment;
      return `gpui ${env.gpuiSource} ${env.gpuiVersion} (${env.os}/${env.arch}, ${env.kind})`;
    })(),
  ];

  const summary = {
    schema: "poodle.button-visual-comparison.v1",
    batch: inventory.batch,
    generated: "same-run evidence; no timestamps by design",
    environment: environments,
    policy: {
      exact: "svelte↔react: identical dimensions, zero landmark edge delta, exact roles, zero differing pixels",
      rendererAware: { geometry: GEOMETRY, roles: ROLES, pixels: PIXELS },
      knownRendererDeltas: KNOWN_RENDERER_DELTAS,
    },
    metrics: {
      fixtures: fixtures.length,
      captures: records.length,
      repeatMismatches: records.filter((record) => record.sha256 !== record.repeatSha256).length,
      comparisons: fixtures.length * 2,
      blockingFailures: blocking,
      knownDeltas: knownDeltaCount,
    },
    fixtures: summaryFixtures,
  };
  writeFileSync(join(outDir, "summary.json"), JSON.stringify(summary, null, 2));
  writeFileSync(join(outDir, "contact-sheet.html"), renderContactSheet(reports, environments));

  console.log(
    `\n${records.length} captures, ${fixtures.length * 2} comparisons, ${blocking} blocking failure(s), ${knownDeltaCount} known-delta finding(s)`,
  );
  console.log(`summary: ${join(outDir, "summary.json")}`);
  console.log(`contact sheet: ${join(outDir, "contact-sheet.html")}`);
  if (blocking > 0) {
    console.log("## RESULT: FAILED — see findings above");
    process.exit(1);
  }
  console.log("## RESULT: comparison complete (review contact sheet before sign-off)");
}

await main();

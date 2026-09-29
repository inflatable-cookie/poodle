import { mkdir, rm, writeFile } from "node:fs/promises";

import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";
import { chromium, type Browser } from "playwright";

import { ALLOWLIST, DEFAULT_MAX_DIFF_RATIO } from "./allowlist";
import { DEBT, type VisualDebtEntry } from "./debt";
import { captureSpecimen, type CaptureResult } from "./capture";
import { SKIPPED, VIEWPORT, tierPlan, type Axis, type Tier } from "./config";
import { startPreviews } from "./server";
import { PageDeadlineError, captureSession } from "./session";

/**
 * Cross-framework visual gate (g12.009).
 *
 *   bun test/visual/run.ts [--tier=smoke|axis|sweep] [--report] [--slug=<slug>]
 *
 * Diffs the Svelte and React previews at the same slug and axis. `--report`
 * diffs against the committed parity-debt inventory (`debt.ts`): a failing
 * pair that the inventory records reports as known debt; any other failing
 * pair is a new regression and fails the process. Without `--report`, every
 * failure fails the process.
 */

const OUT_DIR = "test/visual/out";

type Failure = {
  slug: string;
  axis: string;
  kind: "capture" | "size" | "pixels";
  detail: string;
  diffRatio?: number;
};

function arg(name: string, fallback?: string): string | undefined {
  const hit = process.argv.find((value) => value.startsWith(`--${name}=`));
  return hit ? hit.slice(name.length + 3) : fallback;
}

function maxDiffRatio(slug: string): number {
  return ALLOWLIST[slug]?.maxDiffRatio ?? DEFAULT_MAX_DIFF_RATIO;
}

/** The inventory entry that records this failing pair, if any. */
function debtFor(slug: string, axis: string, kind: Failure["kind"]): VisualDebtEntry | undefined {
  return DEBT.find(
    (entry) => entry.slug === slug && entry.kind === kind && entry.axes.includes(axis),
  );
}

async function diffPair(
  slug: string,
  axis: Axis,
  svelte: Buffer,
  react: Buffer,
): Promise<{ diffRatio: number; diff: Buffer } | { mismatch: string }> {
  const a = PNG.sync.read(svelte);
  const b = PNG.sync.read(react);

  if (a.width !== b.width || a.height !== b.height) {
    return {
      mismatch: `svelte ${a.width}x${a.height} vs react ${b.width}x${b.height}`,
    };
  }

  const diff = new PNG({ width: a.width, height: a.height });
  const differing = pixelmatch(a.data, b.data, diff.data, a.width, a.height, {
    threshold: 0.1,
  });

  return {
    diffRatio: differing / (a.width * a.height),
    diff: PNG.sync.write(diff),
  };
}

async function main(): Promise<void> {
  const tier = (arg("tier", "smoke") as Tier) ?? "smoke";
  const reportOnly = process.argv.includes("--report");
  const onlySlug = arg("slug");

  const plan = tierPlan(tier);
  const requested = onlySlug ? onlySlug.split(",") : plan.slugs;
  const slugs = requested.filter((slug) => !SKIPPED[slug]);
  const skipped = requested.filter((slug) => SKIPPED[slug]);

  console.log(
    `visual gate: tier=${tier} slugs=${slugs.length} axes=${plan.axes.length} ` +
      `captures=${slugs.length * plan.axes.length * 2}`,
  );

  await rm(OUT_DIR, { recursive: true, force: true });
  await mkdir(OUT_DIR, { recursive: true });

  const servers = await startPreviews();
  let browser: Browser | undefined;
  const failures: Failure[] = [];
  let compared = 0;

  try {
    browser = await chromium.launch();
    const context = await browser.newContext({
      viewport: VIEWPORT,
      deviceScaleFactor: 1,
      colorScheme: "dark",
      reducedMotion: "reduce",
    });

    // One session per preview. The shared helper owns page recycling, the
    // per-capture deadline, and the file-chooser/dialog handlers; each preview
    // keeps its own young page so a degraded one never spreads.
    const sessions = {
      svelte: captureSession({ context }),
      react: captureSession({ context }),
    };

    const capture = async (framework: "svelte" | "react", slug: string, axis: Axis) => {
      const label = `${framework} ${slug} [${axis.id}]`;
      const attempt = (): Promise<CaptureResult> =>
        sessions[framework].run(label, (page) =>
          captureSpecimen(page, servers.urls[framework], slug, axis),
        );

      let first: CaptureResult;
      try {
        first = await attempt();
      } catch (error) {
        // A per-page deadline poisons the page. Treat it like the other
        // infrastructure failures below and give the specimen one attempt on a
        // fresh page before the pair is reported as a capture failure.
        if (!(error instanceof PageDeadlineError)) throw error;
        first = { ok: false, error: error.message };
      }
      if (first.ok) return first;
      // A preview can die mid-run (an externally started dev server outliving
      // its shell); restart it before blaming the specimen.
      if (await sessions[framework].recover(framework)) {
        console.log(`  restarted ${framework} preview`);
      }
      return attempt();
    };

    for (const axis of plan.axes) {
      for (const slug of slugs) {
        // Sequential on purpose: capturing both previews at once starves heavy
        // specimens (ListCard, DataTable) on a loaded machine and produces
        // spurious render timeouts.
        const svelte = await capture("svelte", slug, axis);
        const react = await capture("react", slug, axis);

        if (!svelte.ok || !react.ok) {
          const detail = [
            svelte.ok ? null : `svelte: ${svelte.error}`,
            react.ok ? null : `react: ${react.error}`,
          ]
            .filter(Boolean)
            .join(" | ");
          failures.push({ slug, axis: axis.id, kind: "capture", detail });
          console.log(`  ✗ ${slug} [${axis.id}] capture — ${detail}`);
          continue;
        }

        compared += 1;
        const result = await diffPair(slug, axis, svelte.png, react.png);
        const stem = `${OUT_DIR}/${slug}-${axis.id}`;

        if ("mismatch" in result) {
          failures.push({
            slug,
            axis: axis.id,
            kind: "size",
            detail: result.mismatch,
          });
          await writeFile(`${stem}-svelte.png`, svelte.png);
          await writeFile(`${stem}-react.png`, react.png);
          console.log(`  ✗ ${slug} [${axis.id}] size — ${result.mismatch}`);
          continue;
        }

        if (result.diffRatio > maxDiffRatio(slug)) {
          failures.push({
            slug,
            axis: axis.id,
            kind: "pixels",
            detail: `${(result.diffRatio * 100).toFixed(3)}% of pixels differ`,
            diffRatio: result.diffRatio,
          });
          await writeFile(`${stem}-svelte.png`, svelte.png);
          await writeFile(`${stem}-react.png`, react.png);
          await writeFile(`${stem}-diff.png`, result.diff);
          console.log(
            `  ✗ ${slug} [${axis.id}] pixels — ${(result.diffRatio * 100).toFixed(3)}%`,
          );
        }
      }
    }
  } finally {
    await browser?.close();
    await servers.stop();
  }

  const summary = {
    tier,
    comparisons: compared,
    failures,
    skipped: skipped.map((slug) => ({ slug, reason: SKIPPED[slug] })),
    allowlisted: Object.entries(ALLOWLIST).map(([slug, entry]) => ({
      slug,
      ...entry,
    })),
  };
  await writeFile(`${OUT_DIR}/summary.json`, JSON.stringify(summary, null, 2));

  console.log(`\ncompared ${compared} pairs, ${failures.length} failing`);
  if (skipped.length > 0) {
    console.log(`skipped (non-deterministic, not covered by this gate):`);
    for (const slug of skipped) console.log(`  - ${slug}: ${SKIPPED[slug]}`);
  }
  if (Object.keys(ALLOWLIST).length > 0) {
    console.log(`allowlisted deltas:`);
    for (const [slug, entry] of Object.entries(ALLOWLIST)) {
      console.log(`  - ${slug}: ${entry.reason}`);
    }
  }

  const debtFailures = failures.filter((f) => debtFor(f.slug, f.axis, f.kind));
  const regressions = failures.filter((f) => !debtFor(f.slug, f.axis, f.kind));

  if (reportOnly) {
    if (debtFailures.length > 0) {
      console.log(`known visual debt (${debtFailures.length} pairs, see test/visual/debt.ts):`);
      for (const f of debtFailures) {
        console.log(`  = ${f.slug} [${f.axis}] ${f.kind} — ${debtFor(f.slug, f.axis, f.kind)?.reason}`);
      }
    }
    if (regressions.length > 0) {
      console.error(`new regressions (${regressions.length} pairs, not in the debt inventory):`);
      for (const f of regressions) {
        console.error(`  ✗ ${f.slug} [${f.axis}] ${f.kind} — ${f.detail}`);
      }
      process.exit(1);
    }
    if (debtFailures.length === 0) console.log("no failures");
    return;
  }

  if (failures.length > 0) process.exit(1);
}

await main();

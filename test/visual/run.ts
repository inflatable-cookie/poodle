import { mkdir, rm, writeFile } from "node:fs/promises";

import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";
import { chromium, type Browser, type Page } from "playwright";

import { ALLOWLIST, DEFAULT_MAX_DIFF_RATIO } from "./allowlist";
import { captureSpecimen, pinPage } from "./capture";
import { SKIPPED, VIEWPORT, tierPlan, type Axis, type Tier } from "./config";
import { ensureUp, startPreviews } from "./server";

/**
 * Cross-framework visual gate (g12.009).
 *
 *   bun test/visual/run.ts [--tier=smoke|axis|sweep] [--report] [--slug=<slug>]
 *
 * Diffs the Svelte and React previews at the same slug and axis. `--report`
 * writes artifacts without failing the process.
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

    const pages: Record<"svelte" | "react", Page> = {
      svelte: await context.newPage(),
      react: await context.newPage(),
    };
    await pinPage(pages.svelte);
    await pinPage(pages.react);

    // A page degrades after a few dozen SPA loads (vite client + specimen state
    // accumulate) until heavy specimens stop settling inside the timeout.
    // Recycling keeps every capture on a young page; the retry covers the rest.
    let capturesOnPage = 0;
    const RECYCLE_AFTER = 20;

    const recycle = async (framework: "svelte" | "react") => {
      await pages[framework].close();
      pages[framework] = await context.newPage();
      await pinPage(pages[framework]);
    };

    const capture = async (framework: "svelte" | "react", slug: string, axis: Axis) => {
      const first = await captureSpecimen(pages[framework], servers.urls[framework], slug, axis);
      if (first.ok) return first;
      // A preview can die mid-run (an externally started dev server outliving
      // its shell); restart it before blaming the specimen.
      if (await ensureUp(framework)) console.log(`  restarted ${framework} preview`);
      await recycle(framework);
      return captureSpecimen(pages[framework], servers.urls[framework], slug, axis);
    };

    for (const axis of plan.axes) {
      for (const slug of slugs) {
        if (capturesOnPage >= RECYCLE_AFTER) {
          await recycle("svelte");
          await recycle("react");
          capturesOnPage = 0;
        }
        capturesOnPage += 1;

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

  if (failures.length > 0 && !reportOnly) process.exit(1);
}

await main();

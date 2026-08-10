import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";

import { captureSlugStable, ensurePreviewBuilt } from "./capture";
import {
  Axis,
  BASELINE_DIR,
  DEFAULT_AXIS,
  MAX_DIFF_RATIO,
  OUT_DIR,
  SKIPPED,
  gpuiSlugs,
  repoRoot,
} from "./config";

/**
 * Native visual gate:
 *
 *   bun test/native-visual/run.ts                 # diff against local baselines
 *   bun test/native-visual/run.ts --update        # (re)write baselines
 *   bun test/native-visual/run.ts --slug=button   # one component
 *
 * Needs a live macOS window-server session — the GPUI preview screenshots its
 * own window. Local-only, like `check:jetstream`.
 */

type Result =
  | { slug: string; status: "ok" }
  | { slug: string; status: "skipped"; detail: string }
  | { slug: string; status: "new" }
  | { slug: string; status: "failed"; detail: string };

function parseArgs() {
  const args = process.argv.slice(2);
  const slugArg = args.find((a) => a.startsWith("--slug="))?.split("=")[1];
  return {
    update: args.includes("--update"),
    slugs: slugArg ? slugArg.split(",") : null,
  };
}

function baselinePath(slug: string, axis: Axis): string {
  return path.join(repoRoot, BASELINE_DIR, `${slug}-${axis.id}.png`);
}

function compare(a: Buffer, b: Buffer): { ratio: number; diff: Buffer } | { mismatch: string } {
  const left = PNG.sync.read(a);
  const right = PNG.sync.read(b);
  if (left.width !== right.width || left.height !== right.height) {
    return {
      mismatch: `${left.width}x${left.height} vs baseline ${right.width}x${right.height}`,
    };
  }
  const diff = new PNG({ width: left.width, height: left.height });
  const differing = pixelmatch(left.data, right.data, diff.data, left.width, left.height, {
    threshold: 0.1,
  });
  return { ratio: differing / (left.width * left.height), diff: PNG.sync.write(diff) };
}

const { update, slugs: only } = parseArgs();
const axis = DEFAULT_AXIS;
const slugs = (only ?? gpuiSlugs()).filter((s) => only || !(s in SKIPPED));

mkdirSync(path.join(repoRoot, BASELINE_DIR), { recursive: true });
rmSync(path.join(repoRoot, OUT_DIR), { recursive: true, force: true });
mkdirSync(path.join(repoRoot, OUT_DIR), { recursive: true });

console.log(
  `native visual gate: ${slugs.length} components, axis=${axis.id}` +
    (update ? " (writing baselines)" : ""),
);
if (!only && Object.keys(SKIPPED).length > 0) {
  console.log(`  skipped (non-deterministic): ${Object.keys(SKIPPED).join(", ")}`);
}

ensurePreviewBuilt();

const results: Result[] = [];

/**
 * `screencapture` refuses with "could not create image from window" when the
 * display session is locked or asleep, or when the terminal has lost Screen
 * Recording permission. That failure is systemic, not per-component: once it
 * starts, every remaining capture fails identically. Grinding through a hundred
 * of them buries the one fact that matters, so the run stops and says what is
 * actually wrong. Found the hard way — a baseline run produced 75 good captures
 * and then 59 identical failures.
 */
let consecutiveCaptureFailures = 0;

for (const [index, slug] of slugs.entries()) {
  const shot = path.join(repoRoot, OUT_DIR, `${slug}-${axis.id}.png`);
  // Relaunching immediately after the previous window closed makes captures
  // time out that succeed fine when run by hand; give the window server a beat.
  if (index > 0) await Bun.sleep(1000);

  // Announced *before* the capture, so a stalled slug is visible while it is
  // stalling. Progress used to print only every tenth success, which meant a
  // focused run said nothing at all between the header and the verdict — a
  // capture that hangs and one that is merely slow looked identical for 90
  // seconds. Naming the slug first is what tells them apart.
  process.stdout.write(`  → ${index + 1}/${slugs.length} ${slug}\n`);
  const startedAt = Date.now();
  const capture = await captureSlugStable(slug, axis, shot);
  const took = (Date.now() - startedAt) / 1000;

  if (!capture.ok) {
    results.push({ slug, status: "failed", detail: capture.reason });
    console.log(`  ✗ ${slug} — ${capture.reason}`);

    consecutiveCaptureFailures += 1;
    if (consecutiveCaptureFailures >= 3) {
      const captured = results.filter((r) => r.status === "ok" || r.status === "new").length;
      console.log(
        `\nStopping: ${consecutiveCaptureFailures} captures in a row could not be taken.\n` +
          `This is the display session, not the components. screencapture cannot read\n` +
          `a window when the screen is locked or asleep, or when the terminal has lost\n` +
          `Screen Recording permission (System Settings > Privacy & Security).\n\n` +
          `Captured ${captured} of ${slugs.length} before this started.`,
      );
      break;
    }
    continue;
  }

  consecutiveCaptureFailures = 0;

  const baseline = baselinePath(slug, axis);
  const shotBytes = readFileSync(shot);

  if (update || !existsSync(baseline)) {
    writeFileSync(baseline, shotBytes);
    results.push({ slug, status: update ? "ok" : "new" });
    if (!update) console.log(`  + ${slug} — baseline written (was missing)`);
    continue;
  }

  const result = compare(shotBytes, readFileSync(baseline));
  if ("mismatch" in result) {
    results.push({ slug, status: "failed", detail: `size ${result.mismatch}` });
    console.log(`  ✗ ${slug} — size ${result.mismatch}`);
    continue;
  }

  if (result.ratio > MAX_DIFF_RATIO) {
    writeFileSync(path.join(repoRoot, OUT_DIR, `${slug}-${axis.id}-diff.png`), result.diff);
    results.push({
      slug,
      status: "failed",
      detail: `${(result.ratio * 100).toFixed(4)}% of pixels differ`,
    });
    console.log(`  ✗ ${slug} — ${(result.ratio * 100).toFixed(4)}% of pixels differ`);
    continue;
  }

  results.push({ slug, status: "ok" });
  // Duration on the success line: a slug that takes markedly longer than its
  // neighbours is the first sign of a capture going wrong, and it is invisible
  // if only failures print.
  console.log(`  ✓ ${slug} (${took.toFixed(1)}s)`);
}

const failed = results.filter((r) => r.status === "failed");
const fresh = results.filter((r) => r.status === "new");

console.log(`\ncompared ${results.length} components, ${failed.length} failing`);
if (fresh.length > 0) {
  console.log(`${fresh.length} baseline(s) written for the first time — commit them.`);
}
if (failed.length > 0) {
  console.log(`diffs in ${OUT_DIR}/`);
  process.exit(1);
}

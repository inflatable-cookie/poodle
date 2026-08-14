import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
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
 * Native visual gate (g14.002 capture repair):
 *
 *   bun test/native-visual/run.ts                 # compare (read-only)
 *   bun test/native-visual/run.ts --refresh       # replace baselines; preserve prior
 *   bun test/native-visual/run.ts --slug=button   # one component
 *   bun test/native-visual/run.ts --control-size=lg
 *
 * Comparison never writes references. A missing baseline fails with the
 * refresh command. Refresh keeps the previous PNG beside the new one and
 * emits a machine-readable manifest.
 */

type Result =
  | { slug: string; status: "ok" }
  | { slug: string; status: "skipped"; detail: string }
  | { slug: string; status: "missing"; detail: string }
  | { slug: string; status: "failed"; detail: string };

type RefreshEntry = {
  slug: string;
  axis: string;
  width: number;
  height: number;
  oldHash: string | null;
  newHash: string;
  previousPath: string | null;
  baselinePath: string;
  capturePath: string;
  reason: string;
};

function parseArgs() {
  const args = process.argv.slice(2);
  const slugArg = args.find((a) => a.startsWith("--slug="))?.split("=")[1];
  const sizeArg = args.find((a) => a.startsWith("--control-size="))?.split("=")[1];
  const reasonArg = args.find((a) => a.startsWith("--reason="))?.slice("--reason=".length);
  const refresh = args.includes("--refresh") || args.includes("--update");
  return {
    refresh,
    reason: reasonArg ?? (refresh ? "explicit refresh" : ""),
    controlSize: sizeArg ?? null,
    slugs: slugArg ? slugArg.split(",") : null,
  };
}

function axisFor(controlSize: string | null): Axis {
  if (!controlSize || controlSize === DEFAULT_AXIS.controlSize) return DEFAULT_AXIS;
  return {
    id: `${DEFAULT_AXIS.theme}-${DEFAULT_AXIS.density}-${controlSize}`,
    theme: DEFAULT_AXIS.theme,
    density: DEFAULT_AXIS.density,
    controlSize,
  };
}

function baselinePath(slug: string, axis: Axis): string {
  return path.join(repoRoot, BASELINE_DIR, `${slug}-${axis.id}.png`);
}

function previousPath(slug: string, axis: Axis): string {
  return path.join(repoRoot, BASELINE_DIR, `${slug}-${axis.id}.previous.png`);
}

function sha256(bytes: Buffer): string {
  return createHash("sha256").update(bytes).digest("hex");
}

function pngSize(bytes: Buffer): { width: number; height: number } {
  const png = PNG.sync.read(bytes);
  return { width: png.width, height: png.height };
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

const { refresh, reason, controlSize, slugs: only } = parseArgs();
const axis = axisFor(controlSize);
const slugs = (only ?? gpuiSlugs()).filter((s) => only || !(s in SKIPPED));
const refreshCommand =
  `bun test/native-visual/run.ts --refresh --control-size=${axis.controlSize}` +
  (only ? ` --slug=${only.join(",")}` : "") +
  ` --reason='bootstrap or intentional refresh'`;

mkdirSync(path.join(repoRoot, BASELINE_DIR), { recursive: true });
rmSync(path.join(repoRoot, OUT_DIR), { recursive: true, force: true });
mkdirSync(path.join(repoRoot, OUT_DIR), { recursive: true });

console.log(
  `native visual gate: ${slugs.length} components, axis=${axis.id}` +
    (refresh ? " (refresh; preserving previous baselines)" : " (compare; read-only)"),
);
if (!only && Object.keys(SKIPPED).length > 0) {
  console.log(`  skipped (non-deterministic): ${Object.keys(SKIPPED).join(", ")}`);
}

ensurePreviewBuilt();

const results: Result[] = [];
const refreshManifest: RefreshEntry[] = [];
let consecutiveCaptureFailures = 0;

for (const [index, slug] of slugs.entries()) {
  const shot = path.join(repoRoot, OUT_DIR, `${slug}-${axis.id}.png`);
  if (index > 0) await Bun.sleep(1000);

  process.stdout.write(`  → ${index + 1}/${slugs.length} ${slug}\n`);
  const startedAt = Date.now();
  const capture = await captureSlugStable(slug, axis, shot);
  const took = (Date.now() - startedAt) / 1000;

  if (!capture.ok) {
    results.push({ slug, status: "failed", detail: capture.reason });
    console.log(`  ✗ ${slug} — ${capture.reason}`);

    consecutiveCaptureFailures += 1;
    if (consecutiveCaptureFailures >= 3) {
      const captured = results.filter((r) => r.status === "ok").length;
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
  const size = pngSize(shotBytes);
  const newHash = sha256(shotBytes);

  if (refresh) {
    let oldHash: string | null = null;
    let preserved: string | null = null;
    if (existsSync(baseline)) {
      const previous = previousPath(slug, axis);
      copyFileSync(baseline, previous);
      oldHash = sha256(readFileSync(previous));
      preserved = previous;
    }
    writeFileSync(baseline, shotBytes);
    refreshManifest.push({
      slug,
      axis: axis.id,
      width: size.width,
      height: size.height,
      oldHash,
      newHash,
      previousPath: preserved,
      baselinePath: baseline,
      capturePath: shot,
      reason,
    });
    results.push({ slug, status: "ok" });
    console.log(
      `  ✓ ${slug} refreshed (${took.toFixed(1)}s)` +
        (preserved ? `; previous kept at ${path.relative(repoRoot, preserved)}` : "; first baseline"),
    );
    continue;
  }

  if (!existsSync(baseline)) {
    results.push({
      slug,
      status: "missing",
      detail: `no baseline — run: ${refreshCommand}`,
    });
    console.log(`  ✗ ${slug} — missing baseline; refresh with:\n      ${refreshCommand}`);
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
  console.log(`  ✓ ${slug} (${took.toFixed(1)}s)`);
}

if (refresh && refreshManifest.length > 0) {
  const manifestPath = path.join(repoRoot, OUT_DIR, `refresh-manifest-${axis.id}.json`);
  writeFileSync(
    manifestPath,
    `${JSON.stringify(
      {
        schema: "native-visual-refresh-manifest.v1",
        axis: axis.id,
        controlSize: axis.controlSize,
        reason,
        generatedAt: new Date().toISOString(),
        entries: refreshManifest,
      },
      null,
      2,
    )}\n`,
  );
  console.log(`\nrefresh manifest: ${path.relative(repoRoot, manifestPath)}`);
}

const failed = results.filter((r) => r.status === "failed" || r.status === "missing");
console.log(`\ncompared ${results.length} components, ${failed.length} failing`);
if (failed.length > 0) {
  console.log(`diffs in ${OUT_DIR}/`);
  process.exit(1);
}

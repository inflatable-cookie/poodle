import { mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

import { applyBaseline } from "./baseline";
import { repoRoot } from "./config";

/**
 * Jetstream native visual gate — the headless one.
 *
 *   bun test/native-visual/jetstream.ts                  # diff against baselines
 *   bun test/native-visual/jetstream.ts --update         # (re)write baselines
 *   bun test/native-visual/jetstream.ts --slug=button    # one component
 *
 * Where the GPUI gate opens a window per component and screenshots it through
 * the macOS window server, this renders every specimen offscreen on a headless
 * wgpu device in one process. The difference is not incremental:
 *
 *   | | GPUI | Jetstream |
 *   |-|------|-----------|
 *   | full sweep | ~20 min | **90 s** |
 *   | flake | ~3% per run | **0** — 135/135 bit-identical across two sweeps |
 *   | needs | awake, unlocked display | nothing |
 *   | stability workaround | two agreeing captures | none needed |
 *
 * Every failure mode the GPUI gate fought — incomplete frames, antialiasing
 * jitter, hover state, window activation, display sleep — is structurally
 * absent here. There is no compositor and no guessed moment: the scene is
 * rendered, then the texture is read back.
 *
 * So the tolerance is a true zero, not a measured noise floor, and there is no
 * retry loop. A difference means the render changed. A specimen with no
 * baseline fails unless this run is `--update`.
 */

const SNAP_OUT = "/tmp/poodle-specimens";
const BASELINE_DIR = "packages/jetstream/preview/baselines";
const OUT_DIR = "test/native-visual/out-jetstream";
const PREVIEW_CWD = "packages/jetstream/preview";

const update = process.argv.includes("--update");
/**
 * Comma-separated slugs, or null for everything.
 *
 * A filtered sweep skips the two non-component chrome pages as well, so
 * checking one component costs one render rather than 138.
 */
const slugs =
  process.argv.find((a) => a.startsWith("--slug="))?.slice("--slug=".length) ?? null;

function runSweep(): void {
  rmSync(SNAP_OUT, { recursive: true, force: true });
  const args = ["cargo", "run", "--quiet", "--bin", "snap", "--", "specimens"];
  if (slugs) args.push(`--slug=${slugs}`);
  // stderr is inherited rather than piped: `snap` names each specimen as it
  // renders, and swallowing that made the render phase a silent ~80s block in
  // which a stall and normal progress looked the same.
  const proc = Bun.spawnSync(args, {
    cwd: path.join(repoRoot, PREVIEW_CWD),
    stdout: "ignore",
    stderr: "inherit",
  });
  if (!proc.success) {
    throw new Error("snap sweep failed — see the render output above");
  }
}

const baselineDir = path.join(repoRoot, BASELINE_DIR);
const outDir = path.join(repoRoot, OUT_DIR);
mkdirSync(baselineDir, { recursive: true });
rmSync(outDir, { recursive: true, force: true });
mkdirSync(outDir, { recursive: true });

console.log(
  `jetstream visual gate${update ? " (writing baselines)" : ""}${slugs ? ` — ${slugs}` : ""} — rendering offscreen…`,
);
runSweep();

const rendered = readdirSync(SNAP_OUT).filter((f) => f.endsWith(".png")).sort();
console.log(`  ${rendered.length} specimens rendered`);

let written = 0;
const failed: string[] = [];

for (const file of rendered) {
  const shot = readFileSync(path.join(SNAP_OUT, file));
  const baseline = path.join(baselineDir, file);
  const result = applyBaseline({ file, shot, baselinePath: baseline, update });

  if (result.status === "written") {
    written++;
    if (result.first) console.log(`  + ${file} — baseline written (was missing)`);
    continue;
  }
  if (result.status === "missing") {
    failed.push(result.detail);
    console.log(`  ✗ ${result.detail}`);
    continue;
  }
  if (result.status === "failed") {
    failed.push(result.detail);
    if (result.diff) {
      writeFileSync(path.join(outDir, file.replace(".png", "-diff.png")), result.diff);
    }
    console.log(`  ✗ ${result.detail}`);
    continue;
  }
}

console.log(`\ncompared ${rendered.length} specimens, ${failed.length} failing`);
if (written > 0) console.log(`${written} baseline(s) written — commit them.`);
if (failed.length > 0) {
  console.log(`diffs in ${OUT_DIR}/`);
  process.exit(1);
}

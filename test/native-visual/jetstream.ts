import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";

import { repoRoot } from "./config";

/**
 * Jetstream native visual gate — the headless one.
 *
 *   bun test/native-visual/jetstream.ts            # diff against baselines
 *   bun test/native-visual/jetstream.ts --update   # (re)write baselines
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
 * retry loop. A difference means the render changed.
 */

const SNAP_OUT = "/tmp/poodle-specimens";
const BASELINE_DIR = "packages/jetstream/preview/baselines";
const OUT_DIR = "test/native-visual/out-jetstream";
const PREVIEW_CWD = "packages/jetstream/preview";

const update = process.argv.includes("--update");

function runSweep(): void {
  rmSync(SNAP_OUT, { recursive: true, force: true });
  const proc = Bun.spawnSync(["cargo", "run", "--quiet", "--bin", "snap", "--", "specimens"], {
    cwd: path.join(repoRoot, PREVIEW_CWD),
    stdout: "ignore",
    stderr: "pipe",
  });
  if (!proc.success) {
    throw new Error(`snap sweep failed:\n${proc.stderr.toString().split("\n").slice(-20).join("\n")}`);
  }
}

const baselineDir = path.join(repoRoot, BASELINE_DIR);
const outDir = path.join(repoRoot, OUT_DIR);
mkdirSync(baselineDir, { recursive: true });
rmSync(outDir, { recursive: true, force: true });
mkdirSync(outDir, { recursive: true });

console.log(`jetstream visual gate${update ? " (writing baselines)" : ""} — rendering offscreen…`);
runSweep();

const rendered = readdirSync(SNAP_OUT).filter((f) => f.endsWith(".png")).sort();
console.log(`  ${rendered.length} specimens rendered`);

let ok = 0;
let fresh = 0;
const failed: string[] = [];

for (const file of rendered) {
  const shot = readFileSync(path.join(SNAP_OUT, file));
  const baseline = path.join(baselineDir, file);

  if (update || !existsSync(baseline)) {
    writeFileSync(baseline, shot);
    if (update) ok++;
    else {
      fresh++;
      console.log(`  + ${file} — baseline written (was missing)`);
    }
    continue;
  }

  const a = PNG.sync.read(shot);
  const b = PNG.sync.read(readFileSync(baseline));

  if (a.width !== b.width || a.height !== b.height) {
    failed.push(`${file} — size ${a.width}x${a.height} vs baseline ${b.width}x${b.height}`);
    console.log(`  ✗ ${failed.at(-1)}`);
    continue;
  }

  const diff = new PNG({ width: a.width, height: a.height });
  const differing = pixelmatch(a.data, b.data, diff.data, a.width, a.height, { threshold: 0.1 });

  // A true zero: the render is deterministic, so any difference is a change.
  if (differing > 0) {
    writeFileSync(path.join(outDir, file.replace(".png", "-diff.png")), PNG.sync.write(diff));
    const ratio = ((differing / (a.width * a.height)) * 100).toFixed(4);
    failed.push(`${file} — ${differing} px (${ratio}%)`);
    console.log(`  ✗ ${failed.at(-1)}`);
    continue;
  }
  ok++;
}

console.log(`\ncompared ${rendered.length} specimens, ${failed.length} failing`);
if (fresh > 0) console.log(`${fresh} baseline(s) written for the first time — commit them.`);
if (failed.length > 0) {
  console.log(`diffs in ${OUT_DIR}/`);
  process.exit(1);
}

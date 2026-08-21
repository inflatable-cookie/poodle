/**
 * g15.045 — offscreen capture smoke (headless GPUI pixels).
 *
 * Drives the `poodle-offscreen-capture` one-shot target: three captures of
 * identical input must hash identically, each receipt must check out against
 * its PNG, and every negative case (unsupported scale, unknown theme, unknown
 * control size, missing output arguments, a tampered PNG) must fail loudly.
 *
 * Everything lands in a temporary directory that is deleted on exit; nothing
 * is written into the repository, and no baseline is created. Unsupported OS
 * or a missing Metal device is a hard failure inside the capture binary —
 * this script never skips green.
 */
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import {
  existsSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const PREVIEW = new URL("..", import.meta.url).pathname;
const MANIFEST = join(PREVIEW, "Cargo.toml");
const BIN = join(PREVIEW, "target", "debug", "poodle-offscreen-capture");
const GPUI_REVISION = "1ea16c1ab9dd6d36649e002dc60995634da04daf";
const RECEIPT_SCHEMA = "poodle.gpui-offscreen-capture.v1";
const WIDTH = 240;
const HEIGHT = 80;
const THEME = "default";
const CONTROL_SIZE = "md";
const SCALE = "2.0";
const REPEATS = 3;

let failures = 0;
function check(label: string, ok: boolean, detail = ""): void {
  if (ok) {
    console.log(`  PASS  ${label}`);
  } else {
    console.log(`  FAIL  ${label}${detail ? `: ${detail}` : ""}`);
    failures += 1;
  }
}

function sha256(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

function pngSize(path: string): { width: number; height: number } {
  const header = readFileSync(path);
  return {
    width: header.readUInt32BE(16),
    height: header.readUInt32BE(20),
  };
}

interface Receipt {
  schema: string;
  gpui_revision: string;
  renderer: string;
  platform: string;
  theme: string;
  control_size: string;
  logical_viewport: { width: number; height: number };
  scale: number;
  device_dimensions: { width: number; height: number };
  png_sha256: string;
}

/** Every claim a receipt makes about its PNG, verified against the files. */
function verifyPair(pngPath: string, receiptPath: string): string | null {
  if (!existsSync(pngPath)) return "PNG is missing";
  if (!existsSync(receiptPath)) return "receipt is missing";
  let receipt: Receipt;
  try {
    receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as Receipt;
  } catch {
    return "receipt is not valid JSON";
  }
  if (receipt.schema !== RECEIPT_SCHEMA) return `schema drifted: ${receipt.schema}`;
  if (receipt.gpui_revision !== GPUI_REVISION)
    return `gpui revision drifted: ${receipt.gpui_revision}`;
  if (receipt.png_sha256 !== sha256(pngPath)) return "receipt hash does not match the PNG";
  const size = pngSize(pngPath);
  if (receipt.device_dimensions.width !== size.width) return "receipt width mismatch";
  if (receipt.device_dimensions.height !== size.height) return "receipt height mismatch";
  if (receipt.scale !== 2.0) return "receipt scale is not 2.0";
  if (size.width !== receipt.logical_viewport.width * 2) return "device width != logical x 2";
  if (size.height !== receipt.logical_viewport.height * 2) return "device height != logical x 2";
  return null;
}

function capture(dir: string, name: string, extra: string[] = []) {
  const png = join(dir, `${name}.png`);
  const receipt = join(dir, `${name}.json`);
  const args = [
    "--out", png,
    "--receipt", receipt,
    "--width", String(WIDTH),
    "--height", String(HEIGHT),
    "--theme", THEME,
    "--control-size", CONTROL_SIZE,
    "--scale", SCALE,
    ...extra,
  ];
  const result = spawnSync(BIN, args, { encoding: "utf8" });
  return { png, receipt, result };
}

// 1. Build the capture target.
console.log("## build");
const build = spawnSync(
  "cargo",
  ["build", "--quiet", "--manifest-path", MANIFEST, "--bin", "poodle-offscreen-capture", "--features", "capture"],
  { encoding: "utf8" },
);
if (build.status !== 0) {
  console.error(build.stderr);
  console.error("FATAL: capture target does not build");
  process.exit(1);
}
check("capture target builds", true);

const work = mkdtempSync(join(tmpdir(), "poodle-offscreen-smoke-"));
try {
  // 2. Repeatability: identical input, repeated, one hash.
  console.log("## repeatability — identical input");
  const runs = Array.from({ length: REPEATS }, (_, i) => capture(work, `cap${i}`));
  for (const [i, run] of runs.entries()) {
    check(`capture ${i} exits zero`, run.result.status === 0, run.result.stderr.trim());
  }
  const hashes = runs.map((r) => (existsSync(r.png) ? sha256(r.png) : "<missing>"));
  for (const [i, hash] of hashes.entries()) console.log(`  cap${i}: ${hash}`);
  check(
    "one distinct hash across repeated captures",
    new Set(hashes).size === 1 && !hashes.includes("<missing>"),
  );
  for (const [i, run] of runs.entries()) {
    check(`capture ${i} receipt checks out against its PNG`, verifyPair(run.png, run.receipt) === null,
      verifyPair(run.png, run.receipt) ?? "");
  }
  const receipt0 = JSON.parse(readFileSync(runs[0].receipt, "utf8")) as Receipt;
  check("receipt records theme and control size",
    receipt0.theme === THEME && receipt0.control_size === CONTROL_SIZE);
  check("device dimensions are 480x160 (240x80 logical x 2)",
    receipt0.device_dimensions.width === 480 && receipt0.device_dimensions.height === 160);

  // 3. Negative cases: each must fail nonzero and write no output.
  console.log("## negative cases");
  const negatives: Array<[string, string[]]> = [
    ["unsupported scale", ["--scale", "1.0"]],
    ["unknown theme", ["--theme", "dracula"]],
    ["unknown control size", ["--control-size", "xxl"]],
  ];
  for (const [label, override] of negatives) {
    const name = `neg-${label.replace(/\s+/g, "-")}`;
    const base = ["--out", join(work, `${name}.png`), "--receipt", join(work, `${name}.json`),
      "--width", String(WIDTH), "--height", String(HEIGHT),
      "--theme", THEME, "--control-size", CONTROL_SIZE, "--scale", SCALE];
    // Replace the target flag's value in place.
    const flag = override[0];
    const idx = base.indexOf(flag);
    base[idx + 1] = override[1];
    const result = spawnSync(BIN, base, { encoding: "utf8" });
    check(`${label} fails`, result.status !== 0, "unexpectedly succeeded");
    check(`${label} writes no output`,
      !existsSync(join(work, `${name}.png`)) && !existsSync(join(work, `${name}.json`)));
  }
  {
    // Missing required output argument.
    const result = spawnSync(BIN, [
      "--out", join(work, "neg-missing.png"),
      "--width", String(WIDTH), "--height", String(HEIGHT),
      "--theme", THEME, "--control-size", CONTROL_SIZE, "--scale", SCALE,
    ], { encoding: "utf8" });
    check("missing --receipt fails", result.status !== 0, "unexpectedly succeeded");
    check("missing --receipt writes no PNG", !existsSync(join(work, "neg-missing.png")));
  }
  {
    // A tampered PNG must be caught by the receipt check — proves the check
    // has teeth rather than being a tautology.
    const tampered = join(work, "tampered.png");
    writeFileSync(tampered, Buffer.concat([readFileSync(runs[0].png), Buffer.from([0])]));
    const verdict = verifyPair(tampered, runs[0].receipt);
    check("tampered PNG is detected", verdict !== null, "mismatch went unnoticed");
  }
} finally {
  rmSync(work, { recursive: true, force: true });
}

if (failures === 0) {
  console.log("## RESULT: offscreen capture smoke passed");
  process.exit(0);
} else {
  console.log(`## RESULT: FAILED — ${failures} check(s) failed`);
  process.exit(1);
}

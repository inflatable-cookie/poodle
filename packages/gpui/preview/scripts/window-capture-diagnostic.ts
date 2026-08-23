/**
 * g16.005 — window capture diagnostic (WINDOWED, operator-approved only).
 *
 * This opens real macOS windows. It needs a window server and Screen
 * Recording permission, and it is deliberately reachable only through
 * `effigy capture:gpui-windowed`, whose `-windowed` suffix is the
 * repository's standing "ask the operator first" marker (AGENTS.md).
 *
 * What it proves that the headless smoke cannot:
 *
 *  - the transport captures genuine window-server pixels at the exact device
 *    size the receipt claims;
 *  - identical input captured repeatedly is byte-identical;
 *  - every receipt verifies against its PNG;
 *  - the frontmost application did not change during any capture, sampled by
 *    the capture process itself for the whole of its own run.
 *
 * Everything lands in a temporary directory that is deleted on exit. No
 * baseline is written, nothing enters the repository, and no comparison is
 * made — that is `effigy test:visual-button-comparison-windowed`.
 */
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const PREVIEW = new URL("..", import.meta.url).pathname;
const MANIFEST = join(PREVIEW, "Cargo.toml");
const BIN = join(PREVIEW, "target", "debug", "poodle-window-capture");
const FEATURE = "window-capture";
const RECEIPT_SCHEMA = "poodle.gpui-window-capture.v1";
const TRANSPORT = "macos-window-server-nonactivating";
const GPUI_SOURCE = "crates.io";
const GPUI_VERSION = "0.2.2";
const WIDTH = 240;
const HEIGHT = 80;
const THEME = "default";
const CONTROL_SIZE = "md";
const REPEATS = 3;
/** Mirrors `MIN_FOREGROUND_SAMPLES` in `window_capture/transport.rs`. */
const MIN_FOREGROUND_SAMPLES = 8;

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
  return { width: header.readUInt32BE(16), height: header.readUInt32BE(20) };
}

interface Receipt {
  schema: string;
  component: { name: string; variant: string; label: string };
  gpui_source: string;
  gpui_version: string;
  transport: string;
  platform: string;
  theme: string;
  control_size: string;
  logical_viewport: { width: number; height: number };
  scale: number;
  device_dimensions: { width: number; height: number };
  png_sha256: string;
  foreground: { baseline: string; observed: string[]; samples: number; verdict: string };
}

/** Every claim a receipt makes, verified against the files it describes. */
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
  if (receipt.gpui_source !== GPUI_SOURCE) return `gpui source drifted: ${receipt.gpui_source}`;
  if (receipt.gpui_version !== GPUI_VERSION) return `gpui version drifted: ${receipt.gpui_version}`;
  if (receipt.transport !== TRANSPORT) return `transport drifted: ${receipt.transport}`;
  if (receipt.component?.name !== "Button" || receipt.component.variant !== "primary" ||
      receipt.component.label !== "Save") return "component smoke identity drifted";
  if (receipt.platform !== "macos") return `platform drifted: ${receipt.platform}`;
  if (receipt.theme !== THEME) return `theme drifted: ${receipt.theme}`;
  if (receipt.control_size !== CONTROL_SIZE) return `control size drifted: ${receipt.control_size}`;
  if (receipt.logical_viewport.width !== WIDTH || receipt.logical_viewport.height !== HEIGHT)
    return "logical viewport drifted";
  if (receipt.png_sha256 !== sha256(pngPath)) return "receipt hash does not match the PNG";
  const size = pngSize(pngPath);
  if (receipt.device_dimensions.width !== size.width) return "receipt width mismatch";
  if (receipt.device_dimensions.height !== size.height) return "receipt height mismatch";
  if (receipt.scale !== 2) return "receipt scale is not 2.0";
  if (size.width !== WIDTH * 2) return `device width ${size.width} != logical x 2`;
  if (size.height !== HEIGHT * 2) return `device height ${size.height} != logical x 2`;
  // Three-valued on purpose: "did not change" and "could not tell" are
  // different answers and only one is proof.
  if (receipt.foreground.verdict !== "proved") {
    return `foreground verdict is '${receipt.foreground.verdict}', not 'proved'`;
  }
  if (typeof receipt.foreground.baseline !== "string" || receipt.foreground.baseline.length === 0) {
    return "the run read no baseline frontmost application";
  }
  if (receipt.foreground.observed.some((app) => app !== receipt.foreground.baseline)) {
    return "another application was frontmost during the run";
  }
  if (receipt.foreground.samples < MIN_FOREGROUND_SAMPLES) {
    return `only ${receipt.foreground.samples} frontmost-application samples, ${MIN_FOREGROUND_SAMPLES} required`;
  }
  return null;
}

console.log("## build");
const build = spawnSync(
  "cargo",
  ["build", "--quiet", "--manifest-path", MANIFEST, "--bin", "poodle-window-capture", "--features", FEATURE],
  { encoding: "utf8" },
);
if (build.status !== 0) {
  console.error(build.stderr);
  console.error("FATAL: capture target does not build");
  process.exit(1);
}
check("capture target builds", true);

const work = mkdtempSync(join(tmpdir(), "poodle-window-capture-"));
try {
  console.log("## repeatability — identical input, one window each");
  const runs = Array.from({ length: REPEATS }, (_, i) => {
    const png = join(work, `cap${i}.png`);
    const receipt = join(work, `cap${i}.json`);
    const result = spawnSync(BIN, [
      "--out", png, "--receipt", receipt,
      "--width", String(WIDTH), "--height", String(HEIGHT),
      "--theme", THEME, "--control-size", CONTROL_SIZE, "--scale", "2.0",
    ], { encoding: "utf8" });
    return { png, receipt, result };
  });

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
    const problem = verifyPair(run.png, run.receipt);
    check(`capture ${i} receipt checks out against its PNG`, problem === null, problem ?? "");
  }

  if (existsSync(runs[0].receipt)) {
    const receipt = JSON.parse(readFileSync(runs[0].receipt, "utf8")) as Receipt;
    console.log(`  transport: ${receipt.transport}`);
    console.log(`  device:    ${receipt.device_dimensions.width}x${receipt.device_dimensions.height}`);
    console.log(
      `  foreground: baseline=${receipt.foreground.baseline} observed=${JSON.stringify(receipt.foreground.observed)} ` +
        `samples=${receipt.foreground.samples} verdict=${receipt.foreground.verdict}`,
    );
    check(
      "every capture proved it left the frontmost application alone",
      runs.every((run) =>
        existsSync(run.receipt) &&
        (JSON.parse(readFileSync(run.receipt, "utf8")) as Receipt).foreground.verdict === "proved"
      ),
    );
  }
} finally {
  rmSync(work, { recursive: true, force: true });
}

console.log(failures === 0 ? "\nwindow capture diagnostic: all checks pass" : `\nwindow capture diagnostic: ${failures} failed`);
process.exit(failures === 0 ? 0 : 1);

/**
 * g16.005 — inset-shadow evidence run (WINDOWED, operator-approved only).
 *
 * The backend paints inset (inner) shadow layers itself, because crates.io
 * `gpui::BoxShadow` has no `inset` flag. Headless tests prove the bands reach
 * `paint_quad` with the right widths, colour, and padding box; only a real
 * capture shows how they RASTERISE.
 *
 * The Button visual comparison cannot show it: its closed 18-case inventory
 * is Button-only, and `poodle_render::button` emits no shadow layers at all.
 * This is the run that actually exercises the path, through real components —
 * Accordion (offset edge band), ListCard (spread ring AND leading bar,
 * stacked), Tabs (spread ring on a drop target), and Popover (edge band on a
 * DEFERRED overlay surface).
 *
 * All four scenes render in ONE non-activating process through the shared
 * batch transport. Output is disposable and gitignored; nothing is compared,
 * no baseline is written, and no PNG enters the repository.
 */
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";

const PREVIEW = new URL("..", import.meta.url).pathname;
const REPO = new URL("../../../..", import.meta.url).pathname;
const MANIFEST = join(PREVIEW, "Cargo.toml");
const BIN = join(PREVIEW, "target", "debug", "poodle-window-capture");
const FEATURE = "window-capture";
const OUT = join(REPO, "test/visual/inset-shadow-evidence/out");
const RECEIPT_SCHEMA = "poodle.gpui-inset-shadow-evidence.v1";
const TRANSPORT = "macos-window-server-nonactivating";
const MIN_FOREGROUND_SAMPLES = 8;

/** The closed scene set, mirroring `SCENES` in `inset_evidence.rs`. */
const SCENES = ["accordion", "list-card", "tabs", "popover"] as const;

type Band = {
  element: string;
  left: number;
  right: number;
  top: number;
  bottom: number;
  color: [number, number, number, number];
  bounds: [number, number, number, number];
};

type Receipt = {
  schema: string;
  scene: string;
  bands: Band[];
  gpui_source: string;
  gpui_version: string;
  transport: string;
  platform: string;
  theme: string;
  logical_viewport: [number, number];
  scale: number;
  device_dimensions: [number, number];
  png_sha256: string;
  foreground: { baseline: string; observed: string[]; samples: number; verdict: string };
};

let failures = 0;
function check(label: string, ok: boolean, detail = ""): void {
  if (ok) {
    console.log(`  PASS  ${label}`);
  } else {
    console.log(`  FAIL  ${label}${detail ? `: ${detail}` : ""}`);
    failures += 1;
  }
}

function pngSize(bytes: Buffer): { width: number; height: number } {
  return { width: bytes.readUInt32BE(16), height: bytes.readUInt32BE(20) };
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

rmSync(OUT, { recursive: true, force: true });
mkdirSync(OUT, { recursive: true });

console.log(`## capture — ${SCENES.length} scenes, one process`);
const run = spawnSync(BIN, ["--inset-evidence", "all", "--out-dir", OUT], { encoding: "utf8" });
check(`the batch exits zero`, run.status === 0, run.stderr.trim());
if (run.stderr.trim()) console.log(run.stderr.trim().split("\n").map((l) => `  ${l}`).join("\n"));

for (const scene of SCENES) {
  const pngPath = join(OUT, `${scene}.png`);
  const receiptPath = join(OUT, `${scene}.json`);
  if (!existsSync(pngPath) || !existsSync(receiptPath)) {
    check(`${scene} produced a PNG and receipt`, false, "one or both are missing");
    continue;
  }

  const png = readFileSync(pngPath);
  const receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as Receipt;

  check(`${scene}: schema and transport are current`,
    receipt.schema === RECEIPT_SCHEMA && receipt.transport === TRANSPORT,
    `${receipt.schema} / ${receipt.transport}`);
  check(`${scene}: pixels came from crates.io GPUI`,
    receipt.gpui_source === "crates.io" && receipt.gpui_version === "0.2.2",
    `${receipt.gpui_source} ${receipt.gpui_version}`);
  check(`${scene}: receipt hash matches the PNG`,
    receipt.png_sha256 === createHash("sha256").update(png).digest("hex"));

  const size = pngSize(png);
  check(`${scene}: device dimensions are logical x 2`,
    size.width === receipt.logical_viewport[0] * 2 && size.height === receipt.logical_viewport[1] * 2,
    `${size.width}x${size.height} for ${receipt.logical_viewport.join("x")}`);

  // The point of the run: bands were actually painted, with real geometry.
  check(`${scene}: at least one inset band painted`, receipt.bands.length > 0);
  check(`${scene}: every band has a positive side and a real padding box`,
    receipt.bands.every((band) =>
      (band.left > 0 || band.right > 0 || band.top > 0 || band.bottom > 0) &&
      band.bounds[2] > 0 && band.bounds[3] > 0),
    JSON.stringify(receipt.bands));

  check(`${scene}: the capture proved it left the foreground alone`,
    receipt.foreground.verdict === "proved" &&
      receipt.foreground.samples >= MIN_FOREGROUND_SAMPLES &&
      receipt.foreground.observed.every((app) => app === receipt.foreground.baseline),
    `verdict=${receipt.foreground.verdict} samples=${receipt.foreground.samples}`);

  for (const band of receipt.bands) {
    console.log(
      `  ${scene} ${band.element}: l=${band.left} r=${band.right} t=${band.top} b=${band.bottom} ` +
        `in ${band.bounds.join(",")}`,
    );
  }
}

// The stacked case is the one a single-layer scene cannot show.
const listCard = join(OUT, "list-card.json");
if (existsSync(listCard)) {
  const receipt = JSON.parse(readFileSync(listCard, "utf8")) as Receipt;
  check("list-card stacks two inset layers on one surface",
    receipt.bands.length >= 2, `${receipt.bands.length} band(s)`);
}

console.log(`\nEvidence PNGs: ${OUT}`);
console.log("Look at: the ring thickness and corner clipping (tabs, list-card),");
console.log("the 1px top highlight (accordion, popover), the leading bar (list-card),");
console.log("and that the popover panel's highlight painted at all (deferred surface).");
console.log(
  failures === 0 ? "\ninset-shadow evidence: all checks pass" : `\ninset-shadow evidence: ${failures} failed`,
);
process.exit(failures === 0 ? 0 : 1);

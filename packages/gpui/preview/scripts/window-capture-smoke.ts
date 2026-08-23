/**
 * g16.005 — window capture smoke (headless).
 *
 * Everything about the `poodle-window-capture` target that can be proved
 * WITHOUT a window server:
 *
 *  - it builds;
 *  - its own unit tests pass, which is where the activation boundary
 *    (`window_capture/forbidden.rs`), the device-size policy, the
 *    frontmost-application rule, and the publish atomicity live;
 *  - every negative invocation is rejected during argument validation, before
 *    any window exists, and writes nothing.
 *
 * This script opens no window, takes no focus, and needs no Screen Recording
 * permission, so it is safe in an ordinary worker. The capture itself is
 * `effigy capture:gpui-windowed`, which is operator-approved and does need
 * all three.
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const PREVIEW = new URL("..", import.meta.url).pathname;
const MANIFEST = join(PREVIEW, "Cargo.toml");
const BIN = join(PREVIEW, "target", "debug", "poodle-window-capture");
const FEATURE = "window-capture";
const WIDTH = "240";
const HEIGHT = "80";
const THEME = "default";
const CONTROL_SIZE = "md";
const SCALE = "2.0";

let failures = 0;
function check(label: string, ok: boolean, detail = ""): void {
  if (ok) {
    console.log(`  PASS  ${label}`);
  } else {
    console.log(`  FAIL  ${label}${detail ? `: ${detail}` : ""}`);
    failures += 1;
  }
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

console.log("## target unit tests (activation boundary, device size, publish, foreground)");
const unit = spawnSync(
  "cargo",
  ["test", "--quiet", "--manifest-path", MANIFEST, "--bin", "poodle-window-capture", "--features", FEATURE],
  { encoding: "utf8" },
);
check("capture target unit tests pass", unit.status === 0, unit.stdout.trim().split("\n").slice(-3).join(" | "));

const work = mkdtempSync(join(tmpdir(), "poodle-window-capture-smoke-"));
try {
  // Every case below must be rejected while parsing arguments — before a
  // window is opened, before the window server is touched, and before any
  // file is written. The binary exits 2 for a rejected command line and 1 for
  // a failure that happened after the window opened, so `status === 2` is
  // itself the proof that no window was involved.
  console.log("## negative cases (rejected before any window opens)");

  const smokeArgs = (overrides: Record<string, string> = {}, drop: string[] = []): string[] => {
    const base: Record<string, string> = {
      "--out": join(work, "out.png"),
      "--receipt": join(work, "out.json"),
      "--width": WIDTH,
      "--height": HEIGHT,
      "--theme": THEME,
      "--control-size": CONTROL_SIZE,
      "--scale": SCALE,
      ...overrides,
    };
    for (const flag of drop) delete base[flag];
    return Object.entries(base).flat();
  };

  const negatives: Array<[string, string[]]> = [
    ["unsupported scale", smokeArgs({ "--scale": "1.0" })],
    ["unknown theme", smokeArgs({ "--theme": "dracula" })],
    ["unknown control size", smokeArgs({ "--control-size": "xxl" })],
    ["missing --receipt", smokeArgs({}, ["--receipt"])],
    ["missing --width", smokeArgs({}, ["--width"])],
    ["missing --scale", smokeArgs({}, ["--scale"])],
    ["non-positive viewport", smokeArgs({ "--width": "0" })],
    ["unknown flag", [...smokeArgs(), "--baseline", "x"]],
    [
      "colliding PNG and receipt paths",
      smokeArgs({ "--out": join(work, "same"), "--receipt": join(work, "same") }),
    ],
    [
      "unknown fixture",
      ["--fixture", "button/bogus", "--out", join(work, "f.png"), "--receipt", join(work, "f.json")],
    ],
    [
      "legacy flag in fixture mode",
      ["--fixture", "button/rest-secondary", "--out", join(work, "f.png"), "--receipt", join(work, "f.json"), "--scale", SCALE],
    ],
    [
      "unknown focus-evidence scene",
      ["--focus-evidence", "dialog", "--out", join(work, "r.png"), "--receipt", join(work, "r.json")],
    ],
  ];

  for (const [label, args] of negatives) {
    const result = spawnSync(BIN, args, { encoding: "utf8" });
    check(
      `${label} is rejected before a window opens`,
      result.status === 2,
      `exit ${result.status}: ${result.stderr.trim()}`,
    );
  }

  const written = [
    join(work, "out.png"),
    join(work, "out.json"),
    join(work, "same"),
    join(work, "f.png"),
    join(work, "f.json"),
    join(work, "r.png"),
    join(work, "r.json"),
  ].filter((path) => existsSync(path));
  check("no rejected invocation wrote a file", written.length === 0, written.join(", "));
} finally {
  rmSync(work, { recursive: true, force: true });
}

console.log(failures === 0 ? "\nwindow capture smoke: all checks pass" : `\nwindow capture smoke: ${failures} failed`);
process.exit(failures === 0 ? 0 : 1);

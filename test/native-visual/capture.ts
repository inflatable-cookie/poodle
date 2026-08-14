import { existsSync, readFileSync, unlinkSync } from "node:fs";
import path from "node:path";
import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";

import { Axis, CAPTURE_TIMEOUT_S, GPUI_PREVIEW_CWD, MAX_DIFF_RATIO, repoRoot } from "./config";

/**
 * The built binary, run directly.
 *
 * `cargo run` re-checks freshness on every invocation, which costs 10–20s
 * before the window even opens. At two captures per component across 133
 * components that is over an hour of pure overhead, and it was most of why a
 * full rebuild kept outliving the session. `ensurePreviewBuilt` compiles once;
 * every capture after that execs the binary.
 */
const PREVIEW_BIN = path.join(repoRoot, GPUI_PREVIEW_CWD, "target/debug/poodle-preview");

/** Compile once, before any capture. Throws with cargo's output if it fails. */
export function ensurePreviewBuilt(): void {
  const build = Bun.spawnSync(["cargo", "build", "--quiet"], {
    cwd: path.join(repoRoot, GPUI_PREVIEW_CWD),
    stdout: "ignore",
    stderr: "pipe",
  });
  if (!build.success) {
    throw new Error(`preview build failed:\n${build.stderr.toString()}`);
  }
}

/** Fraction of pixels differing between two images. */
function diffRatio(a: Buffer, b: Buffer): number {
  const left = PNG.sync.read(a);
  const right = PNG.sync.read(b);
  if (left.width !== right.width || left.height !== right.height) return 1;
  const differing = pixelmatch(left.data, right.data, null, left.width, left.height, {
    threshold: 0.1,
  });
  return differing / (left.width * left.height);
}

/**
 * Drive one GPUI preview capture.
 *
 * The preview owns the screenshot: given `--screenshot`, it opens its window,
 * waits for the first render, finds itself by PID through a CoreGraphics
 * lookup, shells out to `screencapture`, and exits. So this is a subprocess
 * launch plus a wait for the file to appear — there is no window to drive.
 *
 * That also means it needs a live macOS window-server session, which is why
 * this gate is local-only, like `check:jetstream`.
 */
/**
 * Capture until two consecutive attempts agree.
 *
 * The preview waits a fixed 1.5s for its first render and then captures
 * whatever is on screen. That is usually enough and sometimes is not: a
 * `segmented-control` baseline written this way shows the selected segment
 * barely painted, and a fresh capture shows it filled. Same code, same flags —
 * the earlier one simply caught an incomplete frame.
 *
 * That is the whole flake story. Failures rotated between runs, ratios drifted
 * for the same component, and one bad frame written into a baseline stayed
 * wrong forever. Rather than raise the tolerance until incomplete frames pass —
 * which would also let real regressions through — a capture is only accepted
 * once two runs of it produce the same bytes.
 */
export async function captureSlugStable(
  slug: string,
  axis: Axis,
  outPath: string,
): Promise<{ ok: true; receiptPath: string } | { ok: false; reason: string }> {
  const probe = `${outPath}.probe.png`;

  for (let attempt = 0; attempt < 3; attempt++) {
    const first = await captureSlug(slug, axis, outPath);
    if (!first.ok) return first;

    // The two attempts need the same breathing room as two components do.
    // Launching the second immediately after the first closed is the exact
    // rapid-relaunch condition that makes captures unreliable — without this
    // pause the check reported `button` as unsettled, and `button` was the
    // component whose two launches were bit-identical in the first place.
    await Bun.sleep(1000);

    const second = await captureSlug(slug, axis, probe);
    if (!second.ok) return second;

    const a = readFileSync(outPath);
    const b = readFileSync(probe);
    unlinkSync(probe);
    if (existsSync(second.receiptPath)) unlinkSync(second.receiptPath);

    // Agreement is measured with the same tolerance the gate itself uses, not
    // byte equality. Some components carry a persistent antialiasing jitter —
    // `callout` differs by 11 pixels out of 6.4 million between any two runs —
    // and demanding identical bytes declared those permanently unsettled.
    if (diffRatio(a, b) <= MAX_DIFF_RATIO) return first;
  }

  return {
    ok: false,
    reason: "no two consecutive captures agreed — the render never settled",
  };
}

async function captureSlug(
  slug: string,
  axis: Axis,
  outPath: string,
): Promise<{ ok: true; receiptPath: string } | { ok: false; reason: string }> {
  if (existsSync(outPath)) unlinkSync(outPath);
  const receiptPath = `${outPath}.axis.json`;
  if (existsSync(receiptPath)) unlinkSync(receiptPath);

  const proc = Bun.spawn(
    [
      PREVIEW_BIN,
      "--component",
      slug,
      "--theme",
      axis.theme,
      "--density",
      axis.density,
      "--control-size",
      axis.controlSize,
      "--screenshot",
      outPath,
      "--capture-receipt",
      receiptPath,
    ],
    { cwd: path.join(repoRoot, GPUI_PREVIEW_CWD), stdout: "ignore", stderr: "pipe" },
  );

  const deadline = Date.now() + CAPTURE_TIMEOUT_S * 1000;
  while (Date.now() < deadline) {
    if (existsSync(outPath) && existsSync(receiptPath)) {
      // The file appears before `screencapture` has finished flushing it; wait
      // for the size to settle rather than reading a truncated PNG.
      let last = -1;
      for (;;) {
        const size = Bun.file(outPath).size;
        if (size > 0 && size === last) break;
        last = size;
        await Bun.sleep(150);
      }
      const receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as {
        schema?: string;
        controlSize?: string;
      };
      if (
        receipt.schema !== "native-visual-axis-receipt.v1" ||
        receipt.controlSize !== axis.controlSize
      ) {
        proc.kill();
        return {
          ok: false,
          reason: `capture axis receipt reported controlSize=${receipt.controlSize ?? "missing"}, expected ${axis.controlSize}`,
        };
      }
      proc.kill();
      return { ok: true, receiptPath };
    }
    if (proc.exitCode !== null && !existsSync(outPath)) {
      const err = await new Response(proc.stderr).text();
      const lines = err.trim().split("\n").filter(Boolean);
      // The last line is `screencapture failed with status: exit status: 1`,
      // which says nothing. The line before it carries the actual cause
      // ("could not create image from window"), so report that when present.
      const cause = lines.find((l) => l.includes("could not create image"));
      return {
        ok: false,
        reason: cause ?? lines.at(-1) ?? "preview exited without capturing",
      };
    }
    await Bun.sleep(250);
  }

  proc.kill();
  return { ok: false, reason: `no screenshot within ${CAPTURE_TIMEOUT_S}s` };
}

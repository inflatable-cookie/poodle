/**
 * g15.047 — GPUI capture driver. Builds the `poodle-window-capture` one-shot
 * target once, then invokes it twice per fixture: the pair must be
 * byte-identical, the receipt must verify against its PNG, and only then is
 * the pair retained. A failed invocation writes nothing and stops the batch
 * — there is no retry and no frame picking.
 *
 * WINDOWED (g16.005). Each invocation opens one real GPUI window with
 * `focus: false` and captures it by window id; there is no offscreen path on
 * crates.io GPUI 0.2.2. This driver therefore needs a macOS window server and
 * Screen Recording permission, and the comparison selector that calls it is
 * operator-approved rather than part of any default board.
 */

import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";

import type { ButtonFixture } from "../fixtures/button-visual-inventory.ts";
import { CaptureIntegrityError } from "./capture-set.ts";
import {
  parseButtonCaptureReceipt,
  sha256Hex,
  verifyReceiptAgainstPng,
} from "./receipt.ts";

const PREVIEW_MANIFEST = "packages/gpui/preview/Cargo.toml";
const BIN = "packages/gpui/preview/target/debug/poodle-window-capture";

export type GpuiCaptureRecord = {
  fixture: string;
  runtime: "gpui";
  pngPath: string;
  receiptPath: string;
  sha256: string;
  repeatSha256: string;
};

export function fixtureFileStem(name: string): string {
  return name.replace("/", "--");
}

function captureOnce(fixture: string, pngPath: string, receiptPath: string): void {
  const result = spawnSync(
    BIN,
    ["--fixture", fixture, "--out", pngPath, "--receipt", receiptPath],
    { encoding: "utf8" },
  );
  if (result.status !== 0) {
    throw new Error(
      `poodle-window-capture failed for ${fixture} (exit ${result.status}): ${result.stderr.trim()}`,
    );
  }
}

export function captureGpuiBatch(fixtures: ButtonFixture[], outDir: string): GpuiCaptureRecord[] {
  const build = spawnSync(
    "cargo",
    ["build", "--quiet", "--manifest-path", PREVIEW_MANIFEST, "--bin", "poodle-window-capture", "--features", "window-capture"],
    { encoding: "utf8" },
  );
  if (build.status !== 0) {
    throw new Error(`capture target does not build:\n${build.stderr}`);
  }

  const dir = join(outDir, "gpui");
  mkdirSync(dir, { recursive: true });

  const records: GpuiCaptureRecord[] = [];
  for (const fixture of fixtures) {
    const stem = fixtureFileStem(fixture.name);
    const pngPath = join(dir, `${stem}.png`);
    const receiptPath = join(dir, `${stem}.json`);
    const repeatPngPath = join(dir, `${stem}.repeat.png`);
    const repeatReceiptPath = join(dir, `${stem}.repeat.json`);

    captureOnce(fixture.name, pngPath, receiptPath);
    captureOnce(fixture.name, repeatPngPath, repeatReceiptPath);

    const png = readFileSync(pngPath);
    const repeat = readFileSync(repeatPngPath);
    const sha256 = sha256Hex(png);
    const repeatSha256 = sha256Hex(repeat);
    if (!png.equals(repeat)) {
      throw new CaptureIntegrityError(
        `repeat captures differ for ${fixture.name} [gpui]: ${sha256} vs ${repeatSha256} — fixed input must render byte-identically`,
      );
    }

    const receipt = parseButtonCaptureReceipt(
      JSON.parse(readFileSync(receiptPath, "utf8")),
      { fixture, runtime: "gpui" },
    );
    const problems = verifyReceiptAgainstPng(receipt, png);
    if (problems.length > 0) {
      throw new CaptureIntegrityError(`gpui receipt does not verify for ${fixture.name}:\n  - ${problems.join("\n  - ")}`);
    }
    // The receipt is written by the binary itself; assert the repeat receipt
    // agrees, then discard the repeat pair — one retained capture per runtime.
    const repeatReceipt = parseButtonCaptureReceipt(
      JSON.parse(readFileSync(repeatReceiptPath, "utf8")),
      { fixture, runtime: "gpui" },
    );
    if (repeatReceipt.pngSha256 !== receipt.pngSha256) {
      throw new CaptureIntegrityError(`repeat receipt hash disagrees for ${fixture.name} [gpui]`);
    }
    // One retained capture per runtime/fixture: the verified repeat pair is
    // discarded, its hash recorded on the record.
    rmSync(repeatPngPath);
    rmSync(repeatReceiptPath);

    records.push({ fixture: fixture.name, runtime: "gpui", pngPath, receiptPath, sha256, repeatSha256 });
  }
  return records;
}

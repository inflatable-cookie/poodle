import { existsSync, readFileSync, writeFileSync } from "node:fs";
import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";

export type BaselineResult =
  | { status: "ok" }
  | { status: "written"; first: boolean }
  | { status: "missing"; detail: string }
  | { status: "failed"; detail: string; diff?: Buffer };

/**
 * Compare a rendered shot to its baseline. A missing baseline is a failure
 * unless this run is an explicit update (`--update` / `--refresh`).
 */
export function applyBaseline(options: {
  file: string;
  shot: Buffer;
  baselinePath: string;
  update: boolean;
}): BaselineResult {
  const { file, shot, baselinePath, update } = options;
  const present = existsSync(baselinePath);

  if (update) {
    writeFileSync(baselinePath, shot);
    return { status: "written", first: !present };
  }

  if (!present) {
    return {
      status: "missing",
      detail: `no baseline — ${file}; rerun with --update to write one`,
    };
  }

  const a = PNG.sync.read(shot);
  const b = PNG.sync.read(readFileSync(baselinePath));

  if (a.width !== b.width || a.height !== b.height) {
    return {
      status: "failed",
      detail: `${file} — size ${a.width}x${a.height} vs baseline ${b.width}x${b.height}`,
    };
  }

  const diff = new PNG({ width: a.width, height: a.height });
  const differing = pixelmatch(a.data, b.data, diff.data, a.width, a.height, {
    threshold: 0.1,
  });
  if (differing > 0) {
    const ratio = ((differing / (a.width * a.height)) * 100).toFixed(4);
    return {
      status: "failed",
      detail: `${file} — ${differing} px (${ratio}%)`,
      diff: PNG.sync.write(diff),
    };
  }
  return { status: "ok" };
}

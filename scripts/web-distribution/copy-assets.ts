import { mkdirSync, copyFileSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";

import { distDir } from "./staging";
import type { AssetCopy } from "./types";

export function copyAssets(
  packageRoot: string,
  assets: readonly AssetCopy[],
  outDir: string = distDir(packageRoot),
): void {
  const defaultDist = distDir(packageRoot);
  for (const asset of assets) {
    const from = join(packageRoot, asset.from);
    const dest = join(packageRoot, asset.to);
    const rel = relative(defaultDist, dest);
    const to = rel === ".." || rel.startsWith("../") ? dest : join(outDir, rel);
    if (!existsSync(from)) {
      throw new Error(`missing asset ${asset.from}`);
    }
    mkdirSync(dirname(to), { recursive: true });
    copyFileSync(from, to);
  }
}

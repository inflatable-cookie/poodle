import { mkdirSync, copyFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";

import type { AssetCopy } from "./types";

export function copyAssets(packageRoot: string, assets: readonly AssetCopy[]): void {
  for (const asset of assets) {
    const from = join(packageRoot, asset.from);
    const to = join(packageRoot, asset.to);
    if (!existsSync(from)) {
      throw new Error(`missing asset ${asset.from}`);
    }
    mkdirSync(dirname(to), { recursive: true });
    copyFileSync(from, to);
  }
}

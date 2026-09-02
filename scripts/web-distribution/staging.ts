import { existsSync, rmSync, mkdirSync } from "node:fs";
import { join } from "node:path";

export function distDir(packageRoot: string): string {
  return join(packageRoot, "dist");
}

export function cleanStaging(packageRoot: string): string {
  const outDir = distDir(packageRoot);
  if (existsSync(outDir)) rmSync(outDir, { recursive: true, force: true });
  mkdirSync(outDir, { recursive: true });
  return outDir;
}

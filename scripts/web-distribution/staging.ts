import { existsSync, mkdtempSync, readdirSync, renameSync, rmSync, mkdirSync } from "node:fs";
import { basename, join } from "node:path";

export function distDir(packageRoot: string): string {
  return join(packageRoot, "dist");
}

export function cleanStaging(packageRoot: string): string {
  const outDir = distDir(packageRoot);
  if (existsSync(outDir)) rmSync(outDir, { recursive: true, force: true });
  mkdirSync(outDir, { recursive: true });
  return outDir;
}

const STAGING_PREFIX = ".dist-staging-";
const PREVIOUS_SUFFIX = ".__previous-";

function previousBackups(packageRoot: string): string[] {
  let entries: string[];
  try {
    entries = readdirSync(packageRoot);
  } catch {
    return [];
  }
  return entries
    .filter((entry) => entry.startsWith(`dist${PREVIOUS_SUFFIX}`))
    .map((entry) => join(packageRoot, entry));
}

/**
 * Create a fresh sibling staging directory on the same filesystem as `dist`.
 * The package build writes its full tree here; `publishStaging` renames it
 * into place so readers never see a half-built `dist`.
 */
export function createStagingDir(packageRoot: string): string {
  for (const stale of previousBackups(packageRoot)) {
    rmSync(stale, { recursive: true, force: true });
  }
  return mkdtempSync(join(packageRoot, STAGING_PREFIX));
}

export function discardStaging(stagedDir: string): void {
  rmSync(stagedDir, { recursive: true, force: true });
}

function isStagingDir(packageRoot: string, dir: string): boolean {
  return basename(dir).startsWith(STAGING_PREFIX) && join(packageRoot, basename(dir)) === dir;
}

/**
 * Replace `dist` with the fully built staging tree. The old tree is kept as a
 * sibling backup until the staged rename succeeds, then removed. A failed
 * build must call `discardStaging` instead so the previous `dist` is untouched.
 */
export function publishStaging(packageRoot: string, stagedDir: string): string {
  if (!isStagingDir(packageRoot, stagedDir)) {
    throw new Error(`refusing to publish non-staging directory ${stagedDir}`);
  }
  const outDir = distDir(packageRoot);
  if (!existsSync(outDir)) {
    try {
      renameSync(stagedDir, outDir);
    } catch (error) {
      throw asRenameError(error);
    }
    return outDir;
  }
  const backup = `${outDir}${PREVIOUS_SUFFIX}${process.pid}-${Date.now()}`;
  try {
    renameSync(outDir, backup);
  } catch (error) {
    throw asRenameError(error);
  }
  try {
    renameSync(stagedDir, outDir);
  } catch (error) {
    try {
      renameSync(backup, outDir);
    } catch {
      // The backup still holds the previous tree; leave it for the next build
      // to clean up rather than masking the original failure.
    }
    throw asRenameError(error);
  }
  rmSync(backup, { recursive: true, force: true });
  return outDir;
}

function asRenameError(error: unknown): Error {
  const code = (error as { code?: string } | null)?.code;
  if (code === "EXDEV") {
    return new Error(
      `atomic dist swap needs staged output on the same filesystem as dist (rename returned EXDEV)`,
    );
  }
  return error as Error;
}

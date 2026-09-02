import { join } from "node:path";

import { auditStagedDist } from "./audit";
import { copyAssets } from "./copy-assets";
import { emitDeclarations } from "./declarations";
import { readLockedTools } from "./lockfile";
import { receiptPath, writeReceipt } from "./receipt";
import { cleanStaging } from "./staging";
import type { BuiltPackage, PackageBuildSpec } from "./types";
import { buildViteLibrary } from "./vite-library";

export async function buildPackage(
  repoRoot: string,
  spec: PackageBuildSpec,
  publicFiles: readonly string[],
): Promise<BuiltPackage> {
  const packageRoot = join(repoRoot, spec.packageDir);
  const outDir = cleanStaging(packageRoot);

  const entries: Record<string, string> = {};
  for (const entry of spec.entries) {
    entries[entry.name] = join(packageRoot, entry.source);
  }

  await buildViteLibrary({
    root: packageRoot,
    outDir,
    entries,
    fileName: (entryName) => {
      const entry = spec.entries.find((item) => item.name === entryName);
      return `${entryName}${entry?.outputExt ?? ".js"}`;
    },
    externals: spec.forbiddenModules,
  });

  copyAssets(packageRoot, spec.assets);
  emitDeclarations({
    repoRoot,
    packageRoot,
    tsconfigPath: spec.declarationTsconfig,
  });
  if (spec.extraDeclarationCopies?.length) {
    copyAssets(packageRoot, spec.extraDeclarationCopies);
  }

  const tools = readLockedTools(repoRoot);
  const receipt = writeReceipt({ repoRoot, packageRoot, spec, tools });
  auditStagedDist({
    distDir: outDir,
    publicFiles,
    forbiddenModules: spec.forbiddenModules,
  });

  return {
    packageDir: spec.packageDir,
    distDir: outDir,
    receipt,
    receiptPath: receiptPath(packageRoot),
  };
}

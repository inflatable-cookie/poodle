import { join } from "node:path";

import { auditStagedDist } from "./audit";
import { copyAssets } from "./copy-assets";
import { emitDeclarations } from "./declarations";
import { readLockedTools } from "./lockfile";
import {
  assertReceiptCoversViteSources,
  assertTypeScriptAuthority,
  packageRelativeViteSources,
  receiptPath,
  writeReceipt,
} from "./receipt";
import { createStagingDir, discardStaging, publishStaging } from "./staging";
import type { BuiltPackage, PackageBuildSpec } from "./types";
import { buildViteLibrary } from "./vite-library";

export async function buildPackage(
  repoRoot: string,
  spec: PackageBuildSpec,
  publicFiles: readonly string[],
): Promise<BuiltPackage> {
  const packageRoot = join(repoRoot, spec.packageDir);
  const stagedDir = createStagingDir(packageRoot);
  try {
    const entries: Record<string, string> = {};
    for (const entry of spec.entries) {
      entries[entry.name] = join(packageRoot, entry.source);
    }

    const graph = await buildViteLibrary({
      root: packageRoot,
      outDir: stagedDir,
      entries,
      fileName: (entryName) => {
        const entry = spec.entries.find((item) => item.name === entryName);
        return `${entryName}${entry?.outputExt ?? ".js"}`;
      },
      externals: spec.externalModules ?? spec.forbiddenModules,
      plugins: spec.vitePlugins,
      chunkFileNames: spec.chunkFileNames,
    });
    const viteSources = packageRelativeViteSources(packageRoot, graph.moduleIds);
    assertTypeScriptAuthority(viteSources);

    copyAssets(packageRoot, spec.assets, stagedDir);
    emitDeclarations({
      repoRoot,
      packageRoot,
      tsconfigPath: spec.declarationTsconfig,
      outDir: stagedDir,
    });
    if (spec.extraDeclarationCopies?.length) {
      copyAssets(packageRoot, spec.extraDeclarationCopies, stagedDir);
    }

    const tools = readLockedTools(repoRoot);
    const receipt = writeReceipt({ repoRoot, packageRoot, spec, tools, distDir: stagedDir });
    assertReceiptCoversViteSources(receipt.inputs, viteSources);
    auditStagedDist({
      distDir: stagedDir,
      publicFiles,
      forbiddenModules: spec.forbiddenModules,
      moduleIds: graph.moduleIds,
      specifiers: graph.specifiers,
    });

    const outDir = publishStaging(packageRoot, stagedDir);
    return {
      packageDir: spec.packageDir,
      distDir: outDir,
      receipt,
      receiptPath: receiptPath(packageRoot),
    };
  } catch (error) {
    discardStaging(stagedDir);
    throw error;
  }
}

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { auditPackageDependencies } from "./audit";
import {
  CORE_FORBIDDEN_MODULES,
  CORE_PACKAGE_DIR,
  CORE_PACKAGE_NAME,
  assertCoreInventoriesMatchDisk,
  coreAssetCopies,
  coreLibraryEntries,
  corePackageExports,
  corePublicCssFiles,
  corePublicDeclarationFiles,
  corePublicJsFiles,
  readCorePackageVersion,
} from "./core-contract";
import { buildPackage } from "./driver";
import type { PackageBuildSpec } from "./types";

export function findRepoRoot(start: string = import.meta.dir): string {
  let current = start;
  while (true) {
    try {
      readFileSync(join(current, "bun.lock"));
      return current;
    } catch {
      const parent = dirname(current);
      if (parent === current) {
        throw new Error("could not find repository root from web-distribution driver");
      }
      current = parent;
    }
  }
}

export function coreBuildSpec(repoRoot: string): PackageBuildSpec {
  assertCoreInventoriesMatchDisk(repoRoot);
  return {
    packageDir: CORE_PACKAGE_DIR,
    packageName: CORE_PACKAGE_NAME,
    version: readCorePackageVersion(repoRoot),
    lanes: ["single"],
    cssPolicy: "core-owned",
    markdownPolicy: "none",
    entries: coreLibraryEntries(),
    assets: coreAssetCopies(),
    declarationTsconfig: "tsconfig.build.json",
    forbiddenModules: [...CORE_FORBIDDEN_MODULES],
  };
}

export function corePublicFiles(): string[] {
  return [...corePublicJsFiles(), ...corePublicCssFiles(), ...corePublicDeclarationFiles()];
}

export function assertCoreManifest(repoRoot: string): void {
  const manifest = JSON.parse(
    readFileSync(join(repoRoot, CORE_PACKAGE_DIR, "package.json"), "utf8"),
  ) as {
    bin?: { "poodle-icons"?: string };
    exports?: unknown;
    files?: string[];
    sideEffects?: unknown;
  } & Record<string, unknown>;
  const expectedExports = corePackageExports();
  if (JSON.stringify(manifest.exports) !== JSON.stringify(expectedExports)) {
    throw new Error("packages/core/package.json exports do not match spec 070");
  }
  if (manifest.bin?.["poodle-icons"] !== "./dist/icons/build.mjs") {
    throw new Error("bin.poodle-icons must target ./dist/icons/build.mjs");
  }
  if (JSON.stringify(manifest.files) !== JSON.stringify(["dist", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"])) {
    throw new Error("core files must be dist plus package docs and licences");
  }
  if (JSON.stringify(manifest.sideEffects) !== JSON.stringify(["**/*.css"])) {
    throw new Error('core sideEffects must be ["**/*.css"]');
  }
  auditPackageDependencies(manifest, CORE_FORBIDDEN_MODULES);
}

export async function buildCore(repoRoot: string = findRepoRoot()) {
  assertCoreManifest(repoRoot);
  return buildPackage(repoRoot, coreBuildSpec(repoRoot), corePublicFiles());
}

if (import.meta.main) {
  await buildCore();
}

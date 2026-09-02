import { readFileSync } from "node:fs";
import { join } from "node:path";

import { auditPackageDependencies, auditStagedDist } from "./audit";
import { buildCore, findRepoRoot } from "./core-build";
import { buildPackage } from "./driver";
import {
  REACT_EXTERNAL_MODULES,
  REACT_PACKAGE_DIR,
  REACT_PACKAGE_NAME,
  assertReactInventoriesMatchDisk,
  reactLibraryEntries,
  reactPackageExports,
  reactPublicFiles,
  readPackageVersion,
  shellFiles,
  shellSideEffects,
} from "./shell-contract";
import type { PackageBuildSpec } from "./types";

export function reactBuildSpec(repoRoot: string): PackageBuildSpec {
  assertReactInventoriesMatchDisk(repoRoot);
  return {
    packageDir: REACT_PACKAGE_DIR,
    packageName: REACT_PACKAGE_NAME,
    version: readPackageVersion(repoRoot, REACT_PACKAGE_DIR),
    lanes: ["single"],
    cssPolicy: "core-owned",
    markdownPolicy: "optional-peer-on-./markdown",
    entries: reactLibraryEntries(),
    assets: [],
    declarationTsconfig: "tsconfig.build.json",
    forbiddenModules: [],
    externalModules: [...REACT_EXTERNAL_MODULES],
  };
}

export function assertReactManifest(repoRoot: string): void {
  const manifest = JSON.parse(
    readFileSync(join(repoRoot, REACT_PACKAGE_DIR, "package.json"), "utf8"),
  ) as {
    private?: boolean;
    exports?: unknown;
    files?: string[];
    sideEffects?: unknown;
    publishConfig?: { access?: string };
    peerDependencies?: Record<string, string>;
    peerDependenciesMeta?: Record<string, { optional?: boolean }>;
    dependencies?: Record<string, string>;
  };
  if (manifest.private !== true) {
    throw new Error("React package must remain private");
  }
  if (manifest.publishConfig?.access === "public") {
    throw new Error("React package must not be public");
  }
  if (JSON.stringify(manifest.exports) !== JSON.stringify(reactPackageExports())) {
    throw new Error("packages/react/components/package.json exports do not match spec 070");
  }
  if (JSON.stringify(manifest.exports).includes('"browser"')) {
    throw new Error("React exports must not include a browser condition");
  }
  if (JSON.stringify(manifest.files) !== JSON.stringify(shellFiles())) {
    throw new Error("React files must be dist plus package docs");
  }
  if (JSON.stringify(manifest.sideEffects) !== JSON.stringify(shellSideEffects())) {
    throw new Error('React sideEffects must be ["**/*.css"]');
  }
  if (manifest.peerDependencies?.marked !== "^18.0.9") {
    throw new Error("marked must be an optional React peer");
  }
  if (manifest.peerDependenciesMeta?.marked?.optional !== true) {
    throw new Error("marked React peer must be optional");
  }
  if (manifest.dependencies?.marked) {
    throw new Error("marked must not be a hard React dependency");
  }
  auditPackageDependencies(manifest, ["svelte"]);
}

export async function buildReact(repoRoot: string = findRepoRoot()) {
  await buildCore(repoRoot);
  assertReactManifest(repoRoot);
  return buildPackage(repoRoot, reactBuildSpec(repoRoot), reactPublicFiles());
}

if (import.meta.main) {
  await buildReact();
}

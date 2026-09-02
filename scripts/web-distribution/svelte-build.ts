import { readFileSync } from "node:fs";
import { join } from "node:path";

import { svelte } from "@sveltejs/vite-plugin-svelte";

import { auditPackageDependencies, auditStagedDist } from "./audit";
import { buildCore, findRepoRoot } from "./core-build";
import { generateSvelteComponentDeclarations } from "./copy-svelte-declarations";
import { buildViteLibrary, type ViteLibraryGraph } from "./vite-library";
import { readLockedTools } from "./lockfile";
import {
  assertReceiptCoversViteSources,
  assertTypeScriptAuthority,
  packageRelativeViteSources,
  receiptPath,
  writeReceipt,
} from "./receipt";
import { cleanStaging } from "./staging";
import {
  SVELTE_EXTERNAL_MODULES,
  SVELTE_PACKAGE_DIR,
  SVELTE_PACKAGE_NAME,
  assertSvelteInventoriesMatchDisk,
  readPackageVersion,
  svelteDualEntries,
  sveltePackageExports,
  sveltePublicFiles,
  svelteTypesEntry,
  shellFiles,
  shellSideEffects,
} from "./shell-contract";
import type { BuiltPackage, PackageBuildSpec } from "./types";

function mergeGraphs(graphs: ViteLibraryGraph[]): ViteLibraryGraph {
  return {
    moduleIds: graphs.flatMap((graph) => graph.moduleIds),
    specifiers: graphs.flatMap((graph) => graph.specifiers),
  };
}

export function svelteBuildSpec(repoRoot: string): PackageBuildSpec {
  assertSvelteInventoriesMatchDisk(repoRoot);
  const types = svelteTypesEntry();
  const entries = [...svelteDualEntries(), types].sort((left, right) =>
    left.name < right.name ? -1 : left.name > right.name ? 1 : 0,
  );
  return {
    packageDir: SVELTE_PACKAGE_DIR,
    packageName: SVELTE_PACKAGE_NAME,
    version: readPackageVersion(repoRoot, SVELTE_PACKAGE_DIR),
    lanes: ["client", "server"],
    cssPolicy: "core-owned",
    markdownPolicy: "optional-peer-on-./markdown",
    entries,
    assets: [],
    declarationTsconfig: "tsconfig.declarations.json",
    forbiddenModules: [],
    externalModules: [...SVELTE_EXTERNAL_MODULES],
  };
}

export function assertSvelteManifest(repoRoot: string): void {
  const manifest = JSON.parse(
    readFileSync(join(repoRoot, SVELTE_PACKAGE_DIR, "package.json"), "utf8"),
  ) as {
    exports?: unknown;
    files?: string[];
    sideEffects?: unknown;
    svelte?: unknown;
    peerDependencies?: Record<string, string>;
    peerDependenciesMeta?: Record<string, { optional?: boolean }>;
    dependencies?: Record<string, string>;
  };
  if (JSON.stringify(manifest.exports) !== JSON.stringify(sveltePackageExports())) {
    throw new Error("packages/svelte/components/package.json exports do not match spec 070");
  }
  if ("svelte" in manifest) {
    throw new Error("Svelte package must not have a top-level svelte field");
  }
  if (JSON.stringify(manifest.exports).includes('"import"')) {
    throw new Error("Svelte exports must not include an import condition");
  }
  if (JSON.stringify(manifest.exports).includes('"svelte"')) {
    throw new Error("Svelte exports must not include a svelte condition");
  }
  if (JSON.stringify(manifest.files) !== JSON.stringify(shellFiles())) {
    throw new Error("Svelte files must be dist plus package docs");
  }
  if (JSON.stringify(manifest.sideEffects) !== JSON.stringify(shellSideEffects())) {
    throw new Error('Svelte sideEffects must be ["**/*.css"]');
  }
  if (manifest.peerDependencies?.svelte !== ">=5.56.8 <6") {
    throw new Error("Svelte peer floor must be >=5.56.8 <6");
  }
  if (manifest.peerDependencies?.marked !== "^18.0.9") {
    throw new Error("marked must be an optional Svelte peer");
  }
  if (manifest.peerDependenciesMeta?.marked?.optional !== true) {
    throw new Error("marked Svelte peer must be optional");
  }
  if (manifest.dependencies?.marked) {
    throw new Error("marked must not be a hard Svelte dependency");
  }
  auditPackageDependencies(manifest, ["react", "react-dom"]);
}

export async function buildSvelte(repoRoot: string = findRepoRoot()): Promise<BuiltPackage> {
  await buildCore(repoRoot);
  assertSvelteManifest(repoRoot);
  const spec = svelteBuildSpec(repoRoot);
  const packageRoot = join(repoRoot, spec.packageDir);
  const outDir = cleanStaging(packageRoot);
  const dual = svelteDualEntries();
  const types = svelteTypesEntry();
  const dualEntries: Record<string, string> = {};
  for (const entry of dual) dualEntries[entry.name] = join(packageRoot, entry.source);
  const clientEntries: Record<string, string> = {
    ...dualEntries,
    [types.name]: join(packageRoot, types.source),
  };
  const plugins = [
    svelte({
      compilerOptions: { css: "external" },
      onwarn: (warning, handler) => {
        if (warning.code.startsWith("a11y_")) return;
        handler(warning);
      },
    }),
  ];

  const client = await buildViteLibrary({
    root: packageRoot,
    outDir,
    entries: clientEntries,
    fileName: (entryName) => (entryName === "types" ? "types.js" : `${entryName}.client.js`),
    externals: spec.externalModules,
    plugins,
    chunkFileNames: "chunks/[name].client.js",
  });
  const server = await buildViteLibrary({
    root: packageRoot,
    outDir,
    entries: dualEntries,
    fileName: (entryName) => `${entryName}.server.js`,
    externals: spec.externalModules,
    plugins,
    ssr: true,
    chunkFileNames: "chunks/[name].server.js",
  });
  const graph = mergeGraphs([client, server]);
  const viteSources = packageRelativeViteSources(packageRoot, graph.moduleIds);
  assertTypeScriptAuthority(viteSources);

  generateSvelteComponentDeclarations(packageRoot);

  const tools = readLockedTools(repoRoot);
  const receipt = writeReceipt({ repoRoot, packageRoot, spec, tools });
  assertReceiptCoversViteSources(receipt.inputs, viteSources);
  auditStagedDist({
    distDir: outDir,
    publicFiles: sveltePublicFiles(),
    forbiddenModules: spec.forbiddenModules,
    moduleIds: graph.moduleIds,
    specifiers: graph.specifiers,
  });

  return {
    packageDir: spec.packageDir,
    distDir: outDir,
    receipt,
    receiptPath: receiptPath(packageRoot),
  };
}

if (import.meta.main) {
  await buildSvelte();
}

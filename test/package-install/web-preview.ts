import { createHash } from "node:crypto";
import {
  cpSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  realpathSync,
} from "node:fs";
import { basename, join, resolve } from "node:path";

import {
  FROZEN_COMPONENT_COUNT,
  buildWebPackageRoster,
  readWebPackageRoster,
  type WebPackageRoster,
} from "./roster";

const repoRoot = resolve(import.meta.dir, "../..");
const artifactRoot = join(repoRoot, ".artifacts");
mkdirSync(artifactRoot, { recursive: true });
const runRoot = mkdtempSync(join(artifactRoot, "web-pack-install-"));
const packRoot = join(runRoot, "packs");
const consumerRoot = join(runRoot, "consumer");
mkdirSync(packRoot);
mkdirSync(consumerRoot);

type PackageManifest = {
  name: string;
  version: string;
  files?: string[];
  exports?: unknown;
};

const packages = [
  {
    name: "@inflatable-cookie/poodle-core",
    directory: "packages/core",
    requiredFiles: ["package.json", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"],
  },
  {
    name: "@inflatable-cookie/poodle-svelte",
    directory: "packages/svelte/components",
    requiredFiles: ["package.json", "README.md", "LICENSE"],
  },
  {
    name: "@inflatable-cookie/poodle-react",
    directory: "packages/react/components",
    requiredFiles: ["package.json", "README.md", "LICENSE"],
  },
] as const;

type PackageEntry = (typeof packages)[number];

type PackedPackage = PackageEntry & {
  manifest: PackageManifest;
  archivePath: string;
};

type PackedBoundaryEvidence = {
  archiveFileCount: number;
  requiredFiles: string[];
  declaredExportTargets: string[];
  wildcardExportMatches: Record<string, number>;
};

async function run(command: string[], cwd: string): Promise<void> {
  const process = Bun.spawn(command, {
    cwd,
    stdout: "inherit",
    stderr: "inherit",
  });
  const exitCode = await process.exited;
  if (exitCode !== 0) {
    throw new Error(`Command failed (${exitCode}): ${command.join(" ")}`);
  }
}

async function runCapture(command: string[], cwd: string): Promise<string> {
  const process = Bun.spawn(command, {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(process.stdout).text(),
    new Response(process.stderr).text(),
    process.exited,
  ]);
  if (exitCode !== 0) {
    throw new Error(
      `Command failed (${exitCode}): ${command.join(" ")}\n${stderr.trim()}`,
    );
  }
  return stdout;
}

function archivePathFromPackOutput(
  packageEntry: PackageEntry,
  output: string,
): string {
  const archivePath = output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.endsWith(".tgz"))
    .at(-1);
  if (!archivePath) {
    throw new Error(
      `${packageEntry.name} pack did not report a .tgz archive path:\n${output.trim()}`,
    );
  }
  return resolve(join(repoRoot, packageEntry.directory), archivePath);
}

function normalizeArchiveEntry(entry: string): string {
  return entry.replace(/\/$/, "");
}

function normalizePackagePath(path: string): string {
  return path.replace(/^\.\//, "");
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function archiveMatches(
  archiveEntries: string[],
  packagePath: string,
): string[] {
  const pattern = new RegExp(
    `^package/${packagePath
      .split("*")
      .map(escapeRegExp)
      .join("[^/]+")}$`,
  );
  return archiveEntries.filter((entry) => pattern.test(entry));
}

function collectExportTargets(value: unknown): string[] {
  if (typeof value === "string") return [normalizePackagePath(value)];
  if (Array.isArray(value)) return value.flatMap(collectExportTargets);
  if (value && typeof value === "object") {
    return Object.values(value).flatMap(collectExportTargets);
  }
  return [];
}

function inspectPackedBoundary(
  packageEntry: PackageEntry,
  manifest: PackageManifest,
  archiveEntries: string[],
): PackedBoundaryEvidence {
  const requiredFiles = [
    ...packageEntry.requiredFiles,
    ...(manifest.files ?? []),
  ];
  const missingRequiredFiles = requiredFiles.filter((file) => {
    const normalized = normalizePackagePath(file);
    if (normalized.includes("*")) {
      return archiveMatches(archiveEntries, normalized).length === 0;
    }
    if (normalized === "src") {
      return !archiveEntries.some((entry) => entry.startsWith("package/src/"));
    }
    return !archiveEntries.includes(`package/${normalized}`);
  });
  if (missingRequiredFiles.length > 0) {
    throw new Error(
      `${packageEntry.name} tarball omitted required file(s): ${missingRequiredFiles.join(", ")}`,
    );
  }

  const declaredExportTargets = sortedUnique(
    collectExportTargets(manifest.exports),
  );
  const missingExportTargets = declaredExportTargets.filter((target) => {
    if (target.includes("*")) {
      return archiveMatches(archiveEntries, target).length === 0;
    }
    return !archiveEntries.includes(`package/${target}`);
  });
  if (missingExportTargets.length > 0) {
    throw new Error(
      `${packageEntry.name} tarball omitted declared export target(s): ${missingExportTargets.join(", ")}`,
    );
  }

  const wildcardExportMatches = Object.fromEntries(
    declaredExportTargets
      .filter((target) => target.includes("*"))
      .map((target) => [target, archiveMatches(archiveEntries, target).length]),
  );
  return {
    archiveFileCount: archiveEntries.length,
    requiredFiles: sortedUnique(requiredFiles),
    declaredExportTargets,
    wildcardExportMatches,
  };
}

function importProofTest(
  roster: WebPackageRoster,
  coreSubpaths: string[],
): string {
  const frameworkProof = (
    framework: "svelte" | "react",
    rootIdentifier: "svelteRoot" | "reactRoot",
  ) => {
    const frameworkRoster = roster[framework];
    return `
  it("imports the exact frozen ${framework} component roster from the packed root", () => {
    const expectedComponents = ${JSON.stringify(frameworkRoster.componentNames)};
    const expectedRootExports = ${JSON.stringify(frameworkRoster.rootRuntimeNames)};
    const nonComponentRootExports = new Set(${JSON.stringify(frameworkRoster.nonComponentRootNames)});
    const actualRootExports = Object.keys(${rootIdentifier}).sort();
    const missingComponents = expectedComponents.filter((name) => !(name in ${rootIdentifier}));
    const actualComponents = actualRootExports.filter((name) => !nonComponentRootExports.has(name));
    const extraComponents = actualComponents.filter((name) => !expectedComponents.includes(name));
    const missingRootExports = expectedRootExports.filter((name) => !actualRootExports.includes(name));
    const extraRootExports = actualRootExports.filter((name) => !expectedRootExports.includes(name));

    expect(expectedComponents).toHaveLength(${FROZEN_COMPONENT_COUNT});
    expect({ missingComponents, extraComponents }).toEqual({
      missingComponents: [],
      extraComponents: [],
    });
    expect({ missingRootExports, extraRootExports }).toEqual({
      missingRootExports: [],
      extraRootExports: [],
    });
  });`;
  };

  const publicImportSpecifiers = [
    ...coreSubpaths.map((subpath) =>
      subpath
        ? `@inflatable-cookie/poodle-core/${subpath}`
        : "@inflatable-cookie/poodle-core",
    ),
    "@inflatable-cookie/poodle-svelte/types",
  ];
  const coreImportChecks = publicImportSpecifiers
    .map((specifier) => {
      return `    [${JSON.stringify(specifier)}, () => import(${JSON.stringify(specifier)})],`;
    })
    .join("\n");

  return `import { describe, expect, it } from "vitest";
import * as svelteRoot from "@inflatable-cookie/poodle-svelte";
import * as reactRoot from "@inflatable-cookie/poodle-react";

  describe("packed public root reachability", () => {${frameworkProof("svelte", "svelteRoot")}${frameworkProof("react", "reactRoot")}

  it("resolves every declared exact core public subpath from the packed root", async () => {
    const checks = [
${coreImportChecks}
    ] as const;
    const failures: string[] = [];
    for (const [specifier, load] of checks) {
      try {
        await load();
      } catch (error) {
        failures.push(specifier + ": " + String(error));
      }
    }
    expect(failures).toEqual([]);
  });
});
`;
}

/**
 * The packed v3 `HistoryEntry` type proof (g16.033).
 *
 * `test:components` imports `../src/types`, so a source-only correction looks
 * identical to a corrected package. This compiles a real consumer against the
 * installed tarballs on both public Svelte import paths: `continuationCount`
 * must typecheck, and the retired v2 `branchCount` must fail — unsuppressed,
 * with the exact diagnostic named here rather than "the compiler said no".
 */
const PACKED_TYPE_PROOF_DIR = "packed-types";
const PACKED_TYPE_PROOF_DIAGNOSTIC =
  "error TS2339: Property 'branchCount' does not exist on type 'HistoryEntry'.";
/** Anything that would turn an expected failure into a silent pass. */
const SUPPRESSION_MARKERS = ["@ts-expect-error", "@ts-ignore", "as any", ": any"];

type PackedTypeCompile = {
  config: string;
  exitCode: number;
  output: string;
};

async function runTypeCompile(
  consumerRoot: string,
  compiler: string,
  config: string,
): Promise<PackedTypeCompile> {
  const process = Bun.spawn([compiler, "--project", join(PACKED_TYPE_PROOF_DIR, config)], {
    cwd: consumerRoot,
    stdout: "pipe",
    stderr: "pipe",
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(process.stdout).text(),
    new Response(process.stderr).text(),
    process.exited,
  ]);
  return { config, exitCode, output: `${stdout}${stderr}`.trim() };
}

function assertUnsuppressed(consumerRoot: string, file: string): void {
  // Raw source, comments included: `@ts-expect-error` IS a comment, so a guard
  // that strips comments first can never see the thing it exists to catch.
  // The fixture banners are written to avoid these literals for that reason.
  const source = readFileSync(join(consumerRoot, PACKED_TYPE_PROOF_DIR, file), "utf8");
  const found = SUPPRESSION_MARKERS.filter((marker) => source.includes(marker));
  if (found.length > 0) {
    throw new Error(
      `${file} suppresses its expected failure (${found.join(", ")}); the negative proof would pass for the wrong reason`,
    );
  }
}

async function provePackedHistoryEntryTypes(
  consumerRoot: string,
): Promise<Record<string, unknown>> {
  const compiler = join(consumerRoot, "node_modules", ".bin", "tsc");
  if (!existsSync(compiler)) {
    throw new Error(
      "the packed consumer did not install a TypeScript compiler; the v3 type proof cannot run",
    );
  }

  const positive = await runTypeCompile(consumerRoot, compiler, "tsconfig.positive.json");
  if (positive.exitCode !== 0 || positive.output.length > 0) {
    throw new Error(
      `packed HistoryEntry positive proof failed on the installed tarball:\n${positive.output}`,
    );
  }

  const negatives = [
    {
      importPath: "@inflatable-cookie/poodle-svelte",
      file: "history-entry-root-negative.ts",
      config: "tsconfig.root-negative.json",
    },
    {
      importPath: "@inflatable-cookie/poodle-svelte/types",
      file: "history-entry-types-negative.ts",
      config: "tsconfig.types-negative.json",
    },
  ] as const;

  const negativeEvidence = [];
  for (const negative of negatives) {
    assertUnsuppressed(consumerRoot, negative.file);
    const compile = await runTypeCompile(consumerRoot, compiler, negative.config);
    if (compile.exitCode === 0) {
      throw new Error(
        `packed ${negative.importPath} still accepts the retired v2 branchCount field`,
      );
    }
    if (!compile.output.includes(PACKED_TYPE_PROOF_DIAGNOSTIC)) {
      throw new Error(
        `packed ${negative.importPath} rejected branchCount with the wrong diagnostic:\n${compile.output}`,
      );
    }
    if (!compile.output.includes(negative.file)) {
      throw new Error(
        `packed ${negative.importPath} reported its diagnostic against another file:\n${compile.output}`,
      );
    }
    negativeEvidence.push({
      importPath: negative.importPath,
      file: negative.file,
      exitCode: compile.exitCode,
      diagnostic: compile.output,
      suppressed: false,
    });
  }

  return {
    compiler: realpathSync(compiler),
    importPaths: negatives.map((negative) => negative.importPath),
    positive: {
      file: "history-entry-positive.ts",
      exitCode: positive.exitCode,
      diagnostics: [],
    },
    expectedFailures: negativeEvidence,
    sourceImports: false,
    workspaceAliases: false,
    declarationTextSubstitute: false,
  };
}

const PACKED_TREE_REORDER_DIAGNOSTIC = "Types of property 'onReorder' are incompatible.";
const REACT_PUBLIC_SPECIFIER = "@inflatable-cookie/poodle-react";
const REACT_ROOT_RESOLVE_CONFIG = "tsconfig.tree-react-root-resolve.json";
const REACT_ROOT_RESOLVE_FILE = "tree-reorder-react-root-resolve.ts";
const REACT_ROOT_TYPE_EXPORTS = [
  "TreeProps",
  "TreeReorderSubject",
  "TreeReorderCandidate",
  "TreeReorderAuthority",
  "TreeReorderProps",
] as const;

async function proveInstalledReactPublicRoot(
  consumerRoot: string,
  compiler: string,
): Promise<Record<string, unknown>> {
  const packageRoot = realpathSync(
    join(consumerRoot, "node_modules", "@inflatable-cookie", "poodle-react"),
  );
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8")) as {
    exports?: { "."?: { types?: string; default?: string } };
  };
  const typesEntry = manifest.exports?.["."]?.types;
  if (typesEntry !== "./src/index.ts") {
    throw new Error(
      `installed React package types export is ${JSON.stringify(typesEntry)}, not ./src/index.ts`,
    );
  }
  const indexPath = realpathSync(join(packageRoot, "src", "index.ts"));
  const typesPath = realpathSync(join(packageRoot, "src", "types.ts"));
  const process = Bun.spawn(
    [
      compiler,
      "--project",
      join(PACKED_TYPE_PROOF_DIR, REACT_ROOT_RESOLVE_CONFIG),
      "--traceResolution",
      "--pretty",
      "false",
      "--noEmit",
    ],
    { cwd: consumerRoot, stdout: "pipe", stderr: "pipe" },
  );
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(process.stdout).text(),
    new Response(process.stderr).text(),
    process.exited,
  ]);
  const trace = `${stdout}\n${stderr}`;
  const fromProbe = `Resolving module '${REACT_PUBLIC_SPECIFIER}' from '${join(
    consumerRoot,
    PACKED_TYPE_PROOF_DIR,
    REACT_ROOT_RESOLVE_FILE,
  )}'`;
  if (!trace.includes(fromProbe)) {
    throw new Error(
      `TypeScript did not resolve ${REACT_PUBLIC_SPECIFIER} from ${REACT_ROOT_RESOLVE_FILE} with no paths map`,
    );
  }
  const resolvedMarker = `Module name '${REACT_PUBLIC_SPECIFIER}' was successfully resolved to '`;
  const resolvedStart = trace.indexOf(resolvedMarker);
  if (resolvedStart < 0) {
    throw new Error(
      `TypeScript did not report a successful resolution for ${REACT_PUBLIC_SPECIFIER}`,
    );
  }
  const resolvedFrom = resolvedStart + resolvedMarker.length;
  const resolvedFile = trace.slice(resolvedFrom, trace.indexOf("'", resolvedFrom));
  if (realpathSync(resolvedFile) !== indexPath) {
    throw new Error(
      `public specifier resolved to ${resolvedFile}, not the installed types entry ${indexPath}`,
    );
  }
  if (realpathSync(resolvedFile) === typesPath) {
    throw new Error(
      "public specifier resolved through a types.ts paths bypass, not package exports",
    );
  }
  const indexSource = readFileSync(indexPath, "utf8");
  if (!indexSource.includes('export { Tree } from "./Tree"')) {
    throw new Error('installed React root omitted `export { Tree } from "./Tree"`');
  }
  if (!indexSource.includes('export * from "./types"')) {
    throw new Error('installed React root omitted `export * from "./types"`');
  }
  const typesSource = readFileSync(typesPath, "utf8");
  for (const name of REACT_ROOT_TYPE_EXPORTS) {
    if (!typesSource.includes(name)) {
      throw new Error(`installed React types module omitted ${name}`);
    }
  }
  if (!typesSource.includes("export type TreeProps")) {
    throw new Error("installed React types module omitted exported TreeProps");
  }
  return {
    specifier: REACT_PUBLIC_SPECIFIER,
    packageTypesExport: typesEntry,
    compilerResolvedFile: resolvedFile,
    resolvedVia: "package.json exports types + tsc --traceResolution (no paths)",
    resolveConfig: REACT_ROOT_RESOLVE_CONFIG,
    compilerPathsMapped: false,
    valueBarrelCompileExitCode: exitCode,
    reexportsTree: true,
    reexportsTypesModule: true,
    typesModuleExports: [...REACT_ROOT_TYPE_EXPORTS],
    valueBarrelCompiled: false,
  };
}

async function provePackedTreeReorderTypes(
  consumerRoot: string,
): Promise<Record<string, unknown>> {
  const compiler = join(consumerRoot, "node_modules", ".bin", "tsc");
  if (!existsSync(compiler)) {
    throw new Error(
      "the packed consumer did not install a TypeScript compiler; the Tree reorder type proof cannot run",
    );
  }

  const installedReactTypes = join(
    consumerRoot,
    "node_modules",
    "@inflatable-cookie",
    "poodle-react",
    "src",
    "types.ts",
  );
  if (!existsSync(installedReactTypes)) {
    throw new Error(
      "the packed React tarball omitted src/types.ts; the Tree reorder React type proof cannot run",
    );
  }

  const publicPositives = [
    { file: "tree-reorder-positive.ts", config: "tsconfig.tree-positive.json" },
  ] as const;
  const mappedReactPositives = [
    { file: "tree-reorder-react-positive.ts", config: "tsconfig.tree-react-positive.json" },
  ] as const;
  for (const item of [...publicPositives, ...mappedReactPositives]) {
    const compile = await runTypeCompile(consumerRoot, compiler, item.config);
    if (compile.exitCode !== 0 || compile.output.length > 0) {
      throw new Error(
        `packed Tree reorder positive proof failed on the installed tarball (${item.file}):\n${compile.output}`,
      );
    }
  }

  const publicNegatives = [
    {
      importPath: "@inflatable-cookie/poodle-svelte",
      file: "tree-reorder-root-negative.ts",
      config: "tsconfig.tree-root-negative.json",
    },
    {
      importPath: "@inflatable-cookie/poodle-svelte/types",
      file: "tree-reorder-types-negative.ts",
      config: "tsconfig.tree-types-negative.json",
    },
    {
      importPath: "@inflatable-cookie/poodle-core",
      file: "tree-reorder-core-negative.ts",
      config: "tsconfig.tree-core-negative.json",
    },
  ] as const;
  const mappedReactNegatives = [
    {
      importPath: REACT_PUBLIC_SPECIFIER,
      file: "tree-reorder-react-negative.tsx",
      config: "tsconfig.tree-react-negative.json",
    },
  ] as const;

  const negativeEvidence = [];
  for (const negative of [...publicNegatives, ...mappedReactNegatives]) {
    assertUnsuppressed(consumerRoot, negative.file);
    const compile = await runTypeCompile(consumerRoot, compiler, negative.config);
    if (compile.exitCode === 0) {
      throw new Error(
        `packed ${negative.importPath} still accepts reorderAuthority together with onReorder`,
      );
    }
    if (!compile.output.includes(PACKED_TREE_REORDER_DIAGNOSTIC)) {
      throw new Error(
        `packed ${negative.importPath} rejected the exclusive union with the wrong diagnostic:\n${compile.output}`,
      );
    }
    if (!compile.output.includes(negative.file)) {
      throw new Error(
        `packed ${negative.importPath} reported its diagnostic against another file:\n${compile.output}`,
      );
    }
    negativeEvidence.push({
      importPath: negative.importPath,
      file: negative.file,
      exitCode: compile.exitCode,
      diagnostic: compile.output,
      suppressed: false,
      compilerPathsMapped: negative.file.includes("react"),
    });
  }

  return {
    compiler: realpathSync(compiler),
    importPaths: [...publicNegatives, ...mappedReactNegatives].map((negative) => negative.importPath),
    publicSpecifierCompile: {
      files: publicPositives.map((item) => item.file),
      sourceImports: false,
      workspaceAliases: false,
      compilerPathsMapped: false,
    },
    reactMappedAssignability: {
      files: [
        ...mappedReactPositives.map((item) => item.file),
        ...mappedReactNegatives.map((item) => item.file),
      ],
      pathsMappedTo: "src/types.ts",
      reason: "src/index.ts value barrel is not tsc-clean",
      compilerPathsMapped: true,
      valueBarrelCompiled: false,
      sourceImports: false,
      workspaceAliases: false,
    },
    reactPublicRoot: await proveInstalledReactPublicRoot(consumerRoot, compiler),
    expectedFailures: negativeEvidence,
    sourceImports: false,
    workspaceAliases: false,
    declarationTextSubstitute: false,
    compilerPathsMapped: true,
    valueBarrelCompiled: false,
  };
}

function sortedUnique(values: Iterable<string>): string[] {
  return [...new Set(values)].sort();
}

function assertReactExtraExportRegression(
  repoRoot: string,
  roster: WebPackageRoster,
): { exportName: string; rejected: true } {
  const extraExportName = "AccidentalExtraComponent";
  const svelteSource = readFileSync(
    join(repoRoot, "packages/svelte/components/src/index.ts"),
    "utf8",
  );
  const reactSource = readFileSync(
    join(repoRoot, "packages/react/components/src/index.ts"),
    "utf8",
  );

  try {
    buildWebPackageRoster(
      roster.frozenNames,
      svelteSource,
      `${reactSource}\nexport { Button as ${extraExportName} } from "./Button";\n`,
    );
  } catch (error) {
    const message = String(error);
    if (!message.includes(`\"extra\":[\"${extraExportName}\"]`)) {
      throw new Error(
        `React extra-export regression failed with the wrong error: ${message}`,
      );
    }
    return { exportName: extraExportName, rejected: true };
  }

  throw new Error(
    `React extra-export regression accepted ${extraExportName}`,
  );
}

const roster = readWebPackageRoster(repoRoot);
const rosterRegression = assertReactExtraExportRegression(repoRoot, roster);
const packageManifests = new Map<string, PackageManifest>();
const packedPackages: PackedPackage[] = [];

for (const packageEntry of packages) {
  const manifest = JSON.parse(
    readFileSync(
      join(repoRoot, packageEntry.directory, "package.json"),
      "utf8",
    ),
  ) as PackageManifest;
  if (
    manifest.name !== packageEntry.name ||
    typeof manifest.version !== "string" ||
    manifest.version.length === 0
  ) {
    throw new Error(
      `${packageEntry.directory} must declare ${packageEntry.name} and a non-empty version`,
    );
  }
  packageManifests.set(packageEntry.name, manifest);
  const packOutput = await runCapture(
    ["bun", "pm", "pack", "--destination", packRoot, "--quiet"],
    join(repoRoot, packageEntry.directory),
  );
  packedPackages.push({
    ...packageEntry,
    manifest,
    archivePath: archivePathFromPackOutput(packageEntry, packOutput),
  });
}

const packedBoundaries = new Map<string, PackedBoundaryEvidence>();
for (const packedPackage of packedPackages) {
  const archiveEntries = (
    await runCapture(["tar", "-tzf", packedPackage.archivePath], repoRoot)
  )
    .split("\n")
    .filter(Boolean)
    .map(normalizeArchiveEntry);
  const manifest = packageManifests.get(packedPackage.name);
  if (!manifest) throw new Error(`Missing manifest for ${packedPackage.name}`);
  packedBoundaries.set(
    packedPackage.name,
    inspectPackedBoundary(packedPackage, manifest, archiveEntries),
  );
}

const tarballDependencies = Object.fromEntries(
  packedPackages.map((packedPackage) => [
    packedPackage.name,
    `file:${packedPackage.archivePath}`,
  ]),
);
const consumerManifest = {
  name: "@inflatable-cookie/poodle-packed-install-proof",
  private: true,
  type: "module",
  dependencies: {
    ...tarballDependencies,
    react: "18.0.0",
    "react-dom": "18.0.0",
    svelte: "5.38.6",
  },
  overrides: tarballDependencies,
  devDependencies: {
    "@sveltejs/vite-plugin-svelte": "6.2.1",
    "@testing-library/react": "16.3.0",
    "@testing-library/svelte": "5.4.2",
    "@types/react": "18.3.18",
    "@types/react-dom": "18.3.5",
    "happy-dom": "20.11.1",
    typescript: "7.0.2",
    vite: "7.3.1",
    vitest: "4.1.10",
  },
};
await Bun.write(
  join(consumerRoot, "package.json"),
  `${JSON.stringify(consumerManifest, null, 2)}\n`,
);
cpSync(join(import.meta.dir, "fixture"), consumerRoot, { recursive: true });

const coreManifest = packageManifests.get("@inflatable-cookie/poodle-core");
if (!coreManifest || !coreManifest.exports || typeof coreManifest.exports !== "object") {
  throw new Error("Core package manifest must declare its public exports");
}
const coreSubpaths = Object.keys(coreManifest.exports)
  .filter((subpath) => subpath !== "./styles/*" && subpath !== "./icons/*")
  .map((subpath) => (subpath === "." ? "" : subpath.replace(/^\.\//, "")));
await Bun.write(
  join(consumerRoot, "PackedRosterReachability.test.ts"),
  importProofTest(roster, coreSubpaths),
);

await run(["bun", "install", "--ignore-scripts"], consumerRoot);

for (const packageEntry of packages) {
  const installedManifestPath = join(
    consumerRoot,
    "node_modules",
    ...packageEntry.name.split("/"),
    "package.json",
  );
  const installedRoot = realpathSync(resolve(installedManifestPath, ".."));
  const sourceRoot = realpathSync(join(repoRoot, packageEntry.directory));
  if (
    installedRoot === sourceRoot ||
    installedRoot.startsWith(`${sourceRoot}/`)
  ) {
    throw new Error(
      `${packageEntry.name} resolved to sibling source: ${installedRoot}`,
    );
  }
  const installedManifest = readFileSync(installedManifestPath, "utf8");
  if (installedManifest.includes("workspace:")) {
    throw new Error(
      `${packageEntry.name} retained a workspace dependency in its tarball`,
    );
  }
  for (const requiredFile of packageEntry.requiredFiles) {
    if (!existsSync(join(installedRoot, requiredFile))) {
      throw new Error(
        `${packageEntry.name} omitted required package file ${requiredFile}`,
      );
    }
  }
}

await run(["bunx", "vitest", "run"], consumerRoot);

const packedHistoryEntryProof = await provePackedHistoryEntryTypes(consumerRoot);
const packedTreeReorderProof = await provePackedTreeReorderTypes(consumerRoot);

const artifacts = await Promise.all(
  packedPackages.map(async (packedPackage) => {
    const path = packedPackage.archivePath;
    const bytes = new Uint8Array(await Bun.file(path).arrayBuffer());
    return {
      name: packedPackage.name,
      version: packedPackage.manifest.version,
      filename: basename(path),
      path,
      bytes: bytes.byteLength,
      sha256: createHash("sha256").update(bytes).digest("hex"),
    };
  }),
);
const artifactSetId = createHash("sha256")
  .update(
    artifacts
      .map((artifact) => `${artifact.name}:${artifact.sha256}`)
      .join("\n"),
  )
  .digest("hex");
const evidence = {
  schema: "poodle.web-preview-pack-install.v1",
  artifactSetId,
  generatedAt: new Date().toISOString(),
  frameworkFloors: {
    react: "18.0.0",
    svelte: "5.38.6",
  },
  peerRanges: {
    react: ">=18",
    svelte: ">=5.38.6 <6",
  },
  consumerRoot,
  constraints: {
    viteAliases: false,
    siblingSourceResolution: false,
    workspaceDependencies: false,
    privateDomSelectors: false,
    privateMimeKnowledge: false,
  },
  rosterProof: {
    denominator: 175,
    sourceComponentCounts: {
      svelte: roster.svelte.componentNames.length,
      react: roster.react.componentNames.length,
    },
    installedRootExactExportSets: true,
    extraReactExportRegression: rosterRegression,
    coreExactPublicSubpaths: coreSubpaths,
    svelteExactPublicSubpaths: ["types"],
  },
  packedTarballs: Object.fromEntries(packedBoundaries),
  packedHistoryEntryProof,
  packedTreeReorderProof,
  mountedProof: {
    svelte: {
      components: [
        "DockRegion",
        "LicenceStatus",
        "LicenceActivation",
        "LicenceSeats",
        "ModelConnectionPicker",
        "ModelConnectionSetup",
        "ModelConnectionCard",
        "ModelCatalogueEditor",
        "MeterSurface",
      ],
      publicImportsOnly: true,
      licenceStyleSubpath: true,
      modelConnectionStyleSubpath: true,
      externalPayload: true,
      sameRegionReorder: true,
      accessibleRegionName: true,
      overlayGeometry: true,
    },
    react: {
      components: [
        "Button",
        "Icon",
        "IconProvider",
        "LicenceStatus",
        "LicenceActivation",
        "LicenceSeats",
        "ModelConnectionPicker",
        "ModelConnectionSetup",
        "ModelConnectionCard",
        "ModelCatalogueEditor",
        "MeterSurface",
        "AgentPlan",
        "AgentPlanRecord",
      ],
      publicImportsOnly: true,
      licenceStyleSubpath: true,
      modelConnectionStyleSubpath: true,
      scopedDefaultIcons: true,
      clickHandler: true,
    },
  },
  artifacts,
};
const evidencePath = join(runRoot, "evidence.json");
await Bun.write(evidencePath, `${JSON.stringify(evidence, null, 2)}\n`);

console.log(JSON.stringify({ evidencePath, ...evidence }, null, 2));

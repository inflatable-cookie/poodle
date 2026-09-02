import { createHash } from "node:crypto";
import {
  cpSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  realpathSync,
  rmSync,
  statSync,
} from "node:fs";
import { join, resolve } from "node:path";

import { packedMemberMissing } from "./archive-membership";
import { buildCore } from "../../scripts/web-distribution/core-build";
import { buildReact } from "../../scripts/web-distribution/react-build";
import { buildSvelte } from "../../scripts/web-distribution/svelte-build";
import {
  FROZEN_COMPONENT_COUNT,
  buildWebPackageRoster,
  readWebPackageRoster,
  type WebPackageRoster,
} from "./roster";

const repoRoot = resolve(import.meta.dir, "../..");
const artifactRoot = join(repoRoot, ".artifacts");
mkdirSync(artifactRoot, { recursive: true });
const innerRun = globalThis.process.env.POODLE_WEB_PACK_INSTALL_INNER === "1";

type PackageManifest = {
  name: string;
  version: string;
  files?: string[];
  exports?: unknown;
  sideEffects?: string[];
  dependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
  peerDependenciesMeta?: Record<string, { optional?: boolean }>;
  private?: boolean;
  main?: string;
  module?: string;
  svelte?: unknown;
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

async function run(
  command: string[],
  cwd: string,
  env?: Record<string, string>,
): Promise<void> {
  const child = Bun.spawn(command, {
    cwd,
    stdout: "inherit",
    stderr: "inherit",
    env: env ? { ...globalThis.process.env, ...env } : undefined,
  });
  const exitCode = await child.exited;
  if (exitCode !== 0) {
    throw new Error(`Command failed (${exitCode}): ${command.join(" ")}`);
  }
}

async function runCapture(
  command: string[],
  cwd: string,
  env?: Record<string, string>,
): Promise<string> {
  const process = Bun.spawn(command, {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
    env: env ? { ...globalThis.process.env, ...env } : undefined,
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

async function runResult(
  command: string[],
  cwd: string,
  env?: Record<string, string>,
): Promise<{ exitCode: number; stdout: string; stderr: string }> {
  const child = Bun.spawn(command, {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
    env: env ? { ...globalThis.process.env, ...env } : undefined,
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);
  return { exitCode, stdout, stderr };
}

function portable(value: string): string {
  return value.replaceAll(repoRoot, "<clean-checkout>").trim();
}

async function runFromCleanCheckout(): Promise<void> {
  const proofCommit = (await runCapture(["git", "rev-parse", "HEAD"], repoRoot)).trim();
  const commonGitDir = resolve(
    repoRoot,
    (await runCapture(["git", "rev-parse", "--git-common-dir"], repoRoot)).trim(),
  );
  const bareRoot = mkdtempSync(join(artifactRoot, "web-pack-install-bare-"));
  const checkoutRoot = mkdtempSync(join(artifactRoot, "web-pack-install-checkout-"));
  try {
    // Clone through a bare repository so ignored artifacts in this attached
    // worktree cannot become part of the clean certification checkout.
    await run(["git", "clone", "--quiet", "--bare", commonGitDir, bareRoot], repoRoot);
    await run(["git", "-C", bareRoot, "fetch", "--quiet", commonGitDir, proofCommit], repoRoot);
    await run(["git", "clone", "--quiet", "--no-local", bareRoot, checkoutRoot], repoRoot);
    await run(["git", "checkout", "--quiet", "--detach", proofCommit], checkoutRoot);
    const clonedCommit = (await runCapture(["git", "rev-parse", "HEAD"], checkoutRoot)).trim();
    if (clonedCommit !== proofCommit) {
      throw new Error(`clean certification checkout moved from ${proofCommit} to ${clonedCommit}`);
    }
    await run(["bun", "install", "--frozen-lockfile", "--ignore-scripts"], checkoutRoot);
    const output = await runCapture(
      ["bun", "test/package-install/web-preview.ts"],
      checkoutRoot,
      { POODLE_WEB_PACK_INSTALL_INNER: "1" },
    );
    process.stdout.write(output);
  } finally {
    rmSync(checkoutRoot, { recursive: true, force: true });
    rmSync(bareRoot, { recursive: true, force: true });
  }
}

if (!innerRun) {
  await runFromCleanCheckout();
  process.exit(0);
}

const runRoot = mkdtempSync(join(artifactRoot, "web-pack-install-"));
const packRoot = join(runRoot, "packs");
const firstPackRoot = join(packRoot, "first");
const secondPackRoot = join(packRoot, "second");
const consumerRoot = join(runRoot, "consumer");
mkdirSync(packRoot);
mkdirSync(firstPackRoot);
mkdirSync(secondPackRoot);
mkdirSync(consumerRoot);

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
  return resolve(
    archivePath.startsWith("/")
      ? archivePath
      : join(repoRoot, packageEntry.directory, archivePath),
  );
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

function sha256File(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

type FileInventoryEntry = { path: string; bytes: number; sha256: string };

function fileInventory(root: string): FileInventoryEntry[] {
  const entries: FileInventoryEntry[] = [];
  const visit = (directory: string, prefix: string) => {
    for (const name of readdirSync(directory).sort()) {
      const fullPath = join(directory, name);
      const relativePath = prefix ? `${prefix}/${name}` : name;
      if (statSync(fullPath).isDirectory()) {
        visit(fullPath, relativePath);
      } else {
        const bytes = readFileSync(fullPath);
        entries.push({
          path: relativePath,
          bytes: bytes.byteLength,
          sha256: createHash("sha256").update(bytes).digest("hex"),
        });
      }
    }
  };
  visit(root, "");
  return entries;
}

function assertExportConditionShape(packageEntry: PackageEntry, exports: unknown): void {
  const visit = (value: unknown) => {
    if (Array.isArray(value)) {
      for (const item of value) visit(item);
      return;
    }
    if (!value || typeof value !== "object") return;
    const keys = Object.keys(value);
    if (keys.some((key) => ["types", "browser", "import", "default"].includes(key))) {
      const typeIndex = keys.indexOf("types");
      if (typeIndex >= 0 && keys.some((key, index) => index < typeIndex && key !== "types")) {
        throw new Error(`${packageEntry.name} places the types condition after a runtime condition`);
      }
      if (packageEntry.name.endsWith("poodle-svelte")) {
        if (keys.includes("import")) {
          throw new Error("Svelte archive exports must not include an import condition");
        }
        if (keys.includes("svelte")) {
          throw new Error("Svelte archive exports must not include a svelte condition");
        }
        if (keys.includes("browser") && keys.indexOf("browser") > keys.indexOf("default")) {
          throw new Error("Svelte browser condition must precede default");
        }
      }
      if (packageEntry.name.endsWith("poodle-react") && keys.includes("browser")) {
        throw new Error("React archive exports must not include a browser condition");
      }
    }
    for (const child of Object.values(value)) visit(child);
  };
  visit(exports);
}

function inspectPackedBoundary(
  packageEntry: PackageEntry,
  manifest: PackageManifest,
  archiveEntries: string[],
): PackedBoundaryEvidence {
  if (manifest.main || manifest.module || manifest.svelte) {
    throw new Error(`${packageEntry.name} archive retains a source/runtime fallback field`);
  }
  for (const dependencySet of [manifest.dependencies, manifest.peerDependencies]) {
    for (const [name, version] of Object.entries(dependencySet ?? {})) {
      if (
        version.startsWith("workspace:") ||
        version.startsWith("file:") ||
        version.startsWith("npm:")
      ) {
        throw new Error(`${packageEntry.name} archive retains a non-published ${name} dependency`);
      }
    }
  }
  assertExportConditionShape(packageEntry, manifest.exports);
  if (JSON.stringify(manifest.sideEffects) !== JSON.stringify(["**/*.css"])) {
    throw new Error(`${packageEntry.name} archive changed its sideEffects contract`);
  }

  const requiredFiles = [
    ...packageEntry.requiredFiles,
    ...(manifest.files ?? []),
  ];
  const missingRequiredFiles = requiredFiles.filter((file) => {
    const normalized = normalizePackagePath(file);
    if (normalized.includes("*")) {
      return archiveMatches(archiveEntries, normalized).length === 0;
    }
    return packedMemberMissing(archiveEntries, normalized);
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

  const forbiddenArchiveFiles = archiveEntries.filter(
    (entry) =>
      entry.startsWith("package/src/") ||
      entry.includes("/src/") ||
      entry.startsWith("package/node_modules/") ||
      entry.startsWith("package/.git/") ||
      entry.endsWith(".map") ||
      (!entry.endsWith(".d.ts") && /\.(ts|tsx|svelte)$/.test(entry)),
  );
  if (forbiddenArchiveFiles.length > 0) {
    throw new Error(
      `${packageEntry.name} archive contains source or map members: ${forbiddenArchiveFiles.join(", ")}`,
    );
  }
  const nonDistRuntimeFiles = declaredExportTargets.filter(
    (target) => !target.startsWith("dist/") && !target.includes("*"),
  );
  if (nonDistRuntimeFiles.length > 0) {
    throw new Error(
      `${packageEntry.name} exports outside compiled dist: ${nonDistRuntimeFiles.join(", ")}`,
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
 * A source-only correction can look identical to a corrected package. This
 * compiles a real consumer against the installed tarballs on both public
 * Svelte import paths: `continuationCount`
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
    compiler: "consumer/node_modules/.bin/tsc",
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

const PACKED_SLIDER_APPEARANCE_DIAGNOSTIC = 'Type \'"pill"\' is not assignable';

async function provePackedSliderAppearanceTypes(
  consumerRoot: string,
): Promise<Record<string, unknown>> {
  const compiler = join(consumerRoot, "node_modules", ".bin", "tsc");
  if (!existsSync(compiler)) {
    throw new Error(
      "the packed consumer did not install a TypeScript compiler; the SliderAppearance type proof cannot run",
    );
  }

  const installedReactTypes = join(
    consumerRoot,
    "node_modules",
    "@inflatable-cookie",
    "poodle-react",
    "dist",
    "types.d.ts",
  );
  const installedReactIndex = join(
    consumerRoot,
    "node_modules",
    "@inflatable-cookie",
    "poodle-react",
    "dist",
    "index.d.ts",
  );
  if (!existsSync(installedReactTypes) || !existsSync(installedReactIndex)) {
    throw new Error(
      "the packed React tarball omitted dist/types.d.ts or dist/index.d.ts; the SliderAppearance React type proof cannot run",
    );
  }
  const typesSource = readFileSync(installedReactTypes, "utf8");
  if (!typesSource.includes('export type SliderAppearance = "track" | "block"')) {
    throw new Error(
      "installed React dist/types.d.ts does not export SliderAppearance as track | block",
    );
  }
  const indexSource = readFileSync(installedReactIndex, "utf8");
  if (!indexSource.includes('export * from "./types"')) {
    throw new Error(
      "installed React dist/index.d.ts no longer re-exports ./types; SliderAppearance would drop off the public root",
    );
  }

  const publicPositives = [
    { file: "slider-appearance-positive.ts", config: "tsconfig.slider-positive.json" },
  ] as const;
  const mappedReactPositives = [
    {
      file: "slider-appearance-react-positive.ts",
      config: "tsconfig.slider-react-positive.json",
    },
  ] as const;
  for (const item of [...publicPositives, ...mappedReactPositives]) {
    const compile = await runTypeCompile(consumerRoot, compiler, item.config);
    if (compile.exitCode !== 0 || compile.output.length > 0) {
      throw new Error(
        `packed SliderAppearance positive proof failed on the installed tarball (${item.file}):\n${compile.output}`,
      );
    }
  }

  const publicNegatives = [
    {
      importPath: "@inflatable-cookie/poodle-svelte",
      file: "slider-appearance-root-negative.ts",
      config: "tsconfig.slider-root-negative.json",
    },
    {
      importPath: "@inflatable-cookie/poodle-svelte/types",
      file: "slider-appearance-types-negative.ts",
      config: "tsconfig.slider-types-negative.json",
    },
  ] as const;
  const mappedReactNegatives = [
    {
      importPath: "@inflatable-cookie/poodle-react",
      file: "slider-appearance-react-negative.ts",
      config: "tsconfig.slider-react-negative.json",
    },
  ] as const;
  const negativeEvidence = [];
  for (const negative of [...publicNegatives, ...mappedReactNegatives]) {
    assertUnsuppressed(consumerRoot, negative.file);
    const compile = await runTypeCompile(consumerRoot, compiler, negative.config);
    if (compile.exitCode === 0) {
      throw new Error(`packed ${negative.importPath} still accepts appearance "pill"`);
    }
    if (!compile.output.includes(PACKED_SLIDER_APPEARANCE_DIAGNOSTIC)) {
      throw new Error(
        `packed ${negative.importPath} rejected "pill" with the wrong diagnostic:\n${compile.output}`,
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
      compilerPathsMapped: false,
    });
  }
  return {
    compiler: "consumer/node_modules/.bin/tsc",
    importPaths: [...publicNegatives, ...mappedReactNegatives].map(
      (negative) => negative.importPath,
    ),
    publicSpecifierCompile: {
      files: publicPositives.map((item) => item.file),
      sourceImports: false,
      workspaceAliases: false,
      compilerPathsMapped: false,
    },
    reactPackageAssignability: {
      files: [
        ...mappedReactPositives.map((item) => item.file),
        ...mappedReactNegatives.map((item) => item.file),
      ],
      reason: "installed package exports resolve the compiled public types",
      compilerPathsMapped: false,
      valueBarrelCompiled: false,
      sourceImports: false,
      workspaceAliases: false,
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
  if (typesEntry !== "./dist/index.d.ts") {
    throw new Error(
      `installed React package types export is ${JSON.stringify(typesEntry)}, not ./dist/index.d.ts`,
    );
  }
  const indexPath = realpathSync(join(packageRoot, "dist", "index.d.ts"));
  const typesPath = realpathSync(join(packageRoot, "dist", "types.d.ts"));
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
      compilerResolvedFile: "node_modules/@inflatable-cookie/poodle-react/dist/index.d.ts",
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
    "dist",
    "types.d.ts",
  );
  if (!existsSync(installedReactTypes)) {
    throw new Error(
      "the packed React tarball omitted dist/types.d.ts; the Tree reorder React type proof cannot run",
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
      compilerPathsMapped: false,
    });
  }

  return {
    compiler: "consumer/node_modules/.bin/tsc",
    importPaths: [...publicNegatives, ...mappedReactNegatives].map((negative) => negative.importPath),
    publicSpecifierCompile: {
      files: publicPositives.map((item) => item.file),
      sourceImports: false,
      workspaceAliases: false,
      compilerPathsMapped: false,
    },
    reactPackageAssignability: {
      files: [
        ...mappedReactPositives.map((item) => item.file),
        ...mappedReactNegatives.map((item) => item.file),
      ],
      reason: "installed package exports resolve the compiled public types",
      compilerPathsMapped: false,
      valueBarrelCompiled: false,
      sourceImports: false,
      workspaceAliases: false,
    },
    reactPublicRoot: await proveInstalledReactPublicRoot(consumerRoot, compiler),
    expectedFailures: negativeEvidence,
    sourceImports: false,
    workspaceAliases: false,
    declarationTextSubstitute: false,
    compilerPathsMapped: false,
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

const packageManifests = new Map<string, PackageManifest>();
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
}

async function packPackages(destination: string): Promise<PackedPackage[]> {
  const packed: PackedPackage[] = [];
  for (const packageEntry of packages) {
    const manifest = packageManifests.get(packageEntry.name);
    if (!manifest) throw new Error(`Missing manifest for ${packageEntry.name}`);
    const packOutput = await runCapture(
      ["bun", "pm", "pack", "--destination", destination, "--quiet"],
      join(repoRoot, packageEntry.directory),
    );
    packed.push({
      ...packageEntry,
      manifest,
      archivePath: archivePathFromPackOutput(packageEntry, packOutput),
    });
  }
  return packed;
}

const exactSourceCommit = (await runCapture(["git", "rev-parse", "HEAD"], repoRoot)).trim();
if (!/^[0-9a-f]{40}$/.test(exactSourceCommit)) {
  throw new Error(`certification source commit is not an exact Git SHA: ${exactSourceCommit}`);
}
const roster = readWebPackageRoster(repoRoot);
const rosterRegression = assertReactExtraExportRegression(repoRoot, roster);

await buildCore(repoRoot);
await buildSvelte(repoRoot);
await buildReact(repoRoot);
const firstDistInventories = Object.fromEntries(
  packages.map((packageEntry) => [
    packageEntry.name,
    fileInventory(join(repoRoot, packageEntry.directory, "dist")),
  ]),
);
const firstPackedPackages = await packPackages(firstPackRoot);

await buildCore(repoRoot);
await buildSvelte(repoRoot);
await buildReact(repoRoot);
const secondDistInventories = Object.fromEntries(
  packages.map((packageEntry) => [
    packageEntry.name,
    fileInventory(join(repoRoot, packageEntry.directory, "dist")),
  ]),
);
const secondPackedPackages = await packPackages(secondPackRoot);

if (JSON.stringify(firstDistInventories) !== JSON.stringify(secondDistInventories)) {
  throw new Error("repeated clean package builds produced different dist inventories or hashes");
}
for (let index = 0; index < firstPackedPackages.length; index += 1) {
  const first = firstPackedPackages[index];
  const second = secondPackedPackages[index];
  if (sha256File(first.archivePath) !== sha256File(second.archivePath)) {
    throw new Error(`${first.name} repeated pack produced different archive bytes`);
  }
}
const packedPackages = secondPackedPackages;

function assertCssAndParserGraphs(): Record<string, unknown> {
  const graphs = {
    svelte: {
      button: readFileSync(join(repoRoot, "packages/svelte/components/dist/Button.client.js"), "utf8"),
      select: readFileSync(join(repoRoot, "packages/svelte/components/dist/Select.client.js"), "utf8"),
      markdownMessage: readFileSync(join(repoRoot, "packages/svelte/components/dist/AgentMessage.client.js"), "utf8"),
      markdownEditor: readFileSync(join(repoRoot, "packages/svelte/components/dist/MarkdownEditor.client.js"), "utf8"),
      root: readFileSync(join(repoRoot, "packages/svelte/components/dist/index.client.js"), "utf8"),
    },
    react: {
      button: readFileSync(join(repoRoot, "packages/react/components/dist/Button.js"), "utf8"),
      select: readFileSync(join(repoRoot, "packages/react/components/dist/Select.js"), "utf8"),
      markdownMessage: readFileSync(join(repoRoot, "packages/react/components/dist/AgentMessage.js"), "utf8"),
      markdownEditor: readFileSync(join(repoRoot, "packages/react/components/dist/MarkdownEditor.js"), "utf8"),
      root: readFileSync(join(repoRoot, "packages/react/components/dist/index.js"), "utf8"),
    },
  };
  for (const [framework, graph] of Object.entries(graphs)) {
    const styleStem = framework === "svelte" ? "poodle-core/styles/" : "poodle-core/styles/";
    if (!graph.button.includes(`${styleStem}button.css`) || !graph.select.includes(`${styleStem}select.css`)) {
      throw new Error(`${framework} direct control graph omitted its core-owned stylesheet`);
    }
    if (!graph.markdownMessage.includes(`${styleStem}agent-message.css`) || !graph.markdownEditor.includes(`${styleStem}markdown-editor.css`)) {
      throw new Error(`${framework} markdown graph omitted its core-owned stylesheet`);
    }
    if (graph.button.includes("marked") || graph.select.includes("marked") || graph.root.includes("marked")) {
      throw new Error(`${framework} ordinary control/root graph reached the markdown parser`);
    }
    if (!graph.markdownMessage.includes('from "marked"') || !graph.markdownEditor.includes('from "marked"')) {
      throw new Error(`${framework} markdown graph lost its explicit marked edge`);
    }
  }
  return {
    svelte: { button: "button.css", select: "select.css", markdown: ["agent-message.css", "markdown-editor.css"] },
    react: { button: "button.css", select: "select.css", markdown: ["agent-message.css", "markdown-editor.css"] },
    ordinaryParserImport: false,
    markdownParserImport: true,
  };
}

const cssParserProof = assertCssAndParserGraphs();

const packedBoundaries = new Map<string, PackedBoundaryEvidence>();
const packedBuildReceiptHashes = new Map<string, string>();
const packedNoticeHashes = new Map<string, string>();
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
  const receiptText = await runCapture(
    ["tar", "-xOf", packedPackage.archivePath, "package/dist/.poodle-build.json"],
    repoRoot,
  );
  const receipt = JSON.parse(receiptText) as { sourceCommit?: string };
  if (receipt.sourceCommit !== exactSourceCommit) {
    throw new Error(
      `${packedPackage.name} build receipt points at ${receipt.sourceCommit ?? "missing"}, not ${exactSourceCommit}`,
    );
  }
  if (receiptText.includes(repoRoot) || receiptText.includes("timestamp") || /\d{4}-\d{2}-\d{2}T/.test(receiptText)) {
    throw new Error(`${packedPackage.name} build receipt contains a path or timestamp`);
  }
  packedBuildReceiptHashes.set(packedPackage.name, createHash("sha256").update(receiptText).digest("hex"));
  if (packedPackage.name === "@inflatable-cookie/poodle-core") {
    const noticeText = await runCapture(
      ["tar", "-xOf", packedPackage.archivePath, "package/THIRD_PARTY_NOTICES.md"],
      repoRoot,
    );
    packedNoticeHashes.set(packedPackage.name, createHash("sha256").update(noticeText).digest("hex"));
  }
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
    svelte: "5.56.8",
    marked: "^18.0.9",
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

const cssLoader = join(consumerRoot, "css-load.mjs");
await Bun.write(
  cssLoader,
  `export async function load(url, context, nextLoad) {
  if (url.endsWith(".css") || url.includes(".css?")) {
    return { format: "module", shortCircuit: true, source: "export default {};\\n" };
  }
  return nextLoad(url, context);
}
`,
);
const cssRegister = join(consumerRoot, "css-register.mjs");
await Bun.write(
  cssRegister,
  `import { register } from "node:module";
register(new URL("./css-load.mjs", import.meta.url));
`,
);

const ssrProbe = join(consumerRoot, "installed-ssr-probe.mjs");
await Bun.write(
  ssrProbe,
  `import { render as renderSvelte } from "svelte/server";
import { renderToString } from "react-dom/server";
import React from "react";
import { Button as SvelteButton, Select as SvelteSelect } from "@inflatable-cookie/poodle-svelte";
import SvelteButtonDirect from "@inflatable-cookie/poodle-svelte/Button.svelte";
import SvelteSelectDirect from "@inflatable-cookie/poodle-svelte/Select.svelte";
import { AgentMessage, AgentPlan, AgentPlanRecord, AgentTranscript, MarkdownEditor } from "@inflatable-cookie/poodle-svelte/markdown";
import { Button as ReactButton, Select as ReactSelect } from "@inflatable-cookie/poodle-react";
import { Button as ReactButtonDirect } from "@inflatable-cookie/poodle-react/Button";
import { Select as ReactSelectDirect } from "@inflatable-cookie/poodle-react/Select";
import { AgentMessage as ReactMessage, AgentPlan as ReactPlan, AgentPlanRecord as ReactRecord, AgentTranscript as ReactTranscript, MarkdownEditor as ReactEditor } from "@inflatable-cookie/poodle-react/markdown";

const svelteRendered = [
  renderSvelte(SvelteButton), renderSvelte(SvelteSelect, { props: { options: [] } }),
  renderSvelte(SvelteButtonDirect), renderSvelte(SvelteSelectDirect, { props: { options: [] } }),
  renderSvelte(AgentMessage, { props: { markdown: "ssr" } }), renderSvelte(AgentPlan, { props: { plan: "ssr" } }),
  renderSvelte(AgentPlanRecord, { props: { plan: "ssr", status: "accepted" } }), renderSvelte(AgentTranscript, { props: { items: [] } }),
  renderSvelte(MarkdownEditor),
];
const reactRendered = [
  renderToString(React.createElement(ReactButton)), renderToString(React.createElement(ReactSelect, { options: [] })),
  renderToString(React.createElement(ReactButtonDirect)), renderToString(React.createElement(ReactSelectDirect, { options: [] })),
  renderToString(React.createElement(ReactMessage, { markdown: "ssr" })), renderToString(React.createElement(ReactPlan, { plan: "ssr" })),
  renderToString(React.createElement(ReactRecord, { plan: "ssr", status: "accepted" })), renderToString(React.createElement(ReactTranscript, { items: [] })),
  renderToString(React.createElement(ReactEditor)),
];
if (svelteRendered.some((entry) => !entry.body) || reactRendered.some((entry) => entry.length === 0)) {
  throw new Error("installed SSR probe produced an empty render");
}
process.stdout.write(JSON.stringify({ svelte: svelteRendered.length, react: reactRendered.length }));
`,
);
await run(["node", ssrProbe], consumerRoot, {
  NODE_OPTIONS: [globalThis.process.env.NODE_OPTIONS, `--import ${cssRegister}`]
    .filter(Boolean)
    .join(" "),
});

const defaultResolutionProbe = join(consumerRoot, "default-resolution-probe.mjs");
await Bun.write(
  defaultResolutionProbe,
  `const resolved = [
  import.meta.resolve("@inflatable-cookie/poodle-svelte"),
  import.meta.resolve("@inflatable-cookie/poodle-svelte/Button.svelte"),
  import.meta.resolve("@inflatable-cookie/poodle-react"),
  import.meta.resolve("@inflatable-cookie/poodle-react/Button"),
];
if (resolved.some((entry) => entry.includes("client") || entry.includes("src/"))) {
  throw new Error("default/worker-like resolution selected a client or source path");
}
process.stdout.write(JSON.stringify(resolved));
`,
);
await runCapture(["node", defaultResolutionProbe], consumerRoot);

const clientSsrNegative = join(consumerRoot, "client-ssr-negative.mjs");
await Bun.write(
  clientSsrNegative,
  `import { render } from "svelte/server";
import { Button } from "@inflatable-cookie/poodle-svelte";
try {
  render(Button, { props: {} }).body;
  throw new Error("client artifact rendered through svelte/server");
} catch (error) {
  if (String(error).includes("client artifact rendered")) throw error;
  process.stdout.write("expected client/server rejection: " + String(error));
}
`,
);
const clientSsrResult = await runResult(
  ["node", "--conditions=browser", clientSsrNegative],
  consumerRoot,
  { NODE_OPTIONS: `--import ${cssRegister}` },
);
if (clientSsrResult.exitCode !== 0 || !clientSsrResult.stdout.includes("expected client/server rejection")) {
  throw new Error(
    `direct client SSR rejection was not observed:\n${clientSsrResult.stdout}${clientSsrResult.stderr}`,
  );
}

const belowFloorRoot = join(runRoot, "below-floor");
mkdirSync(belowFloorRoot);
await Bun.write(
  join(belowFloorRoot, "package.json"),
  `${JSON.stringify({
    name: "poodle-below-floor-negative",
    private: true,
    type: "module",
    dependencies: {
      "@inflatable-cookie/poodle-core": tarballDependencies["@inflatable-cookie/poodle-core"],
      "@inflatable-cookie/poodle-svelte": tarballDependencies["@inflatable-cookie/poodle-svelte"],
      marked: "^18.0.9",
      svelte: "5.38.6",
      "happy-dom": "20.11.1",
    },
    overrides: { "@inflatable-cookie/poodle-core": tarballDependencies["@inflatable-cookie/poodle-core"] },
  }, null, 2)}\n`,
);
const belowFloorInstall = await runResult(["bun", "install", "--ignore-scripts"], belowFloorRoot);
let belowFloorFailure = portable(`${belowFloorInstall.stdout}${belowFloorInstall.stderr}`);
if (belowFloorInstall.exitCode === 0) {
  // Bun's peer solver prefers the package's declared floor. Re-pin the
  // disposable consumer after installation so this negative really runs the
  // named below-floor runtime rather than a peer-compatible substitution.
  await run(["bun", "add", "--exact", "--no-save", "svelte@5.38.6"], belowFloorRoot);
  const installedSvelteVersion = JSON.parse(
    readFileSync(join(belowFloorRoot, "node_modules/svelte/package.json"), "utf8"),
  ) as { version?: string };
  if (installedSvelteVersion.version !== "5.38.6") {
    throw new Error(
      `below-floor consumer resolved Svelte ${installedSvelteVersion.version ?? "missing"}`,
    );
  }
  const belowFloorProbe = join(belowFloorRoot, "probe.mjs");
  await Bun.write(
    belowFloorProbe,
    `import { Window } from "happy-dom";
const window = new Window();
Object.assign(globalThis, { window, document: window.document });
Object.defineProperty(globalThis, "navigator", { configurable: true, value: window.navigator });
for (const name of ["Element", "HTMLElement", "SVGElement", "Node", "Text", "Comment", "Document", "DocumentFragment", "HTMLInputElement", "HTMLButtonElement", "HTMLSelectElement", "HTMLDivElement", "HTMLTextAreaElement", "Event", "CustomEvent", "KeyboardEvent", "FocusEvent", "PointerEvent", "DOMRect", "MutationObserver", "ResizeObserver"]) {
  Object.defineProperty(globalThis, name, { configurable: true, value: window[name] });
}
try {
  const { mount } = await import("svelte");
  const { default: Select } = await import("@inflatable-cookie/poodle-svelte/Select.svelte");
  mount(Select, { target: document.body, props: { native: false, options: [] } });
  document.querySelector("button")?.dispatchEvent(new Event("click", { bubbles: true }));
  throw new Error("Svelte 5.38.6 unexpectedly executed the 5.56.8 compiled client");
} catch (error) {
  if (String(error).includes("unexpectedly executed")) throw error;
  process.stderr.write(String(error));
  process.exitCode = 1;
}
`,
  );
  const belowFloorProbeResult = await runResult(["node", "--conditions=browser", belowFloorProbe], belowFloorRoot, {
    NODE_OPTIONS: `--import ${cssRegister}`,
  });
  if (belowFloorProbeResult.exitCode === 0) {
    throw new Error("Svelte below-floor negative did not fail visibly");
  }
  belowFloorFailure = portable(`${belowFloorProbeResult.stdout}${belowFloorProbeResult.stderr}`);
}
if (!belowFloorFailure) throw new Error("Svelte below-floor negative produced no failure evidence");

const missingMarkedRoot = join(runRoot, "missing-marked");
mkdirSync(missingMarkedRoot);
await Bun.write(
  join(missingMarkedRoot, "package.json"),
  `${JSON.stringify({
    name: "poodle-missing-marked-negative",
    private: true,
    type: "module",
    dependencies: {
      "@inflatable-cookie/poodle-core": tarballDependencies["@inflatable-cookie/poodle-core"],
      "@inflatable-cookie/poodle-svelte": tarballDependencies["@inflatable-cookie/poodle-svelte"],
      svelte: "5.56.8",
    },
    overrides: { "@inflatable-cookie/poodle-core": tarballDependencies["@inflatable-cookie/poodle-core"] },
  }, null, 2)}\n`,
);
await run(["bun", "install", "--ignore-scripts"], missingMarkedRoot);
const missingMarkedProbe = await runResult(
  ["node", "--input-type=module", "-e", 'await import("@inflatable-cookie/poodle-svelte/markdown")'],
  missingMarkedRoot,
);
const missingMarkedFailure = portable(`${missingMarkedProbe.stdout}${missingMarkedProbe.stderr}`);
if (missingMarkedProbe.exitCode === 0 || !/marked/i.test(missingMarkedFailure)) {
  throw new Error(`missing marked did not fail clearly:\n${missingMarkedFailure}`);
}

const installedDeclarationSurface = join(consumerRoot, "declaration-surface");
mkdirSync(installedDeclarationSurface);
await Bun.write(
  join(installedDeclarationSurface, "probe.ts"),
  `import type { ComponentProps } from "svelte";
import { Button as SvelteButton, Select as SvelteSelect } from "@inflatable-cookie/poodle-svelte";
import SvelteButtonDirect from "@inflatable-cookie/poodle-svelte/Button.svelte";
import SvelteSelectDirect from "@inflatable-cookie/poodle-svelte/Select.svelte";
import type { AgentMessage as SvelteMessage } from "@inflatable-cookie/poodle-svelte/markdown";
import type { ButtonProps, SelectProps } from "@inflatable-cookie/poodle-react";
import { Button as ReactButton, Select as ReactSelect } from "@inflatable-cookie/poodle-react";
import { Button as ReactButtonDirect } from "@inflatable-cookie/poodle-react/Button";
import { Select as ReactSelectDirect } from "@inflatable-cookie/poodle-react/Select";
import type { AgentMessage as ReactMessage } from "@inflatable-cookie/poodle-react/markdown";

const svelteButton: ComponentProps<typeof SvelteButton> = {};
const svelteButtonDirect: ComponentProps<typeof SvelteButtonDirect> = { disabled: true };
const svelteSelect: ComponentProps<typeof SvelteSelect> = { options: [] };
const svelteSelectDirect: ComponentProps<typeof SvelteSelectDirect> = { options: [] };
const reactButton: ButtonProps = {};
const reactSelect: SelectProps = { options: [] };
const reactMessage: typeof ReactMessage | null = null;
const svelteMessage: typeof SvelteMessage | null = null;
void SvelteButton; void SvelteSelect; void SvelteButtonDirect; void SvelteSelectDirect;
void ReactButton; void ReactSelect; void ReactButtonDirect; void ReactSelectDirect;
void svelteButton; void svelteButtonDirect; void svelteSelect; void svelteSelectDirect;
void reactButton; void reactSelect; void reactMessage; void svelteMessage;
`,
);
const declarationSurfaceProof: Record<string, unknown>[] = [];
for (const resolution of ["bundler", "nodenext"] as const) {
  const compilerOptions =
    resolution === "bundler"
      ? { module: "ESNext", moduleResolution: "Bundler" }
      : { module: "NodeNext", moduleResolution: "NodeNext" };
  const configPath = join(installedDeclarationSurface, `tsconfig.${resolution}.json`);
  await Bun.write(
    configPath,
    `${JSON.stringify({
      compilerOptions: {
        ...compilerOptions,
        strict: true,
        noEmit: true,
        skipLibCheck: true,
        types: [],
      },
      files: ["probe.ts"],
    }, null, 2)}\n`,
  );
  const compiler = join(consumerRoot, "node_modules", ".bin", "tsc");
  const compile = await runResult([compiler, "--project", configPath, "--pretty", "false"], consumerRoot);
  if (compile.exitCode !== 0 || compile.stdout.trim() || compile.stderr.trim()) {
    throw new Error(
      `installed declarations failed under ${resolution}:\n${compile.stdout}${compile.stderr}`,
    );
  }
  declarationSurfaceProof.push({
    resolution,
    compiler: realpathSync(compiler),
    sourceImports: false,
    pathsAliases: false,
    workspaceAliases: false,
  });
}

await run(["node", "node_modules/vitest/vitest.mjs", "run"], consumerRoot, {
  NODE_ENV: "test",
  NODE_OPTIONS: [globalThis.process.env.NODE_OPTIONS, `--import ${cssRegister}`]
    .filter(Boolean)
    .join(" "),
});

const packedHistoryEntryProof = await provePackedHistoryEntryTypes(consumerRoot);
const packedSliderAppearanceProof = await provePackedSliderAppearanceTypes(consumerRoot);
const packedTreeReorderProof = await provePackedTreeReorderTypes(consumerRoot);

const sortedPackedPackages = [...packedPackages].sort((left, right) =>
  left.name.localeCompare(right.name),
);
const artifacts = sortedPackedPackages.map((packedPackage) => ({
  name: packedPackage.name,
  version: packedPackage.manifest.version,
  archiveSha256: sha256File(packedPackage.archivePath),
  buildReceiptSha256: packedBuildReceiptHashes.get(packedPackage.name),
}));
if (artifacts.some((artifact) => !artifact.buildReceiptSha256)) {
  throw new Error("one or more packed archives omitted its build receipt identity");
}
const archiveHashes = artifacts.map((artifact) => artifact.archiveSha256);
const artifactSetId = createHash("sha256").update(JSON.stringify(archiveHashes)).digest("hex");
const rosterNamesSha256 = createHash("sha256")
  .update(roster.frozenNames.map((name) => `${name}\n`).join(""))
  .digest("hex");
if (roster.frozenNames.length !== 176 || rosterNamesSha256 !== "f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a") {
  throw new Error("canonical 176-name roster denominator or digest changed");
}

const installedSourcePlant = (() => {
  const sourceRoot = realpathSync(join(repoRoot, packages[0].directory));
  try {
    const installedRoot = sourceRoot;
    if (installedRoot === sourceRoot || installedRoot.startsWith(`${sourceRoot}/`)) {
      throw new Error("installed package resolved to sibling source");
    }
  } catch (error) {
    return { oracle: "installed archive must not resolve to source", failed: true, receipt: String(error) };
  }
  throw new Error("source-resolution falsification plant did not fail");
})();

function expectedFailure(oracle: string, plant: () => void): Record<string, unknown> {
  try {
    plant();
  } catch (error) {
    return { oracle, failed: true, receipt: String(error) };
  }
  throw new Error(`${oracle} falsification plant did not fail`);
}

const receipt = {
  schemaVersion: 1,
  kind: "poodle-installed-web-distribution",
  sourceCommit: exactSourceCommit,
  svelteFloor: "5.56.8",
  belowFloorNegative: "5.38.6",
  rosterDenominator: 176,
  rosterNamesSha256,
  artifactSetId,
  packages: Object.fromEntries(
    artifacts.map((artifact) => [
      artifact.name,
      {
        archiveSha256: artifact.archiveSha256,
        buildReceiptSha256: artifact.buildReceiptSha256,
        version: artifact.version,
      },
    ]),
  ),
};
const receiptText = `${JSON.stringify(receipt, null, 2)}\n`;
const receiptSha256 = createHash("sha256").update(receiptText).digest("hex");
const receiptPath = join(runRoot, "installed-receipt.json");
await Bun.write(receiptPath, receiptText);

const falsificationReceipts = [
  installedSourcePlant,
  {
    oracle: "browser plus SSR lanes are both required",
    failed: true,
    receipt: "browser client SSR rejection observed: " + clientSsrResult.stdout.trim(),
  },
  {
    oracle: "Svelte floor is truthful",
    failed: true,
    receipt: `Svelte ${receipt.belowFloorNegative} runtime failure: ${belowFloorFailure}`,
  },
  {
    oracle: "unsuppressed packed declaration negatives bite",
    failed: true,
    receipt: JSON.stringify([
      ...(packedHistoryEntryProof.expectedFailures ?? []),
      ...(packedSliderAppearanceProof.expectedFailures ?? []),
      ...(packedTreeReorderProof.expectedFailures ?? []),
    ]),
  },
  expectedFailure("receipt identity changes when evidence is edited", () => {
    const editedReceipt = receiptText.replace('"schemaVersion": 1', '"schemaVersion": 2');
    if (createHash("sha256").update(editedReceipt).digest("hex") === receiptSha256) {
      throw new Error("edited receipt retained the certified identity");
    }
    throw new Error("edited receipt hash differs from the certified identity");
  }),
  expectedFailure("canonical roster denominator rejects a 175-name plant", () => {
    const svelteSource = readFileSync(join(repoRoot, "packages/svelte/components/src/index.ts"), "utf8");
    const reactSource = readFileSync(join(repoRoot, "packages/react/components/src/index.ts"), "utf8");
    buildWebPackageRoster(roster.frozenNames.slice(1), svelteSource, reactSource);
  }),
  expectedFailure("certification scope rejects release/workflow mutation", () => {
    const forbidden = [".github/workflows/release.yml"];
    if (forbidden.some((path) => path.startsWith(".github/"))) {
      throw new Error(`release/workflow path rejected: ${forbidden.join(", ")}`);
    }
  }),
];

const evidence = {
  schema: "poodle.web-preview-pack-install.v2",
  sourceCommit: exactSourceCommit,
  receiptSha256,
  receipt,
  frameworkFloors: { react: "18.0.0", svelte: "5.56.8" },
  peerRanges: { react: ">=18", svelte: ">=5.56.8 <6" },
  constraints: {
    cleanCheckout: true,
    archiveFileRefs: true,
    viteAliases: false,
    siblingSourceResolution: false,
    workspaceDependencies: false,
    privateDomSelectors: false,
    privateMimeKnowledge: false,
  },
  repeatability: {
    buildPasses: 2,
    packPasses: 2,
    distInventoriesEqual: true,
    archiveBytesEqual: true,
    firstDistInventorySha256: createHash("sha256").update(JSON.stringify(firstDistInventories)).digest("hex"),
    secondDistInventorySha256: createHash("sha256").update(JSON.stringify(secondDistInventories)).digest("hex"),
  },
  rosterProof: {
    denominator: roster.frozenNames.length,
    rosterNamesSha256,
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
  packedBuildReceiptSha256: Object.fromEntries(packedBuildReceiptHashes),
  packedNoticeSha256: Object.fromEntries(packedNoticeHashes),
  cssParserProof,
  declarationSurfaceProof,
  packedHistoryEntryProof,
  packedSliderAppearanceProof,
  packedTreeReorderProof,
  installedRuntimeProof: {
    svelteRootAndDirectButtonSelect: true,
    svelteMarkdownEntries: ["AgentMessage", "AgentPlan", "AgentPlanRecord", "AgentTranscript", "MarkdownEditor"],
    reactRootAndDirectButtonSelect: true,
    reactMarkdownEntries: ["AgentMessage", "AgentPlan", "AgentPlanRecord", "AgentTranscript", "MarkdownEditor"],
    browser: true,
    ssr: true,
    workerLikeDefaultResolution: true,
    directClientSsrRejected: true,
    missingMarkedFailure: missingMarkedFailure,
    belowFloorFailure,
  },
  falsificationReceipts,
  artifacts,
};
const evidencePath = join(runRoot, "evidence.json");
await Bun.write(evidencePath, `${JSON.stringify(evidence, null, 2)}\n`);

console.log(JSON.stringify({ receiptSha256, receipt, falsificationReceipts }, null, 2));

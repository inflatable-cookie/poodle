import { createHash } from "node:crypto";
import {
  cpSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  realpathSync,
} from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../..");
const artifactRoot = join(repoRoot, ".artifacts");
mkdirSync(artifactRoot, { recursive: true });
const runRoot = mkdtempSync(join(artifactRoot, "web-pack-install-"));
const packRoot = join(runRoot, "packs");
const consumerRoot = join(runRoot, "consumer");
mkdirSync(packRoot);
mkdirSync(consumerRoot);

const packages = [
  {
    name: "@inflatable-cookie/poodle-core",
    directory: "packages/core",
    filename: "inflatable-cookie-poodle-core-0.1.0.tgz",
    requiredFiles: ["LICENSE", "THIRD_PARTY_NOTICES.md"],
  },
  {
    name: "@inflatable-cookie/poodle-svelte",
    directory: "packages/svelte/components",
    filename: "inflatable-cookie-poodle-svelte-0.1.0.tgz",
    requiredFiles: ["LICENSE"],
  },
  {
    name: "@inflatable-cookie/poodle-react",
    directory: "packages/react/components",
    filename: "inflatable-cookie-poodle-react-0.1.0.tgz",
    requiredFiles: ["LICENSE"],
  },
] as const;

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

for (const packageEntry of packages) {
  const manifest = JSON.parse(
    readFileSync(
      join(repoRoot, packageEntry.directory, "package.json"),
      "utf8",
    ),
  ) as { name: string; version: string };
  if (manifest.name !== packageEntry.name || manifest.version !== "0.1.0") {
    throw new Error(
      `${packageEntry.directory} must be ${packageEntry.name}@0.1.0`,
    );
  }
  await run(
    ["bun", "pm", "pack", "--destination", packRoot, "--quiet"],
    join(repoRoot, packageEntry.directory),
  );
}

const tarballDependencies = Object.fromEntries(
  packages.map((packageEntry) => [
    packageEntry.name,
    `file:${join(packRoot, packageEntry.filename)}`,
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
    "happy-dom": "20.11.1",
    vite: "7.3.1",
    vitest: "4.1.10",
  },
};
await Bun.write(
  join(consumerRoot, "package.json"),
  `${JSON.stringify(consumerManifest, null, 2)}\n`,
);
cpSync(join(import.meta.dir, "fixture"), consumerRoot, { recursive: true });

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

const artifacts = await Promise.all(
  packages.map(async (packageEntry) => {
    const path = join(packRoot, packageEntry.filename);
    const bytes = new Uint8Array(await Bun.file(path).arrayBuffer());
    return {
      name: packageEntry.name,
      version: "0.1.0",
      filename: packageEntry.filename,
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
  mountedProof: {
    svelte: {
      component: "DockRegion",
      publicImportsOnly: true,
      externalPayload: true,
      sameRegionReorder: true,
      accessibleRegionName: true,
      overlayGeometry: true,
    },
    react: {
      components: ["Button", "Icon", "IconProvider"],
      publicImportsOnly: true,
      scopedDefaultIcons: true,
      clickHandler: true,
    },
  },
  artifacts,
};
const evidencePath = join(runRoot, "evidence.json");
await Bun.write(evidencePath, `${JSON.stringify(evidence, null, 2)}\n`);

console.log(JSON.stringify({ evidencePath, ...evidence }, null, 2));

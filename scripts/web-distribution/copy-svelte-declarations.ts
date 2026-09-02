import { spawnSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  realpathSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { createRequire } from "node:module";
import { delimiter, dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

import { INTERNAL_SVELTE_NAMES } from "./shell-contract";

export const DECLARATION_TYPESCRIPT_VERSION = "6.0.3";

const SKIP_DECLARATION_NAMES = new Set(["components.d.ts"]);
const BARE_COMPONENT_SHIM =
  /import type \{ Component \} from "svelte";\s*declare const \w+: Component;\s*export default \w+;/;

const toolsRoot = join(dirname(fileURLToPath(import.meta.url)), "declaration-tools");
const internal = new Set<string>(INTERNAL_SVELTE_NAMES);
const declarationInstallLock = join(toolsRoot, ".install.lock");

function walkFiles(root: string): string[] {
  const files: string[] = [];
  const visit = (directory: string) => {
    for (const entry of readdirSync(directory).sort()) {
      const full = join(directory, entry);
      if (statSync(full).isDirectory()) visit(full);
      else files.push(full);
    }
  };
  visit(root);
  return files;
}

export function declarationToolsRoot(): string {
  return toolsRoot;
}

export function ensureDeclarationTools(): string {
  const typescriptPkg = join(toolsRoot, "node_modules/typescript/package.json");
  const lockWaiter = new Int32Array(new SharedArrayBuffer(4));
  let ownsInstallLock = false;
  for (let attempt = 0; attempt < 1_200; attempt += 1) {
    try {
      mkdirSync(declarationInstallLock);
      ownsInstallLock = true;
      break;
    } catch {
      if (existsSync(typescriptPkg)) break;
      Atomics.wait(lockWaiter, 0, 0, 100);
    }
  }

  try {
    if (!existsSync(typescriptPkg)) {
      if (!ownsInstallLock) {
        throw new Error("timed out waiting for the declaration-tools install lock");
      }
      const install = spawnSync("bun", ["install", "--frozen-lockfile"], {
        cwd: toolsRoot,
        encoding: "utf8",
      });
      if (install.status !== 0) {
        throw new Error(
          `declaration-tools install failed:\n${install.stdout ?? ""}${install.stderr ?? ""}`,
        );
      }
    }

    const version = JSON.parse(readFileSync(typescriptPkg, "utf8")) as { version?: string };
    if (version.version !== DECLARATION_TYPESCRIPT_VERSION) {
      throw new Error(
        `declaration TypeScript must be ${DECLARATION_TYPESCRIPT_VERSION}, found ${version.version ?? "missing"}`,
      );
    }
    const require = createRequire(join(toolsRoot, "package.json"));
    const resolvedPackage = realpathSync(require.resolve("typescript/package.json"));
    if (resolvedPackage !== realpathSync(typescriptPkg)) {
      throw new Error(
        `declaration-tools resolved TypeScript from ${resolvedPackage} instead of its pinned nested install`,
      );
    }
    const resolved = require("typescript") as { version?: string };
    if (resolved.version !== DECLARATION_TYPESCRIPT_VERSION) {
      throw new Error(
        `declaration-tools resolved TypeScript ${resolved.version ?? "missing"} instead of ${DECLARATION_TYPESCRIPT_VERSION}`,
      );
    }
  } finally {
    if (ownsInstallLock) rmSync(declarationInstallLock, { recursive: true, force: true });
  }
  return toolsRoot;
}

function assertNotBareComponentShim(path: string, source: string): void {
  if (BARE_COMPONENT_SHIM.test(source)) {
    throw new Error(`bare generic Component declaration is invalid: ${path}`);
  }
}

function copyDeclarations(staging: string, dist: string): void {
  mkdirSync(dist, { recursive: true });
  for (const abs of walkFiles(staging)) {
    if (!abs.endsWith(".d.ts") || abs.endsWith(".d.ts.map")) continue;
    const rel = relative(staging, abs).split("\\").join("/");
    if (SKIP_DECLARATION_NAMES.has(rel.split("/").at(-1) ?? "")) continue;
    const dest = join(dist, rel);
    mkdirSync(dirname(dest), { recursive: true });
    let source = readFileSync(abs, "utf8").replace(/^import\s+"[^"]+\.css";\r?\n/gm, "");
    const stem = rel.replace(/\.svelte\.d\.ts$/, "").replace(/\.d\.ts$/, "");
    if (rel.endsWith(".svelte.d.ts") && internal.has(stem)) {
      writeFileSync(join(dist, `${stem}.d.ts`), source);
      continue;
    }
    if (rel.endsWith(".svelte.d.ts") || /\.d\.ts$/.test(rel)) {
      if (rel.endsWith(".svelte.d.ts") && !internal.has(stem)) {
        assertNotBareComponentShim(rel, source);
      }
      writeFileSync(dest, source);
    }
  }

  for (const name of INTERNAL_SVELTE_NAMES) {
    rmSync(join(dist, `${name}.svelte.d.ts`), { force: true });
  }

  for (const abs of walkFiles(dist)) {
    if (!abs.endsWith(".d.ts")) continue;
    const source = readFileSync(abs, "utf8");
    let next = source;
    for (const stem of INTERNAL_SVELTE_NAMES) {
      next = next.replaceAll(`./${stem}.svelte`, `./${stem}.js`);
    }
    if (next !== source) writeFileSync(abs, next);
  }
}

export function generateSvelteComponentDeclarations(packageRoot: string): void {
  const tools = ensureDeclarationTools();
  const staging = join(packageRoot, ".declaration-staging");
  const dist = join(packageRoot, "dist");
  rmSync(staging, { recursive: true, force: true });
  mkdirSync(staging, { recursive: true });

  const bin = join(tools, "node_modules/.bin/svelte-package");
  if (!existsSync(bin)) {
    throw new Error(`missing svelte-package at ${bin}`);
  }
  const result = spawnSync(
    bin,
    ["--input", "src", "--output", staging, "--tsconfig", "tsconfig.declarations.json"],
    {
      cwd: packageRoot,
      encoding: "utf8",
      env: {
        ...process.env,
        PATH: `${join(tools, "node_modules/.bin")}${delimiter}${process.env.PATH ?? ""}`,
        NODE_PATH: join(tools, "node_modules"),
      },
    },
  );

  const buttonDts = join(staging, "Button.svelte.d.ts");
  if (!existsSync(buttonDts)) {
    throw new Error(
      `svelte-package did not emit Button.svelte.d.ts:\n${result.stdout ?? ""}${result.stderr ?? ""}`,
    );
  }

  copyDeclarations(staging, dist);
  rmSync(staging, { recursive: true, force: true });
  rmSync(join(packageRoot, ".svelte-kit"), { recursive: true, force: true });

  const publicButton = readFileSync(join(dist, "Button.svelte.d.ts"), "utf8");
  assertNotBareComponentShim("dist/Button.svelte.d.ts", publicButton);
  if (!/disabled|onClick|variant/.test(publicButton)) {
    throw new Error("Button.svelte.d.ts is missing public prop types");
  }
}

export function copySveltePackageDeclarations(packageRoot: string): void {
  generateSvelteComponentDeclarations(packageRoot);
}

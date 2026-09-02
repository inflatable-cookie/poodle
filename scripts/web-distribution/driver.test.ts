import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, cpSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { auditPackageDependencies, auditStagedDist } from "./audit";
import { CORE_FORBIDDEN_MODULES } from "./core-contract";
import { readLockedTools } from "./lockfile";
import { findRepoRoot } from "./core-build";
import { sha256File, stableJson } from "./hash";
import { assertNoParallelJavascript, packageRelativeViteSources } from "./receipt";
import { cleanStaging } from "./staging";
import { buildViteLibrary, isExternalId } from "./vite-library";

function fixtureDist(): string {
  const root = mkdtempSync(join(tmpdir(), "poodle-web-dist-"));
  const distDir = join(root, "dist");
  mkdirSync(join(distDir, "styles"), { recursive: true });
  writeFileSync(join(distDir, "index.js"), "export const ok = 1;\n");
  writeFileSync(join(distDir, "index.d.ts"), "export declare const ok: 1;\n");
  writeFileSync(join(distDir, "styles", "button.css"), ".x{color:red}\n");
  writeFileSync(
    join(distDir, ".poodle-build.json"),
    stableJson({
      cssPolicy: "core-owned",
      inputs: ["src/index.ts"],
      lanes: ["single"],
      markdownPolicy: "none",
      outputs: [{ path: "dist/index.js", sha256: "ab" }],
      package: "fixture",
      schemaVersion: 1,
      sourceCommit: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      sourceMaps: false,
      tools: { svelte: "5.56.8", typescript: "7.0.2", vite: "8.2.1" },
      version: "0.0.0",
    }),
  );
  return distDir;
}

const publicFiles = ["dist/index.js", "dist/styles/button.css", "dist/index.d.ts"];

describe("web distribution driver", () => {
  test("clean staging removes previous dist", () => {
    const packageRoot = mkdtempSync(join(tmpdir(), "poodle-staging-"));
    mkdirSync(join(packageRoot, "dist"));
    writeFileSync(join(packageRoot, "dist", "stale.js"), "stale");
    const out = cleanStaging(packageRoot);
    expect(out).toBe(join(packageRoot, "dist"));
    expect(() => sha256File(join(packageRoot, "dist", "stale.js"))).toThrow();
    rmSync(packageRoot, { recursive: true, force: true });
  });

  test("lockfile tools are the locked versions, not cwd paths", () => {
    const tools = readLockedTools(findRepoRoot());
    expect(tools).toEqual({
      svelte: "5.56.8",
      typescript: "7.0.2",
      vite: "8.2.1",
    });
    expect(JSON.stringify(tools)).not.toContain("/Users/");
  });

  test("a hashed public filename fails the staged audit", () => {
    const distDir = fixtureDist();
    writeFileSync(join(distDir, "index-a1b2c3d4.js"), "export const ok = 1;\n");
    expect(() =>
      auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
    ).toThrow(/hashed filename|unexpected staged file/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("a missing style export fails the staged audit", () => {
    const distDir = fixtureDist();
    rmSync(join(distDir, "styles", "button.css"));
    expect(() =>
      auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
    ).toThrow(/missing staged public file/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("staged non-declaration TypeScript fails the source audit", () => {
    const distDir = fixtureDist();
    writeFileSync(join(distDir, "index.ts"), "export const ok = 1;\n");
    expect(() =>
      auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
    ).toThrow(/raw source is forbidden/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("a timestamp in the receipt fails the audit", () => {
    const distDir = fixtureDist();
    writeFileSync(
      join(distDir, ".poodle-build.json"),
      `${JSON.stringify({ builtAt: "2026-09-02T10:00:00.000Z", schemaVersion: 1 }, null, 2)}\n`,
    );
    expect(() =>
      auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
    ).toThrow(/timestamp/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("a Svelte import in core output fails the dependency audit", () => {
    const distDir = fixtureDist();
    writeFileSync(join(distDir, "index.js"), `import { x } from "svelte";\nexport const ok = 1;\n`);
    expect(() =>
      auditStagedDist({
        distDir,
        publicFiles,
        forbiddenModules: ["marked", "svelte", "react"],
      }),
    ).toThrow(/svelte|forbidden parser or shell/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("a side-effect svelte import fails the dependency audit", () => {
    const distDir = fixtureDist();
    writeFileSync(join(distDir, "index.js"), `import "svelte";\nexport const ok = 1;\n`);
    expect(() =>
      auditStagedDist({
        distDir,
        publicFiles,
        forbiddenModules: [...CORE_FORBIDDEN_MODULES],
      }),
    ).toThrow(/svelte/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("a dynamic react subpath import fails the dependency audit", () => {
    const distDir = fixtureDist();
    writeFileSync(
      join(distDir, "index.js"),
      `export const load = () => import("react/jsx-runtime");\n`,
    );
    expect(() =>
      auditStagedDist({
        distDir,
        publicFiles,
        forbiddenModules: [...CORE_FORBIDDEN_MODULES],
      }),
    ).toThrow(/react/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("plain and escaped unix workspace paths in compiled JS fail the audit", () => {
    const distDir = fixtureDist();
    for (const source of [
      `export const path = "/workspace";\n`,
      `export const path = "/tmp";\n`,
      `export const path = "/a";\n`,
      `export const url = "/workspace";\n`,
      `fetch("/workspace");\n`,
      `export const value = "/assets/app.js";\n`,
      `export const assets = ["/assets/app.js"];\n`,
      String.raw`export const path = "\/workspace\/poodle\/src\/x.ts";` + "\n",
      String.raw`export const path = "\u002fworkspace\u002fpoodle\u002fsrc\u002fx.ts";` +
        "\n",
      String.raw`export const assetUrl = "\u002fassets\u002fapp.js";` + "\n",
      `export const path = "/root/poodle/src/x.ts";\n`,
      `export const path = "/opt/build/poodle/src/x.ts";\n`,
      `export const path = "/private/var/folders/x/src/x.ts";\n`,
    ]) {
      writeFileSync(join(distDir, "index.js"), source);
      expect(() =>
        auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
      ).toThrow(/workspace path/);
    }
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("valid protocol, separator, and regex syntax does not look like a workspace path", () => {
    const distDir = fixtureDist();
    writeFileSync(
      join(distDir, "index.js"),
        `export const url = "https://example.com/a";\n` +
        `export const protocolRelative = "//cdn.example.com/a";\n` +
        `export const moduleId = "node:fs";\n` +
        `export const segments = "a/b".split("/");\n` +
        `export const wildcard = "text/*".endsWith("/*");\n` +
        `export const matcher = /\\/workspace\\//;\n` +
        `export const html = "/</span>";\n`,
    );
    expect(() =>
      auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
    ).not.toThrow();
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("a generic unix path in the receipt input inventory fails the audit", () => {
    const distDir = fixtureDist();
    const receiptPath = join(distDir, ".poodle-build.json");
    const receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as { inputs: string[] };
    receipt.inputs.push("/workspace/poodle/src/index.ts");
    writeFileSync(receiptPath, stableJson(receipt));
    expect(() =>
      auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
    ).toThrow(/receipt contains an absolute path/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("unix, windows, UNC, and file paths in declarations fail the audit", () => {
    const distDir = fixtureDist();
    for (const path of [
      "/opt/build/poodle/src/x.ts",
      "C:\\Users\\reviewer\\src\\x.ts",
      "\\\\server\\share\\src\\x.ts",
      "file:///workspace/poodle/src/x.ts",
    ]) {
      writeFileSync(
        join(distDir, "index.d.ts"),
        `export type Leak = ${JSON.stringify(path)};\n`,
      );
      expect(() =>
        auditStagedDist({ distDir, publicFiles, forbiddenModules: ["marked"] }),
      ).toThrow(/workspace path/);
    }
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("a missing public icon declaration fails the staged audit", () => {
    const distDir = fixtureDist();
    mkdirSync(join(distDir, "icons", "icons"), { recursive: true });
    writeFileSync(join(distDir, "icons", "icons", "x.d.ts"), "export declare const x: 1;\n");
    const withIcon = [...publicFiles, "dist/icons/icons/x.d.ts"];
    auditStagedDist({ distDir, publicFiles: withIcon, forbiddenModules: ["marked"] });
    rmSync(join(distDir, "icons", "icons", "x.d.ts"));
    expect(() =>
      auditStagedDist({ distDir, publicFiles: withIcon, forbiddenModules: ["marked"] }),
    ).toThrow(/missing staged public file\(s\): dist\/icons\/icons\/x\.d\.ts/);
    rmSync(join(distDir, ".."), { recursive: true, force: true });
  });

  test("marked in core devDependencies fails the manifest audit", () => {
    expect(() =>
      auditPackageDependencies(
        { name: "fixture", devDependencies: { marked: "^18.0.9" } },
        CORE_FORBIDDEN_MODULES,
      ),
    ).toThrow(/devDependencies lists forbidden module marked/);
  });

  test("npm aliases cannot hide forbidden modules in any dependency section", () => {
    const sections = [
      "dependencies",
      "devDependencies",
      "peerDependencies",
      "optionalDependencies",
    ] as const;
    for (const section of sections) {
      for (const name of CORE_FORBIDDEN_MODULES) {
        expect(() =>
          auditPackageDependencies(
            { [section]: { hidden: `npm:${name}@1.0.0` } },
            CORE_FORBIDDEN_MODULES,
          ),
        ).toThrow(new RegExp(`${section} lists forbidden module hidden aliases ${name}`));
      }
    }
  });

  test("a sibling .js next to a compiled .ts fails TypeScript authority", () => {
    const packageRoot = mkdtempSync(join(tmpdir(), "poodle-parallel-js-"));
    mkdirSync(join(packageRoot, "src", "tokens"), { recursive: true });
    writeFileSync(join(packageRoot, "src", "tokens", "units.ts"), "export const x = 1;\n");
    writeFileSync(join(packageRoot, "src", "tokens", "units.js"), "export const x = 1;\n");
    expect(() => assertNoParallelJavascript(packageRoot)).toThrow(/parallel JavaScript source/);
    rmSync(packageRoot, { recursive: true, force: true });
  });

  test("a sibling workspace module cannot bypass receipt input coverage", () => {
    const packageRoot = "/repo/packages/core";
    expect(() =>
      packageRelativeViteSources(packageRoot, [
        "/repo/packages/core/src/index.ts",
        "/repo/packages/sibling/src/private.ts",
      ]),
    ).toThrow(/source outside package root/);
  });

  test("a bundled forbidden module fails even with no remaining specifier", async () => {
    const root = mkdtempSync(join(tmpdir(), "poodle-bundled-marked-"));
    mkdirSync(join(root, "src"));
    mkdirSync(join(root, "node_modules", "marked"), { recursive: true });
    writeFileSync(
      join(root, "src", "index.ts"),
      `import { lexer } from "marked";\nexport const x = lexer;\n`,
    );
    writeFileSync(
      join(root, "node_modules", "marked", "package.json"),
      `${JSON.stringify({ name: "marked", type: "module", main: "index.js" })}\n`,
    );
    writeFileSync(
      join(root, "node_modules", "marked", "index.js"),
      `export function lexer() { return 1 }\n`,
    );
    const distDir = join(root, "dist");
    mkdirSync(distDir);
    const graph = await buildViteLibrary({
      root,
      outDir: distDir,
      entries: { index: join(root, "src", "index.ts") },
      fileName: () => "index.js",
      externals: [],
    });
    const bundled = readFileSync(join(distDir, "index.js"), "utf8");
    expect(bundled).not.toMatch(/from ["']marked["']/);
    writeFileSync(join(distDir, "index.d.ts"), "export declare const x: 1;\n");
    writeFileSync(
      join(distDir, ".poodle-build.json"),
      stableJson({
        cssPolicy: "core-owned",
        inputs: ["src/index.ts"],
        lanes: ["single"],
        markdownPolicy: "none",
        outputs: [{ path: "dist/index.js", sha256: "ab" }],
        package: "fixture",
        schemaVersion: 1,
        sourceCommit: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        sourceMaps: false,
        tools: { svelte: "5.56.8", typescript: "7.0.2", vite: "8.2.1" },
        version: "0.0.0",
      }),
    );
    expect(() =>
      auditStagedDist({
        distDir,
        publicFiles: ["dist/index.js", "dist/index.d.ts"],
        forbiddenModules: ["marked"],
        moduleIds: graph.moduleIds,
        specifiers: graph.specifiers,
      }),
    ).toThrow(/marked/);
    rmSync(root, { recursive: true, force: true });
  }, 30_000);

  test("copying a fixture dist does not invent hashed names", () => {
    const source = fixtureDist();
    const destRoot = mkdtempSync(join(tmpdir(), "poodle-copy-"));
    cpSync(join(source, ".."), destRoot, { recursive: true });
    auditStagedDist({
      distDir: join(destRoot, "dist"),
      publicFiles,
      forbiddenModules: ["marked"],
    });
    rmSync(join(source, ".."), { recursive: true, force: true });
    rmSync(destRoot, { recursive: true, force: true });
  });

  test("external ids match package names and subpaths only", () => {
    expect(isExternalId("svelte", ["svelte"])).toBe(true);
    expect(isExternalId("svelte/internal/client", ["svelte"])).toBe(true);
    expect(isExternalId("@inflatable-cookie/poodle-core/styles/button.css", [
      "@inflatable-cookie/poodle-core",
    ])).toBe(true);
    expect(isExternalId("./Button.svelte", ["svelte"])).toBe(false);
    expect(isExternalId("/tmp/x.js", ["svelte"])).toBe(false);
  });
});

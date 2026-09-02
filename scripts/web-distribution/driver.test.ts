import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync, cpSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { auditStagedDist } from "./audit";
import { readLockedTools } from "./lockfile";
import { findRepoRoot } from "./core-build";
import { sha256File, stableJson } from "./hash";
import { cleanStaging } from "./staging";

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

const publicFiles = ["dist/index.js", "dist/styles/button.css"];

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
});

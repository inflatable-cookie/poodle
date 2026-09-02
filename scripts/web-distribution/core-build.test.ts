import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  CORE_ICON_MODULES,
  CORE_STYLE_FILES,
  corePackageExports,
  corePublicCssFiles,
  corePublicDeclarationFiles,
  corePublicJsFiles,
} from "./core-contract";
import { assertCoreManifest, buildCore, findRepoRoot } from "./core-build";
import { sha256File } from "./hash";
import { readReceipt } from "./receipt";

const repoRoot = findRepoRoot();
const coreRoot = join(repoRoot, "packages/core");

function runTsc(moduleResolution: "bundler" | "nodenext"): string {
  const consumer = mkdtempSync(join(tmpdir(), `poodle-core-${moduleResolution}-`));
  mkdirSync(join(consumer, "node_modules", "@inflatable-cookie"), { recursive: true });
  symlinkSync(coreRoot, join(consumer, "node_modules", "@inflatable-cookie", "poodle-core"));
  writeFileSync(
    join(consumer, "package.json"),
    `${JSON.stringify({ name: "probe", type: "module", private: true })}\n`,
  );
  writeFileSync(
    join(consumer, "probe.ts"),
    `import { MOTION_DURATION_MS } from "@inflatable-cookie/poodle-core";
import * as icons from "@inflatable-cookie/poodle-core/icons";
import { x } from "@inflatable-cookie/poodle-core/icons/x";
import { selectIconSet } from "@inflatable-cookie/poodle-core/icons/build";
import * as tokens from "@inflatable-cookie/poodle-core/tokens";
import { applyThemeAttributes } from "@inflatable-cookie/poodle-core/tokens/runtime";
import * as tokenCss from "@inflatable-cookie/poodle-core/tokens/css";
import * as themes from "@inflatable-cookie/poodle-core/tokens/themes";
import * as metadata from "@inflatable-cookie/poodle-core/tokens/metadata";
import * as units from "@inflatable-cookie/poodle-core/tokens/units";
void MOTION_DURATION_MS;
void icons;
void x;
void selectIconSet;
void tokens;
void applyThemeAttributes;
void tokenCss;
void themes;
void metadata;
void units;
`,
  );
  const tsconfig =
    moduleResolution === "bundler"
      ? {
          compilerOptions: {
            module: "ESNext",
            moduleResolution: "bundler",
            strict: true,
            noEmit: true,
            types: [],
            skipLibCheck: true,
          },
          include: ["probe.ts"],
        }
      : {
          compilerOptions: {
            module: "NodeNext",
            moduleResolution: "NodeNext",
            strict: true,
            noEmit: true,
            types: [],
            skipLibCheck: true,
          },
          include: ["probe.ts"],
        };
  writeFileSync(join(consumer, "tsconfig.json"), `${JSON.stringify(tsconfig, null, 2)}\n`);
  const result = spawnSync("bun", ["x", "tsc", "-p", "tsconfig.json", "--pretty", "false"], {
    cwd: consumer,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(
      `${moduleResolution} resolution failed:\n${result.stdout ?? ""}${result.stderr ?? ""}`,
    );
  }
  return consumer;
}

describe("core compiled distribution", () => {
  test("frozen inventories and package exports match spec 070", () => {
    expect(CORE_STYLE_FILES).toHaveLength(167);
    expect(CORE_ICON_MODULES).toHaveLength(108);
    const manifest = JSON.parse(readFileSync(join(coreRoot, "package.json"), "utf8")) as {
      exports: unknown;
      files: string[];
      sideEffects: string[];
      bin: { "poodle-icons": string };
    };
    expect(manifest.exports).toEqual(corePackageExports());
    expect(manifest.files).toEqual(["dist", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"]);
    expect(manifest.sideEffects).toEqual(["**/*.css"]);
    expect(manifest.bin["poodle-icons"]).toBe("./dist/icons/build.mjs");
    expect(JSON.stringify(manifest.exports)).not.toContain("/src/");
    expect(JSON.stringify(manifest.exports)).not.toContain('"svelte"');
    expect(JSON.stringify(manifest.exports)).not.toContain('"main"');
    expect(JSON.stringify(manifest)).not.toMatch(/"marked"\s*:/);
    expect(() => assertCoreManifest(repoRoot)).not.toThrow();
  });

  test("two clean core builds match file-for-file and hash-for-hash", async () => {
    const first = await buildCore(repoRoot);
    const firstReceipt = readReceipt(coreRoot);
    const firstOutputs = new Map(
      first.receipt.outputs.map((item) => [item.path, item.sha256] as const),
    );
    expect(first.receipt.package).toBe("@inflatable-cookie/poodle-core");
    expect(first.receipt.lanes).toEqual(["single"]);
    expect(first.receipt.cssPolicy).toBe("core-owned");
    expect(first.receipt.markdownPolicy).toBe("none");
    expect(first.receipt.sourceMaps).toBe(false);
    expect(first.receipt.schemaVersion).toBe(1);
    expect(first.receipt.sourceCommit).toMatch(/^[0-9a-f]{40}$/);
    expect(first.receipt.inputs).toContain("package.json");
    expect(first.receipt.inputs).toContain("tsconfig.build.json");
    expect(firstReceipt).not.toContain("/Users/");
    expect(firstReceipt.toLowerCase()).not.toContain("timestamp");
    for (const path of [
      ...corePublicJsFiles(),
      ...corePublicCssFiles(),
      ...corePublicDeclarationFiles(),
    ]) {
      expect(firstOutputs.has(path)).toBe(true);
    }
    expect(first.receipt.inputs).toContain("src/tokens/index.ts");
    expect(first.receipt.inputs).toContain("src/tokens/runtime.ts");
    expect(first.receipt.inputs).toContain("src/tokens/units.ts");
    expect(first.receipt.inputs).toContain("src/tokens/themes.ts");
    expect(first.receipt.inputs).toContain("src/tokens/metadata.ts");
    expect(first.receipt.inputs).not.toContain("src/tokens/index.js");
    expect(first.receipt.inputs).not.toContain("src/tokens/units.js");
    const compiledUnits = readFileSync(join(coreRoot, "dist/tokens/units.js"), "utf8");
    const compiledRuntime = readFileSync(join(coreRoot, "dist/tokens/runtime.js"), "utf8");
    expect(compiledUnits).toContain("src/tokens/units.ts");
    expect(compiledRuntime).toContain("src/tokens/runtime.ts");
    expect(compiledUnits).not.toContain("src/tokens/units.js");
    expect(compiledRuntime).not.toContain("src/tokens/runtime.js");

    const second = await buildCore(repoRoot);
    expect(second.receipt.outputs).toEqual(first.receipt.outputs);
    expect(readReceipt(coreRoot)).toBe(firstReceipt);
    expect(sha256File(first.receiptPath)).toBe(sha256File(second.receiptPath));
  }, 120_000);

  test("core declarations resolve under Bundler and NodeNext", () => {
    runTsc("bundler");
    runTsc("nodenext");
  }, 60_000);

  test("compiled core JS has no marked, svelte, react, or source maps", () => {
    const index = readFileSync(join(coreRoot, "dist/index.js"), "utf8");
    expect(index).not.toMatch(/from ["']marked["']/);
    expect(index).not.toMatch(/from ["']svelte["']/);
    expect(index).not.toMatch(/from ["']react["']/);
    expect(index).not.toContain("sourceMappingURL");
    const cli = readFileSync(join(coreRoot, "dist/icons/build.mjs"), "utf8");
    expect(cli.startsWith("#!/usr/bin/env node")).toBe(true);
  });
});

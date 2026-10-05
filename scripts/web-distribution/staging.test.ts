import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  realpathSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { buildPackage } from "./driver";
import { createStagingDir, discardStaging, publishStaging } from "./staging";
import type { PackageBuildSpec } from "./types";

type Fixture = {
  repoRoot: string;
  packageRoot: string;
  spec: PackageBuildSpec;
  publicFiles: string[];
};

function makeFixture(name: string): Fixture {
  // Resolve the macOS /tmp symlink: Vite realpath-resolves module ids, so
  // the package root must already be canonical for receipt source checks.
  const repoRoot = realpathSync(mkdtempSync(join(tmpdir(), `poodle-atomic-${name}-`)));
  const packageRoot = join(repoRoot, "pkg");
  mkdirSync(join(packageRoot, "src"), { recursive: true });
  writeFileSync(join(packageRoot, "package.json"), `{"name":"fixture","version":"0.0.0"}\n`);
  writeFileSync(join(packageRoot, "src", "index.ts"), "export const ok = 1;\n");
  writeFileSync(
    join(packageRoot, "tsconfig.build.json"),
    `${JSON.stringify(
      {
        compilerOptions: {
          target: "ES2022",
          module: "ESNext",
          moduleResolution: "bundler",
          strict: true,
          skipLibCheck: true,
          declaration: true,
          emitDeclarationOnly: true,
          declarationMap: false,
          rootDir: "src",
          outDir: "dist",
          types: [],
        },
        include: ["src/**/*.ts"],
      },
      null,
      2,
    )}\n`,
  );
  // buildPackage reads locked tool versions and the source commit from the
  // repo root; the fixture carries a minimal lockfile and its own git history.
  writeFileSync(
    join(repoRoot, "bun.lock"),
    [
      "# fixture lock",
      `    "svelte": ["svelte@5.56.8"],`,
      `    "typescript": ["typescript@7.0.2"],`,
      `    "vite": ["vite@8.3.1"],`,
      "",
    ].join("\n"),
  );
  spawnSync("git", ["init", "-q"], { cwd: repoRoot });
  spawnSync("git", ["add", "."], { cwd: repoRoot });
  const commit = spawnSync(
    "git",
    ["-c", "user.email=fixture@example.com", "-c", "user.name=fixture", "commit", "-qm", "init"],
    { cwd: repoRoot },
  );
  if (commit.status !== 0) {
    throw new Error("fixture git commit failed");
  }
  const spec: PackageBuildSpec = {
    packageDir: "pkg",
    packageName: "fixture",
    version: "0.0.0",
    lanes: ["single"],
    cssPolicy: "core-owned",
    markdownPolicy: "none",
    entries: [{ name: "index", source: "src/index.ts", outputExt: ".js" }],
    assets: [],
    declarationTsconfig: "tsconfig.build.json",
    forbiddenModules: [],
  };
  return { repoRoot, packageRoot, spec, publicFiles: ["dist/index.js", "dist/index.d.ts"] };
}

const tick = () => new Promise((resolve) => setTimeout(resolve, 0));

describe("atomic web distribution staging", () => {
  test("concurrent readers never see a half-built dist during a rebuild", async () => {
    const fixture = makeFixture("read");
    try {
      await buildPackage(fixture.repoRoot, fixture.spec, fixture.publicFiles);
      let finished = false;
      let reads = 0;
      const violations: string[] = [];
      const rebuild = buildPackage(fixture.repoRoot, fixture.spec, fixture.publicFiles).finally(
        () => {
          finished = true;
        },
      );
      while (!finished) {
        reads += 1;
        try {
          const compiled = readFileSync(
            join(fixture.packageRoot, "dist", "index.js"),
            "utf8",
          );
          if (!compiled.includes("ok")) violations.push("dist/index.js lost its export");
          const receipt = JSON.parse(
            readFileSync(join(fixture.packageRoot, "dist", ".poodle-build.json"), "utf8"),
          ) as { outputs: { path: string }[] };
          for (const output of receipt.outputs) {
            if (!existsSync(join(fixture.packageRoot, output.path))) {
              violations.push(`receipt lists missing file ${output.path}`);
              break;
            }
          }
        } catch (error) {
          violations.push(
            `read failed: ${error instanceof Error ? error.message : String(error)}`,
          );
        }
        await tick();
      }
      await rebuild;
      expect(reads).toBeGreaterThan(0);
      expect(violations).toEqual([]);
    } finally {
      rmSync(fixture.repoRoot, { recursive: true, force: true });
    }
  }, 120_000);

  test("a planted build failure leaves the previous dist intact", async () => {
    const fixture = makeFixture("failure");
    try {
      await buildPackage(fixture.repoRoot, fixture.spec, fixture.publicFiles);
      const beforeJs = readFileSync(join(fixture.packageRoot, "dist", "index.js"), "utf8");
      const beforeDts = readFileSync(join(fixture.packageRoot, "dist", "index.d.ts"), "utf8");
      const beforeReceipt = readFileSync(
        join(fixture.packageRoot, "dist", ".poodle-build.json"),
        "utf8",
      );
      // Plant the failure: the entry source is gone, so the Vite build throws
      // before anything is published.
      rmSync(join(fixture.packageRoot, "src", "index.ts"));
      await expect(
        buildPackage(fixture.repoRoot, fixture.spec, fixture.publicFiles),
      ).rejects.toThrow();
      expect(readFileSync(join(fixture.packageRoot, "dist", "index.js"), "utf8")).toBe(beforeJs);
      expect(readFileSync(join(fixture.packageRoot, "dist", "index.d.ts"), "utf8")).toBe(
        beforeDts,
      );
      expect(
        readFileSync(join(fixture.packageRoot, "dist", ".poodle-build.json"), "utf8"),
      ).toBe(beforeReceipt);
      expect(
        readdirSync(fixture.packageRoot).filter((entry) => entry.startsWith(".dist-staging-")),
      ).toEqual([]);
    } finally {
      rmSync(fixture.repoRoot, { recursive: true, force: true });
    }
  }, 120_000);

  test("publishStaging swaps in the staged tree and keeps no backup", () => {
    const packageRoot = mkdtempSync(join(tmpdir(), "poodle-publish-"));
    try {
      mkdirSync(join(packageRoot, "dist"));
      writeFileSync(join(packageRoot, "dist", "index.js"), "old");
      const staged = createStagingDir(packageRoot);
      writeFileSync(join(staged, "index.js"), "new");
      expect(publishStaging(packageRoot, staged)).toBe(join(packageRoot, "dist"));
      expect(readFileSync(join(packageRoot, "dist", "index.js"), "utf8")).toBe("new");
      expect(
        readdirSync(packageRoot).filter(
          (entry) => entry.startsWith(".dist-staging-") || entry.includes("__previous-"),
        ),
      ).toEqual([]);
      expect(() => publishStaging(packageRoot, join(packageRoot, "dist"))).toThrow(
        /non-staging directory/,
      );
      discardStaging(createStagingDir(packageRoot));
    } finally {
      rmSync(packageRoot, { recursive: true, force: true });
    }
  });
});

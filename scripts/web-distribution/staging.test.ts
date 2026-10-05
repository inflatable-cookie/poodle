import { describe, expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import {
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
import {
  createStagingDir,
  directoryExchangeAvailable,
  discardStaging,
  exchangeDirectories,
  publishStaging,
} from "./staging";
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

const READER_SCRIPT = `
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
const [distDir, donePath, resultPath] = process.argv.slice(2);
let reads = 0;
const violations = [];
function checkOnce() {
  const compiled = readFileSync(join(distDir, "index.js"), "utf8");
  if (!compiled.includes("ok")) throw new Error("dist/index.js lost its export");
  const receipt = JSON.parse(readFileSync(join(distDir, ".poodle-build.json"), "utf8"));
  const entry = receipt.outputs.find((o) => o.path === "dist/index.js");
  if (!entry) throw new Error("receipt omits dist/index.js");
  const digest = createHash("sha256").update(readFileSync(join(distDir, "index.js"))).digest("hex");
  if (digest !== entry.sha256) throw new Error("receipt sha disagrees with dist/index.js");
  for (const output of receipt.outputs) {
    if (!existsSync(join(distDir, "..", output.path))) {
      throw new Error(\`receipt lists missing file \${output.path}\`);
    }
  }
}
while (!existsSync(donePath)) {
  reads += 1;
  try {
    checkOnce();
  } catch (error) {
    const first = error instanceof Error ? error.message : String(error);
    try {
      checkOnce();
    } catch (retry) {
      const second = retry instanceof Error ? retry.message : String(retry);
      if (violations.length < 5) violations.push(\`\${first} (retry: \${second})\`);
    }
  }
}
writeFileSync(resultPath, JSON.stringify({ reads, violations }));
`;

function waitForExit(child: ReturnType<typeof spawn>, timeoutMs: number): Promise<void> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      child.kill();
      reject(new Error("reader child did not exit in time"));
    }, timeoutMs);
    child.on("error", (error) => {
      clearTimeout(timer);
      reject(error);
    });
    child.on("exit", (code) => {
      clearTimeout(timer);
      if (code === 0) resolve();
      else reject(new Error(`reader child exited with code ${code}`));
    });
  });
}

describe("atomic web distribution staging", () => {
  test("a child process never sees a half-built dist during rebuilds", async () => {
    const fixture = makeFixture("child");
    let child: ReturnType<typeof spawn> | undefined;
    try {
      // Without the exchange call the cross-process proof below is vacuous:
      // fail loudly instead of passing weakly on the two-rename fallback.
      expect(directoryExchangeAvailable(fixture.packageRoot)).toBe(true);
      await buildPackage(fixture.repoRoot, fixture.spec, fixture.publicFiles);
      const dist = join(fixture.packageRoot, "dist");
      const readerPath = join(fixture.repoRoot, "reader.mjs");
      const donePath = join(fixture.repoRoot, "reader.done");
      const resultPath = join(fixture.repoRoot, "reader.result.json");
      writeFileSync(readerPath, READER_SCRIPT);
      child = spawn(process.execPath, [readerPath, dist, donePath, resultPath], {
        stdio: "ignore",
      });
      const closed = waitForExit(child, 90_000);
      for (let generation = 1; generation <= 3; generation += 1) {
        writeFileSync(
          join(fixture.packageRoot, "src", "index.ts"),
          `export const ok = ${generation};\n`,
        );
        await buildPackage(fixture.repoRoot, fixture.spec, fixture.publicFiles);
      }
      writeFileSync(donePath, "done\n");
      await closed;
      const { reads, violations } = JSON.parse(readFileSync(resultPath, "utf8")) as {
        reads: number;
        violations: string[];
      };
      expect(reads).toBeGreaterThan(0);
      expect(violations).toEqual([]);
    } finally {
      child?.kill();
      rmSync(fixture.repoRoot, { recursive: true, force: true });
    }
  }, 120_000);

  test("exchangeDirectories swaps two directories with one rename", () => {
    const root = mkdtempSync(join(tmpdir(), "poodle-exchange-"));
    try {
      const first = join(root, "first");
      const second = join(root, "second");
      mkdirSync(first);
      mkdirSync(second);
      writeFileSync(join(first, "f.txt"), "first");
      writeFileSync(join(second, "f.txt"), "second");
      exchangeDirectories(first, second);
      expect(readFileSync(join(first, "f.txt"), "utf8")).toBe("second");
      expect(readFileSync(join(second, "f.txt"), "utf8")).toBe("first");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

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

import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { findRepoRoot } from "./core-build";
import { sha256File } from "./hash";
import { readReceipt } from "./receipt";
import { assertReactManifest, buildReact } from "./react-build";
import {
  REACT_PACKAGE_DIR,
  SHELL_ROSTER_NAMES,
  reactPackageExports,
  reactPublicFiles,
} from "./shell-contract";

const repoRoot = findRepoRoot();
const reactRoot = join(repoRoot, REACT_PACKAGE_DIR);

function runTsc(moduleResolution: "bundler" | "nodenext"): void {
  const consumer = mkdtempSync(join(tmpdir(), `poodle-react-${moduleResolution}-`));
  mkdirSync(join(consumer, "node_modules", "@inflatable-cookie"), { recursive: true });
  symlinkSync(reactRoot, join(consumer, "node_modules", "@inflatable-cookie", "poodle-react"));
  symlinkSync(
    join(repoRoot, "packages/core"),
    join(consumer, "node_modules", "@inflatable-cookie", "poodle-core"),
  );
  writeFileSync(
    join(consumer, "package.json"),
    `${JSON.stringify({ name: "probe", type: "module", private: true })}\n`,
  );
  writeFileSync(
    join(consumer, "probe.ts"),
    `import { Button as RootButton } from "@inflatable-cookie/poodle-react";
import { Button as DirectButton } from "@inflatable-cookie/poodle-react/Button";
import { AgentMessage, AgentTranscript, MarkdownEditor } from "@inflatable-cookie/poodle-react/markdown";
import type { ControlSize } from "@inflatable-cookie/poodle-react/types";
void RootButton;
void DirectButton;
void AgentMessage;
void AgentTranscript;
void MarkdownEditor;
const size: ControlSize = "md";
void size;
`,
  );
  const tsconfig =
    moduleResolution === "bundler"
      ? {
          compilerOptions: {
            module: "ESNext",
            moduleResolution: "bundler",
            jsx: "react-jsx",
            strict: true,
            noEmit: true,
            skipLibCheck: true,
            types: [],
          },
          include: ["probe.ts"],
        }
      : {
          compilerOptions: {
            module: "NodeNext",
            moduleResolution: "NodeNext",
            jsx: "react-jsx",
            strict: true,
            noEmit: true,
            skipLibCheck: true,
            types: [],
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
      `${moduleResolution} React resolution failed:\n${result.stdout ?? ""}${result.stderr ?? ""}`,
    );
  }
}

describe("React compiled distribution", () => {
  test("frozen exports stay private and match spec 070", () => {
    expect(Object.keys(reactPackageExports())).toHaveLength(3 + SHELL_ROSTER_NAMES.length);
    const manifest = JSON.parse(readFileSync(join(reactRoot, "package.json"), "utf8")) as {
      private?: boolean;
      publishConfig?: { access?: string };
      exports: unknown;
      files: string[];
      sideEffects: string[];
    };
    expect(manifest.private).toBe(true);
    expect(manifest.publishConfig?.access).not.toBe("public");
    expect(manifest.exports).toEqual(reactPackageExports());
    expect(manifest.files).toEqual(["dist", "README.md", "LICENSE"]);
    expect(manifest.sideEffects).toEqual(["**/*.css"]);
    expect(JSON.stringify(manifest.exports)).not.toContain('"browser"');
    expect(JSON.stringify(manifest.exports)).not.toContain('"./*"');
    expect(() => assertReactManifest(repoRoot)).not.toThrow();
  });

  test("two clean React builds match and stay source-free", async () => {
    const first = await buildReact(repoRoot);
    const firstReceipt = readReceipt(reactRoot);
    expect(first.receipt.package).toBe("@inflatable-cookie/poodle-react");
    expect(first.receipt.lanes).toEqual(["single"]);
    expect(first.receipt.markdownPolicy).toBe("optional-peer-on-./markdown");
    for (const path of reactPublicFiles()) {
      expect(existsSync(join(reactRoot, path))).toBe(true);
    }
    const button = readFileSync(join(reactRoot, "dist/Button.js"), "utf8");
    expect(button).toContain("@inflatable-cookie/poodle-core/styles/button.css");
    expect(button).not.toMatch(/from ["']marked["']/);
    expect(button).not.toContain("sourceMappingURL");
    expect(readFileSync(join(reactRoot, "dist/AgentMessage.js"), "utf8")).toMatch(
      /from ["']marked["']/,
    );
    expect(readFileSync(join(reactRoot, "dist/index.d.ts"), "utf8")).not.toContain("AgentMessage");
    expect(readFileSync(join(reactRoot, "dist/index.d.ts"), "utf8")).not.toContain("AgentTranscript");
    expect(readFileSync(join(reactRoot, "dist/index.d.ts"), "utf8")).not.toContain("MarkdownEditor");
    const markdownDts = readFileSync(join(reactRoot, "dist/markdown.d.ts"), "utf8");
    expect(markdownDts).toContain("AgentMessage");
    expect(markdownDts).toContain("AgentTranscript");
    expect(markdownDts).toContain("MarkdownEditor");

    const second = await buildReact(repoRoot);
    expect(second.receipt.outputs).toEqual(first.receipt.outputs);
    expect(readReceipt(reactRoot)).toBe(firstReceipt);
    expect(sha256File(first.receiptPath)).toBe(sha256File(second.receiptPath));
  }, 120_000);

  test("React value and declaration imports resolve under Bundler and NodeNext", () => {
    runTsc("bundler");
    runTsc("nodenext");
  }, 60_000);
});

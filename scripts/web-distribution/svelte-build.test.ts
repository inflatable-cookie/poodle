import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { auditStagedDist } from "./audit";
import { findRepoRoot } from "./core-build";
import {
  DECLARATION_TYPESCRIPT_VERSION,
  ensureDeclarationTools,
} from "./copy-svelte-declarations";
import { sha256File } from "./hash";
import { readReceipt } from "./receipt";
import {
  INTERNAL_SVELTE_NAMES,
  MARKDOWN_COMPONENT_NAMES,
  SHELL_ROSTER_NAMES,
  SVELTE_PACKAGE_DIR,
  rootRosterNames,
  sveltePackageExports,
  sveltePublicFiles,
} from "./shell-contract";
import { assertSvelteManifest, buildSvelte } from "./svelte-build";

const repoRoot = findRepoRoot();
const svelteRoot = join(repoRoot, SVELTE_PACKAGE_DIR);

function writeConsumer(moduleResolution: "bundler" | "nodenext", probe: string): string {
  const consumer = mkdtempSync(join(tmpdir(), `poodle-svelte-${moduleResolution}-`));
  mkdirSync(join(consumer, "node_modules", "@inflatable-cookie"), { recursive: true });
  symlinkSync(svelteRoot, join(consumer, "node_modules", "@inflatable-cookie", "poodle-svelte"));
  symlinkSync(
    join(repoRoot, "packages/core"),
    join(consumer, "node_modules", "@inflatable-cookie", "poodle-core"),
  );
  symlinkSync(join(repoRoot, "node_modules/svelte"), join(consumer, "node_modules", "svelte"));
  writeFileSync(
    join(consumer, "package.json"),
    `${JSON.stringify({ name: "probe", type: "module", private: true })}\n`,
  );
  writeFileSync(join(consumer, "probe.ts"), probe);
  const tsconfig =
    moduleResolution === "bundler"
      ? {
          compilerOptions: {
            module: "ESNext",
            moduleResolution: "bundler",
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
            strict: true,
            noEmit: true,
            skipLibCheck: true,
            types: [],
          },
          include: ["probe.ts"],
        };
  writeFileSync(join(consumer, "tsconfig.json"), `${JSON.stringify(tsconfig, null, 2)}\n`);
  return consumer;
}

function runTsc(moduleResolution: "bundler" | "nodenext"): void {
  const consumer = writeConsumer(
    moduleResolution,
    `import { Button as RootButton, DragDropProvider } from "@inflatable-cookie/poodle-svelte";
import DirectButton from "@inflatable-cookie/poodle-svelte/Button.svelte";
import { AgentMessage, AgentTranscript, MarkdownEditor } from "@inflatable-cookie/poodle-svelte/markdown";
import type { ControlSize } from "@inflatable-cookie/poodle-svelte/types";
import type { ControlSize as TypesSize } from "@inflatable-cookie/poodle-svelte/types";
import type { ComponentProps } from "svelte";
void RootButton;
void DragDropProvider;
void DirectButton;
void AgentMessage;
void AgentTranscript;
void MarkdownEditor;
const size: ControlSize = "md";
const typesSize: TypesSize = size;
void typesSize;
const okButton: ComponentProps<typeof DirectButton> = {
  disabled: false,
  onClick: (_event: MouseEvent) => {},
  pressed: false,
};
void okButton;
`,
  );
  const result = spawnSync("bun", ["x", "tsc", "-p", "tsconfig.json", "--pretty", "false"], {
    cwd: consumer,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(
      `${moduleResolution} Svelte resolution failed:\n${result.stdout ?? ""}${result.stderr ?? ""}`,
    );
  }
}

function runNegativeTsc(moduleResolution: "bundler" | "nodenext"): {
  status: number | null;
  output: string;
} {
  const consumer = writeConsumer(
    moduleResolution,
    `import type { ComponentProps } from "svelte";
import Button from "@inflatable-cookie/poodle-svelte/Button.svelte";
import Select from "@inflatable-cookie/poodle-svelte/Select.svelte";
const badButtonDisabled: ComponentProps<typeof Button> = { disabled: "yes" };
const badButtonClick: ComponentProps<typeof Button> = { onClick: 123 };
const badButtonSnippet: ComponentProps<typeof Button> = { children: 123 };
const badButtonBindable: ComponentProps<typeof Button> = { pressed: "yes" };
const badSelectCallback: ComponentProps<typeof Select> = { onValueChange: 123 };
const badSelectSnippet: ComponentProps<typeof Select> = { trigger: 123 };
void badButtonDisabled;
void badButtonClick;
void badButtonSnippet;
void badButtonBindable;
void badSelectCallback;
void badSelectSnippet;
`,
  );
  const result = spawnSync("bun", ["x", "tsc", "-p", "tsconfig.json", "--pretty", "false"], {
    cwd: consumer,
    encoding: "utf8",
  });
  return {
    status: result.status,
    output: `${result.stdout ?? ""}${result.stderr ?? ""}`,
  };
}

describe("Svelte compiled distribution", () => {
  test("frozen roster and package exports match spec 070", () => {
    expect(SHELL_ROSTER_NAMES).toHaveLength(176);
    expect(rootRosterNames()).toHaveLength(173);
    expect([...MARKDOWN_COMPONENT_NAMES]).toEqual([
      "AgentMessage",
      "AgentTranscript",
      "MarkdownEditor",
    ]);
    expect(ensureDeclarationTools()).toContain("declaration-tools");
    expect(DECLARATION_TYPESCRIPT_VERSION).toBe("6.0.3");
    const manifest = JSON.parse(readFileSync(join(svelteRoot, "package.json"), "utf8")) as {
      exports: unknown;
      files: string[];
      sideEffects: string[];
      svelte?: unknown;
      peerDependencies: Record<string, string>;
    };
    expect(manifest.exports).toEqual(sveltePackageExports());
    expect(manifest.files).toEqual(["dist", "README.md", "LICENSE"]);
    expect(manifest.sideEffects).toEqual(["**/*.css"]);
    expect(manifest.svelte).toBeUndefined();
    expect(JSON.stringify(manifest.exports)).not.toContain('"import"');
    expect(JSON.stringify(manifest.exports)).not.toContain('"svelte"');
    expect(manifest.peerDependencies.svelte).toBe(">=5.56.8 <6");
    expect(() => assertSvelteManifest(repoRoot)).not.toThrow();
  });

  test("two clean Svelte builds match and keep client/server lanes distinct", async () => {
    const first = await buildSvelte(repoRoot);
    const firstReceipt = readReceipt(svelteRoot);
    expect(first.receipt.package).toBe("@inflatable-cookie/poodle-svelte");
    expect(first.receipt.lanes).toEqual(["client", "server"]);
    expect(first.receipt.markdownPolicy).toBe("optional-peer-on-./markdown");
    expect(first.receipt.sourceMaps).toBe(false);
    expect(firstReceipt).not.toContain("/Users/");
    for (const path of sveltePublicFiles()) {
      expect(existsSync(join(svelteRoot, path))).toBe(true);
    }
    for (const name of INTERNAL_SVELTE_NAMES) {
      expect(existsSync(join(svelteRoot, "dist", `${name}.client.js`))).toBe(false);
      expect(existsSync(join(svelteRoot, "dist", `${name}.svelte.d.ts`))).toBe(false);
    }
    const jsWithSvelteName = readdirSync(join(svelteRoot, "dist"), { recursive: true })
      .map(String)
      .filter((name) => name.endsWith(".js") && name.includes(".svelte"));
    expect(jsWithSvelteName).toEqual([]);
    const buttonClient = readFileSync(join(svelteRoot, "dist/Button.client.js"), "utf8");
    const buttonServer = readFileSync(join(svelteRoot, "dist/Button.server.js"), "utf8");
    expect(buttonClient).toContain("svelte/internal/client");
    expect(buttonServer).toContain("svelte/internal/server");
    expect(buttonClient).toContain("@inflatable-cookie/poodle-core/styles/button.css");
    expect(buttonClient).not.toContain("markdown-editor.css");
    expect(buttonClient).not.toMatch(/from ["']marked["']/);
    expect(readFileSync(join(svelteRoot, "dist/index.client.js"), "utf8")).not.toMatch(
      /from ["']marked["']/,
    );
    expect(readFileSync(join(svelteRoot, "dist/AgentMessage.client.js"), "utf8")).toMatch(
      /from ["']marked["']/,
    );
    expect(readFileSync(join(svelteRoot, "dist/index.d.ts"), "utf8")).not.toContain("AgentMessage");
    expect(readFileSync(join(svelteRoot, "dist/index.d.ts"), "utf8")).not.toContain(
      "AgentTranscript",
    );
    expect(readFileSync(join(svelteRoot, "dist/index.d.ts"), "utf8")).not.toContain("MarkdownEditor");
    const markdownDts = readFileSync(join(svelteRoot, "dist/markdown.d.ts"), "utf8");
    expect(markdownDts).toContain("AgentMessage");
    expect(markdownDts).toContain("AgentTranscript");
    expect(markdownDts).toContain("MarkdownEditor");
    const buttonDts = readFileSync(join(svelteRoot, "dist/Button.svelte.d.ts"), "utf8");
    expect(buttonDts).not.toMatch(/declare const Button: Component;/);
    expect(buttonDts).toMatch(/disabled|onClick|variant/);

    const second = await buildSvelte(repoRoot);
    expect(second.receipt.outputs).toEqual(first.receipt.outputs);
    expect(readReceipt(svelteRoot)).toBe(firstReceipt);
    expect(sha256File(first.receiptPath)).toBe(sha256File(second.receiptPath));
  }, 120_000);

  test("Svelte declarations resolve under Bundler and NodeNext", () => {
    runTsc("bundler");
    runTsc("nodenext");
  }, 60_000);

  test("invalid Button/Select props fail unsuppressed under Bundler and NodeNext", () => {
    for (const resolution of ["bundler", "nodenext"] as const) {
      const negative = runNegativeTsc(resolution);
      expect(negative.status).not.toBe(0);
      expect(negative.output).toMatch(/TS2322/);
      expect((negative.output.match(/TS2322/g) ?? []).length).toBeGreaterThanOrEqual(6);
    }
  }, 60_000);

  test("client artifacts fail svelte/server while server artifacts render", async () => {
    const { render } = await import("svelte/server");
    const server = await import(join(svelteRoot, "dist/Button.server.js"));
    const rendered = render(server.default, { props: { type: "button" } });
    expect(rendered.body).toContain("poodle-button");
    const client = await import(join(svelteRoot, "dist/Button.client.js"));
    expect(() => render(client.default, { props: { type: "button" } }).body).toThrow();
  }, 30_000);

  test("oracle plants fail closed and restore", () => {
    const distDir = join(svelteRoot, "dist");
    const publicFiles = sveltePublicFiles();
    const plantedInternal = join(distDir, "MenuSurface.client.js");
    const plantedSource = join(distDir, "Button.svelte");
    writeFileSync(plantedInternal, "export default {};\n");
    try {
      expect(() =>
        auditStagedDist({ distDir, publicFiles, forbiddenModules: [] }),
      ).toThrow(/unexpected staged file dist\/MenuSurface\.client\.js/);
    } finally {
      rmSync(plantedInternal, { force: true });
    }
    writeFileSync(plantedSource, "<button></button>\n");
    try {
      expect(() =>
        auditStagedDist({ distDir, publicFiles, forbiddenModules: [] }),
      ).toThrow(/raw source is forbidden/);
    } finally {
      rmSync(plantedSource, { force: true });
    }
    const typesJs = join(distDir, "types.js");
    const typesBytes = readFileSync(typesJs);
    rmSync(typesJs);
    try {
      expect(() =>
        auditStagedDist({ distDir, publicFiles, forbiddenModules: [] }),
      ).toThrow(/missing staged public file/);
    } finally {
      writeFileSync(typesJs, typesBytes);
    }
    expect(() =>
      auditStagedDist({ distDir, publicFiles, forbiddenModules: [] }),
    ).not.toThrow();

    const buttonDts = join(distDir, "Button.svelte.d.ts");
    const selectDts = join(distDir, "Select.svelte.d.ts");
    const realButtonDts = readFileSync(buttonDts, "utf8");
    const realSelectDts = readFileSync(selectDts, "utf8");
    const shim = (stem: string) =>
      [
        `import type { Component } from "svelte";`,
        `declare const ${stem}: Component;`,
        `export default ${stem};`,
        "",
      ].join("\n");
    writeFileSync(buttonDts, shim("Button"));
    writeFileSync(selectDts, shim("Select"));
    try {
      const planted = runNegativeTsc("bundler");
      expect(planted.status).toBe(0);
    } finally {
      writeFileSync(buttonDts, realButtonDts);
      writeFileSync(selectDts, realSelectDts);
    }
    const restored = runNegativeTsc("bundler");
    expect(restored.status).not.toBe(0);

    const plantedImport = mkdtempSync(join(tmpdir(), "poodle-import-oracle-"));
    writeFileSync(
      join(plantedImport, "package.json"),
      `${JSON.stringify({
        name: "oracle-import",
        type: "module",
        exports: {
          ".": {
            types: "./index.d.ts",
            import: "./client.js",
            browser: "./client.js",
            default: "./server.js",
          },
        },
      })}\n`,
    );
    writeFileSync(join(plantedImport, "client.js"), "export const lane = 'client';\n");
    writeFileSync(join(plantedImport, "server.js"), "export const lane = 'server';\n");
    const consumer = mkdtempSync(join(tmpdir(), "poodle-import-consumer-"));
    writeFileSync(
      join(consumer, "package.json"),
      `${JSON.stringify({
        name: "oracle-import-consumer",
        type: "module",
        dependencies: { "oracle-import": `file:${plantedImport}` },
      })}\n`,
    );
    const install = spawnSync("bun", ["install"], { cwd: consumer, encoding: "utf8" });
    expect(install.status).toBe(0);
    const resolved = spawnSync(
      "node",
      ["--input-type=module", "-e", `process.stdout.write(import.meta.resolve("oracle-import"))`],
      { cwd: consumer, encoding: "utf8" },
    );
    expect(resolved.status).toBe(0);
    expect(resolved.stdout).toContain("client.js");
    rmSync(plantedImport, { recursive: true, force: true });
    rmSync(consumer, { recursive: true, force: true });

    const packInstallDiff = spawnSync(
      "git",
      ["diff", "--name-only", "origin/main", "--", "test/package-install"],
      { cwd: repoRoot, encoding: "utf8" },
    );
    expect(packInstallDiff.stdout.trim().split("\n").filter(Boolean).sort()).toEqual([
      "test/package-install/fixture/packed-types/tsconfig.slider-react-negative.json",
      "test/package-install/fixture/packed-types/tsconfig.slider-react-positive.json",
      "test/package-install/fixture/packed-types/tsconfig.tree-react-negative.json",
      "test/package-install/fixture/packed-types/tsconfig.tree-react-positive.json",
      "test/package-install/roster.ts",
      "test/package-install/web-preview.ts",
    ]);
  });
});

/**
 * Planted orphan coverage for `build-tokens.ts --check` and write mode.
 *
 * `--check` used to compare only files the generator still emits, so a
 * committed artifact the emitter no longer writes stayed green. Write mode
 * used to copy an artifact-root orphan into the core/Svelte mirror before
 * unlinking the source, so `--check` stayed red after regeneration. These
 * tests plant that file in the live artifact roots, then always unlink it.
 * The third root is the generated tree inside the poodle-tokens crate, where
 * the Rust artifact family has lived since the crate became self-contained.
 */

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, expect, test } from "bun:test";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const tokensDir = path.resolve(scriptDir, "..");
const script = path.join(scriptDir, "build-tokens.ts");
const planted = [
  path.join(tokensDir, "artifacts", "css", "stale-orphan.css"),
  path.join(tokensDir, "../core/src/tokens/generated/css", "stale-orphan.css"),
  path.join(tokensDir, "../contracts/tokens/src/generated", "stale-orphan.rs"),
];

afterEach(() => {
  for (const file of planted) {
    fs.rmSync(file, { force: true });
  }
});

function runScript(args: string[] = [], entryPoint = script): { status: number; output: string } {
  try {
    const output = execFileSync("bun", [entryPoint, ...args], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
    return { status: 0, output };
  } catch (error) {
    const execError = error as { status?: number; stdout?: string | Buffer; stderr?: string | Buffer };
    return {
      status: execError.status ?? 1,
      output: `${execError.stdout?.toString() ?? ""}${execError.stderr?.toString() ?? ""}`,
    };
  }
}

function runCheck(): { status: number; output: string } {
  return runScript(["--check"]);
}

test("a stale committed artifact the generator no longer emits fails --check", () => {
  fs.writeFileSync(planted[0], "/* planted orphan */\n");
  const result = runCheck();
  expect(result.status).not.toBe(0);
  expect(result.output).toContain("packages/tokens/artifacts/css/stale-orphan.css");
});

test("a stale core token mirror the generator no longer emits fails --check", () => {
  fs.writeFileSync(planted[1], "/* planted orphan */\n");
  const result = runCheck();
  expect(result.status).not.toBe(0);
  expect(result.output).toContain("packages/core/src/tokens/generated/css/stale-orphan.css");
});

test("a stale generated file inside the poodle-tokens crate fails --check", () => {
  fs.writeFileSync(planted[2], "// planted orphan\n");
  const result = runCheck();
  expect(result.status).not.toBe(0);
  expect(result.output).toContain(
    "packages/contracts/tokens/src/generated/stale-orphan.rs",
  );
});

test("write mode removes an artifact-root orphan from every root so --check passes", () => {
  fs.writeFileSync(planted[0], "/* planted orphan */\n");
  fs.writeFileSync(planted[1], "/* planted orphan */\n");
  fs.writeFileSync(planted[2], "// planted orphan\n");
  const write = runScript();
  expect(write.status).toBe(0);
  expect(fs.existsSync(planted[0])).toBe(false);
  expect(fs.existsSync(planted[1])).toBe(false);
  expect(fs.existsSync(planted[2])).toBe(false);
  const result = runCheck();
  expect(result.status).toBe(0);
});

test("the live token artifacts pass --check", () => {
  const result = runCheck();
  expect(result.status).toBe(0);
});

function withStyleFixture(run: (fixture: {
  writeJson: (relativePath: string, data: unknown) => void;
  build: () => ReturnType<typeof runScript>;
  read: (relativePath: string) => string;
  root: string;
}) => void): void {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "poodle-style-build-"));
  try {
    fs.mkdirSync(path.join(root, "tokens/scripts"), { recursive: true });
    fs.symlinkSync(path.resolve(tokensDir, "../../node_modules"), path.join(root, "node_modules"));
    fs.cpSync(path.join(tokensDir, "schema"), path.join(root, "tokens/schema"), { recursive: true });
    const entryPoint = path.join(root, "tokens/scripts/build-tokens.ts");
    fs.copyFileSync(script, entryPoint);
    run({
      root,
      writeJson: (relativePath, data) => {
        fs.writeFileSync(path.join(root, "tokens/schema", relativePath), JSON.stringify(data));
      },
      build: () => runScript([], entryPoint),
      read: (relativePath) => fs.readFileSync(path.join(root, relativePath), "utf8"),
    });
  } finally {
    fs.rmSync(root, { recursive: true, force: true });
  }
}

test("the style builder rejects a theme token written by a style mode or treatment", () => {
  withStyleFixture(({ writeJson, build }) => {
    const treatment = { fill: "outlined", surface: "border" };
    const color = { background: { canvas: { $value: "{color.background.surface}" } } };
    writeJson("modes/style/default.json", { treatment, color });
    const direct = build();
    expect(direct.status).not.toBe(0);
    expect(direct.output).toContain('Style "default" and theme');
    expect(direct.output).toContain("both override color.background.canvas");

    writeJson("modes/style/default.json", { treatment });
    writeJson("style-treatments/fill.json", { outlined: { color } });
    const expanded = build();
    expect(expanded.status).not.toBe(0);
    expect(expanded.output).toContain('Treatment "fill=outlined" and theme');
    expect(expanded.output).toContain("both override color.background.canvas");
  });
});

test("a treatment added in schema expands into CSS and a typed Rust mode", () => {
  withStyleFixture(({ writeJson, build, read }) => {
    writeJson("style-treatments/shape.json", {
      flat: { radius: { control: { $value: "0rem" } } },
    });
    writeJson("modes/style/default.json", {
      treatment: { fill: "filled", surface: "border", shape: "flat" },
    });
    const result = build();
    expect(result.status).toBe(0);
    const css = read("tokens/artifacts/css/poodle-style-default.css");
    expect(css).toContain("--poodle-radius-control: 0rem;");
    expect(css).toContain("--poodle-style-control-fill: var(--poodle-color-background-elevated);");
    expect(css).toContain(`@scope ([data-style="default"]) to ([data-style]) {
  [data-theme] {
    --poodle-style-control-fill: var(--poodle-color-background-elevated);
  }
}`);
    const rust = read("contracts/tokens/src/generated/styles.rs");
    expect(rust).toContain("shape: ShapeTreatment::Flat");
    expect(rust).toContain('("style.control.fill", "{color.background.elevated}")');
  });
});

test("shared component CSS cannot select on a colour theme", () => {
  withStyleFixture(({ root, build }) => {
    const styles = path.join(root, "core/src/styles");
    fs.mkdirSync(styles, { recursive: true });
    fs.writeFileSync(path.join(styles, "test.css"), '/* a comment containing [data-theme] */\n[data-theme = "iceberg"] .card {}\n');
    const result = build();
    expect(result.status).not.toBe(0);
    expect(result.output).toContain("test.css:2: shared CSS must not select on data-theme");
  });
});

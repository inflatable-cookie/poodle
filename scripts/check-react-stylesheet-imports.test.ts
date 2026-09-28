/**
 * Planted coverage for the React stylesheet-import gate.
 *
 * A component that renders `poodle-ui-presentation-provider` without
 * importing its core sheet must fail; the same file with the import must
 * pass. The gate runs hermetically against a throwaway mini-repo so the
 * test never mutates the live tree.
 */

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, expect, test } from "bun:test";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "..");
const gateSource = fs.readFileSync(path.join(repoRoot, "scripts", "check-react-stylesheet-imports.ts"), "utf8");

const tempRoots: string[] = [];

function fixtureRepo(): string {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "poodle-react-css-"));
  tempRoots.push(root);
  fs.mkdirSync(path.join(root, "scripts"), { recursive: true });
  fs.writeFileSync(path.join(root, "scripts", "check-react-stylesheet-imports.ts"), gateSource);
  return root;
}

function writeSheet(root: string): void {
  const dir = path.join(root, "packages", "core", "src", "styles");
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(
    path.join(dir, "ui-presentation-provider.css"),
    ".poodle-ui-presentation-provider { display: contents; }\n",
  );
}

function writeComponent(root: string, withImport: boolean): void {
  const dir = path.join(root, "packages", "react", "components", "src");
  fs.mkdirSync(dir, { recursive: true });
  const importLine = withImport
    ? 'import "@inflatable-cookie/poodle-core/styles/ui-presentation-provider.css";\n'
    : "";
  fs.writeFileSync(
    path.join(dir, "UiPresentationProvider.tsx"),
    `${importLine}export function UiPresentationProvider({ children }: { children: unknown }) {
  return <div className="poodle-ui-presentation-provider">{children}</div>;
}
`,
  );
}

function runGate(root: string): { status: number; output: string } {
  try {
    const output = execFileSync("bun", [path.join(root, "scripts", "check-react-stylesheet-imports.ts")], {
      cwd: root,
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

afterAll(() => {
  for (const root of tempRoots) {
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test("a React component that renders a shared class without its stylesheet fails", () => {
  const root = fixtureRepo();
  writeSheet(root);
  writeComponent(root, false);
  const result = runGate(root);
  expect(result.status).not.toBe(0);
  expect(result.output).toContain("packages/react/components/src/UiPresentationProvider.tsx");
  expect(result.output).toContain("poodle-ui-presentation-provider");
  expect(result.output).toContain("ui-presentation-provider.css");
});

test("the same component with the stylesheet import is green", () => {
  const root = fixtureRepo();
  writeSheet(root);
  writeComponent(root, true);
  const result = runGate(root);
  expect(result.status).toBe(0);
});

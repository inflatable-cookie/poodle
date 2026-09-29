/**
 * Planted coverage for the React stylesheet-import gate.
 *
 * A component that renders `poodle-ui-presentation-provider` without
 * importing its core sheet must fail; the same file with the import must
 * pass. The gate runs against a throwaway mini-repo so the test never
 * mutates the live tree.
 *
 * Each case used to spawn `bun <copied-script>`; on a host whose OS temp root
 * had grown to ~237k entries, bun's walk of that ancestor made one spawn cost
 * 11-12s and both cases died at bun's 5s default (Queue's gate for #59).
 * Calling the exported gate directly removes the spawn: measured 2026-09-29 at
 * load ~50, each case runs in <60ms, so the 5s default keeps >80x headroom and
 * no per-test cap is needed.
 */

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterAll, expect, test } from "bun:test";

import { runReactStylesheetImportGate } from "./check-react-stylesheet-imports.ts";

const tempRoots: string[] = [];

function fixtureRepo(): string {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "poodle-react-css-"));
  tempRoots.push(root);
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

afterAll(() => {
  for (const root of tempRoots) {
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test("a React component that renders a shared class without its stylesheet fails", () => {
  const root = fixtureRepo();
  writeSheet(root);
  writeComponent(root, false);
  const result = runReactStylesheetImportGate(root);
  expect(result.status).not.toBe(0);
  expect(result.output).toContain("packages/react/components/src/UiPresentationProvider.tsx");
  expect(result.output).toContain("poodle-ui-presentation-provider");
  expect(result.output).toContain("ui-presentation-provider.css");
});

test("the same component with the stylesheet import is green", () => {
  const root = fixtureRepo();
  writeSheet(root);
  writeComponent(root, true);
  const result = runReactStylesheetImportGate(root);
  expect(result.status).toBe(0);
});

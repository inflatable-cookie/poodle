import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { runTypeProof, type TypeProofCase } from "./type-proof";

const roots: string[] = [];
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });
function fixture(source: string, compiler = true): string {
  const root = mkdtempSync(join(tmpdir(), "type-proof-test-"));
  roots.push(root);
  mkdirSync(join(root, "packed-types"));
  writeFileSync(join(root, "packed-types", "case.ts"), source);
  writeFileSync(join(root, "packed-types", "tsconfig.json"), JSON.stringify({ compilerOptions: { strict: true, noEmit: true, skipLibCheck: true, types: [] }, files: ["case.ts"] }));
  if (compiler) {
    mkdirSync(join(root, "node_modules", ".bin"), { recursive: true });
    symlinkSync(resolve(import.meta.dir, "../../node_modules/.bin/tsc"), join(root, "node_modules", ".bin", "tsc"));
  }
  return root;
}
const positive: TypeProofCase = { outcome: "pass", file: "case.ts", config: "tsconfig.json" };
const negative: TypeProofCase = { outcome: "fail", importPath: "example", file: "case.ts", config: "tsconfig.json", diagnostic: "error TS2322" };

test("records a successful compile", async () => {
  const result = await runTypeProof(fixture("const value: string = 'ok';"), [positive]);
  expect(result.positives).toEqual([{ file: "case.ts", exitCode: 0, diagnostics: [] }]);
});
test("rejects the wrong diagnostic", async () => {
  await expect(runTypeProof(fixture("missingName;"), [negative])).rejects.toThrow("wrong diagnostic");
});
test("rejects a suppressed negative", async () => {
  await expect(runTypeProof(fixture("// @ts-expect-error\nconst value: string = 1;"), [negative])).rejects.toThrow("suppresses its expected failure");
});
test("rejects a missing compiler", async () => {
  await expect(runTypeProof(fixture("", false), [positive])).rejects.toThrow("did not install a TypeScript compiler");
});

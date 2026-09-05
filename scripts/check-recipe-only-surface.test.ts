/**
 * Planted tests for the retired-token drift gate (g16.108 item 6,
 * operator-authorized 2026-09-05).
 *
 * The gate (scripts/check-recipe-only-surface.ts) treats docs/logs,
 * docs/parity, docs/roadmaps, and docs/archive as historical prefixes, and
 * fails on any retired recipe-token CSS-variable reference in every other
 * scanned file. The forbidden literals below are assembled at runtime so the
 * gate never trips on its own test source. These tests run the gate
 * hermeticly: the script is copied into a throwaway mini-repo whose only
 * files are the planted fixtures, so the test never mutates the real working
 * tree and proves both directions:
 *
 * 1. an active-path reference under docs/guides/ still fails the gate, while
 *    the same wording under docs/archive/parity/ stays exempt — archived
 *    content is never edited to satisfy a gate;
 * 2. a repo whose only retired-token references live under the historical
 *    prefixes is green.
 */

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, expect, test } from "bun:test";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "..");
const gateSource = fs.readFileSync(path.join(repoRoot, "scripts", "check-recipe-only-surface.ts"), "utf8");

const tempRoots: string[] = [];

function fixtureRepo(): string {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "poodle-recipe-drift-"));
  tempRoots.push(root);
  fs.mkdirSync(path.join(root, "scripts"), { recursive: true });
  fs.writeFileSync(path.join(root, "scripts", "check-recipe-only-surface.ts"), gateSource);
  return root;
}

function runGate(root: string): { status: number; output: string } {
  try {
    const output = execFileSync("bun", [path.join(root, "scripts", "check-recipe-only-surface.ts")], {
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

// Assembled at runtime: a literal occurrence in this file would trip the
// very gate under test.
const retiredPrefix = "--poodle-" + "treat" + "ment-";
const plantedLine = `Svelte references the \`${retiredPrefix}surface-elevated-*\` fallback vars.`;

afterAll(() => {
  for (const root of tempRoots) {
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test("an active-path reference under docs/guides/ still fails the gate", () => {
  const root = fixtureRepo();
  const guidesDir = path.join(root, "docs", "guides");
  fs.mkdirSync(guidesDir, { recursive: true });
  fs.writeFileSync(path.join(guidesDir, "planted.md"), `# Planted\n\n- ${plantedLine}\n`);
  // Same wording under the archive must stay exempt: archived content is
  // evidence and is never edited to satisfy a gate.
  const archiveDir = path.join(root, "docs", "archive", "parity");
  fs.mkdirSync(archiveDir, { recursive: true });
  fs.writeFileSync(path.join(archiveDir, "legacy-audit.md"), `# Legacy\n\n- ${plantedLine}\n`);

  const result = runGate(root);

  expect(result.status).not.toBe(0);
  expect(result.output).toContain("docs/guides/planted.md");
  expect(result.output).not.toContain("docs/archive/parity/legacy-audit.md");
});

test("archived parity content alone is green (no active-path reference)", () => {
  const root = fixtureRepo();
  const archiveDir = path.join(root, "docs", "archive", "parity");
  fs.mkdirSync(archiveDir, { recursive: true });
  fs.writeFileSync(path.join(archiveDir, "legacy-audit.md"), `# Legacy\n\n- ${plantedLine}\n`);
  const parityDir = path.join(root, "docs", "parity");
  fs.mkdirSync(parityDir, { recursive: true });
  fs.writeFileSync(
    path.join(parityDir, "README.md"),
    "# Pointer\n\nThe audits live under docs/archive/parity and mention the retired token layer.\n",
  );

  const result = runGate(root);

  expect(result.status).toBe(0);
});

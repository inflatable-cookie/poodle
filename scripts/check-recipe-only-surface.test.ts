/**
 * Planted tests for the retired-token drift gate (g16.108 item 6,
 * operator-authorized 2026-09-05).
 *
 * The gate (scripts/check-recipe-only-surface.ts) treats superseded specs
 * under docs/knowledge/specs/archive/ as historical, and fails on any retired
 * recipe-token CSS-variable reference in every other scanned file. The forbidden literals below are assembled at runtime so the
 * gate never trips on its own test source. These tests run the gate in-process
 * against a throwaway mini-repo whose only files are the planted fixtures, so
 * the test never mutates the real working tree and proves both directions:
 *
 * 1. an active-path reference under docs/guides/ still fails the gate, while
 *    the same wording under the specs archive stays exempt — archived
 *    content is never edited to satisfy a gate;
 * 2. a repo whose only retired-token references live under the historical
 *    prefix is green.
 *
 * The fixtures used to spawn `bun <copied-script>` once per case. On a host
 * whose OS temp root had accumulated ~237k entries, each spawn cost 7-9s
 * (bun walks the entry-heavy ancestor) and both cases died at bun's 5s
 * default — Queue's gate for #59. Calling the exported scan directly removes
 * the spawn: measured 2026-09-29 at load ~50, each case runs in <5ms, so the
 * 5s default keeps >1000x headroom and no per-test cap is needed.
 */

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterAll, expect, test } from "bun:test";

import { scanRetiredTreatment } from "./check-recipe-only-surface.ts";

const tempRoots: string[] = [];

function fixtureRepo(): string {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "poodle-recipe-drift-"));
  tempRoots.push(root);
  return root;
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

test("an active-path reference under docs/guides/ still fails the gate", async () => {
  const root = fixtureRepo();
  const guidesDir = path.join(root, "docs", "guides");
  fs.mkdirSync(guidesDir, { recursive: true });
  fs.writeFileSync(path.join(guidesDir, "planted.md"), `# Planted\n\n- ${plantedLine}\n`);
  // Same wording under the archive must stay exempt: archived content is
  // evidence and is never edited to satisfy a gate.
  const archiveDir = path.join(root, "docs", "knowledge", "specs", "archive");
  fs.mkdirSync(archiveDir, { recursive: true });
  fs.writeFileSync(path.join(archiveDir, "legacy-audit.md"), `# Legacy\n\n- ${plantedLine}\n`);

  const result = await scanRetiredTreatment(root);

  expect(result.status).not.toBe(0);
  expect(result.output).toContain("docs/guides/planted.md");
  expect(result.output).not.toContain("docs/knowledge/specs/archive/legacy-audit.md");
});

test("archived spec content alone is green (no active-path reference)", async () => {
  const root = fixtureRepo();
  const archiveDir = path.join(root, "docs", "knowledge", "specs", "archive");
  fs.mkdirSync(archiveDir, { recursive: true });
  fs.writeFileSync(path.join(archiveDir, "legacy-audit.md"), `# Legacy\n\n- ${plantedLine}\n`);

  const result = await scanRetiredTreatment(root);

  expect(result.status).toBe(0);
});

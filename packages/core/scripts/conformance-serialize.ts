/**
 * Serialize conformance authority (spec 066): portable interfaces, case
 * corpora, and the primitive capability roster into neutral JSON fixtures.
 *
 *   bun packages/core/scripts/conformance-serialize.ts
 *   bun packages/core/scripts/conformance-serialize.ts --check
 */

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { serializeInterface } from "../src/conformance/define";
import { buttonInterface } from "../src/conformance/button";
import { buttonCases } from "../src/conformance/button-cases";
import { serializePrimitiveRoster } from "../src/conformance/primitives";

const CHECK = process.argv.includes("--check");

const fixturesRoot = join(
  fileURLToPath(new URL("..", import.meta.url)),
  "..",
  "codegen",
  "fixtures",
  "conformance",
);

const artifacts: Array<{ path: string; document: string; label: string }> = [
  {
    path: join(fixturesRoot, "button-interface.json"),
    document: `${JSON.stringify(serializeInterface(buttonInterface), null, 2)}\n`,
    label: "button interface",
  },
  {
    path: join(fixturesRoot, "button-cases.json"),
    document: `${JSON.stringify(buttonCases, null, 2)}\n`,
    label: "button cases",
  },
  {
    path: join(fixturesRoot, "primitive-capability-roster.json"),
    document: `${JSON.stringify(serializePrimitiveRoster(), null, 2)}\n`,
    label: "primitive capability roster",
  },
];

let failed = false;
for (const artifact of artifacts) {
  if (CHECK) {
    let committed: string;
    try {
      committed = readFileSync(artifact.path, "utf8");
    } catch {
      console.error(`conformance fixture missing: ${artifact.path} — run conformance:build`);
      failed = true;
      continue;
    }
    if (committed === artifact.document) {
      console.log(`Conformance ${artifact.label} is current.`);
    } else {
      console.error(
        `Conformance ${artifact.label} is stale under ${artifact.path} — run conformance:build`,
      );
      failed = true;
    }
  } else {
    mkdirSync(artifact.path.split("/").slice(0, -1).join("/"), { recursive: true });
    writeFileSync(artifact.path, artifact.document);
    console.log(`Authored conformance ${artifact.label} (${artifact.document.length} bytes).`);
  }
}

if (failed) {
  process.exit(1);
}

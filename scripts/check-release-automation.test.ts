import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { collectReleaseWorkflowFailures, releaseCertificateRunBlocks } from "./check-release-automation";
import { readNpmPublicationAuthority } from "./npm-publication";

const root = join(import.meta.dir, "..");
const release = readFileSync(join(root, ".github/workflows/release.yml"), "utf8");
const publication = readNpmPublicationAuthority(root);
const certificateBlocks = releaseCertificateRunBlocks(release);

test("accepts the certificate in a literal run block", () => {
  expect(collectReleaseWorkflowFailures(certificateBlocks.single, publication)).toEqual([]);
});

test("counts a second certificate invocation in a run block", () => {
  expect(collectReleaseWorkflowFailures(certificateBlocks.duplicate, publication).join("\n"))
    .toMatch(/exactly one Effigy certificate entry/);
});

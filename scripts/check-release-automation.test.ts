import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { collectReleaseWorkflowFailures } from "./check-release-automation";
import { readNpmPublicationAuthority } from "./npm-publication";

const root = join(import.meta.dir, "..");
const release = readFileSync(join(root, ".github/workflows/release.yml"), "utf8");
const publication = readNpmPublicationAuthority(root);
const certificate = "effigy release:web-certificate";

test("accepts the certificate in a literal run block", () => {
  const source = release.replace(
    `run: ${certificate}`,
    `run: |- # certify the candidate\n          # shell comment\n          ${certificate}`,
  );
  expect(collectReleaseWorkflowFailures(source, publication)).toEqual([]);
});

test("counts a second certificate invocation in a run block", () => {
  const source = release.replace(
    `run: ${certificate}`,
    `run: |\n          ${certificate}\n          ${certificate}`,
  );
  expect(collectReleaseWorkflowFailures(source, publication).join("\n"))
    .toMatch(/exactly one Effigy certificate entry/);
});

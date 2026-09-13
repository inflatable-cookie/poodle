// g18.032 / spec 071: the changed-range admission selector.
//
// Separate from archive certification on purpose. It only classifies the
// compared commit range; it builds, packs and installs nothing.

import { resolve } from "node:path";

import { requireExactCommit, resolveCertificationHead, runCapture } from "./scope";
import { assertWebCandidateScope } from "./web-candidate";

const repoRoot = resolve(import.meta.dir, "../..");

const checkedOutCommit = (await runCapture(["git", "rev-parse", "HEAD"], repoRoot)).trim();
const proofCommit = await resolveCertificationHead(repoRoot, checkedOutCommit);
const requiredBaseCommit = requireExactCommit(
  globalThis.process.env.POODLE_WEB_PACK_INSTALL_BASE_COMMIT ??
    (await runCapture(["git", "merge-base", proofCommit, "origin/main"], repoRoot)).trim(),
  "required base commit",
);

const proof = await assertWebCandidateScope(repoRoot, requiredBaseCommit, proofCommit);

process.stdout.write(
  `${JSON.stringify(
    {
      schema: "poodle.web-candidate-admission.v1",
      frozenReleaseInputCommit: proof.frozenReleaseInputCommit,
      requiredBaseCommit,
      sourceCommit: proofCommit,
      sourceVersion: proof.sourceVersion,
      targetVersion: proof.targetVersion,
      releaseNotePath: proof.releaseNotePath,
      releaseInputCount: proof.releaseInputPaths.length,
      changedPathCount: proof.changedPaths.length,
    },
    null,
    2,
  )}\n`,
);

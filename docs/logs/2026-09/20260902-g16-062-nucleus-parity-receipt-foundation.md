# g16.062 — Nucleus parity receipt foundation

Date: 2026-09-02
Status: implementation complete; worker PR review pending

## Outcome

The Nucleus denominator is now a checked-in manifest of 29 rendered
components. `IconProvider` is recorded as one separate construction
prerequisite, not row 30. The manifest pins `poodle-gpui-preview` 0.3.0, the
workspace distribution, source commit
`20d340383a684f3c0c07c21bb5c8b344c4be8890`, and the GPUI preview Cargo.lock
resolution.

The real `effigy regressions:native` selector emits a deterministic receipt
only after `HeadlessDriver` has painted and dispatched input through the GPUI
test platform. The canonical Button receipt is:

- `docs/roadmaps/g16/nucleus-parity-receipts/button--nucleus-shell-button.json`
- schema `poodle.g16-nucleus-parity-receipt.v1`
- proof `M1`, runtime `gpui-headless`, outcome `passed`
- lockfile SHA-256 `8bb8f8edaba8f381b9dec39532f5299231d2dfaa1c4509c7f87e41ca27711a55`

The generated ledger now reports one mounted Nucleus row (Button) and 28
missing rendered rows. Its historical map retains 57 expected component
entries across 65 named tests as planning traceability only. Switch remains
`missing` despite its mapped regression name because no validated receipt
exists.

## Review oracle

- Wrong source commit, runtime, direct-handler observation, and `A1` proof
  substitutions are rejected.
- Extra properties forbidden by the manifest and receipt schemas are rejected
  by the production validator, including nested objects.
- Empty M1 `artifact_paths` remain valid; every nonempty artifact must name a
  repository-relative regular file and match its SHA-256 digest.
- Promoting `IconProvider` to row 30 is rejected.
- An unmanifested receipt component is rejected.
- `M1` does not infer `A1` or `V1`.

## Validation

- `effigy regressions:native` — 174 passed.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — passed.
- `effigy docs:check` — passed.
- `effigy qa` — passed.
- `git diff --check` — passed.

No windowed or native-visual selector ran. The first QA attempt correctly
rejected a temporary `Cargo.toml` dependency mutation as a forbidden
certification-scope surface; that mutation was removed before the final
passing QA run.

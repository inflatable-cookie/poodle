# g18.030 — Nucleus receipt lock provenance

Status: repinned — implementation and complete current cohort evidence are
committed; the PR opens for exact-head Queue review, CI observation and merge
Date: 2026-09-12
Branch: `ns-54e5251f-3cfe-4e0a-95de-f25b814304e5`
Card: `docs/roadmaps/g18/030-nucleus-receipt-lock-provenance.md`
Handoff: `docs/handoffs/20260912-222500-g18-030-nucleus-receipt-lock-provenance.md`
Base: pushed `main` at `a057041942364989b3bbd31428df5c30b029f341` (planning
promotion that carries this handoff)

## Outcome

`packages/gpui/preview/src/nucleus_receipts.rs` no longer embeds the preview
lockfile SHA-256 or any Poodle release version. The M1 and A1 emitters read the
exact `packages/gpui/preview/Cargo.lock` bytes once, hash those bytes, and
derive the canonical five-package resolution — `gpui` (`crates.io`, with
checksum), then `poodle-gpui`, `poodle-gpui-preview`, `poodle-node`, and
`poodle-render` (`workspace`) — in fixed order.

A small fail-closed `[[package]]` parser replaces the literals. A selected
package that is missing, duplicated, incomplete, carries an unexpected source
(`gpui` without a `registry+` source, or a workspace package with any source),
carries a workspace checksum, or is a registry entry without a well-formed
64-hex checksum rejects emission before any receipt is published. Receipt
schema, field order, package order and byte shape are otherwise unchanged.

The emitter source is inside the mounted runtime identity
`scripts/nucleus-parity-receipts.ts` binds (`packages/gpui/preview`), so the
accepted implementation required one complete current headless repin. The
cohort was regenerated once on the frozen implementation identity
`73c99bed7062fd3e22ffedbd045f8d2ac79a9b93` (254 headless tests passed, 0
failed: the 242 regression tests plus 12 new focused provenance laws). Every
generated receipt now differs from the superseded `390095f6f` cohort only in
`source_commit` (and `run_id` for census receipts); lock hash, lock resolution,
observations and captures are byte-identical.

## Provenance proof

- Current lock: derived `lockfile_sha256` equals the SHA-256 of the committed
  lock bytes (`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`)
  and the derived resolution reports the lock's own `gpui 0.2.2` crates.io entry
  plus `0.3.0` for the four workspace packages; the preview entry equals
  `CARGO_PKG_VERSION`. No test embeds that hash or those versions.
- Planted release: a disposable lock carrying `0.4.0` for the four workspace
  packages derives a changed SHA-256 (equal to the SHA-256 of those exact bytes)
  and reports `0.4.0` for every Poodle package with the external `gpui` entry
  untouched. `0.4.0` preparation therefore requires no edit to this source.
- Negative laws: changed bytes change the hash; missing, duplicate,
  incomplete, workspace-with-source, workspace-with-checksum, registry-without-
  source, unexpected-source and malformed-checksum locks all reject.

## Repinned evidence

- 29 M1 + 29 A1 receipts in
  `docs/evidence/nucleus/nucleus-parity-receipts/` and the manifest
  `resolution.source_commit` in `docs/evidence/nucleus/nucleus-parity-manifest.json`.
- 65 mounted receipts, the execution record
  (`run_id 2026-09-12-g18-030-receipt-lock-provenance-expected`) and the two
  generated census documents in `docs/evidence/gpui/`.
- The parity ledger is derived from receipt paths, not receipt identity, and
  regenerates byte-identical; it is unchanged by this repin.

## Validation

- Focused `cargo test ... --test headless_regressions receipt_lock`: 12 passed,
  0 failed.
- One complete headless emission, `effigy regressions:native`: 254 passed,
  0 failed, publishing all 58 M1/A1 receipts.
- `effigy test:nucleus-parity-receipts`: 17 pass / 0 fail;
  `effigy check:parity-evidence-ledger`: 176 rows validated;
  `effigy check:gpui-census`: checked-in artifacts match the generator;
  `effigy docs:lint` green; `git diff --check` clean.
- Exact-head GitHub `web`/`rust` checks, review and merge are Queue-owned.

## Boundaries held

No Cargo manifest or lockfile, version, changelog, release note, workflow,
public-surface policy, candidate, tag, registry, Desktop or retained g18.006
mutation. The diff is one emitter source file plus generated evidence and this
log.

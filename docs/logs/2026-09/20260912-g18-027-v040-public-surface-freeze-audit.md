# g18.027 — `v0.4.0` public-surface freeze audit

Status: complete — merged as `8a1f7dc4cb7fa23077c06bc2ad5040db20760182` (PR #259) on 2026-09-12 after exact-head independent review
Date: 2026-09-12
Branch: `ns-b9e45c4b-f9cd-48ff-b0fa-8bfe910b8d2e`
Card: `docs/roadmaps/g18/027-v040-public-surface-freeze-audit.md`
Handoff: `docs/handoffs/20260912-g18-027-v040-public-surface-freeze-audit.md`
Governing refs: `docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
`docs/specs/044-deprecation-change-control-and-release-channel-operations.md`,
`docs/specs/070-compiled-web-distribution-contract.md`,
`packages/release-manifest.json`
Base: pushed `main` at `31d529a6f0dd4dc632b11010cf1e16a29e6eb80b`
(post-g18.026, PR #258 merge `bfc0783b2020f6be7158e7f1a3cd74c4ed314be4`)

## Merge and review

- PR #259 merged `8a1f7dc4cb7fa23077c06bc2ad5040db20760182` on 2026-09-12 with
  parents `31d529a6f0dd4dc632b11010cf1e16a29e6eb80b` (post-g18.026 main) and
  `e4ff3f89b7e572cff84c7618d29ff4ab758467cf` (reviewed audit head): the merge
  matches the reviewed head exactly, no refresh was needed.
- Independent review (issue comment `5647015250`, `ready_to_merge`) re-derived
  every identity, count and claim from the diff at the exact head: 65 breaking
  rows enumerated with migration text, 433 additive and 12 internal-only rows
  verified, reproducible auditor output confirmed, no unclassified break.
- Merge-time checks were green (`rust`, `web`); the branch diff stayed inside
  the owned audit paths with no version, lock, changelog, release-note,
  workflow, Desktop, candidate or tag mutation.

## Outcome

The frozen `v0.3.0` → post-g18.026 public delta is classified in
`docs/evidence/releases/g18-027-v040-public-surface-delta.md`: 433 additive,
65 breaking, 12 internal-only rows, each breaking row with consumer migration
text. `0.4.0` is frozen against that table; any later public break must target
`0.5.0`.

Reported identities: baseline tag `v0.3.0`
(`85609d941a208ff2f854e9f7c0e457089cc77d0e`, tree
`d7a4a6533114ca8cd4864a5d62dc5c56a414917f`); final main
`31d529a6f0dd4dc632b11010cf1e16a29e6eb80b`, tree
`270547d0f033d882c7b736be8d24c68cbc5a9bce`. `packages/release-manifest.json`
and `packages/release-operations.json` are byte-identical across the range.

## Execution

- Built a read-only cross-ref auditor, `scripts/audit-public-surface-delta.ts`,
  because no existing route compared two immutable refs and classified the
  delta. It reads both trees through `git show`/`git ls-tree` and inventories
  package entries, dependency/peer constraints, root and subpath exports
  (core/Svelte/packed React), Svelte prop/default surfaces, packed React prop
  types, the icon registry, token custom properties, public recipe hooks, and
  Rust `pub` declarations for the manifest's public-intent crates.
- Added `scripts/audit-public-surface-delta.test.ts` with planted negatives for
  removed export, removed package entry, changed prop default, removed recipe
  hook, moved internal variable, removed/reshaped Rust item, and added-file
  classes, plus the type-alias false positive. 15 tests, 65 assertions, green.
- Classified the 65 breaking rows into five groups: Slider/RangeSlider
  presentation API replacement; Markdown safe-HTML default; 18 removed public
  recipe hooks; React-only parity removals; and Rust source/tag API changes.
  Recorded 12 renamed implementation variables as internal-only.
- Wrote the compatible behavioral section (Tabs card fill and pinned reorder,
  Slider/RangeSlider normalization and display-text laws, Select navigation,
  input-modality focus chrome) and the additive families (new entries,
  components, 91 core exports, 6 icons, token/theme syntax ramps).
- Stopped at the audit boundary: no product, Rust, contract, version, lock,
  changelog, release-note, workflow, Desktop, g18.006 or g18.009 change.

## Findings that need a decision

g18.026's execution log claimed "Public API is unchanged", but the shared
family foundation replaced the hook names read by RangeSlider and dropped four
Slider hooks: 18 documented `--poodle-recipe-*` hooks are no longer read. The
Slider and RangeSlider contracts still list them, so the contract text now
overstates the implementation. The report records this as a breaking row with a
successor table and returns a decision capsule recommending the contract
recipe-hook lists be corrected in the post-release docs sweep. No product
source fix is required, and the alternative (forwarding the old RangeSlider
names) would be a pre-v1 compatibility shim.

## Validation

- `effigy audit:public-surface` — planted-negative suite plus the frozen v0.3.0
  cross-ref summary; exits 0.
- `bun scripts/audit-public-surface-delta.ts --base v0.3.0 --head 31d529a6f0dd4dc632b11010cf1e16a29e6eb80b` — reproducible, exits 0.
- `bun test scripts/audit-public-surface-delta.test.ts` — 15 pass, 0 fail.
- `effigy check:release-automation`, `effigy docs:lint`, `git diff --check`.
- No version, lock, changelog, release-note, workflow, Desktop, candidate or
  tag mutation; `git status` shows only this audit's files.

## Closeout (integration checkout, 2026-09-12)

- Verified local `main` clean at `8a1f7dc4cb7fa23077c06bc2ad5040db20760182`,
  matching `origin/main`; merge parents confirm the reviewed head merged
  without refresh.
- `git diff --check` clean; `effigy docs:lint` green on the closeout tree.
- Deferred, not repaired here: the decision capsule stands — the Slider and
  RangeSlider contract recipe-hook lists still overstate the implementation
  (18 documented `--poodle-recipe-*` hooks no longer read) and are corrected
  in the post-release docs sweep, never as a pre-v1 shim.

## Next task

Retained g18.006 consumes this report: re-run the auditor against the candidate,
require no breaking row absent from the report, then prepare `0.4.0`.

# HistoryCenter Release And Loophole Adoption Decision

Status: planning packet for orchestrator review; no publication or sibling
mutation from this branch
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Scope: publish the corrected Poodle preview packages, then Loophole pin and
rejection mapping
Promotion authority: orchestrator after accepted review and merge; merge is
intake only
Handoff: `docs/handoffs/20260901-230400-history-release-adoption-planning.md`

This packet locks the release/adoption mechanics named by that handoff. It is
not a tag, workflow dispatch, npm publish, roadmap card, or Loophole PR.

## Settled Decisions Preserved

- The operator authorized this release/adoption lane on 2026-09-01.
- `g16.033` is complete: five Poodle-owned refusal codes and packed v3
  `HistoryEntry` proof, merged in PR #120 as `df0c7acd9` (implementation
  `ffe357d50753a6eb94c38bc77a585a3fdac57266`).
- Poodle publication precedes Loophole pin movement and rejection mapping.
- Longhorn `AlreadyAtTarget` is complete and must not be reopened. Canonical
  receipts: Longhorn PR #20 at `c216fd79012d77b8da00b67c88de80a5f26d9794`;
  Loophole wire-code adoption PR #16 at
  `8699c76fa06190bf2cef01d822a6834dd4a8067d`.
- Poodle retains review/merge authority for Poodle work. Loophole owns
  adoption.
- No compatibility shim, alias, or silent fallback for the removed v2
  `HistoryEntry.branchCount` surface.

## Packet Authority

No operator conversation ran in this delegate thread. The handoff forbade
re-asking settled product choices and required this packet to fix versions,
package set, selectors, provenance/rollback receipts, stop conditions, and the
repository boundary from current release docs and live registry/tag evidence.

Those locked values are this packet's decisions for orchestrator review. They
are not yet a candidate SHA, a tag, or a published package.

## Candidate Versions

**Publish `0.2.4`.** Do not tag or publish `0.2.3`.

| Identity | Value | Role |
| --- | --- | --- |
| Last published npm `latest` | `0.2.2` | `@inflatable-cookie/poodle-core` and `@inflatable-cookie/poodle-svelte` |
| Last published Git tag | `v0.2.2` at `d5607def24c6833913df1b5dcfa06372fcd5dd81` | immutable; never move, delete, or reuse |
| In-tree lockstep today | `0.2.3` | unpublished ContextMenu bump at `ea1763786`; no `v0.2.3` tag on `origin` |
| Candidate | **`0.2.4`** | next lockstep for all release-bearing manifests |
| Tag to create later | `v0.2.4` | lightweight tag on the accepted candidate SHA only |
| Loophole desktop pin today | exact `0.2.2` | `apps/desktop/package.json`; move only after npm `latest` is `0.2.4` |

Why `0.2.4` rather than tagging current `0.2.3`:

- npm `latest` is still `0.2.2`. Source already carries `0.2.3` for triggerless
  `ContextMenu` (`docs/release-notes/0.2.3.md`, `CHANGELOG.md` dated
  2026-08-30). That version was never tagged or published.
- `g16.033` and the rest of post-`v0.2.2` main landed after that bump. Current
  HEAD is not the ContextMenu-only tree the `0.2.3` notes describe.
- Publishing HEAD as `0.2.3` would make the existing notes and package READMEs
  false. Reusing `0.2.3` on npm after skipping a tag is also a consumer trap.
- The adoption driver versus published `0.2.2` is breaking: packed Svelte
  `HistoryEntry` is v3 (`continuationCount`; `branchCount` must not exist).
  That change deserves its own published identity.

The candidate worker lockstep-bumps every release-bearing TypeScript manifest
and every `packages/**/Cargo.toml` from `0.2.3` to `0.2.4`, including
intra-repository version requirements and generated `poodle-codegen`
stamps. Private tooling stays put: repository-root `package.json` `0.1.0`;
`packages/tokens`, both web previews, and `packages/svelte/install-smoke`
remain `0.0.0`.

Change class for the HistoryCenter delta versus last published `0.2.2`:
**breaking** (removed v2 `HistoryEntry` / `branchCount`) plus **additive**
(five-code rejection surface; unpublished `ContextMenu` `trigger={false}`;
other public-intent work already on `main`). Pre-1.0 may ship that mix in a
patch. Release notes must still name the break and the downstream re-check.

`0.2.3` notes stay in git as an unpublished lockstep. The `0.2.4` notes must
say `0.2.3` was prepared and never tagged, and must inventory the public
delta from tag `v0.2.2`, not from the in-tree `0.2.3` bump. Amend package
READMEs that currently tell consumers to install unpublished `0.2.3`.

Do not invent a HistoryCenter-only cherry-pick off `v0.2.2`. Lockstep tags
the whole tree. The candidate freezes a SHA of `main` after required stops
are clear. `g16.036` is already on this `main` and rides the tag. Other
unrelated in-flight cards are not waits; a candidate-bearing merge after
freeze replaces the SHA and requires a full rerun.

## Package Set

Preserve the `v0.2.2` publication set from spec 022, `packages/release-manifest.json`,
and `.github/workflows/release.yml`. Do not edit the workflow.

### Publish to npm (preview channel)

| Package | Path | `0.2.4` disposition |
| --- | --- | --- |
| `@inflatable-cookie/poodle-core` | `packages/core` | trusted-publish |
| `@inflatable-cookie/poodle-svelte` | `packages/svelte/components` | trusted-publish |

Consumers must install both at exactly `0.2.4`.

### Pack and certify, do not publish

| Package | Path | Disposition |
| --- | --- | --- |
| `@inflatable-cookie/poodle-react` | `packages/react/components` | experimental; packed and certified; no npm publish |

`check:release-automation` already forbids a React publish step.

### Lockstep Rust (source/tag distribution, not crates.io)

All 17 `packages/**/Cargo.toml` files carry `0.2.4` so tag `v0.2.4` names the
Rust graph too:

- contracts: `poodle-adapter`, `poodle-events`, `poodle-headless`, `poodle-ir`,
  `poodle-layout`, `poodle-markdown`, `poodle-node`, `poodle-specs`,
  `poodle-style`, `poodle-tokens`;
- composition: `poodle-render`;
- backends: `poodle-gpui`, `poodle-gpui-node-backend`, `poodle-jetstream`;
- internal crates that still lockstep because the tag check walks every
  `packages/**/Cargo.toml`: `poodle-codegen`, `poodle-gpui-preview`,
  `poodle-jetstream-preview`.

Jetstream remains program-deferred. Tagging its adapter/preview crates is
lockstep identity, not admission.

### Out of the denominator

Internal TypeScript tooling and the private root package do not move.
Downstream repos must not depend on them.

## Release Selectors

Use current Effigy names. `test:svelte-pack-install` is an alias of
`test:web-pack-install`. The configured release authority is one headless gate:
`effigy.toml` `[release.gates.headless]` runs `effigy qa`.

### Required Papercuts worker before freeze

`effigy qa` is red on current `main` because `audit:security` matches
`sk-plus-translated-hi…` inside the English phrase
`mask-plus-translated-highlight` (`scripts/audit-repository-security.ts`
OpenAI pattern has no left boundary). Hits remain in `PAPERCUTS.md`, the
`g16.033` handoff, and the `g16.033` / `g16.034` logs.

That worker is a papercut lane. Its workspace **must** carry the capitalized
label `Papercuts` before launch. Scope: anchor the matcher (the recorded
plausible fix is `\b`) and prove real keys still match. Do not fold this into
version/lock churn unless the orchestrator explicitly collapses the two
workers.

Until that board is green, no candidate SHA is freezeable.

### Candidate worker (Poodle, no release mutation)

Two commits, same pattern as `g15.060`:

1. candidate tree: versions, requirements, locks, generated stamps, changelog,
   `docs/release-notes/0.2.4.md`, honesty edits to READMEs/`0.2.3` notes;
2. evidence-only receipt naming that candidate SHA. The receipt must not
   repin it.

Run every gate from a clean checkout of the candidate commit. Tarballs stay
outside the tracked tree (ignored `.artifacts/` or equivalent).

Required evidence from that exact SHA:

- lockstep agreement vs `tag=0.2.4` (replica of the `Versions agree with the
  tag` step in `release.yml`): three public TypeScript manifests plus all 17
  Cargo manifests;
- `bun.lock` workspace versions actually moved (known papercut: ordinary
  `bun install` can leave them stale);
- `effigy ir:build` then `effigy ir:check`, and `effigy catalogue:build` then
  `effigy catalogue:check` (known papercut: `GENERATOR_VERSION` restamp);
- `effigy test:web-pack-install` with the `g16.033` packed proof still present:

  ```text
  packedHistoryEntryProof.positive.exitCode = 0
  one unsuppressed TS2339 branchCount failure per
    @inflatable-cookie/poodle-svelte
    @inflatable-cookie/poodle-svelte/types
  sourceImports / workspaceAliases / declarationTextSubstitute = false
  ```

- local packs of core, Svelte, and React: filename, byte size, SHA-256;
- replica of `release.yml` pack-content verification for core and Svelte;
- `effigy check:release-automation`
- `effigy audit:licenses`
- `effigy audit:security` (must be green after the Papercuts fix)
- `effigy drift:gpui-consumer-identity`
- `effigy docs:check`
- `effigy qa`
- read-only `effigy release gates` (exactly one configured `headless` gate
  executed; fail-fast must not skip it)
- `git diff --check` before the candidate commit, then
  `git diff --check origin/main...HEAD` before PR handoff
- local and remote proof that `v0.2.4` is absent before the orchestrator tags

Forbidden in the candidate and Papercuts workers: `effigy release prepare`,
`execute`, `simulate`; tag create/push; `gh workflow run`; `npm publish`;
registry mutation; `.github/workflows/` edits; `*-windowed` selectors;
Jetstream QA; sibling-repository writes.

Lane authorization is not windowed-diagnostic authorization. This payload
does not change public GPUI crate identity, so
`test:visual-button-comparison-windowed` is out unless the operator later
approves it separately.

### Certification (orchestrator, after accepted candidate merge)

Human-owned, never a worker:

1. confirm the reviewed HEAD is still the candidate SHA;
2. create and push lightweight `v0.2.4` at that SHA only;
3. `gh workflow run release.yml --ref v0.2.4 -f dry-run=false`;
4. record the Actions run, npm `latest`, tarball artifact digest, and a
   clean-registry install of exact core/Svelte `0.2.4`.

`release.yml` already packs only core and Svelte, publishes only those two
under trusted publishing / OIDC, and uploads `packed-tarballs`. Dry-run
defaults to true; publication requires the tag ref and `dry-run=false`.

## Provenance And Rollback Receipts

### Last published baseline (do not disturb)

| Receipt | Value |
| --- | --- |
| Candidate / tag SHA | `d5607def24c6833913df1b5dcfa06372fcd5dd81` |
| Integration merge | PR #74 at `6ea561be8c45ec7fbdbab4ebeaba4f31284e2596` |
| Tag | `v0.2.2` (lightweight, same SHA locally and on `origin`) |
| Publication run | [GitHub Actions `32756610293`](https://github.com/inflatable-cookie/poodle/actions/runs/32756610293) |
| Packed-artifact digest | `sha256:0b83427da8fac0ac068f53bd47759be2716edfd5afe9a7419caa6b555ab96740` |
| npm `latest` at packet time | `0.2.2` for core and Svelte; React absent |

`v0.2.0` at `7922a3a951e94b607566563ff2750fe825ad7b0d` remains immutable.
The `0.2.0` workflow died before publication. `0.2.1` replaced it in the
registry; `0.2.2` then became `latest`. Do not recreate or move those tags.

### Source-complete, unpublished HistoryCenter proof

| Receipt | Value |
| --- | --- |
| Card | `docs/roadmaps/g16/033-history-center-rejection-surface.md` |
| Merge | PR #120 at `df0c7acd9` |
| Packed proof selector | `effigy test:web-pack-install` / `test:svelte-pack-install` |
| Closeout | `docs/logs/2026-09/20260901-g16-033-history-center-rejection-surface.md` |

This proves the tarball the candidate will publish, not that npm has moved.

### Unpublished `0.2.3` lockstep (do not tag)

| Receipt | Value |
| --- | --- |
| Bump commit | `ea1763786` (`chore: lockstep 0.2.3 for triggerless ContextMenu`) |
| Origin tag `v0.2.3` | absent |
| npm `0.2.3` | absent |

### Required `0.2.4` receipts (fill during candidate/certification)

| Receipt | When |
| --- | --- |
| Candidate SHA | commit 1 of the candidate PR |
| Evidence-receipt SHA | commit 2; does not change the tree that will be tagged |
| Core/Svelte/React tarball names, bytes, SHA-256 | candidate |
| `packedHistoryEntryProof` JSON | candidate `test:web-pack-install` |
| `effigy release gates` one-gate pass from that SHA | candidate |
| Remote tag `v0.2.4` SHA | certification; must equal candidate SHA |
| Workflow run URL and conclusion | certification |
| npm `latest` = `0.2.4` for core and Svelte | certification |
| Clean-registry install of exact `0.2.4` | certification |
| Loophole pin + rejection-map PR | after the registry proof, in Loophole |

### Rollback

- Never move, delete, or reuse `v0.2.0`, `v0.2.1` if present, `v0.2.2`, or a
  failed `v0.2.4`.
- Never tag `v0.2.3`.
- If the workflow is red, or npm `latest` is not `0.2.4`, do not re-tag.
  Fix in the next PATCH (`0.2.5`). npm `latest` stays `0.2.2` until a green
  publish.
- Do not unpublish `0.2.2` or `0.2.4`.
- Loophole stays on `0.2.2` until certification proves `latest` is `0.2.4`.
- A red flake (`smoke:gpui-window-capture`, `gate-tree-guard`) is retry
  evidence, not a waived gate.

## Publication Stop Conditions

Stop and return to the orchestrator. Do not tag, dispatch, or publish.

- `audit:security` still red on the candidate SHA.
- `effigy qa` or `effigy release gates` red for a non-flake reason, or a
  flake not reproduced as such.
- Lockstep mismatch, stale `bun.lock` workspace versions, or unrestamped
  IR/catalogue headers.
- Packed proof missing, suppressed, or still accepting `branchCount`.
- Candidate notes that describe only ContextMenu, omit the HistoryCenter
  break, or claim `0.2.3` was published.
- Any compatibility shim for v2 `HistoryEntry`.
- Workflow, trusted-publisher, or tag-ref mismatch; dispatch against `main`
  instead of `refs/tags/v0.2.4`; dry-run left true for the real publish.
- Attempt to publish React, crates.io, or Jetstream admission.
- Longhorn `AlreadyAtTarget` reopened, or a Poodle worker editing Loophole /
  Longhorn.
- CS20 `groupId` or HistoryCenter keyboard geometry folded into this lane.
- Windowed/native-visual selectors without a later explicit operator approval.
- `.github/workflows/` edits.
- Mainline candidate-bearing drift after freeze without a replaced SHA and
  full rerun.

The preview usage snippet in `packages/svelte/preview/src/component-docs.ts`
still teaches v2 `entries` / `branches` / `branchCount`. It does not ship in
the npm tarball (`files: src` on the component packages). It is **not** a
publication stop. It is a later `Papercuts` docs-honesty follow-on after
publish, unless `docs:check` starts failing on it.

## Repository Handoff Boundary

### Poodle owns

- Papercuts matcher repair (label `Papercuts`).
- `0.2.4` candidate PR: versions, notes, locks, stamps, receipt, headless
  gates.
- Review and merge of those Poodle PRs.
- Tag `v0.2.4`, `release.yml` dispatch, npm trusted publish, provenance
  verification.
- Poodle closeout log after certification.

Poodle workers never merge, never tag, and never touch Loophole or Longhorn.

### Loophole owns, and only after npm `latest` is `0.2.4`

- Exact desktop pins `@inflatable-cookie/poodle-core` and
  `@inflatable-cookie/poodle-svelte` from `0.2.2` to `0.2.4`.
- Hub mapping of deletion refusals onto `StaleHistory`, `ProtectedEntry`,
  and `DeletionUnavailable`. Keep `AlreadyAtTarget` / `UnknownEntry` as they
  already are. No host copy override and no Longhorn types in Poodle.
- Switch any remaining v2 `HistoryEntry` / `branchCount` imports to the
  published v3 shape (`continuationCount`).
- Close Loophole `PAPERCUTS.md` entries “Poodle-svelte types.ts still exports
  v2 HistoryEntry” (pin lag) and “HistoryCentre delete rejections collapse to
  UnknownEntry”.

Loophole review/merge stays in Loophole. Poodle does not combine repositories
in one worker.

### Longhorn does not reopen

`AlreadyAtTarget` wire work is done. No Poodle card, worker, or diff follows
from it. Longhorn `g15.062` v0.2.2 adoption is a different closed lane; this
packet does not move Longhorn pins.

### Explicitly out of this lane

- CS20 coalesced-node `groupId` (Loophole/Pulse recording policy).
- HistoryCenter keyboard vertical geometry (design-deferred).
- Consumer adoption beyond Loophole (Underlay, Soundcheck, Acowtancy, …).
- crates.io publication, React npm publication, Jetstream admission.

## Papercuts Worker Label

Every papercut worker launched from this packet, including the security-matcher
repair and any later docs-snippet follow-on, must carry the capitalized
workspace label `Papercuts` before launch. The release-candidate worker is
not a papercut worker unless the orchestrator deliberately folds the matcher
fix into it; if folded, that workspace still gets the `Papercuts` label.

## Recommendations

- Serialise Papercuts matcher repair, then the `0.2.4` candidate, then
  orchestrator certification, then Loophole adoption. Do not pin Loophole
  from a packed-but-unpublished tarball.
- Keep the candidate on current `main` after the matcher fix. Do not wait
  for unrelated in-flight cards. `g16.036` is already merged and rides the
  same tag.
- Inventory the `v0.2.2...candidate` public-intent delta in `0.2.4` notes so
  Loophole is not surprised by drag-drop, motion policy, or other already-
  merged g16 work riding the same tag.
- Leave CS20 and keyboard geometry in
  `docs/triage/20260831-194043-history-papercut-ownership.md` until those
  owners decide them.

## Alternatives Not Selected

| Alternative | Reason |
| --- | --- |
| Tag current HEAD as `v0.2.3` | Notes/READMEs describe a ContextMenu-only unpublished bump; HEAD is a larger breaking payload |
| Publish `0.2.3` from `ea1763786`, then `0.2.4` | Two publications; the authorized lane is the corrected HistoryCenter package |
| Cherry-pick only `g16.033` onto `v0.2.2` | Breaks lockstep-on-main; omits already-shipped-in-source public work |
| Keep publishing `0.2.3` after rewriting its notes | Reuses a version consumers may already have seen in git as a different meaning |
| Include Loophole pin in the Poodle worker | Violates the settled ownership split |
| Reopen Longhorn `AlreadyAtTarget` | Complete; handoff forbids it |
| Require the windowed Button diagnostic | No GPUI identity change; windowed work needs its own approval |
| Ship a `branchCount` alias | Pre-1.0 rule: no shims; packed proof requires the negative fixture to fail |
| Wait for CS20 / keyboard geometry | Explicitly out of `g16.033` and this lane |

## Explicit Non-Goals

- Version, tag, publish, or sibling-repository mutation from this planning PR.
- Promoting this packet into a ready card by merge alone.
- React npm publication, crates.io publication, Jetstream admission.
- Compatibility shims.
- Moving any consumer except Loophole after certification.
- Changing HistoryCenter data/navigation semantics, drag-and-drop, or motion
  policy in the candidate beyond version/notes/lock honesty.

## Proposed Canonical Destinations

| Meaning | Destination after packet acceptance |
| --- | --- |
| `0.2.4` lockstep, notes, changelog, README honesty | Candidate worker; `CHANGELOG.md`, `docs/release-notes/0.2.4.md`, package READMEs, unpublished-`0.2.3` clarification |
| Headless candidate gates and receipt | New g16 release-candidate card cloned from the `g15.060` shape, version digits replaced |
| Tag, workflow, registry proof | Orchestrator-owned certification card/log cloned from `g15.061`; never dispatched to a worker |
| Loophole pin + rejection map | Loophole-owned adoption after npm `latest` is `0.2.4`; Poodle does not write it |
| Security matcher | Papercuts worker (`Papercuts` label), then drop or mark the `PAPERCUTS.md` entry |
| Preview v2 HistoryCenter snippet | Later `Papercuts` follow-on; not a publish gate |
| CS20 / keyboard geometry | Remain on `docs/triage/20260831-194043-history-papercut-ownership.md` |
| Close the publication/adoption clause on that ownership note | After Loophole adoption merges |

The orchestrator chooses the card numbers and promotion split. This packet
does not make a candidate ready by itself.

## Unresolved Questions

None of the handoff's product choices remain open.

The orchestrator still has to pick, after merge, whether the matcher repair
is a separate `Papercuts` worker or is folded into the candidate workspace
(still labelled `Papercuts` if folded). That is dispatch shape, not a product
question.

Mainline drift after this packet merges can change the freeze SHA. The
orchestrator reconciles that at candidate dispatch.

## Evidence Used

- `docs/handoffs/20260901-230400-history-release-adoption-planning.md`
- `docs/triage/20260831-194043-history-papercut-ownership.md`
- `docs/roadmaps/g16/033-history-center-rejection-surface.md`
- `docs/logs/2026-09/20260901-g16-033-history-center-rejection-surface.md`
- `docs/specs/022-packaging-versioning-and-release-channel-rules.md`
- `docs/specs/044-deprecation-change-control-and-release-channel-operations.md`
- `docs/release-notes/README.md`, `docs/release-notes/0.2.3.md`,
  `docs/release-notes/0.2.2.md`
- `docs/roadmaps/g15/060-v022-release-candidate.md`,
  `docs/roadmaps/g15/061-v022-release-certification.md`,
  `docs/logs/2026-08/20260824-g16-006-v022-release-candidate.md`,
  `docs/logs/2026-08/20260824-g16-007-v022-release-certification.md`
- `docs/roadmaps/g15/076-loophole-poodle-v022-adoption.md`
- `packages/release-manifest.json`, `packages/release-operations.json`
- `effigy.toml`, `tasks/effigy.tasks.toml`,
  `scripts/check-release-automation.ts`,
  `scripts/audit-repository-security.ts`
- `.github/workflows/release.yml` (read-only)
- live `npm view` (`latest` `0.2.2`), `git ls-remote --tags origin 'v0.2.*'`
  (`v0.2.0`, `v0.2.2` only)
- Loophole `apps/desktop/package.json` exact `0.2.2` pins and Loophole
  `PAPERCUTS.md` pin-lag / delete-rejection entries (read-only)

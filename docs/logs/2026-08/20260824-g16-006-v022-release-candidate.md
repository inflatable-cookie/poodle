# g16.006 v0.2.2 Release Candidate — Execution Receipt

Date: 2026-08-24
Card: `../../roadmaps/g16/006-v022-release-candidate.md`
Handoff: `../../handoffs/20260824-095901-g16-006-v022-release-candidate.md`
Spec lane: `../../specs/022-packaging-versioning-and-release-channel-rules.md`
Release notes: `../../release-notes/0.2.2.md`
Recovery this candidate carries: `20260823-g16-005-gpui-cratesio-recovery.md`
PR: [#74](https://github.com/inflatable-cookie/poodle/pull/74)
Worker branch: `t3code/release-candidate-handoff-1`
Worker worktree: `/Users/tom/.t3/worktrees/poodle/t3code-0e602689`
(launcher-supplied, registered, clean at start, non-`main`; the generated
branch name differs from the handoff's suggested
`t3code/g16-006-v022-release-candidate` and was reused per the handoff's own
instruction)
Planning base: `8b0e585342ec5ba68dda61d4bdf60055e15cc277`, an ancestor of the
dispatch tip `efbcc83a`. After PR 74 review round 1 the branch was rebased onto
`a98b924c529d7d5140bb3fa4e9fafea74bc549ce` = `origin/main`, the commit that
repaired the card's scope contradiction.

## Candidate SHA

**`d5607def24c6833913df1b5dcfa06372fcd5dd81`** — the complete candidate tree.

Every result below was measured from a clean worktree at exactly that commit.
This receipt commit adds only this file; it does not repin the candidate.

One earlier candidate commit was **replaced and is not reused**. No result from
it is carried forward, and the complete board was rerun:

| Commit | Why it was replaced |
| --- | --- |
| `bba78bdc` | PR 74 review round 1 required the bounded stale-bzip2 licence-surface correction, which is candidate-bearing |

## Absence Of Release Mutations

No tag was created or pushed. No GitHub release was created. No workflow was
dispatched. No `npm publish` ran. No registry was mutated and no registry was
queried. No `effigy release prepare`, `effigy release execute`, or `effigy
release simulate` ran — `simulate` is not this repository's release path and
its changelog-parser mismatch stays open in `PAPERCUTS.md`. Read-only `effigy
release gates` ran the configured headless gate. No `*-windowed`,
native-visual, or Jetstream preview/QA selector ran.

Read-only tag collision check: `v0.2.2` is absent locally and on `origin`.
`v0.2.0` (`7922a3a9`) and `v0.2.1` (`3d914261`) remain immutable and are not
reused.

Tarballs live in the ignored `.artifacts/` tree and are not committed. This
receipt records how to reproduce them; it is not an artifact store.

## Version Denominator

The mechanical inventory was built before any edit and matched `g16.006`'s
fixed decision and `packages/release-manifest.json` exactly — 3 public-intent
TypeScript manifests and 17 Rust crates under `packages/`. There was no
conflict to report.

The check below replicates the `Versions agree with the tag` step in
`.github/workflows/release.yml` against `tag=0.2.2`. All 20 manifests agree:

| Manifest | Version |
| --- | --- |
| `packages/core/package.json` (`@inflatable-cookie/poodle-core`) | 0.2.2 |
| `packages/svelte/components/package.json` (`@inflatable-cookie/poodle-svelte`) | 0.2.2 |
| `packages/react/components/package.json` (`@inflatable-cookie/poodle-react`) | 0.2.2 |
| `poodle-adapter`, `poodle-events`, `poodle-headless`, `poodle-ir`, `poodle-layout`, `poodle-markdown`, `poodle-node`, `poodle-specs`, `poodle-style`, `poodle-tokens` | 0.2.2 |
| `poodle-render` | 0.2.2 |
| `poodle-gpui`, `poodle-gpui-node-backend`, `poodle-jetstream` | 0.2.2 |
| `poodle-codegen`, `poodle-gpui-preview`, `poodle-jetstream-preview` | 0.2.2 |

Intra-repository requirements moved with them: 39
`poodle-* = { version = "0.2.2", path = ... }` entries across the 17 crate
manifests, and the `@inflatable-cookie/poodle-core: "0.2.2"` dependency pin in
both `packages/svelte/components` and `packages/react/components`.

Lockfiles: `bun.lock`, `packages/gpui/node-backend/Cargo.lock`, and
`packages/gpui/preview/Cargo.lock`. The Cargo locks were refreshed with
`cargo update --manifest-path <crate>/Cargo.toml --workspace`, which changed
exactly 19 lines — 6 in the node-backend lock, 13 in the preview lock — all
`poodle-*` version lines, with zero registry churn.

`packages/svelte/install-smoke/bun.lock` is outside the denominator: the smoke
consumer depends on core through a `file:` path, so no version string in it
moves.

Private manifests stay put by decision, not oversight: the `private`
repository-root `package.json` remains `0.1.0`, and `packages/tokens`,
`packages/svelte/preview`, `packages/react/preview`, and
`packages/svelte/install-smoke` remain `0.0.0`. None is in
`packages/release-manifest.json`'s public-intent set or the release workflow's
tag check.

### Diff purity of the mechanical batch

Across all 68 files in the version/lock/stamp batch, **every changed line
contains a `0.2.1` → `0.2.2` version string**. Filtering that diff for a
changed line without one returned nothing. No component, contract, token,
specimen, schema, or renderer content moved.

The candidate's remaining 12 files are the release narrative and the licence
surface, itemised below.

## The Licence-Surface Correction (review round 1)

`g16.005` recorded that `bzip2` and `libbz2-rs-sys` left both GPUI graphs with
the fork, and assigned re-derivation of the notice surface to `g16.006`. The
first `g16.006` card revision did not make those files writable, so the first
candidate reported the contradiction instead of widening its own scope. The
orchestrator repaired the card and handoff on `main` at `a98b924c`; this
replacement candidate performs the correction.

### Proof the crates are absent

Not asserted — derived, on the candidate:

| Evidence | Result |
| --- | --- |
| `name = "bzip2"` / `name = "libbz2-rs-sys"` in `packages/gpui/node-backend/Cargo.lock` | 0 |
| the same in `packages/gpui/preview/Cargo.lock` | 0 |
| `cargo metadata` resolved packages matching `/bzip2\|libbz2/` across all four `cargo deny` manifests (`render`, `gpui/node-backend`, `gpui/adapter`, `jetstream/adapter`), all platforms | none, none, none, none |
| resolved packages carrying a `bzip2` licence expression in those four graphs | none, none, none, none |

### What changed

| Surface | Change |
| --- | --- |
| `THIRD_PARTY_NOTICES.md` | the whole `## bzip2 and libbzip2` section removed (52 lines); Lucide and Inter sections untouched |
| `packages/gpui/node-backend/THIRD_PARTY_NOTICES.md` | **deleted** — its entire content was that one notice, and the crate has no remaining third-party notice obligation |
| `deny.toml` | the `"bzip2-1.0.6"` allow entry removed; every other identifier and the whole `[sources]` policy untouched |
| `docs/specs/022-…` | the bzip2-specific rule replaced by the durable rule it was an instance of, and the SPDX seed link dropped |
| `scripts/audit-license-compliance.ts` | stale markers and the deleted notice surface removed; a bidirectional lock-derived sweep added |

Deleting the node-backend notice was the honest option rather than leaving a
placeholder: a notice surface asserts what is distributed, and that crate now
distributes no notice-bearing dependency. `audit:licenses` reports 4 notice
surfaces where it reported 5.

### The gate got stronger, not weaker

This is the part worth reviewing closely, because "remove a claim and remove
the check for it" is exactly the shape of weakening a gate. It is not what
happened.

The old check was a marker list: it could only catch a notice that went
**missing**. It was structurally incapable of catching a notice for a crate
that had **left** the graph — which is precisely the drift that reached a
published release and needed this repair. The replacement derives the claim
from the lockfiles instead of from a hand-maintained list:

- if no `Cargo.lock` resolves `bzip2`/`libbz2-rs-sys`, then no third-party
  notice, `deny.toml`, or spec 022 may still name them;
- if a lockfile ever resolves them again, the audit fails and tells the reader
  to restore the notice, the allow entry, and the spec text — so the crate
  returning is a deliberate licence decision, never a silent pass.

Spec 022's rule was generalised to match: a notice-bearing licence is carried
for exactly as long as its crate is in the resolved graph. The bzip2 history
lives in the release notes, the `g16.005` log, and this receipt — not in the
durable policy, and not inside the surface the sweep scans.

All three directions were negative-tested by planting a violation and
confirming the audit fails, then restoring the tree:

| Planted | Audit result |
| --- | --- |
| `"bzip2-1.0.6"` back into `deny.toml` | `error: deny.toml: still claims bzip2, which no lockfile resolves.` |
| a `## bzip2 and libbzip2` section back into the root notice | `error: THIRD_PARTY_NOTICES.md: still claims bzip2, which no lockfile resolves.` |
| a `libbz2-rs-sys` package block into the node-backend `Cargo.lock` | `error: bzip2 is resolved again in packages/gpui/node-backend/Cargo.lock: restore its notice, licence allow entry, and spec text, then remove it from retiredNoticeCrates.` |

After each control the tree was restored and the audit returned
`License compliance clean`.

One implementation note found by the controls: the sweep initially crashed with
`ENOENT` on a path that `git ls-files` still tracked but that had been deleted
in the working tree. A gate that dies on a missing file is not a gate, so the
sweep now skips unreadable paths — `requiredNotices` is what reports a notice
that should have been present.

### Sweep result on the candidate

The only tracked non-history file that still names `bzip2` or `libbz2` is
`scripts/audit-license-compliance.ts` — the gate that enforces their absence.
`CHANGELOG.md` and `docs/release-notes/0.2.2.md` name them as removed history,
which is the correct place for it. No packed tarball contains any file
mentioning either crate.

## Other Deviations And Findings

### 1. `bun.lock` needed the documented 5-line hand edit

The `PAPERCUTS.md` entry from `g15.050` reproduced exactly. `bun install
--lockfile-only` re-saved the lockfile and left the `workspaces` block's
`version` fields and intra-repo range strings at `0.2.1`; a byte diff against
the pre-run copy showed **no change at all**, and `--frozen-lockfile` still
exits 0 against that stale content.

Resolution followed the accepted `g15.050` pattern. A scratch tree containing
only the six workspace manifests was regenerated from no lockfile, and its
`workspaces` block was captured. The repository lockfile then received the same
5 lines by hand — 3 `version` fields and 2 `@inflatable-cookie/poodle-core`
ranges. The result was verified **byte-identical** to the scratch
regeneration's `workspaces` block (2896 bytes on both sides, `diff` empty),
with the registry-resolution section untouched, so no dependency moved and the
licence and security audits below remain about the same graph. `bun install
--frozen-lockfile` passes and reports no changes.

The papercut is already open and unchanged; nothing new was added for it.

### 2. The crate bump forced the 45-file generated restamp

`poodle-codegen`'s `GENERATOR_VERSION` is stamped into every generated artifact
header, so bumping the crate turned `ir:check` and `catalogue:check` red until
`effigy ir:build` and `effigy catalogue:build` restamped them. This is the
second open `g15.050` papercut, also reproduced unchanged.

45 generated files changed. The complete set of changed content across all 45
is three lines:

```
-// Generated by poodle-codegen 0.2.1. Do not edit manually.
+// Generated by poodle-codegen 0.2.2. Do not edit manually.
-<!-- Generated by poodle-codegen 0.2.1. Do not edit manually. -->
+<!-- Generated by poodle-codegen 0.2.2. Do not edit manually. -->
-    "generator": "poodle-codegen 0.2.1",
+    "generator": "poodle-codegen 0.2.2",
```

35 `//` headers, 3 `<!-- -->` headers, 7 JSON `generator` fields. No specimen
content, no component output, no schema version changed. Both `*:check`
selectors pass from the candidate.

### 3. `unmatched-source` warnings are gone

`g15.050` recorded five `cargo deny` `unmatched-source` warnings as an accepted
non-blocking shape. `g16.005` emptied the allowed-source list, so this
candidate produces **none**. All four manifests report `advisories ok, sources
ok` with no warning output.

### 4. Two packers, two tarball sets

Unchanged from `g15.050`: `effigy test:web-pack-install` packs with `bun pm
pack` and `release.yml` packs and publishes with `npm pack`. Both are
reproducible; their outputs are not comparable byte-for-byte. Both sets are
recorded below.

The `npm pack` run here used the machine-default npm `10.9.8`, not the
workflow's isolated npm `12.0.2`. `g15.054` measured those two producing
identical bytes and digests for this package set; that equality was not
re-measured here and is the operator's to confirm at `g16.007` if it matters.

### 5. All six web digests are unchanged from the replaced candidate

Every artifact digest below is identical to the one measured at `bba78bdc`.
That is expected and is useful evidence rather than a copy-forward: the
licence-surface correction touched only the repository root notice,
`deny.toml`, spec 022, the audit script, and a node-backend notice file, and
**none of those ship inside the three web tarballs**. The digests were
independently re-measured at `d5607def`; they were not carried over. Each
tarball was also extracted and confirmed to contain no file mentioning `bzip2`
or `libbz2`.

## Validation

Every command below ran from a clean worktree at `d5607def`.

Toolchain: `effigy v0.11.0+local.e37fcfd`, `bun 1.3.14`,
`cargo 1.97.1 (c980f4866 2026-06-30)`, `node v22.23.2`, `npm 10.9.8`.

| Command | Result |
| --- | --- |
| `effigy qa` | **pass** (exit 0; 1m26s) |
| `effigy release gates` | **pass** — 1 configured, 1 executed, `[1] headless: pass (exit 0; 90448ms)`, fail-fast did not stop early |
| `effigy check:release-automation` | **pass** — 5 retained workflows, Effigy gate, alias, publish set |
| `effigy audit:licenses` | **pass** — `License compliance clean: 8 package manifests, 17 Cargo manifests, and 4 notice surfaces`; `licenses ok` ×4 |
| `effigy audit:security` | **pass** — hygiene clean over 4304 files; `bun audit`: no vulnerabilities; `cargo deny`: `advisories ok, sources ok` ×4 |
| bzip2 source/lock sweep (four graphs + both locks + tracked sources) | **pass** — absent everywhere; only the enforcing gate names it |
| licence-sweep negative controls ×3 | **all fail as required**, tree restored, audit green after |
| `effigy drift:gpui-consumer-identity` | **pass** — 7/7 checks, negative control fails as required |
| `effigy test:web-pack-install` | **pass** — roster proof 175/175/175 |
| `effigy docs:check` | **pass** |
| `effigy ir:check` | **pass** |
| `effigy catalogue:check` | **pass** |
| `bun install --frozen-lockfile` | **pass**; no changes |
| version agreement vs `tag=0.2.2` (workflow step replica) | **pass** — 20/20 manifests |
| `release.yml` pack-content verification (replica) | **pass** — all three tarballs |
| `git diff --check` | clean, before the candidate commit |
| `git diff --check origin/main...HEAD` | clean, before PR handoff |
| local and remote `v0.2.2` tag collision | absent |

`effigy qa` composes `check:release-automation`, `ci` (`ci:web` + `ci:rust`),
`ci:native`, `test:web-pack-install`, `audit:licenses`, and `audit:security`.
`effigy release gates` runs the same complete board through its single
configured `headless` gate, so the board result above was produced twice
independently.

Named results inside the board:

- `test:core` — 767 pass, 0 fail.
- component/parity vitest — 357 files, 3065 tests, 0 fail.
- `audit:icons` — 108 default icon names verified (92 canonical, 16 aliases)
  from `lucide-static 1.31.0`.
- `audit:tokens` — all generated token artifacts verified.
- `docs:lint` — 182 component contracts, 43 operator guides, 137
  contract↔Svelte prop surfaces, 110 callback surfaces, 124 contract↔spec prop
  surfaces validated.
- `test:web-pack-install` — roster proof `denominator: 175`, `svelte: 175`,
  `react: 175`; bounded runtime mount set 9 Svelte / 13 React; the
  `AccidentalExtraComponent` export regression control was rejected as
  required.
- `drift:gpui-consumer-identity` — `resolved: gpui 0.2.2 from
  registry+https://github.com/rust-lang/crates.io-index`, exactly one `gpui`,
  every `gpui*` crate from the registry, no patch/replace/override.

## Packed Artifacts

Reproduce from a clean checkout of `d5607def`.

### `npm pack` — the publication path (`release.yml`)

```
for dir in packages/core packages/svelte/components packages/react/components; do
  (cd "$dir" && npm pack --pack-destination "$OUT")
done
```

| Filename | Bytes | SHA-256 |
| --- | ---: | --- |
| `inflatable-cookie-poodle-core-0.2.2.tgz` | 269021 | `ddf435b74034dc0769ffe1a754a35e2d9abb92539119ba2447088351d3661d72` |
| `inflatable-cookie-poodle-svelte-0.2.2.tgz` | 253638 | `e2a1797786d7f33f7a64db50d48390e8814c7fa326c560fdbd903fbb3f7d86a7` |
| `inflatable-cookie-poodle-react-0.2.2.tgz` | 250071 | `8fb90378306218a98f07190cff0f83b1d803f120a6da9a7e446e16d14203e8bf` |

Determinism: `packages/core` was packed twice into separate destinations and
produced the identical digest.

Content verification, replicating the workflow's `Pack and verify contents`
step:

| Tarball | Entries | LICENSE | README.md | package.json | Icon modules | Icon aliases | Token CSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| core | 393 | 1 | 1 | 1 | 108 (floor 50) | 1 | 22 (floor 20) |
| svelte | 214 | 1 | 1 | 1 | — | — | — |
| react | 208 | 1 | 1 | 1 | — | — | — |

Every packed `package/README.md` was extracted from its tarball and every link
enumerated. All 5 links across core and Svelte are absolute
`https://github.com/inflatable-cookie/poodle/blob/main/...` URLs; React's
README contains no links. Relative targets remaining: **0** on all three. Files
mentioning `bzip2`/`libbz2` inside the tarballs: **0** on all three.

React is packed and content-verified here as experimental evidence only. It is
**not** in the workflow's publish step and must not be added to it.

### `bun pm pack` — the gated path (`effigy test:web-pack-install`)

| Filename | Bytes | SHA-256 |
| --- | ---: | --- |
| `inflatable-cookie-poodle-core-0.2.2.tgz` | 262934 | `7be967151ce28ce85cd11bd0ba060a4a0b29f5ff4b8160e20e8adf250e53caee` |
| `inflatable-cookie-poodle-svelte-0.2.2.tgz` | 253512 | `a5f4cbb136db1d9477b8ffe3dbcd8b2551c4940a98d44f37b13b1169e65d1755` |
| `inflatable-cookie-poodle-react-0.2.2.tgz` | 249857 | `3cdc75aef5945a423655bb3629fb86a01d154a32beefdfc6a211cc1c21410594` |

All six digests moved from `v0.2.1`, because all three READMEs and all three
manifests carry the new version string. React's `bun pm pack` byte count is
unchanged at 249857 — `0.2.1` and `0.2.2` are the same length and its README
changed one character.

## Known Non-Blocking Warnings

- **`effigy doctor` baseline.** The recorded generated-in-src, god-file,
  stale-suppression, and comment-ratio findings are unchanged. This run
  measured no regression in any of them and introduced none. `doctor` was not
  run: selector routing was never ambiguous.
- **Svelte compiler `state_referenced_locally` notices.** `docs:check` emits
  two on `packages/svelte/preview/src/specimens/ErrorBoundaryCrashOnce.svelte`.
  Pre-existing, unrelated to the candidate, and the task exits 0.
- **Visual comparator.** `test:visual-button-comparison-windowed` still exits
  non-zero on its annotated, contract-cited `gpui-omits-box-shadow` findings,
  as `g15.047`, `g15.052`, and the `g16.005` operator review all recorded. It
  is windowed, it is not on the release board, and it was not run here.
  `docs/release-notes/0.2.2.md` claims no visual parity.

## What This Candidate Does Not Claim

- **No true headless GPUI pixels.** Stock crates.io GPUI 0.2.2 publishes no
  offscreen readback API. The retained pixel diagnostic opens a real,
  non-activating window and is explicitly outside QA, CI, and every release
  gate. The release note says this in its own section.
- **No new component behavior.** The only implementation change this release
  carries is the GPUI dependency and capture boundary accepted in `g16.005`.
  The web packages move for version-set alignment and carry no code change.
- **No re-derivation of any licence surface beyond bzip2.** The sweep and the
  spec rule are general, but only `bzip2`/`libbz2-rs-sys` were proved absent
  and corrected here. Other notice surfaces were left exactly as they were.
- **No windowed evidence from this worker.** The `g16.005` operator review
  already ran the three approved windowed selectors; nothing was re-run here
  and nothing new is claimed about them.
- No Jetstream parity; it remains program-deferred.
- No React runtime-behaviour denominator; its evidence is 175/175 import
  reachability plus a bounded 13-component mount set.
- No `0.2.2` publication and no tag. `g16.007` remains the separate operator
  gate for certification, tag, and registry mutation.

## Continuation

The next action is orchestrator review of this PR against `d5607def`, then the
explicit `g16.007` operator decision for the windowed diagnostic, tag, and
publication. Any further candidate-bearing review fix invalidates this receipt
and requires another replacement candidate commit plus a complete rerun of the
board above.

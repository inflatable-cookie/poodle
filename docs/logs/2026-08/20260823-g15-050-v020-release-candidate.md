# g15.050 v0.2.0 Release Candidate — Execution Receipt

Date: 2026-08-23
Card: `../../roadmaps/g15/050-v020-release-candidate.md`
Handoff: `../../handoffs/20260823-124431-g15-050-v020-release-candidate.md`
Spec lane: `../../specs/022-packaging-versioning-and-release-channel-rules.md`
Release notes: `../../release-notes/0.2.0.md`
PR: [#72](https://github.com/inflatable-cookie/poodle/pull/72)
Worker branch: `t3code/release-candidate-handoff`
Worker worktree: `/Users/tom/.t3/worktrees/poodle/t3code-c0202881`
(launcher-supplied, registered, clean at start, non-`main`; the generated
branch name differs from the handoff's suggested
`t3code/g15-050-v020-release-candidate` and was reused per the handoff's own
instruction)
Planning base: `9ae26bfa6017d5b9001edf4f4fe17de3eb781e7b`, an ancestor of
`HEAD` = `origin/main` = `349c0bce1e2f1d217750a60dc7b964d11958bacf`, the
commit carrying this card's handoff

## Candidate SHA

**`4428ad10174ccf36de42658ce0e1346b90191d75`** — the complete candidate tree.

Every result below was measured from a clean worktree at exactly that commit.
This receipt commit adds only this file; it does not repin the candidate.

Four earlier candidate commits were replaced rather than reused. No result
from any of them is carried forward, and the complete board was rerun each
time:

| Commit | Why it was replaced |
| --- | --- |
| `70ef51db` | hit the `test:core` stop condition (below); the fix was candidate-bearing |
| `214725ff` | PR #72 review round 1 requested four release-truth fixes, all candidate-bearing |
| `8fece26f` | PR #72 review round 2 required portable release-note links on the published artifact surface |
| `5f9dd91d` | PR #72 review round 3 extended that to the complete published-README link sweep |

## Absence Of Release Mutations

No tag was created or pushed. No GitHub release was created. No workflow was
dispatched. No `npm publish` ran. No registry was contacted or mutated. No
`effigy release prepare` or `effigy release execute` ran. `effigy release
gates` is read-only and is the only `release` subcommand used. No
`*-windowed`, native-visual, or Jetstream preview/QA selector ran.

Tarballs live in the ignored `.artifacts/` tree and are not committed. This
receipt records how to reproduce them; it is not an artifact store.

## Version Denominator

The mechanical inventory matched `g15.050`'s fixed decision and
`packages/release-manifest.json` exactly — 3 public-intent TypeScript
manifests and 17 Rust crates under `packages/`, with no conflict to report.

The check below replicates the `Versions agree with the tag` step in
`.github/workflows/release.yml` against `tag=0.2.0`. All 20 manifests agree:

| Manifest | Version |
| --- | --- |
| `packages/core/package.json` (`@inflatable-cookie/poodle-core`) | 0.2.0 |
| `packages/svelte/components/package.json` (`@inflatable-cookie/poodle-svelte`) | 0.2.0 |
| `packages/react/components/package.json` (`@inflatable-cookie/poodle-react`) | 0.2.0 |
| `poodle-adapter`, `poodle-events`, `poodle-headless`, `poodle-ir`, `poodle-layout`, `poodle-markdown`, `poodle-node`, `poodle-specs`, `poodle-style`, `poodle-tokens` | 0.2.0 |
| `poodle-render` | 0.2.0 |
| `poodle-gpui`, `poodle-gpui-node-backend`, `poodle-jetstream` | 0.2.0 |
| `poodle-codegen`, `poodle-gpui-preview`, `poodle-jetstream-preview` | 0.2.0 |

Intra-repository requirements moved with them: every
`poodle-* = { version = "0.2.0", path = ... }` entry across the 17 crate
manifests, and the `@inflatable-cookie/poodle-core: "0.2.0"` dependency pin in
both `packages/svelte/components` and `packages/react/components`.

Lockfiles: `bun.lock`, `packages/gpui/node-backend/Cargo.lock`, and
`packages/gpui/preview/Cargo.lock`. The Cargo locks were refreshed with
`cargo update --manifest-path <crate>/Cargo.toml --workspace`, which touched
only `poodle-*` version lines.

`package.json` at the repository root stays at `0.1.0` by decision, not by
oversight: it is `private`, absent from `packages/release-manifest.json`, and
outside the release workflow's tag check. `g15.050` fixes private tooling
manifests at their current versions.

## Deviations And Findings

### 1. `bun.lock` needed a hand edit (5 lines)

`g15.050` says to use supported tooling for lockfile updates. Bun offers no
command that produces the correct minimal result here:

- `bun install`, `bun install --force`, and `bun install --lockfile-only` all
  rewrite `bun.lock` while leaving the `workspaces` block's `version` fields
  and intra-repo range strings at `0.1.0`. `bun install --frozen-lockfile`
  still exits 0 against that stale content, so the drift is silent.
- Deleting `bun.lock` and reinstalling *does* pick the new versions up, but it
  also re-resolves every registry range — in this tree it moved
  `@oxc-project/types` 0.142.0 → 0.146.0, the whole `@rolldown/binding-*` set
  1.2.2 → 1.2.5, and `vitest` 4.1.10 → 4.1.11. That is a dependency change,
  which `g15.050` puts out of scope and which would invalidate the licence and
  security audits recorded here.

Resolution: the wholesale regeneration was performed once in a scratch copy to
learn exactly which lines it produces, the lockfile was restored, and those
same 5 lines — 3 `version` fields and 2 `@inflatable-cookie/poodle-core`
ranges — were applied by hand. The result is byte-identical to the
regeneration's `workspaces` block with zero registry churn.
`bun install --frozen-lockfile` passes against it. Recorded in `PAPERCUTS.md`.
Accepted in review round 1 as a documented constrained exception.

### 2. The crate bump forced a generated-artifact restamp (45 files)

`poodle-codegen` carries `GENERATOR_VERSION = env!("CARGO_PKG_VERSION")`, which
is stamped into every generated artifact's header. Bumping the crate to
`0.2.0` therefore turned `effigy ir:check` red on `vectors.json` and would have
turned `effigy catalogue:check` red too.

`effigy ir:build` and `effigy catalogue:build` restamped the 45 affected files.
The diff was verified to contain nothing but the generator line — the complete
set of changed content across all 45 files is:

```
-// Generated by poodle-codegen 0.1.0. Do not edit manually.
+// Generated by poodle-codegen 0.2.0. Do not edit manually.
-<!-- Generated by poodle-codegen 0.1.0. Do not edit manually. -->
+<!-- Generated by poodle-codegen 0.2.0. Do not edit manually. -->
-    "generator": "poodle-codegen 0.1.0",
+    "generator": "poodle-codegen 0.2.0",
```

No specimen content, no component output, no schema version changed. Neither
`ir:check` nor `catalogue:check` is on the `qa` board, so this drift class is
invisible to the release gate; both were run explicitly and both pass.
Recorded in `PAPERCUTS.md`.

### 3. Stop condition hit: `test:core` icon ceiling (operator-authorised repair)

The first candidate commit `70ef51db` failed `effigy qa` at
`ci:web` → `test:core`:

```
packages/core/test/icons.test.ts:18
expect(Object.keys(defaultLucideIconSet)).toHaveLength(106);
  Expected length: 106
  Received length: 108
766 pass, 1 fail
```

This was **not** introduced by the candidate. `g15.053` added the canonical
`house` icon and its `home` alias in `fadbd9d9` — merged as PR #71 into
`9ae26bfa` — for the accessible icon-only root crumb, without moving the
deliberate ceiling in the test. Proof that the candidate is not the cause:
`git diff origin/main -- packages/core/src/icons/ packages/core/test/icons.test.ts`
was empty at `70ef51db`, so the failure reproduces on `origin/main` unchanged.

The card forbids making a red gate green, and
`packages/core/test/icons.test.ts` is component evidence outside `g15.050`'s
writable scope, so the run stopped and reported rather than editing it. The
operator then explicitly authorised moving the ceiling inside this card rather
than routing a separate one.

The repair moves the assertion to `108` and updates the rationale comment to
name what moved it and why. The gate keeps its shape and its purpose: the
count remains a ceiling that only a conscious edit can raise. Nothing else in
the test, the icon set, or the build changed.

### 4. Two packers produce two different tarballs

`effigy test:web-pack-install` packs with `bun pm pack`;
`.github/workflows/release.yml` packs and publishes with `npm pack`. Both are
reproducible, and neither is wrong, but their outputs are not comparable
byte-for-byte. Both sets are recorded below so a reviewer can reproduce either.

### 5. Review round 1 (PR #72) — release truth

Four groups of candidate-bearing fixes, all inside `g15.050`'s writable scope
(package READMEs, operator guides, release-facing front doors, release
manifest/operations metadata). No workflow, component implementation,
specimen, or visual baseline was touched.

1. **Publication truth.** `packages/core/README.md` and
   `packages/svelte/components/README.md` ship *inside* the tarballs that
   publish them and still said the packages were unpublished, teaching `file:`
   dependencies. Both now document preview-channel npm installation at
   `0.2.0` with an exact-pin warning. `packages/react/components/README.md`
   states plainly that React is packed and certified but not published.
   `README.md`, `docs/README.md`, and
   `docs/guides/svelte-developer-guide.md` draw the same three-way
   distinction — core/Svelte on npm, React source-only, Rust source/tag — and
   "before the first public release" is replaced by the real pre-1.0
   boundary. The Svelte guide keeps a `file:` path for working against an
   unreleased Poodle. The GPUI and Jetstream guides still say the Rust crates
   are not on crates.io, which remains true and matches the candidate.
2. **Rust docs taught a removed API.** `packages/render/README.md`,
   `packages/gpui/adapter/README.md`,
   `packages/gpui/node-backend/README.md`,
   `packages/jetstream/adapter/README.md`, and
   `docs/guides/jetstream-developer-guide.md` called renderers with `&theme`.
   Diagrams, imports, prose, and examples now construct and pass
   `RenderContext`; `packages/render/README.md` gained a Presentation Cascade
   section covering `ctx.theme()`, `Option` semantic inputs, explicit-size
   finality, `scoped`, `ui_presentation_provider`, and `SlotBuilder`. Both
   Jetstream surfaces now state that consuming the shared node tree is
   composition reuse, not parity evidence, and that backend integration stays
   deferred. `docs/guides/gpui-developer-guide.md` was already correct and is
   untouched.
3. **Narrative accuracy.** The shared-render-tree bullet in
   `docs/release-notes/0.2.0.md` no longer claims GPUI and Jetstream "behave
   the same way ... by construction"; it now states that one place decides
   composition, token resolution, and interaction intent — which removes a
   drift class but is not behavioural parity — and names what each backend
   still owns. The twelve themes are no longer listed as a `0.2.0` addition;
   the 22 token stylesheets and 108 icon modules moved into package posture
   with the theme set marked unchanged since `0.1.0`. The note's opening,
   which still said nothing had been published, was aligned to the same
   claim. `CHANGELOG.md` now calls the old `TabVariant` union six-member.
4. **Release authority.** `poodle-ir` was added to spec 022's current
   preview-channel Rust package list, matching `packages/release-manifest.json`,
   the 0.1.0 notes, and this candidate. `packages/release-operations.json`'s
   stale `"Stable release channel is not available during g03."` expectation
   became `"Stable release channel is unavailable until a later generation
   explicitly enables it."`.

### 6. Review rounds 2 and 3 (PR #72) — portable links on the artifact surface

`packages/core/README.md` and `packages/svelte/components/README.md` ship
inside the two npm tarballs, where a repository-relative link is not a portable
public-package contract: it resolves in the checkout and on GitHub, but npm
package pages cannot be relied on to rewrite it, so it would ship as broken
public documentation.

Round 2 converted the two release-note links. Round 3 extended the correction
to the complete sweep, on the reviewer's instruction that the first pass should
have covered every link rather than only those two. Every markdown link in both
published READMEs is now a canonical absolute GitHub URL under
`https://github.com/inflatable-cookie/poodle/blob/main`:

| File | Link | Was |
| --- | --- | --- |
| `packages/core/README.md` | `/docs/release-notes/0.2.0.md` | `../../docs/release-notes/0.2.0.md` (round 2) |
| `packages/core/README.md` | `/docs/architecture/002-token-system-and-package-layout.md` | `../../docs/architecture/002-token-system-and-package-layout.md` (round 3) |
| `packages/core/README.md` | `/README.md` | `../../README.md` (round 3) |
| `packages/svelte/components/README.md` | `/docs/release-notes/0.2.0.md` | `../../../docs/release-notes/0.2.0.md` (round 2) |
| `packages/svelte/components/README.md` | `/docs/guides/svelte-developer-guide.md` | `../../../docs/guides/svelte-developer-guide.md` (round 3) |

`packages/react/components/README.md` contains no markdown links at all.

Verified from inside the built tarballs, not from the working tree: each
`package/README.md` was extracted from its `.tgz` and every link enumerated. A
pattern check for any remaining relative target — `](.`, `](/`, or any
non-`http` scheme — returns **0** on all three packed READMEs.

## Validation

Every command below ran from a clean worktree at `4428ad10`.

Toolchain: `effigy v0.11.0+local.e37fcfd`, `bun 1.3.14`,
`cargo 1.97.1 (c980f4866 2026-06-30)`, `node v22.23.2`.

| Command | Result |
| --- | --- |
| `effigy qa` | **pass** |
| `effigy release gates` | **pass** — 1 configured, 1 executed, `[1] headless: pass (exit 0; 115321ms)`, fail-fast did not stop early |
| `effigy docs:check` | **pass** |
| `effigy ir:check` | **pass** |
| `effigy catalogue:check` | **pass** |
| `bun install --frozen-lockfile` | **pass** |
| version agreement vs `tag=0.2.0` (workflow step replica) | **pass** — 20/20 manifests |
| `git diff --check` | clean, before the candidate commit |
| `git diff --check origin/main...HEAD` | clean, before PR handoff |

`effigy qa` composes `check:release-automation`, `ci` (`ci:web` + `ci:rust`),
`ci:native`, `test:web-pack-install`, `audit:licenses`, and `audit:security`.
`effigy release gates` runs the same complete board through its single
configured `headless` gate, so the board result above was produced twice
independently.

Named results inside the board:

- `test:core` — 767 pass, 0 fail.
- `check:release-automation` — pass; 5 retained workflows, Effigy gate, alias,
  and publish set checked.
- `audit:icons` — 108 default icon names verified (92 canonical, 16 aliases)
  from `lucide-static 1.31.0`.
- `audit:tokens` — all generated token artifacts verified.
- `test:web-pack-install` — roster proof `denominator: 175`, `svelte: 175`,
  `react: 175`; bounded runtime mount set 9 Svelte / 13 React.
- `audit:security` — `Security hygiene clean: 4274 repository files`;
  `bun audit`: `No vulnerabilities found`; `cargo deny`:
  `advisories ok, sources ok` on all four manifests.
- `audit:licenses` — `licenses ok` on all four manifests.
- `docs:lint` — 182 component contracts, 43 operator guides, 137
  contract↔Svelte prop surfaces, 110 callback surfaces, 124 contract↔spec prop
  surfaces validated.

## Packed Artifacts

Reproduce from a clean checkout of `4428ad10`.

### `npm pack` — the publication path (`release.yml`)

```
for dir in packages/core packages/svelte/components packages/react/components; do
  (cd "$dir" && npm pack --pack-destination "$OUT")
done
```

| Filename | Bytes | SHA-256 |
| --- | ---: | --- |
| `inflatable-cookie-poodle-core-0.2.0.tgz` | 268936 | `9e40f67ac55f4a4b1c92221f315fcaa45bca52a84fc64eb0f42710e3ab7d05fc` |
| `inflatable-cookie-poodle-svelte-0.2.0.tgz` | 253565 | `e10af6ea1979266ab3fd336e3a984f0243cf99b8421c3f1777f894477c145fc7` |
| `inflatable-cookie-poodle-react-0.2.0.tgz` | 250071 | `def4af15c7e64a3dd7c2a2e600d546abc1a1dccecbdecf53ab547f65118afb42` |

Determinism: `packages/core` was packed twice into separate destinations and
produced the identical digest.

Content verification, replicating the workflow's `Pack and verify contents`
step:

| Tarball | Entries | LICENSE | README.md | package.json | Icon modules | Icon aliases | Token CSS |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| core | 393 | 1 | 1 | 1 | 108 (floor 50) | 1 | 22 (floor 20) |
| svelte | 214 | 1 | 1 | 1 | — | — | — |
| react | 208 | 1 | 1 | 1 | — | — | — |

Every packed `package/README.md` was extracted from its tarball and checked:
all links absolute, zero relative targets remaining.

React is packed and content-verified here as experimental evidence only. It is
**not** in the workflow's publish step and must not be added to it.

### `bun pm pack` — the gated path (`effigy test:web-pack-install`)

| Filename | Bytes | SHA-256 |
| --- | ---: | --- |
| `inflatable-cookie-poodle-core-0.2.0.tgz` | 262870 | `f53f4dc0b536427c143b9c11a98adf6e239fdff7aba8a5e7d698cf8fcc5816b4` |
| `inflatable-cookie-poodle-svelte-0.2.0.tgz` | 253442 | `2784ed1527608d52476a0cc8126f78c0674393e484b3e0fd80afe8770d223834` |
| `inflatable-cookie-poodle-react-0.2.0.tgz` | 249857 | `1e9c7b3e2ea35923bee9b776182e541e61a0e699b421e9f716d68585fc0c65be` |

### Digest movement across candidates

Round 2 changed all three READMEs, so all six digests moved. Rounds 3 and 4
changed only the core and Svelte READMEs, so **both React digests are unchanged
across all three** — an independent check that each link correction reached
exactly the two published READMEs the review named and nothing else.

| Artifact | `8fece26f` | `5f9dd91d` | `4428ad10` |
| --- | --- | --- | --- |
| core (`npm`) | `ad47bb4f…` | `0e3b03ca…` | `9e40f67a…` |
| svelte (`npm`) | `5b9f39f1…` | `ce3f454b…` | `e10af6ea…` |
| react (`npm`) | `def4af15…` | `def4af15…` | `def4af15…` unchanged |
| core (`bun`) | `3f508227…` | `c1ffd8aa…` | `f53f4dc0…` |
| svelte (`bun`) | `ce90e020…` | `d86213e4…` | `2784ed15…` |
| react (`bun`) | `1e9c7b3e…` | `1e9c7b3e…` | `1e9c7b3e…` unchanged |

## Known Non-Blocking Warnings

- **`cargo deny` `unmatched-source`.** Five allowed sources in `deny.toml`
  (lines 38–42: `inflatable-cookie/zed`, `zed-industries/font-kit`,
  `zed-industries/scap`, `zed-industries/wasm_thread`,
  `proptest-rs/proptest`) report `no crate source matched these criteria` on
  the manifests whose graphs do not reach them. The allowlist is shared across
  four manifests and each one only pulls a subset, so unmatched entries are the
  expected shape of a fail-closed policy, not drift. Every manifest still
  reports `advisories ok, sources ok`.
- **`effigy doctor` baseline.** The recorded generated-in-src, god-file,
  stale-suppression, and comment-ratio findings are unchanged. This run
  measured no regression in any of them and introduced none. They are board
  health, not candidate blockers.
- **Visual comparator.** `test:visual-button-comparison` still exits non-zero
  on its 16 annotated, contract-cited `gpui-omits-box-shadow` findings, exactly
  as `g15.047` and `g15.052` recorded. It is not on the release board, and
  `docs/release-notes/0.2.0.md` states that boundary rather than claiming
  visual parity beyond the 18 Button fixtures.

## What This Candidate Does Not Claim

- No Jetstream parity. `poodle-jetstream` compiles and carries the tag version;
  its backend integration stays program-deferred. Sharing the `poodle-node`
  tree with GPUI is composition reuse, not behavioural evidence.
- No visual parity beyond 18 Button fixtures on one machine.
- No platform accessibility projection for GPUI.
- No React runtime-behaviour denominator; its evidence is 175/175 import
  reachability plus a bounded 13-component mount set.
- No publication. `g15.013` remains the separate operator gate for tag and
  registry mutation, and it stays blocked until the orchestrator accepts this
  evidence.

## Continuation

The PR stops for review. The orchestrator independently checks the manifest
denominator, version and lock agreement, release-note truth, the two-commit
boundary, the candidate SHA, this receipt, the changed files, and the board
results. Any candidate-bearing review fix invalidates this receipt and requires
a replacement candidate commit plus a complete rerun.

Merge authorisation, the tag, and publication remain operator-owned.

# g18.031 — Release metadata and GPUI census provenance

Status: complete — merged as `aa659504b2a6222eb34c7423fcfd877a32138ba8` (PR #264)
on 2026-09-12 after exact-head independent review
Date: 2026-09-12
Branch: `ns-e58c6d92-5f97-4263-8bc6-7756f749727a`
Card: `docs/roadmaps/g18/031-release-metadata-and-census-provenance.md`
Handoff: `docs/handoffs/20260912-230744-g18-031-release-metadata-and-census-provenance.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
`docs/roadmaps/g18/006-v040-web-editor-release-and-desktop-unblock.md`
Base: corrected planning `main` at
`5635ab184cda6610f1f91eae1eff585db5e817b2` (operator correction to the
g18.031 card and handoff), merged into this branch; original base
`086e8b506f958309b6797d92cb2e704140ff0374`.

## Outcome

The two false release identities are removed before `0.4.0` preparation, now
with root repository metadata corrected rather than exempted.

- Root `package.json` moves `0.1.0` -> `0.3.0`. The diff is exactly the
  `version` leaf: `private: true` and every other field are preserved, and the
  package stays unpublished.
- `effigy.toml` selects root `package.json` as the explicit release
  `version-file` with the existing `CHANGELOG.md`, `tag-format = "v{version}"`
  and `pre-1-0 = true` policy. The existing `[release.gates.headless]`
  (`effigy qa`) gate is preserved byte-for-byte.
- The closed g18.006 candidate policy now requires root `package.json` as a
  version-only `0.3.0` -> `0.4.0` release input and rejects any other root
  manifest change.
- Ordinary CI admits exactly the one-time g18.031 precursor root alignment
  (`0.1.0` -> `0.3.0`, version leaf only, `private` preserved). The earlier
  revision left the deterministic `ci-web` gate failing because ordinary mode
  labelled the operator-ordered root bump a forbidden version surface while
  the closed-candidate escape hatch requires a full `0.4.0` candidate diff.
- `scripts/gpui-functionality-census.ts` no longer embeds `package_version:
  "0.3.0"`. The generator derives it from
  `packages/gpui/preview/Cargo.toml` through fail-closed parsing and validates
  every checked-in receipt against the live manifest. The derivation is
  preserved unchanged from the first revision.

Public publication scope is unchanged: core and Svelte publish; React stays
private; Rust remains source/tag distributed.

## Release authority proof

Root manifest diff against the corrected base:

```
-  "version": "0.1.0",
+  "version": "0.3.0",
```

`private: true` and all other fields are untouched. Read-only
`effigy release status --json`:

- `current_version`: `0.3.0`
- `version_source`: root `package.json`, format `package.json`, path `version`
- `changelog`: root `CHANGELOG.md`, valid
- `gates`: 1 configured gate, unchanged
- sole blocker: `unreleased changelog section has no entries`, the
  stage-correct pre-prepare state

No other package or Cargo version, lock, changelog, release note, workflow,
tag, registry or g18.029 policy surface changed.

## Candidate-scope proof

`test/package-install/scope.ts` changes stay inside the closed g18.006 policy;
`g16.054` is untouched:

- `G18_006_JS_MANIFEST_PATHS` adds root `package.json` to the g18.006 lockstep
  JS manifests; `G18_006_RELEASE_INPUT_PATHS` adds it as a required release
  input; `G18_006_VERSION_PATHS` adds it to the g18.006 version-file surface.
- `CANDIDATE_MANIFEST_LEAF_ALLOWLIST["package.json"] = ["version"]`, so the
  leaf-honesty check rejects any other root manifest change.
- The g18.006 policy binds `jsManifestPaths` and `versionPaths` to the
  extended lists; `g16.054` keeps the original `LOCKSTEP_JS_MANIFEST_PATHS`
  and `CANDIDATE_VERSION_PATHS` without root.
- The private-status check now covers root as well as React.

Focused plants in `test/package-install/scope.test.ts`:

| Plant | Result |
| --- | --- |
| complete candidate (root moves `0.3.0` -> `0.4.0`) | admitted in ordinary and explicit g18.006 modes; `changedPaths` contains `package.json` |
| root left at `0.3.0` | `requires the complete 0.4.0 release-input set; missing: ... package.json` |
| root at `0.4.1` | `candidate scope requires package.json version 0.4.0` |
| root `scripts.test` drift with the version bump | `candidate scope rejected unauthorized package.json changes: scripts.test` |
| root `private: false` | `candidate scope rejected` |

### Ordinary precursor alignment (ci-web repair)

`ordinaryAdmitsPrecursorRootAlignment` admits exactly the one-time
`0.1.0` -> `0.3.0` root version-only transition in ordinary mode and removes
only that `package.json`/`version` forbidden label; every other forbidden
surface beside it still fails, and the `0.3.0` -> `0.4.0` transition remains
the closed g18.006 candidate rule.

| Plant | Result |
| --- | --- |
| root `0.1.0` -> `0.3.0`, version leaf only, `private: true` | admitted |
| wrong target (`0.4.0`), `scripts` drift, dependency drift, `private: false` | `forbidden version surface: package.json` |
| precursor alignment plus a `.npmrc` registry change | `certification scope rejected forbidden` |
| lone `0.3.0` -> `0.4.0` root bump without the closed candidate | `forbidden version surface: package.json` |

### Production-path closed-candidate plant (ci-web repair)

`test/package-install/web-preview.ts`'s synthetic `0.3.0` -> `0.4.0` candidate
fixture, `closedCandidateFiles(version)`, omitted root `package.json`. Once the
closed policy required root as a lockstep release input, the production-path
positive plant changed every other release surface but not root, so
`ordinaryAdmitsClosedCandidate` failed and reported all forbidden surfaces.

The fixture now carries a minimal private root manifest (`name`, version,
`private: true`, one stable script) with only `version` parameterized. The
positive production-path proof admits the complete closed candidate again, and
no plant semantics change.

## Census provenance proof

- `parsePreviewPackageVersion` reads exactly one `[package]` table and one
  quoted-semver `version` key. Missing, duplicate `[package]`, duplicate
  `version`, unquoted and non-semver values reject before any receipt is
  produced.
- `expectedTestReceiptContent(...)` takes the live version as an injected
  input; the planted manifest `version = "0.4.0"` yields receipt content with
  `"package_version": "0.4.0"`.
- `validateReceiptPackageVersion` binds every checked-in receipt to the live
  manifest; a planted stale `Button` receipt (`0.2.0` against live `0.3.0`)
  fails `--check` with its own provenance message.
- Deterministic regeneration at the current `0.3.0` identity is a **zero-line
  diff**: 176 rows, 73 rows with admitted capabilities, 65 mounted receipts,
  byte-identical. No capability claim, source commit, lock hash, run ID or
  test body moved.

## Validation

Budget: focused candidate-scope suite, `test:gpui-census`,
`check:gpui-census`, one read-only release status/config probe, `docs:lint`,
`git diff --check`. No mounted tests, full `qa`, release gates or windowed
conformance selectors were run.

- `bun test test/package-install/scope.test.ts` — 48 pass / 0 fail, including
  the exact root version-only transition plants and the ordinary precursor
  alignment plants.
- Read-only ordinary-mode scope probe over the real base-to-head range
  (`assertInstalledScope(repo, main, HEAD, "ordinary")`) — admitted after the
  repair; it rejected before.
- `effigy test:web-pack-install` — the production-path leaf proof (one run,
  clean checkout) admits the complete closed candidate after the root fixture
  repair.
- `effigy test:gpui-census` — 21 pass / 0 fail (17 existing oracles plus 4
  provenance laws).
- `effigy check:gpui-census` — checked-in artifacts match the generator and
  all oracles hold.
- `effigy release status --json` — resolves root `package.json` `0.3.0`
  (read-only).
- `effigy docs:lint` — pass.
- `git diff --check` — clean.

## Merge and review

- PR #264 merged as `aa659504b2a6222eb34c7423fcfd877a32138ba8` on 2026-09-12
  with parents `5635ab184cda6610f1f91eae1eff585db5e817b2` (main) and
  `a3f2a513a1d684a60a639c4904a606c6598c1f4d` (reviewed head). The merge
  matches the reviewed head exactly; no base refresh was needed.
- Accepted independent review is [PR comment
  5649246995](https://github.com/inflatable-cookie/poodle/pull/264#issuecomment-5649246995),
  bound to the exact head with `ready_to_merge`; it found no blocking findings.
- Exact-head GitHub `rust` (34s) and `web` (7m30s) checks were green at the
  merge gate. The `web` repair cycle is recorded above: the ordinary precursor
  admission plus the production-path closed-candidate root fixture restored
  the positive admission while all 37 negative oracles kept rejecting.

## Closeout (integration checkout, 2026-09-12)

- Verified `/Users/tom/Dev/projects/poodle` clean on `main` at
  `aa659504b2a6222eb34c7423fcfd877a32138ba8`, matching `origin/main` and the
  provider merge before this closeout batch.
- Closeout reran no implementation or broad validation suites. The validation
  above is the worker/reviewer evidence plus plugin-owned exact-head merge
  verification. No task-specific failure is deferred.
- Retained g18.006 now resumes on current main for the immutable `0.4.0`
  candidate and its final version-bound census repin. g18.009 remains
  dependency-queued behind that candidate for the hosted branch dry run, tag
  and publication. No release, tag, workflow or Desktop mutation occurred in
  g18.031.

## Limits

This PR does not prepare the `0.4.0` candidate, bump any other version, resume
retained g18.006, run the release gate, mark the PR ready, mutate Desktop, edit
workflows, tag or publish. It does not change `scripts/parity-evidence-ledger.ts`;
its `0.3.0` fallback only applies when no Nucleus row exists and is outside
this card's census scope. The retained task
`17ac3fee-de90-4b32-9672-1134770bb086` remains preserved with no candidate
commit or PR.

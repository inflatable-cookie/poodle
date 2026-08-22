# g15.049 — Release Automation Truthfulness

Status: **complete** — PR #66 accepted at `d8e293fa` and merged as `e47c44e5`
Depends on: none; may run in parallel once approved
Unblocks: `g15.050`
Governing refs: `../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../contracts/001-working-rules.md`, repository `AGENTS.md`,
`013-v020-release-certification.md`

## Problem

The release workflow is broadly sound, but the pre-tag native workflow still
calls the deleted `packages/gpui/components/Cargo.toml`. It therefore cannot
provide the native evidence its comments promise. Separately,
`effigy release gates` currently reports success with zero configured gates;
that result is vacuous and must not be cited as release proof.

## Goal

Make every advertised release/pre-tag path execute current Effigy-owned gates
and fail closed. Keep publication human-dispatched and preserve the current
decision about which npm packages actually publish.

## Fixed Decisions

- Convert every retained manual CI workflow to install pinned Effigy `0.11.0`
  and execute its named Effigy selector rather than duplicating the selector's
  raw commands:
  - `ci-web.yml` → `effigy ci:web`
  - `ci-rust.yml` → `effigy ci:rust`
  - `ci-native.yml` → `effigy ci:native`
  - `ci-visual.yml` → the matching `test:visual-smoke`, `ci:visual`, or
    `test:visual-sweep` selector for its explicit input
- Delete stale `.github/workflows/ci-conformance.yml`. Its only surviving work
  is already inside `ci:native`; its path filters and claims describe the
  removed g14 pilot. Retire the now-unneeded `ci:conformance` compatibility
  selector in the same clean break.
- Configure one non-vacuous local release gate:

  ```toml
  [release.gates.headless]
  command = "effigy qa"
  description = "Run Poodle's complete self-contained headless release board"
  ```

  Do not configure Effigy's version mutation in this card. Poodle's lockstep
  multi-manifest candidate remains owned by `g15.050`; `release prepare` and
  `release execute` remain out of scope.
- Keep `release.yml` human-dispatched, tag-targeted, and dry-run by default.
  Preserve the current publish set: core and Svelte only; React remains packed
  and certified but unpublished for v0.2.0.
- Pin third-party and GitHub-authored actions used by retained workflows to
  reviewed full commit SHAs, with the release tag in a comment. Pin Bun to
  `1.3.14`, Effigy to `0.11.0`, and the npm CLI used for trusted publishing to
  an exact reviewed version. Preserve least-privilege permissions and OIDC;
  do not introduce a registry token.

## Scope Envelope

- Repair the retained CI workflows and release gate exactly as fixed above.
- Reconcile release workflow comments, tag examples, package list, packed
  artifact verification, and native pre-tag instructions with v0.2.0.
- Use current action/security guidance and keep OIDC trusted publishing. Do not
  add automatic tag publication or long-lived tokens.
- Validate workflows without publishing, tagging, or mutating a release.

## Writable Scope

- `.github/workflows/{ci-web,ci-rust,ci-native,ci-visual,release}.yml`
- deletion of `.github/workflows/ci-conformance.yml`
- `effigy.toml` and `tasks/effigy.tasks.toml`
- release/pre-tag operator documentation and `g15.049`'s August log
- `packages/release-operations.json` only if current automation truth requires
  an existing policy field to change; do not alter the publish set
- `PAPERCUTS.md` for newly found execution friction

No component, package version, lockfile, release note, tag, publication,
registry, token, specimen, visual baseline, Rust implementation, or Jetstream
integration change belongs here.

## Acceptance Envelope

- [x] No retained workflow references a deleted package or duplicates a named
      Effigy selector with raw commands.
- [x] The manual native workflow exercises the same supported headless native
      board named by release documentation.
- [x] `effigy release gates` executes exactly one configured `headless` gate,
      which runs `effigy qa`; zero-gate success is impossible.
- [x] Dry-run release evidence packs exactly the intended npm artifacts and
      preserves the no-publish default.
- [x] Retained action references use reviewed immutable SHAs, tool versions are
      explicit, permissions remain least-privilege, and trusted publishing
      still uses OIDC with no long-lived npm token.
- [x] The stale conformance workflow and alias are gone; `ci:native` remains the
      single truthful owner of their retained headless native evidence.
- [x] No release, tag, or registry mutation occurs.

## Validation

- `actionlint .github/workflows/*.yml`
- static checks proving retained workflows invoke the exact Effigy selectors
  and contain no mutable action refs, stale package paths, or `NPM_TOKEN`
- focused execution of the retained selectors on their supported local OS,
  with `effigy release gates` as the final non-vacuous read-only board
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Do not dispatch any GitHub workflow, create or push a tag, run
`effigy release prepare/execute`, use `npm publish`, or run windowed/native-
visual or Jetstream selectors.

## Stop Conditions

- The change needs a workflow outside the fixed retained/deleted set above.
- A required release claim cannot be expressed through a supported Effigy
  selector.
- The fix would silently change the published package set.
- Current official GitHub/npm guidance conflicts with the fixed OIDC or action-
  pinning posture; report the source and stop rather than improvising.

## Continuation

Accepted `g15.049` closes the automation input to `g15.050`. The now-truthful
release board exposes an existing GPUI/Zed dependency-licence policy failure;
that decision remains open alongside `g15.012`/`g15.047`, `g15.043`, and the
other release gaps. No candidate work starts while any of them remains open.

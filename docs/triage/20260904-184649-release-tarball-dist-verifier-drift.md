# Release Tarball Verifier Drifted Behind Compiled Distribution

Status: open — g16.097 stopped before tag or publish; workflow repair awaits
explicit operator approval
Captured: 2026-09-04
Candidate: `b4158a1b68db9292c17be1d8c219f0fc26512a0b`

## Issue

The g16.097 v0.3.0 certification reached a deterministic mismatch between the
compiled web-package contract and `.github/workflows/release.yml`. The release
workflow still requires core tarball members under `package/src/**`; the core
package intentionally publishes only `dist`, README, licence, and attribution
files.

This blocks the dry-run path even though the candidate's headless gates and
installed-package proof are green. No `v0.3.0` tag exists locally or remotely,
and no publish was attempted.

## Confirmed Evidence

- A fresh detached checkout at the exact candidate completed
  `bun install --frozen-lockfile`; `lucide-static` resolved checkout-locally to
  `1.31.0`; the tracked tree and package lock stayed unchanged.
- `env -u POODLE_WEB_PACK_INSTALL_SCOPE_MODE effigy release gates --json`
  passed with `ok=true`, no blockers, in 685816 ms.
- Ordinary no-mode `effigy test:web-pack-install --json` passed 11 files and
  22 tests against the exact candidate.
- Local pack proof produced core, Svelte, and private React archives with
  package metadata present. Core SHA-256:
  `3d46bc39d6247af10f5052e6060afdb42c07220b1633d16b6029b018e0d58719`.
- Core's manifest allowlist is `dist`, `README.md`, `LICENSE`, and
  `THIRD_PARTY_NOTICES.md`. Its exports and canonical build contract point to
  `dist/**`.
- The compiled distribution architecture requires source-free archives and
  the actual archive member `package/dist/.poodle-build.json`.
- `release.yml` still requires at least 50
  `package/src/icons/icons/*.ts` files, one
  `package/src/icons/aliases.generated.ts`, and at least 20
  `package/src/tokens/generated/css/*.css` files.
- Those workflow assertions landed on 2026-08-10. The compiled core package
  boundary landed later in g16.057 on 2026-09-02. No release-workflow migration
  followed.

## Settled Direction

Do not add `src/**` back to the published package. That would violate the
compiled, source-free distribution contract and widen the public artifact.

The narrow repair is a workflow lane that updates pack verification to the
published `dist/**` surface while preserving all release safety gates. At
minimum it should prove:

- `package/dist/.poodle-build.json` exists;
- runtime icon modules exist under `package/dist/icons/icons/*.js`;
- matching icon declarations exist under
  `package/dist/icons/icons/*.d.ts`;
- the compiled alias declaration exists at
  `package/dist/icons/aliases.generated.d.ts`;
- token CSS exists under `package/dist/tokens/generated/css/*.css`;
- licence, README, manifest, tag targeting, dry-run default, publish guard,
  package set, and artifact upload behavior are unchanged.

The repair should add a static counterexample in
`scripts/check-release-automation.ts` so a future source/dist path drift fails
before a release attempt. Validation must include the release-automation
selector, a local pack listing at the exact repaired head, the full headless
release gate, and `git diff --check`. It must not tag, dispatch, or publish.

## Unresolved Operator Choice

Workflow edits require explicit operator approval. Confirm whether to promote
this into an urgent bounded repair card serially before g16.097, then resume
certification from the repaired `main` tip. The current candidate cannot be
tagged because the candidate tree contains the stale release verifier.


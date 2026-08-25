# g15.077 — Jetstream Poodle 0.2.2 Adoption

Date: 2026-08-25
Status: complete
PR: [Jetstream #1](https://github.com/inflatable-cookie/jetstream/pull/1)
Reviewed head: `3efd9dde8c951fb26c1fcfaef4fdcde2ed9445a1`
Merge commit: `be9692595bd0c5da27527969c3631a12133a0e44`

## Result

Jetstream's editor now resolves `@inflatable-cookie/poodle-core` and
`@inflatable-cookie/poodle-svelte` from npm at exact 0.2.2. Both lock entries
carry published integrity and no active `file:../../poodle` web override
remains. The three retained local Longhorn packages stay local, and
`longhorn-poodle-svelte` converges on the same exact Poodle Svelte peer.

The paired Rust boundary did not move. `crates/jetstream-poodle/Cargo.toml`
keeps its six local Poodle paths and the committed Rust lock is unchanged. The
adapter absorbed current 0.2.2 contract shape: AccessKit TabPanel mapping,
ProgressRing's accepted deferred Jetstream fallback, and RenderContext-based
smoke construction. Jetstream `g06.014` records the Poodle npm half as complete
without claiming its retained Longhorn package lane or no-sibling build is
complete.

## Review

The first review found one factual docs error: `g06.014` said six retained
Longhorn web packages where the active editor declares three. Head `3efd9dde`
corrected only that count. The shared GitHub identity prevents formal
self-approval, so the evidence-backed approval is recorded in PR comments.

## Validation

Independent review ran only headless surfaces. It proved:

- frozen Bun install from the PR lock;
- 67 editor tests;
- workspace check, sibling-boundary check, demo registry, and single-UI-stack
  check;
- five `jetstream-poodle` tests;
- docs, formatting, and workspace Clippy through `effigy validate`;
- `git diff --check` and a clean reviewed dependency/lock diff.

The broad test board reached 478 passes before the same two existing
`jetstream-benchmark` milestone-metadata tests failed because
`docs/roadmaps/current-milestone.json` lacks `benchmark_envelope_path`; 2,016
later tests were cancelled by fail-fast. Neither failing code nor metadata was
changed by the PR. Jetstream `main` before the PR failed earlier on the new
TabPanel and ProgressRing enum variants, proving the adapter edits were needed.
No visible editor, demo, or native window ran.

## Generation Closeout

Jetstream was the last of 16 authoritative Poodle consumers. Its merge closes
`g15`. Loophole Legacy `g15.078` remains cancelled because the operator removed
that repository; no recreation or adoption evidence is required.

The transitions.dev motion research and Longhorn-backed conformance-lab notes
remain open in `docs/triage/`. They are not silently promoted by this release
closeout.

## Next Task

Return to an operator-led planning checkpoint. Re-read current vision,
architecture, carry-forward gaps, and open triage before compiling a new
generation.

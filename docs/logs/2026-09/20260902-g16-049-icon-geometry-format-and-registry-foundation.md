# g16.049 — Icon Geometry Format And Registry Foundation

Status: implementation complete — PR #156
Date: 2026-09-02
Card: `docs/roadmaps/g16/049-icon-geometry-format-and-registry-foundation.md`
Handoff: `docs/handoffs/20260902-004204-g16-049-icon-geometry-foundation.md`
Architecture: `docs/architecture/013-icon-geometry-substrate.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/icon.md`,
`docs/contracts/components/icon-provider.md`,
`docs/architecture/012-semantic-motion-policy.md`
Branch: `feature/g16-049-icon-geometry-foundation`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-049-icon-geometry`
Starting exact head: `c1a527898e7425853359bd72b7113a8cf38b8d97`
Planning base ancestor: `7f59ae42f4917c675968819eb23a5e41dc90013c`
Rebased onto live `origin/main`: `a52d0d32bdcf78d219c22449ad870ff3641e2569`
Post-rebase implementation commit: `f617a37d3`
Current closeout head: `8d56191b5`
PR: https://github.com/inflatable-cookie/poodle/pull/156

## Outcome

Delivered the internal icon-geometry foundation only. TypeScript and Rust now
share a versioned 24×24 stroke normalizer, canonical line-segment endpoints,
64-point sampled contours, bounded deterministic correspondence, and the same
13-vector corpus. The authored manifest has 11 explicit pair states: five
accepted, one candidate, and five rejected.

The generated TypeScript and Rust registries carry canonical aliases, source
node and generated-asset SHA-256 digests, topology, diagnostics, quality
review, notice identity, payload size, and derived registry data. The largest
accepted payload is 14,023 bytes; the registry digest is
`410e617ea898b912e3f3eb2f73be458457a203964c6eabeb861e9ae4af06c4cc`.

No `Icon`, `IconProvider`, node vocabulary, package export, runtime, browser,
native visual, or Jetstream surface was changed.

## Falsification

Each plant was applied temporarily, the intended check failed, and the exact
source was restored before the green reruns.

| Oracle | Plant and observed bite | Restored state |
| --- | --- | --- |
| Grid | accepted view-box width check changed from 24 to 23; the shared vector suite failed on all canonical inputs and the off-grid expectation | exact 24×24 check; focused suite green |
| Transform | transform rejection disabled; transformed vector returned `unsupported-attribute`, not `unsupported-transform` | typed transform rejection |
| Topology | contour-count failure mislabeled as `pair-closure`; menu ↔ x vector failed on the required `pair-contour-count` code | typed contour-count rejection |
| Correspondence | reverse traversal disabled; reverse vector expected `true`, received `false` | reverse traversal retained |
| Closed start | closed offsets limited to zero; diamond vector expected offset 48, received 0 | exhaustive cyclic offsets retained |
| Source bytes | one generated SVG stroke byte changed; `effigy audit:icons` failed with source-byte drift | source asset restored |
| Generated payload | registry digest changed by one hex digit; `effigy audit:icons` listed the generated TS projection as stale | generated projection restored |
| Manifest identity | a reversed duplicate pair was introduced; audit rejected it | unique canonical pair identities |
| Candidate review | candidate quality was changed to `accepted`; audit rejected the mismatched review state | candidate remains explicit |
| Static Icon | Svelte `Icon` viewBox changed to 23×24; exact static-contract assertion failed | `Icon.svelte` restored; final diff excludes public Icon surfaces |

Clean regeneration was run twice. The TypeScript projection hash was
`63524ce029c76ca7430970fb73e5a359d31e1abf8ea89363c61cddeccb1427db` and the
Rust projection hash was
`a110e97fab04dfde895a262bd57eb934fb6e66c9ee43c046576cc2990a7b44f3` on both
runs.

## Validation

Focused and repository checks:

- `bun test packages/core/test/icon-geometry.test.ts packages/core/test/icon-geometry-registry.test.ts` — 17 tests pass.
- `cargo test --quiet --manifest-path packages/contracts/components/Cargo.toml --lib icon_geometry` — 2 tests pass.
- `effigy test:core` — 1,172 tests pass / 4,032 expectations.
- `effigy audit:icons` — pass after deterministic regeneration.
- `effigy ci:rust` — pass, including the component contract suite.
- `rustfmt --check --edition 2021 packages/contracts/components/src/icon_geometry.rs` — pass for the new Rust module.
- `bun x tsc -p packages/core/tsconfig.json --noEmit` — existing repository diagnostics remain; no diagnostics match the new geometry files.
- `git diff --check` — pass after every plant restore.

Final board results:

- `effigy ci:web` — all substantive checks passed: 372 test files / 3,498
  tests, 20 packed-consumer tests, zero Svelte-check errors, and docs/specimen
  drift checks clean. The selector exited 1 only at `gate:clean`, which could
  not see the fixed `os.tmpdir()` snapshot after the composed task run. A
  direct `bun scripts/gate-tree-guard.ts --snapshot` followed by `--compare`
  passed; the existing shared-temp-path papercut is recorded in `PAPERCUTS.md`.
- `effigy docs:check` — pass, including the Svelte build and generated report
  checks.
- `effigy ci:rust` — pass; the component suite reports 291 passed tests and
  the full contract board is green.
- The pre-rebase `effigy qa` reached native/headless and license checks, then
  exited 1 at the old baseline `audit:security` false positive. After the
  required rebase onto `fa8e657`, direct `bun scripts/audit-repository-security.ts`
  is clean.
- The standalone post-rebase `effigy ci:web` passed: 372 files / 3,535 tests,
  20 packed-consumer tests, zero Svelte-check errors, and the composed tree
  guard passed. The final post-rebase `effigy qa` reached its nested `ci:web`
  but exited 1 when that invocation lost the fixed `os.tmpdir()` gate snapshot;
  the same known Effigy shared-temp-path race is recorded in `PAPERCUTS.md`.
- No geometry-related selector failed, and no new geometry or public-icon file
  was reported by any board.
- Final `git diff --check` — pass. The exact `git diff --check
  origin/main...HEAD` check is run again after the implementation commit.

## Scope notes

`effigy doctor` retains the repository's pre-existing generated-source,
god-file, stale-suppression, and comment-ratio findings. The two additional
generated projections are intentional checked-in lineage required by this
card; no suppression was added. The root Cargo workspace has no manifest, so
the repository-wide `cargo fmt --all` invocation is not applicable; the new
Rust module's isolated rustfmt check passes.

## Next task

An accepted merge unlocks orchestrator review of `g16.050`; this worker does
not launch it or admit `IconMorph`.

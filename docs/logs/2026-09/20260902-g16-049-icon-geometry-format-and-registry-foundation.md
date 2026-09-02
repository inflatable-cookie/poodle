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
Rebased onto live `origin/main`: `1b0d4032984a65c64398fbcd71572a9093a47ace`
Post-rebase implementation commit: `38beec197`
Parser repair commit: `fea4287a7c6db0c80dc2a8b465e0f6b9e8864ff`
PR: https://github.com/inflatable-cookie/poodle/pull/156

## Outcome

Delivered the internal icon-geometry foundation only. TypeScript and Rust now
share a versioned 24×24 stroke normalizer, canonical line-segment endpoints,
64-point sampled contours, bounded closure-preserving correspondence, a strict
SVG numeric grammar, and the same 18-vector corpus. The authored manifest has
12 explicit pair states: six structurally plannable candidates and six
rejected. No g16.049 pair is accepted or runtime-eligible; candidate geometry
is retained only as internal evidence and a possible g16.050 test fixture.

The generated TypeScript and Rust registries carry canonical aliases, source
node and generated-asset SHA-256 digests, topology, diagnostics, quality
state and notes without reviewer identity, notice identity, payload size, and
derived registry data. The largest candidate fixture payload is 14,023 bytes;
the registry digest is
`369861be0c28f08f638dffa01afae71b001aee1a945bfb94e7b5e78872b34f3c`.

The Rust vector proof uses derived fixture structs and `serde_json::from_str`
with bounded dev dependencies; the handwritten JSON parser is removed. The
two explicit `Vec<usize>` empty-vector assertions in `code_input.rs` are the
minimal type disambiguation required by serde_json's generic numeric equality
implementations.

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
| Closure assignment | assignment edges allowed every closure; mixed vector selected the cheaper invalid `[0→1, 1→0]` mapping with offset `56` | only equal-`closed` assignment edges; TS and Rust focused tests green |
| Numeric attributes | TS `Number(raw)` accepted `x1="0x10"` and `x1="0b10"`; shared malformed-attribute vectors failed | anchored full-string SVG grammar in both parsers |
| Exact paired wire | one Rust sample coordinate changed by `+1`; the circle/ellipse wire digest failed (`0cb3a0d49f232a06` vs `30494cadafe64aa6`) | Rust sampling restored; shared endpoint/pair digests green |
| Bidirectionality | reverse-plan offset changed by `+1`; exact reverse-flight checks failed in TS (2 vectors) and Rust (closed-loop endpoint mapping) | inverse traversal/cyclic-start mapping restored |
| Candidate planning | candidate target changed to `ellipsis`; generator failed `circle-to-dot must normalize and plan: pair-contour-count` | `circle-to-dot` remains a plannable candidate; menu-to-ellipsis rejected |
| Source bytes | one generated SVG stroke byte changed; `effigy audit:icons` failed with source-byte drift | source asset restored |
| Generated payload | registry digest changed by one hex digit; `effigy audit:icons` listed the generated TS projection as stale | generated projection restored |
| Manifest identity | a reversed duplicate pair was introduced; audit rejected it | unique canonical pair identities |
| Candidate gate | a structurally plannable pair was changed from `candidate` to `accepted`; the foundation audit rejected accepted status before the visual gate | six candidate fixtures remain explicit; no pair is runtime-eligible |
| Static Icon | Svelte `Icon` viewBox changed to 23×24; exact static-contract assertion failed | `Icon.svelte` restored; final diff excludes public Icon surfaces |

Clean regeneration was run twice. The TypeScript projection hash was
`53767e226cd96a9439c0cf410eff45257b393931bf1a353c8c18ede568e70222` and the
Rust projection hash was
`5bbe61957753ebdc0cac500b1c7a7e311819a37f2ebd2cf2f3c158ec7bc3ed99` on both
runs.

## Validation

The repair batch passed the required focused and repository checks:

- `bun test packages/core/test/icon-geometry.test.ts packages/core/test/icon-geometry-registry.test.ts` — 22 tests pass / 1,124 expectations, including malformed numeric attributes, exact paired wire/cost, closure assignment, registry planning, and reverse-flight checks.
- `cargo test --quiet --manifest-path packages/contracts/components/Cargo.toml --lib icon_geometry` — 2 tests pass, independently parsing the shared corpus and matching the same endpoint digests, mappings, costs, closure rules, and reverse-flight oracle.
- Rust fixture decoding — derived `serde::Deserialize` structs with `serde_json::from_str`; no handwritten JSON parser remains.
- `effigy icons:build` — pass twice with identical generated projection bytes. SHA-256: TypeScript `53767e226cd96a9439c0cf410eff45257b393931bf1a353c8c18ede568e70222`; Rust `5bbe61957753ebdc0cac500b1c7a7e311819a37f2ebd2cf2f3c158ec7bc3ed99`.
- `effigy audit:icons` — pass: 12 pair records (0 accepted, 6 candidates, 6 rejected) and 108 default icon names (92 canonical, 16 aliases).
- `effigy ci:rust` — pass: 291 component contract tests passed, with the full Rust board green.
- `effigy docs:check` — pass, including docs inventories, generated reports, and the Svelte production build.
- `rustfmt --check --edition 2021 packages/contracts/components/src/icon_geometry.rs` — pass.
- `git diff --check` — pass; the exact `git diff --check origin/main...HEAD` check is repeated after the repair commit.
- `packages/contracts/components/Cargo.toml` restores the bounded `serde` derive and `serde_json` dev dependencies. Their generic numeric `PartialEq<Value>` implementations require two existing empty-vector assertions in `code_input.rs` to carry explicit `Vec<usize>` type information; no formatter-only churn is retained.

Exact paired-oracle receipts:

- `reverse-open-correspondence`: pair digest `3d5068f01f9e23a5`, total cost `0`.
- `closed-loop-offset`: pair digest `3126562101d000af`, total cost `0`.
- `mixed-closure-cross-cost`: pair digest `56a3f427c3c87d1`; mappings `0→0` open cost `1822668` and `1→1` closed cost `2029089`; total cost `3851756`.
- The one-unit Rust sample plant changed the circle/ellipse endpoint digest to `0cb3a0d49f232a06` instead of `30494cadafe64aa6` and failed; restoring the sample returned both languages to the shared oracle.

## Scope notes

`effigy doctor` retains the repository's pre-existing generated-source,
god-file, stale-suppression, and comment-ratio findings. The two additional
generated projections are intentional checked-in lineage required by this
card; no suppression was added. The root Cargo workspace has no manifest, so
the repository-wide `cargo fmt --all` invocation is not applicable; the new
Rust module's isolated rustfmt check passes.

## Next task

Once the g16.049 foundation is merged, orchestrator review may continue with
`g16.050`; candidate geometry remains an internal fixture only. This worker
does not launch it or admit `IconMorph`.

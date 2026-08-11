---
title: g13 batch 015 — deterministic emission and drift-gating patterns (research)
status: complete
milestone: g13.003 research precursor (codegen)
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, research, codegen, emission, drift, ts-rs, schemars, spec-063]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/015-deterministic-emission-and-drift-gating.md`
on branch `thread/g13-015-emission-drift-patterns`. Research only: the card's
five decision questions (invocation shape, determinism, drift gating,
diagnostics, type-mirroring tool role) are addressed with primary-source
evidence — no recommendation, no dependency choice, no code, no benchmark, no
status-file edits.

The strongest evidence is the repo's own production generator
(`packages/tokens/scripts/build-tokens.ts`, one DTCG source → CSS/TS/Rust,
`--check` mode), read first; then the second `--check` generator
(`scripts/build-default-icons.ts`), the three report generators, the five
drift gates, and the 18 generation/drift selectors in `tasks/effigy.tasks.toml`.
Vendored crate sources read on disk: `ts-rs-macros-12.0.1` / `ts-rs-12.0.1`
and `schemars-1.2.2` / `schemars_derive-1.2.2` under
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`. `typeshare` and
`specta` are recorded **unevaluated** — not vendored on this machine.

## Deliverables (only the scoped writes)

- `docs/research/value-tracks/tk-deterministic-emission-and-drift-gating.md` —
  the evidence document: case studies with file:line citations, the
  `docs:check`/`audit:tokens` failure section, the invocation-shape
  comparison table, ten failure modes each mapped to its smallest check,
  vendored-crate assessment, unresolved questions. Every statement labeled
  verified fact / source-author claim / worker inference.
- `docs/logs/2026-08/11-g13-015-emission-drift-patterns.md` — this log.
- `PAPERCUTS.md` — one new entry (stale-orphan blindness in
  `build-tokens.ts --check`); the two existing entries covering this card's
  named failure were already present and were not duplicated.

Nothing outside the three writable paths changed.

## Key findings (full evidence in the value track)

- **In-repo generators are byte-deterministic by construction**: sorted file
  loads (`build-tokens.ts:92`), insertion-order walks, `JSON.stringify(x,
  null, 2)`, locale-independent float formatting (`formatF32`, `:684-687`),
  constant headers (`:362-364`), no timestamps/absolute paths. Icons pins its
  source catalogue version and fails if the resolved package disagrees
  (`build-default-icons.ts:42-44`).
- **`--check` is a proven, read-only pattern**: flag-selected
  (`build-tokens.ts:10`), byte-compares committed output
  (`compare`, `:243-251`), throws one sorted, deduped, all-paths error
  (`:981-987`), exit 1. Verified live at HEAD (see below).
- **The drift gates are readers, not emitters** — all-findings-at-once,
  `DRIFT_REPORT=1` / `VALUE_DOMAIN_ENFORCE=1` escapes, exit 1 on gate-class
  findings, diagnostics pointing at authored source (slug/prop/role), never
  at generated output.
- **`ts-rs` 12.0.1 is a type-mirroring derive**: `#[derive(TS)]` emits
  `impl TS` (name/decl/inline/output_path) over Rust type structure;
  `#[ts(export)]` writes files only from a generated `#[cfg(test)] #[test]`
  fn (its docs: proc macros cannot write at compile time); default output is
  single-line; pretty printing needs the `format` feature. It has no concept
  of IR instances, permitted subsets, registries, or evidence.
- **`schemars` 1.2.2 also derives from types** but produces JSON Schema
  (default draft 2020-12) at runtime via `SchemaGenerator`, with sorted
  internal containers (BTreeSet/BTreeMap) and a documented less-precise
  value-driven path (`root_schema_for_value`). It is the only vendored
  candidate that could emit a schema for the serialized IR artifact itself —
  distinct from component surfaces — but it still mirrors types, not data.
- **No `build.rs` exists anywhere in the repo**; standalone scripts are the
  only invocation shape with precedent. The ruled `poodle-codegen` shape
  (card 003 R1) is lib + `[[bin]]`.

## The live failure, characterised (not fixed)

Both facts were verified, not just cited:

- `bun packages/tokens/scripts/build-tokens.ts --check` at HEAD exits 1,
  listing exactly seven `packages/tokens/artifacts/rust/*` files
  (density, metadata, primitives, themes, typed/mod, typed/primitives,
  typed/semantic), and leaves `git status` clean (check mode is read-only).
- Commit `45caae82` (2026-08-11 08:38, "Format agent subagent contracts and
  generated Rust tokens") hand-formatted those seven artifacts (rustfmt:
  8-space array elements, multi-line `ColorValue(...)` wrapping) without
  touching `build-tokens.ts`, whose templates emit 4-space elements
  (`buildRustDefinitionArray`, `:765-773`) and single-line consts
  (`formatColorConst`, `:689-690`). The removed lines in that commit are
  byte-identical to current generator output, so `audit:tokens` broke exactly
  at that commit.
- `docs:check` rewrites those files because it composes
  `report:parity` (`tasks/effigy.tasks.toml:256`) → `tokens:build`
  (write mode, `:46`, `:11`), not `audit:tokens` (`:17`). A write-mode
  generator inside a gate manufactures the expected state; the only symptom
  is a dirty worktree, which nothing checks.
- **Detection gap**: the check gate exists but is wired into `ci:web`, not
  into `docs:check`; the offending commit is a formatting-only commit with no
  mechanical guard against hand-editing generated files; and the `--check`
  diagnostic lists paths but not why (no diff, no whitespace-vs-content
  classification).
- **Prevention shape** (evidence, not implementation): gates compose check
  selectors only; one authority owns output bytes (emitter or pinned
  formatter, never both); write mode is excluded from read-only gates; a
  post-gate `git status` cleanliness check is the cheapest backstop.

## Validation

| Command | Exit state |
|---------|-----------|
| `bun packages/tokens/scripts/build-tokens.ts --check` | 1 — expected: the seven rust artifacts drift at HEAD (read-only; `git status` empty after) |
| `git diff --check` | 0 |
| `git status --porcelain` | only the three writable paths |
| `effigy docs:lint` (read-only docs gate; no generated data rewritten) | 0 |
| `git push -u origin thread/g13-015-emission-drift-patterns` | 0 — pushed, no merge |

## Not done

Per batch card and worker rules: no recommendation, no dependency selection,
no code, no benchmark, no proof of concept, no edits to specs/architecture/
contracts/roadmaps/dispatch/code/manifests/lockfiles/generated artifacts, no
Tabs/AppHeader files (other workers hold them), no `git add -A`, no merge.
`typeshare`/`specta` intentionally left unevaluated (not vendored). The
`docs:check`/`audit:tokens` failure was characterised, not fixed — fixing
would require editing a generator or its artifacts, which is a stop
condition.

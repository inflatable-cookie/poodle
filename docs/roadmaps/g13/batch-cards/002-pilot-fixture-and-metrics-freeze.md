# 002 Pilot Fixture And Metrics Freeze

Status: ready
Milestone: `g13.001`
Owner: Poodle core
Branch: `thread/g13-002-pilot-fixture-metrics`
Depends on: `g13-b001` merged (`251cc858`), `g13-b005` merged (`2f8dc5db`)
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-05`–`IR-11`), `docs/contracts/components/button.md`,
`docs/contracts/components/range-slider.md`,
`docs/contracts/components/text-input.md`,
`docs/roadmaps/g13/authority-inventory.md`,
`docs/roadmaps/g13/pilot-expressiveness-corpus.md`

## Goal

Turn the merged authority inventory and expressiveness corpus into stable,
identified pilot fixtures and a reproducible quantitative before-state. This is
measurement and binding work. It designs no schema and creates no package.

## Orchestrator Rulings (already made — do not re-decide)

1. **Manifest path and form.** One markdown document,
   `docs/roadmaps/g13/pilot-baseline-manifest.md`. Markdown tables, not JSON,
   and not inside any `packages/` tree — no package, crate, or generated
   evidence surface may be created or extended by this card.
2. **Fixture identifiers.** Fixtures are `FIX-*` scenario IDs. Every fixture
   binds to one or more existing corpus requirement IDs (`CROSS-*`, `BTN-*`,
   `RNG-*`, `TXT-*`, `SHELL-*`) from the merged corpus. Do not invent a second
   requirement vocabulary.
3. **`UNKNOWN-01` and `UNKNOWN-02` stay open.** Embedded RangeSlider
   `aria-orientation` scope and Rust Button `Danger`/`Success` are maintainer
   decisions owned by `g13-b003`. Record them in the manifest as blocked
   fixtures with no assumed answer. Choosing either reading is a stop
   condition.
4. **`GAP-01`–`GAP-07` are measured, not closed.** A gap is recorded as a
   baseline zero or "absent" with its path. Producing the missing evidence is
   out of scope.
5. **Visual baselines are referenced, never refreshed.** No `--update`, no
   baseline regeneration, for web or native.
6. **Token artifacts.** `effigy docs:check` rewrites
   `packages/tokens/artifacts/rust/*` (known papercut). Restore them with
   `git checkout -- packages/tokens/artifacts/rust/` after any such run. Never
   commit that rewrite.
7. **`docs/parity/*.md` is historical evidence only**, never authority. Its
   source paths are stale (corpus `OBS-04`).
8. **Bootstrap.** Run `bun install` once before any web generator or build
   command; the React parity report cannot resolve workspace imports without
   it.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents or parallel research tasks. Read sources directly.
- Component contracts are semantic authority; the merged inventory and corpus
  are the measured evidence base.
- Do not edit roadmap README files, roadmap milestone files, this card's
  status, `docs/roadmaps/dispatch.md`, architecture, specs, working rules, or
  contracts.
- Do not change component behavior, public APIs, implementation source, CSS,
  tokens, specimens, registries, tests, or Effigy configuration.
- Do not create crates, packages, shims, generators, or schema.
- Do not hand-edit generated artifacts.
- Where sources conflict, stop with exact citations. Do not choose an
  authority.
- Commit on the branch above and push with
  `git push -u origin thread/g13-002-pilot-fixture-metrics`. Do not merge.

## Writable Paths

Exactly three:

- `docs/roadmaps/g13/pilot-baseline-manifest.md`
- `docs/logs/2026-08/<DD>-g13-002-pilot-fixture-and-metrics-freeze.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Any other changed path is a scope failure.

## Scope

### In scope

- Fixture identification and binding for Button, RangeSlider, TextInput, the
  preview shell, theme selection, and the size/density specimen axes across all
  four runtimes.
- Quantitative before-state measurement using the method fixed below.
- Reading any repository file needed as evidence.

### Out of scope

- Any schema, representation, codegen, or crate-placement proposal.
- Resolving `UNKNOWN-01`/`UNKNOWN-02` or closing any `GAP-*`.
- Refreshing visual or native baselines.
- Fixing unrelated lint/test failures found during the batch.

## Steps

### 1. Bootstrap and baseline gates

```sh
bun install
effigy svelte:surface-audit
effigy docs:lint
git diff --check
```

Record exit states. If any is non-zero on a clean checkout of `main`, stop and
report — `main` was green at merge `bb3f79ef`.

### 2. Freeze fixtures

Build the manifest's fixture tables. One row per fixture with these columns:

| Column | Content |
|---|---|
| Fixture ID | `FIX-BTN-nn`, `FIX-RNG-nn`, `FIX-TXT-nn`, `FIX-SHELL-nn`, `FIX-AXIS-nn` |
| Scenario | what is rendered/exercised, in one line |
| Corpus IDs | the requirement IDs it covers |
| Owning contract | contract file + section |
| Svelte / React / GPUI / Jetstream | implementation or specimen path per runtime, or `absent` + the gap ID |
| Existing evidence | specimen slug, visual tier/baseline, a11y target, conformance vector, or `none` |
| Status | `frozen`, or `blocked:<UNKNOWN-id>`, or `gap:<GAP-id>` |

Cover at minimum: every specimen in the three contracts' §13 specimen sets;
the four preview-shell theme/size/density/contrast controls (`SHELL-01`–`04`);
navigation, search, and specimen tabs (`SHELL-05`–`07`); and the size/density
axis mechanism per runtime, including that Jetstream has no shared
`SpecimenLayout` helper (inventory §6).

Every fixture must be traceable to an existing surface. Do not add specimens,
tests, or registry entries.

### 3. Measure the quantitative baseline

Fixed measurement method — use these exact definitions so the numbers are
reproducible:

| Measure | Method |
|---|---|
| Authored LOC (per pilot component, per runtime) | `wc -l` over the exact files in inventory §5 for that component/runtime |
| Authored LOC (surface totals) | `wc -l` over the globs in inventory §1, reporting the glob with each number |
| Generated LOC | `wc -l` over `packages/{svelte,react}/preview/artifacts/*.json`, `packages/core/src/tokens/generated/*`, `packages/core/src/icons/generated.ts`, `packages/tokens/artifacts/**` |
| Duplicated definition count | per pilot component, the number of authored expressions of one contract, enumerated by path (inventory §8.1) |
| Runtime extension count | `EXT`-classified corpus rows, total and per component |
| Clean build time | `cargo build -p poodle-render` and `cargo build -p poodle-specs` from `cargo clean`, wall-clock, 1 run each; plus `bun run --cwd packages/svelte/preview build`. Record machine and whether the cache was cold |
| Diagnostic quality | for each drift gate (`docs:contract-drift`, `docs:spec-drift`, `drift:roles`, `drift:adapter-manifests`, `svelte:surface-audit`, `docs:lint`), quote its failure-message construction verbatim from the gate's own source, with file and line. Do **not** induce failures by mutating source |
| Four-runtime drift count | per pilot component: documented intentional deltas + open parity items, counted from `docs/parity/{button,range-slider,text-input}.md`, `packages/gpui/cross-runtime-parity-report.json`, `packages/jetstream/cross-runtime-parity-report.json`, and the corpus `EXT`/`GAP` rows. Report the sources separately before totalling |

Jetstream builds require the sibling jetstream checkout. If it is unavailable,
record `not measurable in this environment` with the reason; do not estimate.

### 4. Validate and commit

```sh
effigy docs:lint
effigy docs:check
git checkout -- packages/tokens/artifacts/rust/
git diff --check
git status --porcelain
```

Record every command and exit state in the batch log. Confirm only the three
writable paths changed. Then commit and push.

## Acceptance Criteria

- [ ] `pilot-baseline-manifest.md` exists at the ruled path with fixture tables
  covering all three pilot components, the preview shell, and the size/density
  axes across four runtimes.
- [ ] Every fixture has a stable `FIX-*` ID, at least one corpus requirement
  ID, an owning contract section, per-runtime paths or a named gap, and a
  status.
- [ ] Every quantitative measure in step 3 is recorded with the command that
  produced it, or an explicit `not measurable` reason.
- [ ] `UNKNOWN-01` and `UNKNOWN-02` appear as blocked fixtures with no assumed
  answer.
- [ ] `GAP-01`–`GAP-07` each map to at least one fixture row or an explicit
  statement that no fixture covers them.
- [ ] No package, crate, schema, generator, or representation appears.
- [ ] No visual baseline refreshed; no token artifact rewrite committed.
- [ ] `effigy docs:lint`, `effigy docs:check`, and `git diff --check` exit 0.
- [ ] Only the three writable paths changed.
- [ ] Batch log records commands, exit states, and measured counts.

## Evidence

- Fixture and measurement tables in `pilot-baseline-manifest.md`.
- Commands with exit states and `git diff --stat` in the batch log.
- No recommendation required; the orchestrator rules on placement in
  `g13-b003`.

## Stop Conditions

- A fixture cannot be bound without resolving `UNKNOWN-01` or `UNKNOWN-02`.
- A contract and a live implementation disagree on public semantics beyond the
  corpus `OBS-*` rows already accepted.
- A required evidence surface named by the inventory does not exist.
- A measurement cannot be taken without changing source, refreshing a baseline,
  or creating a package.
- Work expands into schema, codegen, or crate placement.

Stop with fixture IDs, exact paths, commands, and the smallest unresolved
question. Do not patch around the conflict.

## Exit

- Every pilot acceptance claim has a stable fixture/evidence identifier.
- The quantitative baseline is reproducible from a clean worktree.
- Remaining unknowns are placement/schema decisions owned by `g13-b003`.

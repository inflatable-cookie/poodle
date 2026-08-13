# 052 Vocabulary Coverage — First Tranche, And Its Cost

Status: ready
Milestone: `g13.019` (part 1 — **this card does not close `g13.019`**)
Owner: Poodle core
Branch: `thread/g13-052-vocabulary-coverage-first-tranche`
Depends on: `g13-b051` (`23337fce`), merged — `g13.018` is complete
Governing refs: `docs/roadmaps/g13/019-vocabulary-coverage.md`,
`docs/roadmaps/g13/pilot-verdict-evidence.md` (**§7–8**),
`docs/specs/063-rust-authored-component-and-scene-ir.md` (the narrowed scope)

## Goal

`g13.019` says "extend vocabulary coverage across the corpus, family by
family". Before doing that 165 times, find out what one tranche actually costs
and what it actually catches.

This card produces **two numbers**. They decide whether `g13.019` continues,
narrows, or hands straight to `g13.020`.

## Current State — Measured

### The corpus projection, which is why this card is bounded

| | |
|---|---|
| Components | **168** |
| Total props across the corpus | **1,982** |
| Authored so far (3 definitions) | 5,115 lines for 101 props |
| Rate | **~51 lines per prop** |
| Projected for the remaining 1,881 props | **~95,000 lines of authored Rust** |

That is **triple the entire pilot machinery** (~27,900 after `g13.017`), for
vocabulary that executes nothing.

The rate is not doc-comment padding: `models/button.rs` is 1,372 lines of which
130 are comments and 44 blank — **1,198 lines of code for 34 props**. Nor is it
a pre-narrowing artifact: `g13.017` removed only 35 lines from `button.rs`. The
vocabulary itself is the cost.

**If that rate holds at the narrowed scope, `g13.019` as written is not
viable** and the maintainer needs the number, not a completed sweep.

### The corpus is already covered by thirteen drift gates

`docs:contract-drift`, `docs:value-domain-drift`, `docs:callback-drift`,
`docs:spec-drift`, `docs:role-drift` (`drift:roles`), `drift:events`,
`drift:handlers`, `drift:recipes`, `drift:adapter-manifests`,
`docs:machine-shape-drift`, `docs:focus-ring-drift`,
`docs:container-query-drift`, `docs:react-specimen-drift`.

The verdict's own §6 named this: *"the existing drift gates already cover much
of the same ground without a compiler."* **This card has to answer it with
evidence, not assertion.**

### The tranche

Five display components, small and vocabulary-shaped:

| Component | Props | Lines |
|---|---|---|
| `Callout` | 14 | 121 |
| `EmptyState` | 8 | 66 |
| `Avatar` | 8 | 44 |
| `Pill` | 0 (rest-props) | 72 |
| `Spinner` | 0 (rest-props) | 76 |

`Pill` and `Spinner` carry no `Props` interface but do emit `data-*`
attributes — they test whether the vocabulary scope means anything for a
component whose surface is attributes rather than props.

## Fixed By Ruling (do not re-decide)

### R1 — Declare only. Do not wire any component up.

Vocabulary definitions and drift gating. **No component consumes its artifact
in this card.** The pilot measured consumption at +965 lines across nine files;
`g13.019` says components are only wired where that measures smaller, and
nothing here has been measured yet.

A component file changing is a **stop condition**.

### R2 — Number one: the honest cost per prop, at the narrowed scope.

Report authored lines per prop for the tranche, against the pilot's ~51. Count
the same way the projection did (total lines, and code lines separately).

**Do not optimise the definitions to make the number look good.** Write them
the way a real definition has to be written; if that is verbose, the number is
verbose. A flattering number produced by cutting corners would send the
programme the wrong way.

### R3 — Number two: the marginal catch over the existing thirteen gates.

For each of at least **six** real cross-runtime disagreements you plant — a
`data-*` name differing between Svelte and React, a value domain disagreeing, a
part class renamed in one runtime, an attribute dropped in one runtime, and two
of your choosing — record:

- which existing gate catches it, if any;
- whether the new vocabulary drift gating catches it;
- **which caught it first**.

Restore each plant. The deliverable is a table: **caught by both / caught only
by existing gates / caught only by the IR / caught by neither.**

"Caught only by the IR" is the column that justifies `g13.019`. If it is empty,
say so plainly — that is the most valuable result this card can produce, and it
routes straight to `g13.020`.

### R4 — Reuse the narrowed vocabulary. Invent nothing.

Parts, `data-*` names, value domains, axes, recipe hooks, capability rows —
the vocabulary `g13.017` kept and `g13.018` amended. No new IR construct. If a
tranche component needs one, **stop** and report which and why: a new construct
at component five is a much better finding than at component 165.

### R5 — Artifacts and gates.

Each definition emits and is drift-gated by `ir:check`, as the pilot's are. No
component behaviour changes, no pixel moves, no baseline refreshed.

## Scope

### In scope

- `packages/codegen/src/models/{callout,empty_state,avatar,pill,spinner}.rs`.
- Emitter registration and fixtures for the five.
- Generated artifacts for the five (emitted, **not consumed**).
- `ir:build` / `ir:check` coverage; tests.
- The two measurements and their tables.

### Out of scope — stop conditions if reached

- Wiring any component to its artifact (R1).
- Any component file, CSS, contract, or specimen.
- New IR constructs (R4).
- The other 163 components.
- `packages/contracts/headless/**` and the vectors.
- Refreshing a baseline.

## Required Tests

- `ir:build` / `ir:check` pass; `ir:check` fails on a planted byte in a new
  artifact.
- Existing artifacts (Button, RangeSlider, TextInput, shell) byte-identical.
- No file under `packages/{svelte,react}/components/src/*.svelte|*.tsx`
  changed — assert it.
- Every plant in R3 restored; the tree is clean at the end.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **The two numbers are the deliverable.** A card that lands five tidy
  definitions and no measurement has failed.
- Report an unflattering number as readily as a flattering one. The programme
  reached this point by measuring rather than hoping.
- Run `effigy ci:web` (includes `test:web-pack-install`) and `ci:rust`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
  `packages/codegen/generated/**` is writable — stale dumps fail `ir:check`.
- Commit and push with
  `git push -u origin thread/g13-052-vocabulary-coverage-first-tranche`. Do not
  merge.
- `PAPERCUTS.md` is append-only and shared: do not reflow neighbours.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/tests/**`
- `packages/codegen/fixtures/**`
- `packages/codegen/generated/**`
- `packages/{svelte,react}/components/src/generated/**`
- `packages/render/src/generated/**`
- `tasks/effigy.tasks.toml`
- `docs/logs/2026-08/<DD>-g13-052-vocabulary-coverage-first-tranche.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `git diff --check`. All green. Record artifact checksums.
2. Author the five vocabulary-only definitions.
3. Emit and drift-gate them.
4. Measure R2: authored lines per prop, total and code-only, against ~51.
5. Plant the six disagreements; record the R3 table; restore each.
6. Confirm existing artifacts are byte-identical and no component file moved.
7. State plainly whether the "caught only by the IR" column is empty.
8. Validate:
   ```sh
   effigy ir:build
   effigy ir:check
   effigy ci:rust
   effigy ci:web
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:capability-drift
   git diff --check
   ```

## Acceptance Criteria

- [ ] Five vocabulary-only definitions, emitted and drift-gated.
- [ ] **R2 number reported** — lines per prop against the pilot's ~51.
- [ ] **R3 table reported**, with the "caught only by the IR" column stated
  explicitly, empty or not.
- [ ] No component wired up; no component file changed.
- [ ] Existing artifacts byte-identical; no baseline refreshed.
- [ ] All step-8 commands exit 0.

## Stop Conditions

- A tranche component needs an IR construct that does not exist (R4).
- Declaring vocabulary requires touching a component.
- The cost per prop comes out near the pilot's ~51 — **finish the tranche,
  report it, and stop there**; continuing the sweep is the maintainer's call
  once the number exists.

Stop with exact paths, commands, and the smallest unresolved question.

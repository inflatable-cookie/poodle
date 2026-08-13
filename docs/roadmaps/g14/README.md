# g14 — Runtime-pair Authority And Cross-pair Machine Pinning

Status: planned — opens only after g13 closeout
Governing refs: `../g13/pilot-verdict-evidence.md`,
`../../architecture/006-headless-core-and-machine-model.md`,
`../../contracts/001-working-rules.md`

## Goal

Close the cross-runtime drift holes the g13 pilot proved a vocabulary IR
cannot close. One behaviour source per runtime pair — `poodle-core` for
Svelte + React, `poodle-render` for GPUI + Jetstream — is already working
rules. g14 enforces it, pins the two pairs to each other by execution rather
than description, and turns every hole into a failing gate.

The scene IR survives as the one fixture authority: specimen pages are
authored once and rendered by all four runtimes, so implementation
differences are diagnosable instead of confounded by fixture differences.

## Fixed Inputs

Recorded from the g13.008 verdict and the g13.020 reassessment. Do not
re-decide.

- **No revival of cross-language codegen.** The behavioural IR is dead; the
  expression vocabulary stays dead. Generated machine *interfaces* are the
  only generated surface.
- **The vocabulary authority is retired as a corpus mechanism** — `g13.020`
  verdict drafted from the b052 evidence (zero marginal catch over the
  gates, definitions ~9× the source they describe). The drift-gate estate
  is the corpus-wide authority. The three pilot slices are unwound (card
  `053`).
- **The scene system is kept** — the one pilot surface that replaced
  duplication instead of adding to it (four hand-written preview shells →
  one 178-line Rust source). Specimens migrate onto it (`g14.003`),
  measured tranche first, static tier before interactive.
- **Svelte is the reference implementation** (working rules, Runtime Parity
  Authority). A GPUI or Jetstream hole is a port, not an accepted delta.
- **Pinning mechanisms, and nothing else:** generated machine interfaces,
  differential machine traces, vector completeness gating, capability
  absence registry, specimen evidence gates.

## g13 Closeout (precondition)

g14 does not open until g13 is capped. Closeout sequence:

1. `g13.020` verdict signed (drafted: retire the authority, unwind the
   three pilot slices, close `019`); card `053` dispatched, merged, and
   validated.
2. Tidy pass, known items:
   - b052's five definitions: preserved on the thread branch, unmerged —
     branch deleted per `053` R3, numbers preserved in the `g13.020`
     reassessment.
   - `docs:capability-drift` declaration home rehomed out of the codegen
     models into `contracts/headless/capabilities/` (`053` R2); the gate
     survives. Spec 063's scene half is promoted at `g14.001`; the
     component half retires.
   - Watcher guards diff `main...HEAD` only; a dead worker makes them
     trivially clean. Guards must check the working tree. Ledger rows are
     written before dispatch, not after.
   - React preview dead on the lineage (PAPERCUTS 2026-08-13) — specimen
     registry mismatch; verify at closeout.
   - Jetstream `snap` overwrites captures in place; GPUI visual baselines
     stale with an inert `--control-size`/`--size` flag (PAPERCUTS).
   - Deferred HistoryCentre native-parity card: disposition recorded
     (carried into `g14.008` as a named decision point).
   - `batch-cards/README.md` and the dispatch ledger finalised.
3. Rollover per working rules: close, pause, supersede, or rehome every
   live card; refresh `roadmaps/README.md` and `generation-index.md`;
   retire stale strict-planning specs (`063` per `g14.001`); name one first
   task.

## Sequencing Rule

`001`–`002` are the runway: doctrine in architecture, baseline frozen. No
execution card runs before `001`.

`003` migrates the static specimen surface onto the scene system —
measured tranche first. `004`–`007` build the pinning stack (machine
interfaces, differential traces, vector completeness, capability
absence). `008` closes the native registration gap on the `poodle-render`
path; scene coverage extends as registrations land. `009` converts
specimen evidence into standing gates. The lanes interleave freely once
`001`/`002` land; they touch different surfaces.

`010` reassesses with the same honesty as `g13.020` and decides the
durable stack.

## Runway

1. [001 — Record the verdict and pinning doctrine in architecture](001-record-verdict-and-pinning-doctrine.md)
2. [002 — Frozen baseline and inventories](002-frozen-baseline-and-inventories.md)
3. [003 — Scene-authored specimen migration](003-scene-authored-specimen-migration.md)
4. [004 — Machine interface authority](004-machine-interface-authority.md)
5. [005 — Differential machine testing](005-differential-machine-testing.md)
6. [006 — Vector completeness and depth](006-vector-completeness-and-depth.md)
7. [007 — Capability registry with absence](007-capability-registry-with-absence.md)
8. [008 — Native registration gap closure](008-native-registration-gap-closure.md)
9. [009 — Specimen evidence gates](009-specimen-evidence-gates.md)
10. [010 — Reassess and consolidate](010-reassess-and-consolidate.md)

## Non-goals

- compiling behaviour, or any evaluator — scene included: fixtures bind
  literals and declared axes, nothing executes
- component-surface codegen (`g13.009`–`016` stay closed)
- making `poodle-node` or the IR the web authoring model
- scenes as an application framework — routing, persistence, data fetching
  stay outside
- new framework targets
- one cross-language source of truth — the pairs stay two, pinned

## First Task

`g14.001`: promote the g13 verdict's durable outcomes and the pinning
doctrine into architecture and working rules; write spec 064 (machine
pinning); promote spec 063's scene half into the scene spec; retire the
component half.

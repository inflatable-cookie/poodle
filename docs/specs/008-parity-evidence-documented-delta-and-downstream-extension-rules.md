# 008 Parity Evidence, Documented Deltas, And Downstream Extension Rules

Status: active
Updated: 2026-03-11
Depends on: `002-component-contract-template-and-parity-rules.md`, `003-accessibility-and-assistive-technology-baseline.md`, `004-overlay-focus-dismissal-and-layering-rules.md`, `005-product-composite-composition-and-information-architecture-rules.md`, `006-workstation-shell-and-panel-system-rules.md`, `007-underlay-bridge-and-wrapper-preservation-rules.md`

## Purpose

Close `g01` with explicit rules for:

- what counts as parity evidence
- how intentional deltas are documented and approved
- how downstream repos extend Poodle without bloating or forking it
- and what `g02` may assume as stable baseline

## Core Rule

Shared-surface quality is not proven by a contract file alone.

Poodle requires:

- a canonical contract
- a documented implementation target
- explicit parity evidence expectations
- and a documented extension boundary

before the next generation starts building richer depth on top.

## Parity Evidence Rule

Every shared component that claims meaningful cross-runtime adoption must carry
an evidence bundle proportionate to its maturity.

### Minimum Evidence For `implemented`

- contract exists in the canonical layer
- implementation target is identified
- token usage is explicit
- accessibility expectations are explicit
- known deltas are recorded if any exist

### Minimum Evidence For `parity-reviewed`

- Svelte implementation exists or is concretely reviewed
- GPUI implementation exists or is concretely reviewed
- Tier 1 parity checklist is completed
- keyboard, focus, and accessibility behavior is checked in both runtimes
- any deltas are explicitly approved and recorded
- evidence is linked from a log, review note, or implementation summary

## Evidence Shape Rule

Parity evidence may take multiple forms in `g01` and early `g02`.

Allowed evidence forms include:

- contract checklist completion
- implementation review notes
- targeted tests
- focused manual verification notes
- screenshots or recordings when they help explain visual or interaction parity
- log entries that summarize what was checked

The important rule is traceability, not one rigid artifact format.

## Minimum Cross-Runtime Checks

When a component is parity-reviewed, the evidence should cover:

- semantic inputs and prop meaning
- state transitions
- event meaning and timing
- token-role usage
- accessible name, role, state, and value exposure where relevant
- keyboard behavior
- focus entry, movement, and restoration
- announcement behavior where dynamic changes matter

If a component omits one of these because it is not relevant, the evidence
should say so rather than leaving the omission implicit.

## Documented Delta Rule

An intentional delta is allowed only when all of the following are true:

- the canonical contract still holds at Tier 1
- the delta is recorded in `Known Deltas`
- the reason is concrete and runtime-specific
- approval status is explicit
- follow-up is named when the delta should be revisited

Undocumented deltas are defects.

## Delta Approval Rule

The default approval states are:

- `pending` when a delta is observed but not yet accepted
- `allowed` when the delta is reviewed and compatible with the parity model
- `revisit` when temporarily tolerated but expected to be reduced later
- `rejected` when it breaks the contract and must not ship as accepted parity

Bridge wrappers and downstream adapters do not get a softer delta policy than
core implementations.

## Downstream Extension Rule

Downstream repos may extend Poodle above the canonical layer.

They may build:

- app-specific DAW widgets
- domain-specific editors
- app-specific shells
- migration wrappers
- and app-specific workflow compositions

They must do so by composing:

- canonical tokens
- canonical contracts
- bridge/adapter layers where relevant

They may not redefine Poodle's canonical meaning in place.

## Downstream Extension Contract

### Allowed Downstream Extension

- app-owned composites built above foundation, product, or workstation layers
- app-owned theme application and branding through documented token or bridge
  mechanisms
- app-owned wrappers that preserve public app APIs
- app-owned orchestration layers such as persistence backends, drag/drop
  engines, command registries, and domain workflows

### Forbidden Downstream Behavior

- forking canonical token meaning while still claiming the same token names
- redefining a Poodle contract locally without recording the divergence
- pushing app-specific DAW widgets back into Poodle core as if they were generic
  primitives
- treating bridge-local aliases as canonical token source
- relying on undocumented deltas as stable extension points

## Upstreaming Rule

A downstream pattern becomes an upstream candidate only when:

- it is demonstrably reusable across more than one app or shell context
- it can be described without app-specific nouns at its core
- it composes cleanly from the existing layers
- and it raises the shared baseline instead of smuggling app policy into Poodle

“Loophole needs it” is not sufficient by itself.

## `g02` Entry Rule

`g02` may start from the assumption that `g01` has established:

- a canonical token system with CSS, TypeScript, and Rust artifacts
- a contract template and layered contract IA
- an accessibility baseline
- overlay, product-composite, and workstation-shell rules
- an Underlay bridge baseline with zero-leak posture
- and a documented extension boundary

`g02` should not reopen those baseline questions unless:

- a contract bug is discovered
- a baseline rule is internally contradictory
- or a downstream adoption tranche exposes a real defect

## Stable `g01` Surface Summary

At `g01` close, Poodle has:

- token schema and artifact emission
- Svelte and GPUI substrate policy
- foundation primitive contracts
- product-composite contracts
- workstation-shell contracts
- Underlay bridge scaffolding and rules
- parity and delta rules sufficient to start deeper adoption work

This is enough for `g02` to focus on depth rather than bootstrap.

## Contract Author Checklist

Before a component claims parity maturity, confirm:

- the contract is complete
- evidence exists for the claimed maturity
- deltas are documented
- accessibility behavior has been explicitly checked
- downstream extension assumptions are not being mistaken for core semantics

## Seed Evidence

The first surfaces proving this rule set are:

- `docs/specs/002-component-contract-template-and-parity-rules.md`
- `docs/specs/007-underlay-bridge-and-wrapper-preservation-rules.md`
- `docs/contracts/components/README.md`
- `docs/contracts/components/README.md`
- `docs/contracts/workstation/README.md`
- `docs/logs/2026-03/11-220500-g01-013-underlay-bridge-and-token-ingestion-baseline.md`

## Next Task

Use this baseline while executing `g02.001`, especially when the form-system
depth starts turning contract intent into real implementation and parity
evidence expectations become more concrete.

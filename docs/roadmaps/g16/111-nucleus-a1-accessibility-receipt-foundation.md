# g16.111 — Nucleus A1 Accessibility Receipt Foundation

Status: ready — foundation of the A1 tranche (`g16.112`–`g16.116`)
Type: evidence infrastructure — paired accessibility receipts; no component
behaviour change
Opened: 2026-09-05
Depends on: merged `g16.062` (receipt contract), completed M1 (`g16.093`)
Governing refs: `nucleus-gpui-parity-programme.md` (A1 definition below),
`nucleus-parity-manifest.json`, `nucleus-parity-receipt.schema.json`,
`scripts/nucleus-parity-receipts.ts`, `packages/gpui/preview/src/headless_driver.rs`,
`packages/contracts/node/src/lib.rs` (accessibility record, lines ~915–955),
`../../contracts/003-native-accessibility.md`, `test/a11y/component-a11y.test.ts`
Operator decision: 2026-09-05 — A1 is the node-level accessibility projection
proven under real input and paired with the Svelte ARIA reference; A2 rides
upstream AccessKit via `g16.110`; foundation first, then five family tranches
Dispatch manifest: `../dispatch.md`

## A1, defined

For a cohort row, A1 holds when one executed receipt proves that, for the
row's M1 scenario, the accessibility semantics observable from the mounted
GPUI node tree equal the accessibility semantics observable from the mounted
Svelte DOM: role, accessible name, value and value text, states (expanded,
selected, checked/toggled, disabled, invalid, busy), relationships (controls,
labelled-by, described-by, level, orientation), and focus order under the
same input sequence. The GPUI side reads the `poodle-node` accessibility
record after real input through the M1 headless driver; the Svelte side
reads the DOM through the accessible-name algorithm and ARIA attributes.
Neither side is inferred from source.

## Goal

Build the paired accessibility receipt once, prove it on three rows with
distinct semantics, and leave five tranche cards ready to cover the other
26 rows with ordinary workers.

## Fixed Boundary

- **Snapshot shape.** One `AccessibilitySnapshot` JSON shape shared by both
  runtimes: an ordered list of nodes (document order), each with role, name,
  value, value_text, states, relationships expressed by target index (never
  by DOM id or node id string), level, orientation, and a `focus_order`
  index for focusable nodes. Decorative nodes (`aria-hidden`, no role) are
  excluded on both sides.
- **GPUI extractor.** A walker over the mounted node tree in the M1 headless
  driver that emits the snapshot from the node accessibility record and the
  backend's focus state, after the scenario's actions have run through
  production dispatch. Lives beside the M1 receipt code; no backend
  behaviour change. Missing roles or names are emitted as `null`, never
  filled in.
- **Svelte extractor.** A vitest project `test/nucleus-a11y/` that mounts
  the same component with the same props as the row's M1 scenario, replays
  the same actions through DOM events, and emits the snapshot using
  `dom-accessibility-api` for names and computed roles. Scenario props and
  actions are shared data (`test/nucleus-a11y/scenarios/<row>.json`), read
  by both extractors; the Rust side deserialises the same file so the two
  runs cannot drift.
- **Receipt.** Extend `nucleus-parity-receipt.schema.json` with
  `proof_level: "A1"` and an `accessibility` block: both snapshots' hashes,
  the diff (empty for a pass), and the scenario file hash. Receipts are
  emitted only by executed runs (the M1 rule) and validated by
  `scripts/nucleus-parity-receipts.ts`.
- **Comparison law.** Equal means equal after normalisation: relationships
  by index, names trimmed, value text compared as strings, states compared
  only where the contract declares them for that component. A documented
  web-only attribute (working rules, `WEB_ONLY_PROPS`) is excluded by name
  in the scenario file, with the reason, and appears in the receipt.
- **Ledger.** Validated A1 receipts move the row's "GPUI accessibility" cell
  from `manual` to `mounted`; the generator consumes receipts exactly as it
  does for M1. Nothing else moves.
- **Proof rows in this card:** Switch (toggle state, name), Tabs (roles
  tablist/tab/tabpanel, selected, controls, focus order across tabs), Select
  (combobox/listbox, expanded, value text, active option). A row whose
  snapshots match yields a validated receipt here. A row whose snapshots
  diverge on real semantics yields a recorded divergence (the diff, both
  snapshots, the exact attributes) in the log and the receipt store, and no
  `mounted` cell; the repair is a separate card. Two receipts plus one
  honest divergence satisfy this card (decision 2026-09-05: Select
  diverged; not pulled forward). The remaining 26 rows belong to the tranches.
- No component, contract, or backend behaviour change. Divergences found on
  the three rows are reported in the log and, if real, become bounded repair
  candidates; this card does not fix them.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Both sides are executed | a snapshot produced from source inspection | receipt fails validation without a run record |
| Scenario cannot drift | edit the Svelte actions only | Rust deserialiser rejects the mismatched scenario hash |
| Divergence bites | remove `aria-selected` from the Svelte Tabs fixture | receipt shows a non-empty diff and the run is red |
| Names are computed, not read | a name present only as a text child | Svelte extractor reports it; GPUI extractor reports the node record's label |
| Ledger moves only on validated receipts | unmanifested A1 receipt | checker reports it as review-only |
| M1 untouched | M1 receipts re-validated | identical outcome |

## Validation

`effigy regressions:native` (A1 runs join the same selector), the new
`test/nucleus-a11y` vitest project, `effigy check:parity-evidence-ledger`,
`effigy ci:web`, `effigy docs:check`, `git diff --check origin/main...HEAD`.

## Owned Paths

`test/nucleus-a11y/**` (new), `packages/gpui/preview/src/headless_driver.rs`
(extractor only) and the A1 regression tests, `scripts/nucleus-parity-receipts.ts`
and its test, `docs/roadmaps/g16/nucleus-parity-receipt.schema.json`,
`docs/roadmaps/g16/nucleus-parity-receipts/` (three A1 receipts),
`scripts/parity-evidence-ledger.ts` (A1 consumption), execution log,
`PAPERCUTS.md` (append only).

## Stop Conditions

Stop if the node accessibility record lacks a field the Svelte side has for
one of the three rows (report it as the first tranche's repair, do not add
it here), or if focus order cannot be read from the backend without a
behaviour change. Escalation owner: Chatterbox.

## Continuation

`g16.112`–`g16.116` cover NP-1 through NP-5 with ordinary workers, one PR
each, using this receipt unchanged.

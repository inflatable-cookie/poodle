# Post-g16.017 Native Lane Decision

Status: resolved — two-card Select lane closed through `g16.019`
Captured: 2026-08-28
Resolved: 2026-08-28
Source: merged `g16.017` / PR #92 and the 46 mounted / 128 missing ledger
Closeout: `g16.018` merged in PR #93; `g16.019` merged in PR #94, ledger 47 / 127

## Checkpoint

The ledger is current, but another sequence of one-component/one-cell cards
would turn the remaining programme into churn. The next lane should repair a
shared prerequisite or close a coherent family, not select the easiest missing
row merely to move the count.

The `GPUI mounted behaviour` denominator also includes static layout and
display components. A later evidence-taxonomy checkpoint should decide whether
those rows need mounted render/semantic proof or are not applicable to a
behaviour claim. Do not silently convert them now.

## Recommended Direction — Select As A Two-Card Lane

Select is the highest-leverage unresolved foundation seam. Pagination already
proved its existing toggle/change callback path, and many composites depend on
selection, query, overlay, and focus behavior. Its current state is not one
bounded implementation card:

- Svelte and React own parallel adapter state for open/query/highlight, while
  shared core supplies only option filtering and placement helpers;
- the detailed contract says freeform query becomes the value on blur or Enter
  when no option is highlighted, but both web adapters currently call
  `onValueChange` on every input event;
- Rust carries host-controlled open/query values but no query or highlight
  callback path;
- the Rust search row is display text rather than an editable input;
- option rows take separate focus instead of keeping focus on the
  combobox/trigger, and they lack production focus identity/ring treatment;
- mounted Pagination proof needs a test-only option focus ring because pointer
  hit-testing misses deferred overlay rows; and
- clear, disabled-option, grouped-option, Escape/Tab, Home/End, query, freeform,
  focus-return, and instance-identity behavior are not proved as one Select
  contract.

Recommended sequence:

1. **g16.018 — Select semantic machine and interface convergence.** Settle
   freeform commit timing; move open/query/highlight transitions into shared
   TypeScript machinery with a pure Rust mirror; align Svelte/React; replace
   the legacy Rust toggle/change/clear-only surface with explicit host-owned
   results and stable instance identity. Do not claim the mounted cell yet.
2. **g16.019 — Select mounted overlay parity.** Build the real editable search
   path, repair deferred overlay pointer targeting, keep keyboard focus on the
   trigger/input, prove the complete production interaction path, then move
   Select to mounted.

The recommended freeform rule is the existing contract rule: query changes are
reported on every edit, but selected value changes only on option selection or
an explicit freeform commit (Enter or blur with no highlighted option). This
keeps draft query separate from committed application state.

## Alternatives Kept Separate

- **NumberInput** is now technically reachable after TextInput mounted work,
  but still needs the operator-owned raw-draft/committed-number decision in
  `20260826-213343-number-input-native-value-model.md`.
- **Continuous audio controls** are a valuable family lane, especially for
  Loophole, but the Rust specs/renderers currently accept serializable visual
  state only and expose no interaction handlers. Knob/Fader/XYPad are an API
  and architecture migration, not a quick three-cell mounted batch.
- **Easy static or single-action rows** would move the ledger faster without
  unlocking the selection, editing, overlay, or audio families. Do not choose
  them solely for count velocity.
- Broad visual comparison, native accessibility, the Longhorn conformance lab,
  motion research, and Jetstream admission remain separate programme choices.

## Operator Decision

The operator approved both parts on 2026-08-28:

1. take Select as a deliberate two-card prerequisite lane; and
2. use query-per-edit / value-on-explicit-commit freeform semantics, repairing
   the current Svelte and React live-value behavior as a pre-1.0 break.

`g16.018` owns semantic-machine and interface convergence and merged in PR
#93. `g16.019` has been recompiled against that output; it owns real editable
native search, deferred-overlay interaction, focus behavior, mounted proof, and
the Select ledger-cell move.

## Disposition

The decision is fully promoted into the two roadmap cards. Keep this note as the
resolved rationale; no open triage branch remains here.

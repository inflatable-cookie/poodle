# History Papercut Ownership

Status: open — ownership is settled; Poodle and CS20 choices remain; the
Longhorn wire decision is promoted externally
Captured: 2026-08-31
Source: Loophole `PAPERCUTS.md` after PR #14 merged

## Coordination Decision

Treat the related HistoryCenter, CS20, package-type, and Longhorn-wire entries
as one portfolio coordination packet, not one implementation PR. This Poodle
Northstar thread retains dispatch, review, and merge authority for Poodle
workers promoted from the packet. Root dispatch must not create a duplicate
Poodle lane. Longhorn source work keeps its own repository authority.

Keep the existing sequence intact:

1. `g16.026` is merged.
2. `g16.027` remains the active drag card.
3. `g16.028` remains its ordered closeout.
4. `g16.033` remains the reserved Poodle follow-on and cannot dispatch until
   its public rejection-shape decision is recorded.

Every papercut worker workspace created from this packet must carry the
capitalized `Papercuts` label before launch. No papercut worker is launched
while the current drag runway is active.

## Ownership Split

### Poodle — `g16.033`

Poodle owns reusable HistoryCenter semantics and copy across Svelte, React,
shared Rust composition, and GPUI. `g16.033` may add renderer-neutral deletion
refusal categories after the operator chooses structured Poodle codes or a
host-owned message. Structured Poodle codes remain the recommendation because
they preserve one localized, cross-runtime message surface without importing
host protocol vocabulary.

Poodle source already re-exports the v3 `HistoryEntry` / `HistoryEntryPosition`
shape. `g16.033` owns packed-candidate proof for both the package root and
`@inflatable-cookie/poodle-svelte/types`; it does not own npm publication or a
Loophole dependency update.

### Loophole/Pulse — CS20 `groupId`

Poodle already accepts and renders optional `HistoryEntry.groupId` in every
active runtime. Longhorn already carries `HistoryGroupId` through entry
metadata and its protocol projection. Loophole's `pulse-history` currently
creates the one CS20-coalesced node with
`HistoryEntryMetadata::new(label, None, None)`, and its renderer bridge passes
the resulting `groupId` through unchanged.

The missing value is therefore a Loophole/Pulse recording-policy choice, not a
Poodle component gap or a Longhorn protocol gap. Decide whether one coalesced
node is intentionally label-only or receives a stable `HistoryGroupId`. If an
identity is wanted, implementation belongs in Loophole/Pulse and must prove its
lifecycle and stability; it must not add a second Poodle field or fork the
Longhorn wire shape.

### Longhorn — `AlreadyAtTarget` wire code

Longhorn's domain error has `ForkNavigationError::AlreadyAtTarget`, but
`ForkNavigationRejectionCode` has no matching stable variant. Loophole maps the
case to `invalidRequest`, copies the diagnostic detail, then its renderer
sniffs “already at” to recover Poodle's `AlreadyAtTarget` meaning.

Northstar has now approved and queued the exact wire code as Longhorn papercuts
wave 27. Its canonical dispatch artifact is Longhorn
`docs/handoffs/20260831-203639-papercuts-wave27-already-at-target.md`. The lane
is source-only and remains separate from `g16.033`; Poodle can preserve its
existing semantic code without waiting for the protocol change. Poodle must
not open a duplicate card, worker, or implementation diff for it. A bounded
Loophole adapter cleanup may follow after the Longhorn source change lands.

### Loophole adoption

After the relevant upstream surfaces land, Loophole owns mapping its deletion
failures onto the selected Poodle semantics, consuming a published corrected
Poodle package, and removing detail sniffing if Longhorn gains an exact wire
code. These are adoption changes, not reasons to combine repositories in one
worker.

Keyboard vertical geometry remains design-deferred and outside this packet.

## Open Decisions

- Poodle `g16.033`: structured Poodle rejection codes or host-owned message?
- CS20: label-only presentation or a stable group identity on the one
  coalesced node?

## Promotion Route

1. Finish `g16.027` and `g16.028` without inserting a papercut worker.
2. Resolve the Poodle rejection-shape gate, then dispatch `g16.033` from this
   thread with the required `Papercuts` label.
3. Promote the CS20 decision into Loophole planning only if group identity is
   wanted.
4. Treat Longhorn wave 27 as the sole source lane for the exact wire code; do
   not duplicate it in Poodle.
5. Run publication and Loophole adoption only under separate release/adoption
   authority.

Remove this note once each open choice has either been rejected or promoted
into its owning repository's canonical plan.

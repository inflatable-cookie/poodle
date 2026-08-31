# History Papercut Ownership

Status: open — external wire/adoption work is complete; Poodle rejection,
package publication, CS20, and keyboard-geometry follow-ons remain
Captured: 2026-08-31
Source: Loophole `PAPERCUTS.md` after PR #14 merged

## Coordination Decision

Treat the related HistoryCenter, CS20, package-type, and Longhorn-wire entries
as one portfolio coordination packet, not one implementation PR. This Poodle
Northstar thread retains dispatch, review, and merge authority for Poodle
workers promoted from the packet. Root dispatch must not create a duplicate
Poodle lane. Longhorn source work keeps its own repository authority.

Keep the existing sequence intact:

1. `g16.026` and `g16.027` are merged.
2. `g16.028` remains the ordered drag closeout.
3. `g16.033` remains the reserved Poodle follow-on and cannot dispatch until
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
`@inflatable-cookie/poodle-svelte/types`; it does not mutate npm. This thread
owns the later publication/adoption follow-on, but only after the packed proof
lands and the operator explicitly authorizes release mutation.

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

Longhorn papercuts wave 27 added the exact `alreadyAtTarget` wire rejection and
merged in PR #20 as `c216fd79012d77b8da00b67c88de80a5f26d9794`. Its canonical
dispatch artifact remains Longhorn
`docs/handoffs/20260831-203639-papercuts-wave27-already-at-target.md`.

This source-only lane is complete and remains separate from `g16.033`.
Poodle must not open a duplicate card, worker, or implementation diff for it.

### Loophole adoption

Loophole PR #16 adopted Longhorn's `alreadyAtTarget` wire code and merged as
`8699c76fa06190bf2cef01d822a6834dd4a8067d`; the diagnostic-detail sniff is no
longer Poodle work. Loophole still owns mapping deletion failures onto the
future selected Poodle semantics and consuming a published corrected Poodle
package. Those remain adoption changes, not reasons to combine repositories in
one worker.

Keyboard vertical geometry remains design-deferred. Ownership stays with this
thread, but it is outside `g16.033` and receives no worker until its design is
settled.

## Open Decisions

- Poodle `g16.033`: structured Poodle rejection codes or host-owned message?
- CS20: label-only presentation or a stable group identity on the one
  coalesced node?
- Keyboard: what vertical geometry should the component expose?

## Promotion Route

1. Finish `g16.028` without inserting a papercut worker.
2. Resolve the Poodle rejection-shape gate, then dispatch `g16.033` from this
   thread with the required `Papercuts` label.
3. After `g16.033` proves the packed v3 type surface, seek explicit release
   authority for publication and the remaining Loophole package adoption.
4. Promote the CS20 decision into Loophole planning only if group identity is
   wanted.
5. Keep keyboard vertical geometry design-deferred until the operator settles
   its geometry; do not fold it into HistoryCenter or the drag closeout.

Longhorn PR #20 and Loophole PR #16 close the wire-code branch of this packet.
No Poodle promotion or worker follows from those receipts.

Remove this note once each open choice has either been rejected or promoted
into its owning repository's canonical plan.

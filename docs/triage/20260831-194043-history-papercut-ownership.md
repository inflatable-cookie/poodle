# History Papercut Ownership

Status: open — external wire/adoption work is complete; the Poodle rejection
surface is implemented on `papercuts/g16-033-history-center-rejection-surface`
and awaiting review/merge; package publication/adoption, CS20, and
keyboard-geometry follow-ons remain
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
2. `g16.028` is accepted and merged in PR #118.
3. `g16.033` is implemented on its worker branch with the promoted five-code
   rejection surface and the installed-tarball v3 type proof; review and merge
   remain with this thread.

Every papercut worker workspace created from this packet must carry the
capitalized `Papercuts` label before launch. The drag runway is closed;
`g16.033` was the next bounded papercut worker and has been dispatched and
implemented.

## Ownership Split

### Poodle — `g16.033`

Poodle owns reusable HistoryCenter semantics and copy across Svelte, React,
shared Rust composition, and GPUI. The operator chose structured Poodle codes
with exact component-owned copy: preserve `AlreadyAtTarget` and `UnknownEntry`,
then add `StaleHistory`, `ProtectedEntry`, and `DeletionUnavailable`.
Current-line and pinned/protected refusal share `ProtectedEntry`; no host copy
override or general localization API enters `g16.033`.

Poodle source already re-exports the v3 `HistoryEntry` / `HistoryEntryPosition`
shape. `g16.033` owns packed-candidate proof for both the package root and
`@inflatable-cookie/poodle-svelte/types`; it does not mutate npm. This thread
owns the later publication/adoption follow-on, but only after the packed proof
lands and the operator explicitly authorizes release mutation.

The five codes and the packed proof are implemented: `effigy
test:svelte-pack-install` now compiles a real installed-tarball consumer on
both public Svelte import paths and requires one unsuppressed `branchCount`
failure per path. npm `latest` is still `0.2.2`; nothing in `g16.033` moves a
version, tag, publication, or Loophole pin. Loophole's mapping of deletion
failures onto `StaleHistory` / `ProtectedEntry` / `DeletionUnavailable` now has
a settled Poodle target to adopt once a published package exists.

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

- CS20: label-only presentation or a stable group identity on the one
  coalesced node?
- Keyboard: what vertical geometry should the component expose?

## Promotion Route

1. `g16.028` closed without an inserted papercut worker.
2. `g16.033` was dispatched from this thread with the required `Papercuts`
   label and is implemented on its worker branch.
3. `g16.033` proves the packed v3 type surface; the next step is explicit
   release authority for publication and the remaining Loophole package
   adoption.
4. Promote the CS20 decision into Loophole planning only if group identity is
   wanted.
5. Keep keyboard vertical geometry design-deferred until the operator settles
   its geometry; do not fold it into HistoryCenter or the drag closeout.

Longhorn PR #20 and Loophole PR #16 close the wire-code branch of this packet.
No Poodle promotion or worker follows from those receipts.

Remove this note once each open choice has either been rejected or promoted
into its owning repository's canonical plan.

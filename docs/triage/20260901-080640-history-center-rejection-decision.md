# HistoryCenter Rejection Decision

Status: operator-confirmed — awaiting orchestrator review and canonical promotion
Captured: 2026-09-01
Planning lane: `g16.033`
Authority: Poodle Northstar orchestrator retains promotion, readiness, dispatch,
review, and merge authority

## Decision

Use structured Poodle semantic codes. Hosts map their protocol failures onto
the codes; Poodle resolves every code to exact component-owned copy in each
renderer. Do not accept a host message, message override, or protocol detail.

Keep the existing meanings and copy source-compatible:

| Code | Renderer-neutral meaning | Exact Poodle copy |
| --- | --- | --- |
| `AlreadyAtTarget` | The requested navigation would not change the current target. | `Already at the requested target` |
| `UnknownEntry` | The referenced history entry does not exist. | `Entry does not exist` |

Add three deletion-refusal meanings:

| Code | Renderer-neutral meaning | Exact Poodle copy |
| --- | --- | --- |
| `StaleHistory` | The deletion was evaluated against history that changed before the operation completed. | `History changed; this entry was not deleted` |
| `ProtectedEntry` | The entry is current-line, pinned, or otherwise protected by entry-level policy. These cases are deliberately one meaning. | `This history entry is protected` |
| `DeletionUnavailable` | Deletion is unavailable at the capability or operation level, rather than refused because of this entry's identity or protection. | `History deletion is unavailable` |

The five meanings remain distinct. In particular, stale history, a protected
entry, and unavailable deletion must never fall back to `UnknownEntry`.
`AlreadyAtTarget` remains the existing navigation rejection; this decision
does not repurpose it for deletion.

## Copy And Localization Boundary

Poodle owns the exact English copy through exhaustive code-to-message
resolvers in TypeScript and Rust. Svelte, React, shared Rust composition, and
GPUI consume that resolved Poodle copy. The lookup is one centralized,
closed semantic seam per language; a missing category must fail compilation or
focused proof rather than fall through to a generic message.

`g16.033` does not add a locale input, locale provider, message catalogue
override, per-instance message prop, or host escape hatch. Poodle has no
general localization runtime today. A future general localization contract may
change how Poodle resolves its own codes, but that is a separate decision and
must not move copy ownership into hosts.

## Existing Behavior Preserved

The rejection prop remains a transient semantic input:

- a new non-null code replaces the displayed rejection;
- the same current code is idempotent;
- dismissal stays component-local;
- `null` clears the notice and resets the prop boundary;
- the notice stays a polite `status` live region; and
- open, closed, disabled, and duplicate-input behavior remains deterministic.

This packet changes the accepted vocabulary and copy only. It does not reopen
HistoryCenter navigation, data, deletion-command, or dismissal ownership.

## Packed `HistoryEntry` Proof

Poodle source already re-exports the core v3 `HistoryEntry` and
`HistoryEntryPosition` from both the Svelte package root and `./types`. The
later worker must prove the installed tarball boundary rather than repeat the
source edit.

Extend the existing `effigy test:svelte-pack-install` consumer proof with all
of the following:

1. Pack and install the Svelte candidate tarball in the disposable consumer.
   Preserve the existing proof that resolution does not reach sibling source
   or a workspace alias.
2. Run a Svelte-aware positive typecheck that imports aliased `HistoryEntry`
   types from both `@inflatable-cookie/poodle-svelte` and
   `@inflatable-cookie/poodle-svelte/types`. Each value must use the required
   v3 `continuationCount` field.
3. Run one unsuppressed negative fixture per import path. Each otherwise-valid
   v3 value also supplies removed v2 `branchCount`; the harness must require a
   non-zero checker exit and a diagnostic naming `branchCount`. A negative
   fixture that typechecks is a test failure.
4. Retain the installed package realpath, tarball hash, manifest/export
   inspection, and candidate version in the generated evidence.

This is packed-candidate evidence only. It does not prove npm publication,
move a dist-tag, certify the registry copy, or move Loophole's package pin.
The source manifest currently names candidate `0.2.3`; project front doors
still identify `0.2.2` as the latest published package.

## Evidence

- `docs/contracts/components/history-center.md` already makes HistoryCenter
  authority-agnostic, accepts a structured rejection code, assigns display
  copy to the component, and keeps the notice transient and dismissible.
- `packages/core/src/history-center.ts` defines the closed
  `HistoryCenterRejectionCode` union, the exhaustive message resolver, and the
  `SHOW_REJECTION` transition.
- `packages/contracts/components/src/history_center.rs` mirrors the boundary
  with `HistoryCenterRejection` and `HistoryCenterSpec::rejection_message`.
- `packages/svelte/components/src/HistoryCenter.svelte` and
  `packages/react/components/src/HistoryCenter.tsx` accept the semantic code
  and preserve component-local replacement and dismissal behavior.
- `packages/gpui/preview/src/specimens/history_center_specimen.rs` resolves the
  Rust rejection through the Poodle spec before rendering it.
- `packages/svelte/components/src/types.ts` re-exports the live core
  `HistoryEntry` / `HistoryEntryPosition`; `src/index.ts` re-exports those
  types from the package root; `package.json` exposes `./types`.
- `test/package-install/web-preview.ts` already packs tarballs, installs them
  without sibling-source resolution, verifies export reachability, and records
  hashes. It dynamically imports the Svelte `./types` subpath but does not yet
  typecheck `HistoryEntry` or falsify `branchCount`.
- The focused Svelte and React package-type tests import local source and prove
  a positive `continuationCount` value only. They are useful source evidence,
  not packed-boundary proof.
- `docs/triage/20260831-194043-history-papercut-ownership.md` assigns reusable
  HistoryCenter semantics and copy to Poodle while keeping protocol mapping
  and consumer adoption in their owning hosts.

No external research is required. The live repository shapes already expose
one renderer-neutral seam that can carry the confirmed categories without a
Longhorn or Loophole dependency.

## Alternatives Rejected

### Host-owned message

Rejected. It would replace the existing closed semantic boundary with
host-authored operator copy, duplicate wording and localization responsibility
across hosts, and weaken cross-runtime parity.

### Separate current-line and pinned/protected codes

Rejected. Both are entry-level protection policy and intentionally resolve to
one operator meaning. Poodle does not need the host's policy subtype to render
an honest refusal.

### Broader merged deletion failure

Rejected. Merging stale history, entry protection, capability unavailability,
or absence recreates the ambiguity `g16.033` exists to remove.

### Host copy or catalogue override in `g16.033`

Rejected. It would weaken component ownership or expand the card into a
general localization API without a governing Poodle localization contract.

### Source-only or declaration-only package proof

Rejected. It cannot prove how both public imports resolve from the installed
tarball, and a positive-only fixture cannot falsify a lingering v2-compatible
shape.

## Proposed Promotion Map

The orchestrator owns promotion after this planning PR is accepted and merged:

| Destination | Promote or retain |
| --- | --- |
| Poodle HistoryCenter contract | The structured-code choice, five exact meanings and messages, copy/localization boundary, and preserved rejection lifecycle. |
| `g16.033` card | The accepted vocabulary, installed-tarball positive/negative proof, source-versus-publication distinction, and existing stop conditions. |
| Poodle release follow-on | Package publication only after `g16.033` lands and the operator separately authorizes release mutation. |
| Loophole planning | Mapping deletion failures onto the published Poodle codes and moving its package pin after publication. |
| Existing ownership note | Keep CS20 `groupId` as an open Loophole/Pulse recording-policy choice and keyboard vertical geometry design-deferred. Longhorn PR #20 and Loophole PR #16 remain closed receipts, not new Poodle work. |

The orchestrator should remove this decision packet when all settled meaning
has been promoted into canonical surfaces. Do not remove the existing
ownership note until its remaining CS20, publication/adoption, and keyboard
branches have their own promoted or rejected dispositions.

## Unresolved Questions And External Gates

No HistoryCenter rejection-shape question remains open in this packet.
Implementation still cannot dispatch from this delegate branch:

- the orchestrator must review and merge the planning PR, then promote the
  decision against current `main`;
- `g16.028` must be accepted and merged before the later serial `g16.033`
  worker launches;
- the eventual worker workspace must carry the capitalized `Papercuts` label;
  and
- publication, registry certification, Loophole mapping/pin adoption, CS20
  identity, and keyboard geometry remain separate authority gates.

## Non-Goals

- product implementation or public API edits in this planning branch;
- Longhorn or Loophole wire vocabulary in Poodle;
- changes to `AlreadyAtTarget` or `UnknownEntry` names or copy;
- a general localization system;
- drag-and-drop, keyboard geometry, CS20 recording policy, release, tags,
  publication, consumer pins, or Jetstream admission;
- canonical promotion, readiness, worker launch, PR merge, or Loophole
  adoption by this delegate.

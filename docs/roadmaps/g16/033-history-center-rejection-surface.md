# g16.033 — HistoryCenter Rejection Surface

Status: reserved — do not dispatch; follows `g16.028` and requires a public
API decision
Opened: 2026-08-31
Depends on: accepted and merged `g16.026` only for shared-file serialization;
there is no drag-and-drop semantic dependency. Dispatch stays behind the
ordered `g16.027` → `g16.028` runway.
Governing refs: `../../contracts/components/history-center.md`,
`../../contracts/001-working-rules.md`,
`../../architecture/006-headless-core-and-machine-model.md`
Source evidence: Loophole `PAPERCUTS.md` entries “HistoryCentre delete
rejections collapse to UnknownEntry” and “Poodle-svelte types.ts still exports
v2 HistoryEntry”, reassessed after Loophole PR #14 merged. Portfolio ownership
is recorded in `../../triage/20260831-194043-history-papercut-ownership.md`.

## Goal

Stop HistoryCenter from presenting every refused continuation deletion as
“Entry does not exist”. Give hosts a Poodle-owned, renderer-neutral way to
distinguish stale history, protected entries, and unavailable deletion from a
genuinely unknown entry while keeping protocol vocabulary outside Poodle.

Preserve the already-landed `HistoryEntry` correction. Poodle `main` re-exports
the core v3 shape from `@inflatable-cookie/poodle-svelte`, but the latest
published package remains `0.2.2`. This card proves the packed source surface;
it does not repeat the source edit or claim that a consumer pin has moved.

## Decision Gate

Choose the public rejection shape before dispatch:

1. **Structured Poodle codes — recommended.** Extend the existing semantic
   union/enum with generic categories for stale history, protected entry, and
   unavailable deletion. Poodle retains the copy in every renderer; hosts map
   their own protocols onto the categories.
2. **Host-owned message.** Accept a message-bearing rejection value or a
   separate message prop. This is more flexible, but moves operator copy,
   localization, and cross-runtime consistency into every host.

Do not infer exact code names or messages in implementation. Record the
operator's choice here and in the HistoryCenter contract first. The choice
must keep `AlreadyAtTarget` and `UnknownEntry` source-compatible unless the
operator explicitly authorizes a wider clean break.

## Reserved Scope

- Update the HistoryCenter contract before implementation.
- Align the framework-free TypeScript rejection code/message resolver, Svelte
  and React props/tests, Rust `HistoryCenterRejection` and
  `HistoryCenterSpec::rejection_message`, shared rendering, and the GPUI
  specimen/evidence.
- Keep messages component-owned when structured codes are chosen. Do not
  import Longhorn types or copy Longhorn/Loophole wire strings into Poodle.
- Add focused proof for every accepted category, including replacement and
  dismissal of one rejection by another.
- Extend the packed Svelte-package consumer proof so both the package root and
  `@inflatable-cookie/poodle-svelte/types` accept v3 `HistoryEntry` with
  `continuationCount` and reject the removed v2 `branchCount` shape.
- Record the source-versus-publication distinction in closeout evidence. A
  later release/adoption lane owns npm publication and Loophole pin movement.

## Acceptance Criteria

- A stale deletion does not display the unknown-entry message.
- A current-line or pinned/protected deletion refusal does not display the
  unknown-entry message.
- A capability/unavailable deletion refusal does not display the unknown-entry
  message.
- A genuinely absent entry still displays “Entry does not exist”; navigation's
  existing already-at-target behavior remains unchanged.
- Svelte, React, shared Rust composition, and GPUI resolve the same semantic
  category to the same Poodle-owned copy.
- Rejection prop replacement, duplicate input, dismissal, open/closed state,
  disabled state, and live-region behavior remain deterministic.
- Poodle imports no Longhorn or Loophole package and contains no protocol
  detail-sniffing bridge.
- The Svelte source and packed package expose only the live core
  `HistoryEntry` / `HistoryEntryPosition` shape; no duplicate v2 interface or
  compatibility alias is added.
- No parity-ledger cell moves. This is a public semantic correction and
  regression-proof lane, not a new mounted-evidence claim.

## Review Oracle

| Invariant | Smallest adversarial counterexample | Required proof |
| --- | --- | --- |
| Distinct refusal meanings remain distinct | stale revision, protected entry, and unavailable deletion all map to one fallback | each accepted category resolves to its own exact Poodle message in TypeScript and Rust, then mounts in both web shells and GPUI |
| Hosts do not own Poodle copy | a renderer passes through “stale revision” or another protocol detail | public input is the approved Poodle semantic shape; absence search finds no Longhorn/Loophole vocabulary or dependency |
| Existing navigation semantics survive | adding deletion categories changes `AlreadyAtTarget` or `UnknownEntry` copy | exact existing cases remain green in TypeScript, Svelte, React, Rust, and the native specimen |
| Package type proof is not source-only | local tests import `../src/types` while the packed export still exposes v2 `branchCount` | a disposable consumer typechecks the packed root and `./types` exports with `continuationCount`; a negative `branchCount` fixture must fail |
| The lane does not claim publication | source package is correct while npm `latest` is still older | closeout names the packed candidate evidence and leaves version, tag, publication, and Loophole adoption to a separate authorized lane |

## Writable Scope

- `docs/contracts/components/history-center.md`;
- `packages/core/src/history-center.ts`, its export and focused tests;
- paired Svelte/React HistoryCenter props, focused tests, and curated specimen
  rows only as needed to teach the accepted categories;
- `packages/contracts/components/src/history_center.rs` and its crate export;
- `packages/render/src/history_center.rs` only if the accepted shape requires a
  renderer change;
- exact GPUI HistoryCenter specimen and headless regression files;
- the existing Svelte packed-package install fixture;
- this card, g16/front-door currentness, one August log, and `PAPERCUTS.md`
  only for new execution friction.

Do not edit drag-and-drop behavior, Keyboard geometry, Longhorn, Loophole,
package versions, release notes, tags, publication workflows, consumer pins,
Jetstream admission, or unrelated HistoryCenter navigation/data behavior.

## Validation

Use Effigy selectors discovered after worker startup. At minimum:

- focused core HistoryCenter tests;
- focused Svelte and React HistoryCenter tests;
- focused `poodle-specs` and `poodle-render` HistoryCenter tests;
- the named headless GPUI HistoryCenter regression/specimen probe;
- `effigy test:svelte-pack-install` with positive v3 and negative v2 type
  fixtures;
- contract, callback, value-domain, and capability drift checks relevant to
  HistoryCenter;
- `effigy test:core`, `effigy test:components`, `effigy test:contracts`,
  `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, and
  `effigy docs:check`;
- one final headless `effigy qa`; and
- `git diff --check origin/main...HEAD` plus exact searches for duplicate v2
  `HistoryEntry` and forbidden sibling vocabulary.

Everything stays headless. Never run `*-windowed`, release, tag, publication,
workflow mutation, or sibling-repository commands.

## Stop Conditions

- The operator has not chosen structured codes versus host-owned message.
- The accepted categories or copy cannot remain renderer-neutral.
- Correctness requires a Longhorn wire/API change, including the separate
  `AlreadyAtTarget` wire-code decision.
- The packed type proof shows a release-build problem wider than the existing
  HistoryEntry export.
- Work expands into Keyboard vertical geometry, HistoryCenter data/navigation
  semantics, release certification, or Loophole adoption.
- More than HistoryCenter semantic evidence would move.

## Continuation

After `g16.028` closes, resolve the decision gate and only then prepare the
worker handoff. The worker workspace must carry the capitalized `Papercuts`
label before launch. This Poodle thread owns implementation review and merge.
A later explicitly authorized release/adoption lane may publish the corrected
package and close Loophole's pin-lag entry. The ordered `g16.027` → `g16.028`
drag-and-drop runway is not displaced by this reservation.

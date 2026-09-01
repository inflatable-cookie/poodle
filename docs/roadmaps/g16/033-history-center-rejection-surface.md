# g16.033 — HistoryCenter Rejection Surface

Status: implemented — PR #120 on
`papercuts/g16-033-history-center-rejection-surface`, awaiting orchestrator
review and merge; `g16.028` accepted and merged in PR #118 as `17a25d633`
Opened: 2026-08-31
Depends on: accepted and merged `g16.028` for runway sequencing — satisfied by
PR #118 / `17a25d633`. There is no drag-and-drop semantic dependency.
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

## Promoted Decision

Use the existing structured Poodle rejection seam with component-owned copy.
Preserve `AlreadyAtTarget` → `"Already at the requested target"` and
`UnknownEntry` → `"Entry does not exist"`. Add exactly:

- `StaleHistory` → `"History changed; this entry was not deleted"`;
- `ProtectedEntry` → `"This history entry is protected"`; and
- `DeletionUnavailable` → `"History deletion is unavailable"`.

Current-line and pinned/protected refusal share `ProtectedEntry`. Hosts map
their protocols onto the five Poodle meanings; no host message, message
override, locale input, or general localization API enters this card.

## Implementation Scope

- Implement the promoted HistoryCenter contract across the framework-free
  TypeScript rejection code/message resolver, Svelte
  and React props/tests, Rust `HistoryCenterRejection` and
  `HistoryCenterSpec::rejection_message`, shared rendering, and the GPUI
  specimen/evidence.
- Keep messages component-owned. Do not
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
| Package type proof is not source-only | local tests import `../src/types` while the packed export still exposes v2 `branchCount` | an installed-tarball consumer typechecks the packed root and `./types` exports with `continuationCount`; one unsuppressed negative `branchCount` fixture per import path must fail with a named diagnostic |
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
- this card, g16/front-door currentness, one September log, and `PAPERCUTS.md`
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

- The promoted five-code vocabulary or exact component-owned copy cannot stay
  exhaustive and renderer-neutral.
- Correctness requires a Longhorn wire/API change, including the separate
  `AlreadyAtTarget` wire-code decision.
- The packed type proof shows a release-build problem wider than the existing
  HistoryEntry export.
- Work expands into Keyboard vertical geometry, HistoryCenter data/navigation
  semantics, release certification, or Loophole adoption.
- More than HistoryCenter semantic evidence would move.

## Evidence

Changed surfaces and their proofs:

| Surface | Change | Proof |
| --- | --- | --- |
| `packages/core/src/history-center.ts` | five-code `HistoryCenterRejectionCode`, exhaustive `historyCenterRejectionMessage` | `packages/core/test/history-center.test.ts` — copy table, distinctness, display/replacement/idempotence |
| `packages/contracts/headless/src/history_center.rs` | five-code `HistoryCenterRejectionCode`, exhaustive `history_center_rejection_message` | crate tests `every_rejection_code_owns_its_own_exact_copy`, `show_rejection_displays_replaces_and_repeats_inertly` |
| `packages/contracts/components/src/history_center.rs` | five-variant `HistoryCenterRejection`, exhaustive `rejection_message` | crate tests `rejection_copy_is_component_owned`, `every_refusal_meaning_stays_distinct` |
| `packages/render/src/history_center.rs` | unchanged renderer; new shared-tree proof | `every_rejection_code_renders_its_own_copy_in_the_live_region`, `a_surface_without_a_rejection_paints_no_notice` |
| `packages/gpui/preview/tests/headless_regressions.rs` | mounted native proof | `every_history_center_rejection_mounts_its_own_native_copy` (`effigy regressions:native`) |
| Svelte / React `HistoryCenter` | no source change — the code type flows from core | `mounts every accepted refusal as its own line…`, `replaces one refusal with the next and clears on null` in both shells |
| `test/package-install/fixture/packed-types/`, `test/package-install/web-preview.ts` | installed-tarball v3 `HistoryEntry` type proof | `effigy test:svelte-pack-install` — positive compile on both import paths, one unsuppressed `branchCount` failure per path |

The packed proof covers the **source candidate only**. npm `latest` remains
`0.2.2`; no version, tag, release note, publication, or Loophole pin moved.

## Continuation

The post-merge readiness review classified the lane `strict-ready` and planning
`coherent`: scope, authority, exact API, oracle, validation, and stop conditions
are settled. The worker implemented it on
`papercuts/g16-033-history-center-rejection-surface`; this Poodle thread owns
implementation review and merge.
A later explicitly authorized release/adoption lane may publish the corrected
package and close Loophole's pin-lag entry. The ordered `g16.028`
drag-and-drop closeout merged before this lane opened.

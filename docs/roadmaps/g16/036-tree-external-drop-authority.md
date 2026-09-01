# g16.036 — Tree External Drop Authority

Status: complete — merged in PR #127
Closed: 2026-09-01
Merge: `f5663085aed62abd3d347931a7e7560465bd95ae`
Opened: 2026-09-01
Depends on: merged `g16.028`, `g16.024`, and PR #125 at
`a980cb7748fdf9751dd4ca64b02903111a44d59f`; independent of `g16.034`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/tree.md`,
`../../architecture/011-drag-and-drop-substrate.md`,
`../../specs/069-dependable-drag-and-drop-substrate.md`
Consumer evidence:
`/Users/tom/Dev/projects/figmatic/docs/roadmaps/g01/batch-cards/016-22-poodle-tree-drag-adoption.md`,
`/Users/tom/Dev/projects/figmatic/docs/triage/2026-09-01-poodle-tree-authoritative-drop-integration.md`

## Goal

Add the smallest generic paired Svelte/React Tree seam for a host that owns
placement legality. The host projects one ordered moving set, inspects the
Tree destination resolved by PR #125, accepts, rewrites, or refuses it before
the accepted indicator is painted, and returns the existing substrate's real
sync/async commit result.

Keep `onReorder(from, to, position)` unchanged for ordinary self-contained
trees. Reuse the current Tree source, targets, outline geometry, pointer and
keyboard sensors, focus, auto-scroll, revalidation, and terminal machine. Add
no application vocabulary and no second drag lifecycle.
PR #125 remains the interaction source. Do not import a consumer-owned dwell
controller or make drag-hover expansion a prerequisite for this seam.

## Fixed Public Shape

Shared core owns and both component packages re-export:

- `TreeReorderSubject { sourceValue, movingValues }`;
- `TreeReorderCandidate { subject, intent }`; and
- `TreeReorderAuthority { projectMovingValues, canDrop, onDrop }`.

Tree adds `reorderAuthority?: TreeReorderAuthority | null`.
`projectMovingValues` is pure and returns the ordered moving values; `canDrop`
returns the existing `DropEligibility` synchronously; `onDrop` returns
`DragDropCommitResult | Promise<DragDropCommitResult>`. The public prop type
makes authority mode mutually exclusive with `onReorder`; `reorderable` stays
the explicit switch and is not implied by an authority.

The exact signatures and validity rules are fixed in the Tree contract and
spec 069. Do not rename, split, or widen them during implementation.

## Behavior Envelope

- Project once at session start and latch `{ sourceValue, movingValues }` for
  pointer and one-shot keyboard sessions. Selection changes do not alter it.
- Refuse empty, duplicate, source-omitting, or non-live projections. Never
  normalize them to `[sourceValue]`.
- Run current generic Tree safety and the external `canDrop` during hover and
  again at release. The external callback sees semantics, never coordinates.
- A host rewrite is limited to a current enabled Tree node and
  `before`/`after`/`inside`. Validate it against every moving value.
- An accepted policy result preserves the hovered target, indicator edge, and
  operation; it may rewrite only `intent.destination`. The accepted rewritten
  destination determines indicator depth, announcement, revalidation, and the
  candidate passed to `onDrop`; the hovered row remains the physical indicator
  anchor.
- Return the host commit result unchanged. Pending completion stays in
  `dropping`; stale completion is inert through the current controller.
- Clear the latched Tree subject on every terminal and provider/component
  teardown. A later session projects a fresh set.
- Removing `reorderAuthority` during an authority-owned session refuses it. It
  does not fall through to `onReorder`.
- With no authority installed, current `onReorder` pointer and Alt+↑/↓
  behavior is byte-for-byte API compatible.
- Rust/GPUI remains on its current synchronous single-row `on_reorder` path.
  Do not encode moving values into `DragSubject.id`, add Tree-local durable
  session state, or widen the Node substrate in this card.

## Ordered Work

1. Add the three shared-core types, public exports, validity helpers, and pure
   mapping between Tree destinations and substrate intents.
2. Thread one authority-owned session mode and latched subject through Svelte
   Tree, TreeItem, and logical keyboard targets. Keep the convenience path.
3. Mirror the same behavior and types in React. Avoid framework-specific
   policy or result shapes.
4. Make accepted rewritten destination depth derive from the accepted intent,
   not a second raw pointer geometry pass.
5. Add focused core, paired component, and mounted browser evidence for
   projection, refusal, rewrite, revalidation, async terminal, and unchanged
   convenience behavior.
6. Update packed type/API evidence, contract drift inputs, this card, and one
   September execution log. Open one PR; do not change Figmatic.

## Acceptance

- One selected source projects at least two ordered moving values and every
  hover/revalidation/commit observation sees that exact latched set.
- A generic Tree-safe candidate withheld by the authority paints no accepted
  line and invokes neither `onDrop` nor `onReorder`.
- Rewriting a candidate changes the painted line depth, announcement, and
  `dropCommitDestination(candidate.intent)` together. No original destination
  leaks into commit.
- Withholding the rewritten destination between hover and release rejects the
  drop before `onDrop`.
- A Promise result keeps the session dropping and reaches exactly one matching
  terminal. Its late completion cannot settle a later session.
- Pointer and Alt+↑/↓ exercise the same authority path.
- Ordinary trees without `reorderAuthority` retain the existing single-row
  `onReorder` callback and PR #125 outline behavior.
- Svelte and React expose the same types and behavior from installed package
  artifacts. No Figmatic term, record, target enum, revision, or DOM detail
  enters Poodle.

## Review Oracle

| Invariant | Smallest adversarial counterexample | Required proof |
| --- | --- | --- |
| Subject is a session snapshot | start with two selected rows, change selection after pickup, then start a second session | both shells keep the first pair through commit and project the new set for session two |
| Authority precedes accepted presentation | generic Tree eligibility accepts a target the host refuses | mounted indicator remains refused/absent and neither commit route fires |
| Rewrite has one semantic owner | hover a root gap and rewrite it to an admitted child gap | indicator depth, announcement, and committed destination all name the child gap |
| Release uses live authority | accept on hover, remove the exact destination, then release | revalidation rejects; `onDrop` and `onReorder` stay silent |
| Terminal result is not invented | return pending, then rejected/failed; resolve an older Promise after a new session | dropping posture persists and only the matching result terminates its session |
| Convenience remains intact | omit authority and use pointer plus Alt+↑/↓ | existing `onReorder(from,to,position)` fires once with PR #125 geometry |
| Invalid projection is not normalized | return empty, duplicates, a missing value, or a set without the source | no accepted target or commit; no hidden `[source]` fallback |
| Runtime boundary is honest | attempt to add native moving-set encoding or claim async Node commit | diff contains no Node/Rust authority implementation and docs keep the explicit delta |

Commit the real proof before planting each pre-fix behavior. Confirm the named
proof fails for the intended reason, restore from the commit, and rerun green.

## Writable Scope

- `packages/core/src/tree.ts`, its exports, focused tests, and only the smallest
  shared Tree/drag helper needed for accepted destination depth;
- Svelte and React Tree, TreeItem, logical keyboard target, public export, and
  focused test surfaces;
- the existing drag-drop browser fixture/probe and package-install fixtures
  needed to prove mounted and public behavior;
- the temporary `reorderAuthority` entries in contract prop/spec drift scripts,
  which must be deleted as the paired implementation lands;
- Tree contract, architecture 011/spec 069 only for implementation evidence,
  this card, one September log, and `PAPERCUTS.md` for new friction.

The worker must not edit Rust/GPUI/Jetstream source, generic controller
lifecycle, PR #125's geometry policy, Figmatic, versions, releases, workflows,
or `docs/roadmaps/g16/README.md` / `docs/roadmaps/generation-index.md`.

## Validation

Use Effigy discovery. At minimum:

- focused shared-core Tree geometry/eligibility tests;
- focused Svelte and React Tree tests;
- Chromium and WebKit mounted pointer and keyboard cases;
- Svelte and React pack-install/type resolution for the three public types and
  mutually exclusive prop shape;
- relevant contract/API/drag inventory drift selectors;
- `effigy ci:web`, `effigy docs:check`, and `git diff --check
  origin/main...HEAD`.

No Rust source changes means broad Rust/native boards are not required. Do not
run `*-windowed`, native visual, release, tag, publication, workflow mutation,
or sibling-repository commands.

## Stop Conditions

- Correct behavior requires a second controller, host pointer coordinates,
  DOM ancestry, a domain payload, or changing PR #125's outline policy.
- A synchronous eligibility answer cannot determine accepted presentation
  before paint.
- Svelte and React cannot share one semantic type/result shape.
- The generic controller must change its terminal lifecycle rather than simply
  receive the authority's existing commit result.
- Native parity would require moving-set encoding, pending local Node commits,
  or durable Tree-local session storage. Record it; do not widen this card.
- The implementation needs Figmatic source, target vocabulary, or revision
  logic to pass.

## Continuation

After accepted merge, the orchestrator sends Figmatic the exact Poodle merge
head, public type signatures, package import paths, and validation receipt.
Figmatic then owns `g01.016 / 016-22` and removes its local pointer controller.

## Implementation

Paired Svelte/React Tree now takes `reorderAuthority` over the existing
controller. Shared core owns `TreeReorderSubject`, `TreeReorderCandidate`,
`TreeReorderAuthority`, `TreeReorderProps`, validity helpers, and
`treeAcceptedDropDepth`. Both packages re-export the types. `reorderable`
stays the enable switch. Authority mode never calls `onReorder`.

Native delta is unchanged and explicit: no TreeSpec field, no pending Node
commit, no rewritten full intent on native events, no multi-row `DragSubject`.
`reorderAuthority` remains in `WEB_ONLY_PROPS`. Diff contains no Rust/GPUI
source.

PR #127 changes-requested repair: `treeAuthorityDropEligibility`
snapshots hover fields, hands the host a detached intent, and builds
accepted state from the snapshot so in-place mutation of target, position,
or operation is refused. The latched `TreeReorderSubject` is frozen; `canDrop`
receives a detached subject so hostile replacement of `movingValues` /
`sourceValue` cannot skip dest validation or corrupt `onDrop`.
`treeAcceptedDropDepth` walks the full tree so a collapsed rewritten dest
still paints dest depth. Paired tests change selection after pickup and
before release. A pending Promise is proven on one mounted controller
(exactly one `onDrop`, exact rejected/failed announcement, source-loss then
a later dropping session). Packed tarball types cover core, both Svelte
paths, mapped React `TreeProps` assignability, and an honest installed
public-root resolution/export proof (value barrel is not compiled).

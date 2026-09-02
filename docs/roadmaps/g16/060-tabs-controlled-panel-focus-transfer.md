# g16.060 — Tabs Controlled-panel Focus Transfer

Status: in review — worker implementation complete
Opened: 2026-09-02
Depends on: current Tabs contract and paired Svelte/React implementation;
independent of the web-distribution/release serial lane
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/tabs.md`
Consumer evidence: Figmatic PR #69 review comment
https://github.com/inflatable-cookie/figmatic/pull/69#issuecomment-5514814268
Log: `../../logs/2026-09/20260902-g16-060-tabs-controlled-panel-focus-transfer.md`

## Goal

Add one opt-in, generic controlled-value focus policy to Tabs:
`focusOnValueChange="preserve" | "selected-tab"`, defaulting to `"preserve"`.
When the controlled value changes and focus was inside the outgoing selected
panel, `"selected-tab"` transfers focus to the newly selected tab after the
new controlled state is rendered. Otherwise Tabs does not move focus.

This is Tabs-owned focus continuity. Do not add a consumer focus machine, a
query-selector recipe, panel-specific initial-focus callback, IconButton
autofocus, or exported imperative `focus()` API.

## Fixed Behaviour

- The default `"preserve"` policy keeps current behaviour exactly.
- The policy applies to externally controlled value changes. It does not turn
  ordinary selection into a general autofocus mechanism.
- Transfer occurs only when `document.activeElement` was contained by the
  outgoing selected panel immediately before that panel is replaced or
  unmounted.
- The destination is the newly selected enabled tab. Tabs uses its owned tab
  element/ref registry; it does not query arbitrary consumer DOM.
- Focus already on a tab, outside Tabs, in an overlay, or in another document
  is never stolen.
- A missing/disabled destination, removal of the Tabs root, or teardown makes
  the request inert. No fallback focuses the document body or panel.
- Repeated or superseded controlled changes focus only the latest eligible
  destination once.
- The policy is paired across Svelte and React. It introduces no Rust/GPUI
  promise in this bounded consumer unblock; contract drift must state that
  delta explicitly if the current cohort cannot carry it.

## Ordered Work

1. Add the public policy and contract wording to the shared web Tabs surface,
   preserving `"preserve"` as the default.
2. Capture outgoing-panel focus ownership before controlled value
   reconciliation can unmount it. Apply the latest eligible request after the
   selected tab exists.
3. Add paired Svelte/React tests for focus inside the outgoing interactive
   panel, focus outside, focus already on a tab, default policy, disabled or
   missing destination, superseded changes, and teardown.
4. Add one mounted consumer-shaped proof: an async accepted operation changes
   controlled value from Components to Tree while focus is in a ListCard-like
   interactive descendant; only the selected Tree tab receives focus.
5. Reconcile the Tabs contract, public exports/types, drift evidence, this
   card, and one September execution log. Open one worker PR.

## Acceptance

- With `focusOnValueChange="selected-tab"`, an async controlled Components →
  Tree change transfers focus from an interactive descendant of the outgoing
  Components panel to the Tree tab exactly once.
- The same change with default/`"preserve"` does not invoke focus.
- Focus outside the outgoing panel, including another Tabs instance, is not
  moved.
- A pointer or keyboard selection whose focus is already on a tab does not
  cause a second focus transfer.
- A disabled, removed, or superseded destination cannot receive stale focus.
- Svelte and React expose the same prop values and behaviour. Existing Tabs
  selection, roving tabindex, automatic/manual activation, drag/reorder,
  overflow, history, motion, and panel semantics remain unchanged.
- No Figmatic source, consumer query selector, public imperative handle,
  IconButton API, Rust/GPUI source, version, release, or workflow file changes.

## Review Oracle

| Invariant | Counterexample | Required proof |
| --- | --- | --- |
| Transfer is conditional on outgoing-panel ownership | controlled value changes while focus is outside Tabs | paired tests retain the outside element as active |
| Capture precedes unmount | outgoing interactive child is removed in the same controlled render | selected tab receives focus after render |
| Default remains inert | omit the prop with focus in outgoing panel | no tab `.focus()` call and current behaviour retained |
| Destination is semantic and current | A → B is superseded by B → C before focus applies | only C tab receives focus once |
| Tabs owns the mechanism | consumer panel contains nested buttons/cards | implementation uses panel/tab refs, never consumer selectors |
| Shells stay paired | repair only one framework | paired Svelte/React oracle fails |

Commit the real proof before planting the current no-transfer behaviour. The
consumer-shaped oracle must fail because focus is lost on outgoing-panel
unmount, then pass after restore.

## Writable Scope

- Tabs public types/props and paired Svelte/React implementations/tests;
- the smallest shared Tabs helper if both shells would otherwise restate the
  same policy decision;
- Tabs contract and required contract/spec drift evidence;
- this card, one September execution log, and `PAPERCUTS.md` only for new
  execution friction.

Do not edit Figmatic, Rust/GPUI/Jetstream, other components, release/version
surfaces, workflows, or the web-distribution certification lane.

## Validation

Use Effigy selector discovery. Run focused paired Tabs tests, relevant packed
type/export proof if the public prop crosses package output, Tabs contract and
drift checks, `effigy ci:web`, `effigy docs:check`, and `git diff --check
origin/main...HEAD`. Do not run windowed or release selectors.

## Worker Receipt

Paired web policy landed on `fix/g16-060-tabs-controlled-focus`. Capture is
`$effect.pre` (Svelte) and render-time `panelRef` (React). Apply is one
cancellable `setTimeout(0)` through the owned tab registry. Public prop is
`focusOnValueChange?: "preserve" | "selected-tab"` with default `"preserve"`.
`TabsSpec` is unchanged; `WEB_ONLY_BY_SLUG.tabs` records the delta.

Orchestrator owns exact-head review, merge, roadmap closeout, and the Figmatic
receipt. Do not merge from this worker.

## Continuation

After accepted merge, return the exact Poodle SHA, public prop/import shape,
validation receipt, and local-link/build instructions to Figmatic. Figmatic
owns its PR #69 rebase, mounted counterexample, and consumer validation.

# g16.076 — Nucleus Select M1 Receipt

Status: complete
Type: Nucleus NP-2 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.073`, completed `g16.075`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/select.md`, `../../contracts/components/popover.md`,
`../../architecture/002-anchored-overlays.md`
Log: `../../logs/2026-09/20260903-g16-076-nucleus-select-receipt.md`
PR: `https://github.com/inflatable-cookie/poodle/pull/182`
Handoff: `../../handoffs/20260903-091200-g16-076-nucleus-select-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `Select` row through the
production Rust Select adapter, renderer, Node, GPUI backend, and test-platform
input paths. Strengthen the retained two-instance regression without creating
a second Select machine or claiming browser, accessibility-tree, or pixel
parity.

## Fixed Boundary

- Keep the manifest test name
  `select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds`.
  Strengthen that fixture rather than adding a second receipt test.
- Mount `node_compat::Select::from_spec(...).into_element()` through the
  element-backed HeadlessDriver. Renderer-only Node construction is not
  adapter evidence. Keep caller-scoped instance identities and generic data.
- Preserve the existing Select machine and the merged Popover/layer boundary.
  Host state owns value, query, open state, highlighted value, selection, and
  rebuilds. Drive pointer and keyboard claims through mounted input.
- Prove two duplicate-content instances do not cross values, queries, focus,
  callbacks, listbox ids, or dismiss layers. Opening one must not mutate or
  dismiss the other.
- Prove non-searchable selection, disabled-option inertia, clear-to-authored
  default, outside and Escape dismissal, and exact trigger focus restoration.
- Prove searchable editing through the production text-input path: initial
  enabled highlight, disabled skipping, Home/End and Arrow navigation, query
  filtering, caret/selection movement, option commit, freeform Enter and whole-
  control blur commit, and one ordered callback/effect stream per transition.
- Prove exact listbox/trigger relationships and semantics, selected/highlighted/
  disabled option metadata, group labeling, visible option text, indicator and
  checked icon production metadata, token styling, overlay width constraints,
  positive bounds, containment, and capped long-menu scrolling without a pixel
  claim.
- Cover whole-control disabled state and removal/disablement while open. A
  removed or newly disabled highlighted option must revalidate through the
  machine; no stale option may commit.
- A focused native repair is allowed only after a committed mounted
  counterexample. Stop for a public API, a second machine, browser-only portal
  behavior, app-owned filtering/focus policy, or a new generic overlay model.
- Emit the receipt only after every claimed assertion. Refresh the manifest,
  every existing receipt, generated ledger, this card, and one execution log
  from the exact committed runtime source. No other row advances.

## Acceptance

- Select has one valid `nucleus.navigation.select` M1 receipt naming the
  retained mounted test. The denominator stays 29 and the existing 12 receipts
  remain valid.
- Raw renderer mounting, shared instance identity, direct handler invocation,
  callback-only state without rebuild, stale highlighted commits, disabled
  activation, coupled dismissals, wrong focus restoration, or early receipt
  emission fails the proof.
- M1 does not infer A1 accessibility-tree semantics, browser native/custom
  mode parity, portal collision behavior, V1 pixels, Nucleus M2, or Jetstream.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::select` directly | adapter-path assertion or lifecycle fails |
| Instance identity is scoped | compose duplicate-valued instances without scopes | focus, layer, query, or callbacks cross streams |
| Input is mounted | call transition handlers directly | mounted observation or exact effect trace is absent |
| Controlled ownership is real | record callbacks without rebuilding supplied state | painted value/open/query disagrees with host state |
| Disabled paths are inert | bind trigger or disabled option activation | callback, open state, or committed value changes |
| Highlight revalidates | remove or disable the highlighted option before commit | stale value commits instead of machine revalidation |
| Search editing is real | replace text-input dispatch with direct query mutation | caret, selection, query, or callback trace fails |
| Dismissal is isolated | outside/Escape closes both instances | retained sibling layer or focus witness fails |
| Focus restoration is scoped | restore the other instance trigger | focused runtime id crosses instances |
| Structure and tokens are exact | drop listbox relationship, group name, icon metadata, or surface tokens | Node metadata assertion fails |
| Geometry is bounded | remove width cap, containment, or scroll overflow | mounted bounds/overflow assertion fails |
| Receipt is terminal | fail the final freeform/sibling assertion | no Select receipt is emitted |
| Evidence identity is exact | retain the g16.075 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

The retained Select mounted regression; focused Select spec, machine, renderer,
backend, and GPUI adapter tests; a bounded native repair only when a committed
mounted counterexample requires it; receipt/manifest/ledger refresh; this card;
one execution log; and new papercuts. Do not edit Nucleus, web behavior, public
APIs, accessibility authority, visual-lab code, Jetstream, workflows, versions,
releases, or other component rows.

## Validation

Run focused Select spec/machine/render/backend tests, the named mounted fixture,
the retained long-menu overflow fixture, `effigy regressions:native`,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof needs a public API, another Select or
overlay machine, browser-only selectors, Nucleus data, broad A1/V1 claims,
collision-engine redesign, or app-owned focus/filter policy. Record the exact
gap instead of weakening the receipt.

## Continuation

After merge, compile the next dependency-ready Nucleus M1 receipt tranche from
the refreshed identity `232ae3b73f0e068f1f59690cc8e2f942546dcec2`.
Receipt-producing merges remain serial.

# g14.004 — Tabs Collection And Navigation Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/004-tabs-collection-navigation-proof.md`
Depends on: g14.003 / PR #13
Status: accepted for merge

## Outcome

```text
one structured item collection + stable semantic keys
  -> repeated trigger/panel parts in Svelte / React / GPUI
  -> controlled navigation and relationship observations
  -> corpus-projected specimens in all active runtimes
  -> Jetstream program-deferred
```

## What changed

### Shared authority

- Added collection prop fields and keyed repeated parts to the generic schema.
- Added native id templates and keyed web part resolution.
- Authored one Tabs portable interface and nine cases. Item order changes while
  `trigger:<value>` and `panel:<value>` identity stays stable.
- Added selected, tabbable, orientation, controls, labelled-by, and focused
  observations to both runtime paths and the comparator.

### Runtime execution

- Svelte and React adapters drive real focus, click, and keyboard events.
- GPUI renders tablist/tab/tabpanel roles, controlled value changes, semantic
  relationships, roving focus, and automatic/manual keyboard navigation.
- Native instance ids scope every list, trigger, and panel runtime identity.
  `Tabs::with_id` now reaches the renderer instead of being discarded, so
  equal tab values in separate tabsets cannot share GPUI focus handles while
  portable semantic ids and relationships remain unchanged.
- `NodeInteraction::on_key` can return a semantic focus target. The GPUI
  backend performs the platform focus move and projects roving `tab_index`
  metadata into GPUI tab-stop state; Tabs does not require a backend special
  case.
- The GPUI conformance binary writes `gpui-tabs.json` beside the Button and
  RangeSlider reports. Generic runners discover repeated parts from interface
  metadata; there is no Tabs dispatch branch or alternate item list.

### Defects caught

- React manual activation focused the next tab but derived `tabIndex` from the
  selected item, leaving the focused tab outside the roving tab stop. React now
  seeds focus from selection and preserves independent manual focus movement.
- Native Tabs declared focusability without joining the backend tracked-focus
  channel, so arrow events could not reach a semantic tab. Renderer focus-change
  wiring now creates real handles and records visible focus.
- Native token observation reported the unresolved base size (`md`) instead of
  the default chrome role's resolved size (`sm`).
- The expanded identity comparison exposed missing native orientation on the
  embedded RangeSlider controls; that pre-existing gap is closed.
- Review found that the GPUI driver completed arrow navigation itself after
  dispatching the key. It now only rebuilds the controlled tree; the backend
  must execute the focus request for the observation to pass.
- Review also found global native trigger ids (`tabs:<value>`). A separate
  runtime identity now isolates backend caches without changing observable
  semantic ids or accessibility relationships, with a renderer regression
  covering two tabsets that reuse the same values.

## Geometry and relationships

The default trigger asserts 12px inline padding with an authored ±1px bound.
Every case observes stable trigger-to-panel controls and panel-to-trigger
labelled-by links. Reordered fixtures retain the same semantic ids.

## Replacement and cost

- Corpus projection replaces only the bounded collection/navigation examples.
  Web specimens retain overflow and working close/reorder examples. GPUI
  retains working close plus icons/counts, edge/fill, panel, and scale examples;
  native reorder and overflow remain outside the preview's supported surface.
- Tabs pilot increment: 1,126 LOC, including the generated Rust declaration.
- Tabs generated interface/case data: 23,010 bytes.
- `TabsSpec` is not replaced: overflow, history, close/reorder, tooltips, and
  host actions remain outside this profile and are not counted as replaced.
- Existing Tabs machine vectors are not replaced wholesale. Close/reorder
  claims remain vector-owned and are retained for g14.010/g14.011 disposition.

## Planted failure

A generated report relationship was changed from `panel:overview` to a wrong
target. `conformance:compare` named the runtime, case, repeated part, and
`controls` divergence. Regenerating the report restored the green comparison.

## Validation

| Command | Result |
| --- | --- |
| `effigy conformance:typecheck` | pass |
| `effigy check:gpui` | pass (184 renderer + 11 backend tests) |
| `effigy conformance:test-web` | pass (4 files) |
| `cargo check --manifest-path packages/gpui/preview/Cargo.toml --bins` | pass |
| `effigy conformance:test-gpui-windowed` | pass (20 Button + 10 RangeSlider + 9 Tabs) |
| `effigy conformance:compare` | pass (39 cases × 3 runtimes) |
| planted relationship divergence | fails, then restores |
| `effigy ci:conformance-headless` | pass |
| `effigy docs:check` | pass |
| `git diff --check` | pass |

## Papercuts

- Fresh worktrees need dependency bootstrap before web conformance.
- The documented Jetstream sibling checkout can collide during Rust bootstrap;
  Jetstream remains outside this card's active cohort.
- A one-shot Effigy graph query can leave its refresh lock held after output.

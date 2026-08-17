# g15.008 — Model-Connection Family Native Completion (August batch log)

Date: 2026-08-17
Card: `docs/roadmaps/g15/008-model-connection-family-native-completion.md`
Worktree: `t3code/complete-model-connection` (worker lane, serial)
Handoff: `docs/handoffs/20260817-122537-g15-008-model-connection-native-completion.md`

## Summary

Completed the native surface for the model-connection family —
ModelConnectionPicker, ModelConnectionSetup, ModelConnectionCard,
ModelCatalogueEditor — across hand-written Rust declarations, a pure
`poodle-headless` behaviour mirror, `poodle-render` composition, and GPUI
specimens with mounted interaction evidence. The approved Svelte and React
implementations and specimens are unchanged.

Poodle acquired no provider truth in this lane: no provider registry, no
credential store, no discovery or probe, no persistence, no update policy, no
model defaults or favourites, and no route fallback. Availability, readiness,
catalogue postures, and the set of routes remain host classifications the
components repeat.

## Batches

- **Batch A — declarations and behaviour.** `poodle_headless::model_connection`
  ports the approved `model-connection.ts` semantics: case-folded filtering
  that keeps host source order across provider, route, description, group and
  keywords; first-seen group order; selectability; shell-state resolution;
  result announcements and posture copy; the setup context, events, effects and
  stage guards including the direct-add path; catalogue shown/hidden
  derivation, complete shown-order requests, visibility requests,
  focus-after-hide and announcements; and the availability/readiness tone
  mappings. Core's `listReorderKeyIntent` had no Rust counterpart, so its
  mirror (`model_catalogue_reorder_key_intent`) lives with its only Rust
  consumer rather than inventing a shared module for one caller.
  `ModelConnectionPickerSpec`, `ModelConnectionSetupSpec`,
  `ModelConnectionCardSpec` and `ModelCatalogueEditorSpec` are cloneable
  controlled data.
- **Batch B — render and host bindings.** All four `poodle-render`
  implementations, their handler structs, and the keyed host-composition seams
  (`*Slots`). No host `Node` enters a spec, and repeated content is keyed by
  opaque id.
- **Batch C — GPUI evidence.** Four curated specimens plus six mounted
  regressions on the in-memory test platform, and the AppState host loop that
  owns every controlled value the components request.
- **Batch D — release evidence.** The four native roster rows, the gap
  register's model-connection row and native counts, and this log.

## Evidence per runtime

One runtime does not borrow another's pass.

- **Svelte** (unchanged, re-run): `ModelConnectionPicker.test.ts`,
  `ModelConnectionSetup.test.ts`, `ModelConnectionCard.test.ts`,
  `ModelCatalogueEditor.test.ts` under `effigy test:components`. The card's
  ready-vs-checking status precedence is proved there and is the TS-side
  counterpart of the Rust `model_connection_card_status_label` vectors.
- **React** (unchanged, re-run): `packages/react/components/test/ModelConnection.test.tsx`.
- **Core (TS owner vectors)** (`effigy test:core`, 757 passed): the existing
  `packages/core/test/model-connection.test.ts` cases plus four added here —
  reselection emits nothing, the selected summary repeats supplied labels, and
  catalogue state copy keeps every posture distinct.
- **Rust headless** (`cargo test -p poodle-headless`, 124 passed): 28 new
  owner-local cases naming the same inputs and observable outputs as the TS
  file above — filtering order across every searchable field, first-seen group
  order, selectability, the seven shell-state resolutions, result
  announcements, posture copy, the continue/submit/back/select/pending
  transitions including direct-add, shown/hidden partitions, complete
  shown-order payloads, visibility payloads, focus-after-hide, announcements,
  reorder key intents, and the tone/label mappings.
- **Rust render** (`cargo test -p poodle-render`, 272 passed): 46 new cases —
  9 picker (filter order and group order, radio semantics with the supplied
  reason in the accessible name, one selected indicator replacing the mark,
  disabled/unavailable guards, roving keys with wrap and Home/End, query
  requests plus the announced count, every posture rendering its own copy and
  no choices, keyed host marks and footer, a disabled picker keeping its
  selection); 10 setup (Continue for a configured route, direct-add submitting
  from choose with no stage emitted, guards, host configuration content and
  the host-approved submit, Back with focus restoration, the pending lock and
  its live region, safe feedback without a stage reset, forwarded picker
  requests, cancel, and the root being a region not a dialog); 9 card
  (independent open/enabled, ready-vs-readiness status, closed-accessory
  mounting, the labelled details region and its `controls` relationship, focus
  restoration on close only, an off card dimming copy but never its switch, a
  disabled card staying readable, the enable switch disabled on its own,
  instance-scoped ids, and the inline mark not indenting the summary);
  18 catalogue (explicit moves with the complete order and focus, boundary
  disabling, keyboard grab/move/drop/cancel, boundary announcement, admitted
  pointer drag through the same payload, drag disabled with keyboard intact,
  hide with focus to the next shown model, hide-the-last disclosing the hidden
  section, restore, the optional info action, locked and per-row-disabled
  surfaces, all six postures distinct with no stale rows or counts, host
  posture overrides, the live region, keyed host content, shown-only, and
  duplicate labels staying distinct by id).
- **Rust specs** (`cargo test -p poodle-specs`, 260 passed): unchanged suite
  re-run; the four new specs compile into it.
- **GPUI mounted regressions** (`effigy regressions:native`, 22 passed): the
  16 retained plus six new — the picker's roving focus moving real backend
  focus past the disabled routes; a real pointer click on an unsupported route
  selecting nothing while the available one beside it selects; the setup's
  direct-add submitting from choose and emitting no stage; the card's
  disclosure and switch staying independent with real focus returned to the
  disclosure on close; the catalogue editor grabbing, moving and cancelling
  through the real key dispatch; and hide emitting only a visibility request
  and moving real focus to the next shown model's handle.
- **GPUI preview** (`effigy check:gpui`): the four specimens compile into the
  catalogue (`model-connection-picker`, `model-connection-setup`,
  `model-connection-card`, `model-catalogue-editor`); `cargo test --test catalogue`
  7 passed.
- **Jetstream**: program-deferred. It is not run, not counted, and not claimed
  as an accepted absence.

## Intentional binding differences

- **Uncontrolled seeds are web-only.** `defaultValue`, `defaultQuery`,
  `defaultStage` and `defaultOpen` have no Rust field: GPUI/AppState owns the
  current value and rerenders after a callback requests a change. The four
  contracts mark those rows **Web targets only**, and `contract-spec-drift`
  gained a slug-scoped exemption beside its global one so the
  `defaultValue`/`defaultOpen` guard stays intact for the ~20 components that
  do carry them.
- **Natives label by object.** A picker option's accessible name states
  provider, route, description and the supplied availability reason once,
  where the web composes the same content from descendant text plus a
  visually-hidden line. The card states the provider family on the identity
  group for the same reason.
- **Focus movement is a request.** The vocabulary gives `on_key` a focus
  destination and nothing else, so stage focus, close-restoration and
  focus-after-hide are named through an `on_focus_request` handler that the
  GPUI bridge performs with `request_focus`. The web moves focus itself in an
  effect.
- **Transient interaction state is host state.** The catalogue's grab, drop
  target, hidden disclosure and live copy are spec fields the host owns, like
  every other overlay in the Rust vocabulary, rather than component-internal
  reactive state.
- **Enter and Space are the backend's activation path.** Keyboard grab/drop
  and option selection ride it; `on_key` carries only the arrow/Home/End
  moves, and Escape rides `on_cancel`. Binding Space in both places would act
  twice.
- **Composed chrome stamps its own focus ring.** `poodle_render::icon_button`
  renders no focus patch and the GPUI backend creates a focus handle only for
  a focusable node that carries one, so the card's disclosure and every
  catalogue utility stamp one — the workaround `history_center` already
  carries, recorded again in `PAPERCUTS.md`.

## Papercuts recorded

- `IconButtonSpec::with_expanded` / `with_controls` never reach `Node.a11y`
  through `icon_button`; three components now restate them.
- The vocabulary has no Escape channel for a plain control, so a keyboard
  reorder grab is cancelled through `Interaction::on_cancel`.
- The headless driver's 160×60 mount box clips hit testing as well as paint,
  so a mounted regression over a taller component must drive keyboard
  activation or shrink its fixture.
- `contract-spec-drift` had only a global web-only prop set; the two lists
  should probably become one slug-aware structure if more families land.

## Validation run

- `effigy test:core` — 757 passed (48 files)
- `cargo test -p poodle-headless` — 124 passed
- `cargo test -p poodle-specs` — 260 passed
- `cargo test -p poodle-render` — 272 passed
- `effigy check:gpui` — clean
- `effigy regressions:native` — 22 passed
- `cargo test --test catalogue` — 7 passed
- `effigy docs:check` — clean
- `effigy qa` — see the PR body for the run on the final rebased head
- `git diff --check origin/main...HEAD` — clean

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector was run.

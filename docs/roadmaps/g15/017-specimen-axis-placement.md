# g15.017 — Web Specimen Axis Placement and Evidence

Status: **ready** — exact 24-route paired-web scope approved
Consumes: `g15.011` partial screening baseline
Depends on: `g15.016` (specimen idiom convergence)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`
(Axis Eligibility), `../../contracts/001-working-rules.md`

## Outcome

Every web catalogue page shows `Sizes` and `Densities` exactly when the
component takes that public prop, and every visible axis tab contains one
representative example per step.

The audit found 17 missing size axes, 18 missing density axes, and five pages
with spurious tabs. PR #38 then completed the twelve React audio layouts and
gave `ListCardCounter` a dedicated page, removing one spurious case. This card
starts from that merged state. It does not redo the reviewed audio pages.

Native audio separation and GPUI pane adoption move together in `g15.019`.
Splitting their shared return shape here would leave a compatibility bridge or
a half-migrated consumer, both worse than one bounded native migration.

## Exact Scope

Twenty paired pages are missing eligible evidence:

- **Both size and density (15):** `ConfirmAction`, `SplitView`, `ToastHost`,
  `AlertDialog`, `Dialog`, `Drawer`, `FormDialog`, `BlockEditor`, `LogList`,
  `VideoPlayer`, `LicenceActivation`, `LicenceSeats`, `LicenceStatus`,
  `UpdateCenter`, and `UpdateStatus`.
- **Size only (2):** `Eyebrow` and `Text`.
- **Density only (3):** `IconButton`, `Icon`, and
  `UiPresentationProvider`. `IconButton` and `Icon` keep their existing size
  evidence.

Four paired pages advertise an ineligible axis:

- `Avatar`: keep `Sizes`; remove `Densities` from the authored scene and make
  both web scene renderers obey the scene's declared tab set.
- `Tooltip`: remove both axis tabs. Its current matrices vary the child
  `Button`, not `Tooltip`.
- `PickerShell`: remove `Densities`; presentation-provider density is not a
  `PickerShell` prop.
- `MeterSurface`: remove both empty axis tabs.

The twelve audio pages corrected in PR #38 are validation-only:
`DragNumberField`, `AudioMeter`, `AudioSwitch`, `EnvelopeEditor`, `Fader`,
`GainReductionMeter`, `Keyboard`, `Knob`, `ModMatrixGrid`, `ValueReadout`,
`WaveformDisplay`, and `XYPad`.

## Goals

- [ ] Add one representative Svelte and React example for every missing axis
      step without expanding the `Examples` pane.
- [ ] Remove the four remaining spurious axis surfaces without deleting useful
      example content.
- [ ] Make preview-local `SpecimenLayout` incapable of advertising an empty
      axis tab: a renderer must exist, and `showSizes` / `showDensities` may
      narrow visibility but may not force an empty pane.
- [ ] Make the retained authored-scene renderers respect each scene's declared
      `tabs`; update the Avatar source model and regenerate, never hand-edit
      generated artifacts.
- [ ] Keep paired tab presence, ordering, representative state, and accessible
      labels aligned between Svelte and React.

## Acceptance

- [ ] A complete 175-route web census reports `Sizes` iff the component takes
      `size`, and `Densities` iff it takes `density`, in both runtimes.
- [ ] Every advertised axis tab renders visible content; no callback-less
      `SpecimenLayout` produces an empty tab.
- [ ] Each axis pane contains exactly one representative per step: five sizes
      and three densities. No prop cross-product enters an axis pane.
- [ ] The twelve reviewed audio pages remain in `SpecimenLayout`, with their
      matrices outside `Examples` and paired across Svelte and React.
- [ ] Focused evidence locks helper behaviour, the 24 corrected route
      decisions, authored-scene tab projection, and paired tab parity.
- [ ] No public package API, component behaviour, contract semantics, example
      curation, or native specimen changes.
- [ ] **Operator review of the 24 changed routes in the live Svelte and React
      previews before this card is called complete.** Unreviewed pages remain
      an explicit PR item.

## Stop Conditions

- An axis example exposes a component API or contract mismatch. Return the
  finding instead of changing the component inside this card.
- The work turns into shortening or rewriting `Examples`; that belongs to
  `g15.020`–`g15.025`.
- The worker needs to change `poodle-render`, GPUI, or Jetstream. Native axis
  separation and adoption belong together in `g15.019`.
- Axis eligibility becomes data imported by a runtime. Test-time derivation or
  census evidence is allowed; production authority is not.

## Writable Scope

- paired Svelte and React specimen files for the exact 24 changed routes
- preview-local Svelte and React `SpecimenLayout` and `SceneSpecimen` helpers
- the authored Avatar specimen model, its fixture, and generated specimen
  outputs produced by the repository generator
- focused preview/parity tests or a bounded source/live census
- one batch log

## Validation

- focused helper, scene, and paired axis evidence; full 175-route web census
- `effigy ir:build`, `effigy ir:check` when the authored scene changes
- `effigy check:svelte`, `effigy react:build`, `effigy catalogue:check`,
  `effigy ci:web`, `effigy docs:check`
- live paired review of the 24 changed routes
- `git diff --check origin/main...HEAD`

## Continuation

After merge, advance to `g15.019` for the native axis/caption structure lane.
Do not absorb overloaded-Examples curation into this worker run.

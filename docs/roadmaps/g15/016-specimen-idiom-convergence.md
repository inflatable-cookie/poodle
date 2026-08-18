# g15.016 — Specimen Idiom Convergence

Status: **ready** — exact 29-page web scope and evidence approved
Consumes: `g15.011` partial screening baseline
Depends on: `g15.015` (caption integrity)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Outcome

One caption shell for every web example, plus dedicated pages for the two
components that currently borrow another component's specimen.

`g15.011` found four caption idioms in use: `SpecimenGroup label=`, a bare
`<Eyebrow>` inside a hand-rolled `Surface`, `<section><h3>` in the audio
family, and React's separate `AudioSpecimenGroup title=`. They look different
on the page and carry different spacing. The divergence is what let the broken
fifth idiom hide.

Two pages also borrow another component's specimen outright, so the page title
names one component while the content teaches another.

## Exact Scope

Twenty-nine paired web catalogue routes:

- **Bare-Eyebrow Svelte pages (13):** `SplitButton`, `TriStateSwitch`,
  `Select`, `TextInput`, `TokenInput`, `TimeInput`, `TimeZoneSelect`,
  `Eyebrow`, `AlertDialog`, `Dialog`, `Drawer`, `Menu`, and `MarkdownEditor`.
  Replace caption chrome with `SpecimenGroup`; keep any `Eyebrow` instance
  that is the component being demonstrated.
- **Uncaptioned page (1):** `SettingsShell`. Wrap its existing examples in
  meaningful `SpecimenGroup` sections in both web runtimes without adding new
  examples.
- **Audio-helper pages (13):** `DragNumberField`, `AudioMeter`, `AudioSwitch`,
  `EnvelopeEditor`, `Fader`, `GainReductionMeter`, `Keyboard`, `Knob`,
  `ModMatrixGrid`, `ValueReadout`, `WaveformDisplay`, `XYPad`, and
  `MeterSurface`. Svelte's direct `<section><h3>` captions and React's
  `AudioSpecimenGroup title=` converge on the ordinary preview-local
  `SpecimenGroup label=` helper.
- **Borrowed pages (2):** `ListCardCounter` stops routing to `ListCardSpecimen`;
  `MetaItem` stops routing to `MetaBarSpecimen`. Add dedicated paired specimen
  files and registry entries.

The React `AudioSpecimenPage`, row, and axis helpers may remain until
`g15.017`; only the forked caption helper is removed here.

## Goals

- [ ] Every web specimen page captions its examples through `SpecimenGroup`.
- [ ] The 27 divergent or missing-caption pages converge on it. Dense
      renderer-owned composition may stay; the caption mechanism does not
      fork.
- [ ] `ListCardCounter` and `MetaItem` get their own pages that teach their own
      component.
- [ ] Svelte and React structure and copy agree page for page.
- [ ] The `ListCardCounter` page teaches the component in its intended
      `ListCard` footer context: static/tooltip counters and a wired linked
      counter. The `MetaItem` page teaches labelled, unlabelled/rich, and
      interactive child content with the interaction wired.
- [ ] Reconcile the two component contracts' specimen sections with those
      dedicated pages. This is documentation alignment, not a public component
      API or semantic change.

## Acceptance

- [ ] A source census reports no remaining caption use of bare `Eyebrow`,
      direct `<section><h3>`, or `AudioSpecimenGroup` outside the ordinary
      `SpecimenGroup` helper. `Eyebrow` examples remain allowed as subject
      content.
- [ ] A live sweep reports zero blank captions across all 175 web catalogue
      routes and one visible caption shell on each of the 29 scoped routes.
- [ ] None of the 29 scoped routes has a different caption count or different
      caption copy between Svelte and React.
- [ ] `ListCardCounter` and `MetaItem` pages name and teach themselves.
- [ ] Focused regression evidence locks the idiom census and the two dedicated
      registry mappings.
- [ ] No package export, component API, component behaviour, or contract
      semantics change.
- [ ] **Operator review of the changed pages in the live Svelte and React
      previews before this card is called complete.** Unreviewed pages remain
      an explicit PR item.

## Stop Conditions

- Converging the idiom turns into rewriting example content — that is
  `g15.018`.
- `SpecimenGroup` grows options until it is a layout engine.
- Audio axes move into tabs, axis eligibility changes, or size/density evidence
  is otherwise reworked — that is `g15.017`.
- A dedicated page exposes a component API or behaviour defect. Record it and
  return to the orchestrator instead of repairing the component here.

## Writable Scope

- the 29 named specimen routes in both web previews; existing paired files that
  already use `SpecimenGroup` are read/validation scope unless copy alignment
  requires a bounded edit
- preview-local `SpecimenGroup` and the React `AudioSpecimen*` helpers; do not
  change package components or turn `SpecimenGroup` into a layout engine
- the Svelte and React specimen registries for the two dedicated pages
- `docs/contracts/components/list-card-counter.md` and
  `docs/contracts/components/meta-item.md`, specimen documentation only
- focused preview tests or a bounded source-census script
- one batch log

## Validation

- focused preview tests/source census; scoped live Svelte/React caption sweep
- `effigy check:svelte`, `effigy react:build`, `effigy catalogue:check`,
  `effigy ci:web`, `effigy docs:check`
- `git diff --check origin/main...HEAD`

## Continuation

After merge, advance to `g15.017`. Do not absorb axis placement or overloaded
Examples curation into this worker run.

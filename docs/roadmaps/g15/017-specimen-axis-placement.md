# g15.017 — Specimen Axis Placement and Evidence

Status: **planned** — orchestrator review required before dispatch
Consumes: `g15.011` partial screening baseline
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`
(Axis Eligibility), `../../contracts/001-working-rules.md`

## Outcome

Size and density evidence lives in the tabs that own it, appears wherever the
component actually takes the prop, and is absent where it does not.

`g15.011` measured eligibility from the component props: 126 components take
`size`, 128 take `density`. Three things disagree with that today.

- The audio family's twelve components put their full size **and** density
  matrices in the main view. React has no axis tabs at all for them, and the
  native pages fold `axis_groups` into the page body. This is the specific
  thing the operator asked not to happen.
- A set of components take an axis but their page shows no evidence for it —
  the overlay family, the licence and update family, and others named in the
  audit.
- A few pages advertise an axis tab for a component with no such prop.

## Scope

- the twelve audio components in React and `poodle-render`'s
  `audio_specimens`
- the components listed under **missing axis evidence** in the audit
- the components listed under **spurious axis tab**
- `MeterSurface`, the one page whose advertised axis tabs render empty

## Goals

- [ ] Axis matrices appear only in `Sizes` and `Densities`, never in
      `Examples`.
- [ ] React's audio pages use `SpecimenLayout` like every other page.
- [ ] `poodle_render::audio_specimens` separates its axis groups from its
      example groups so the native shells can place them in panes.
- [ ] A page shows an axis tab exactly when its component takes that prop.
- [ ] Axis panes show one representative example per step. The three-per-size
      sweeps (`RangeSlider`, and others named in the audit) collapse to one.

## Acceptance

- [ ] A live sweep shows no `Examples` pane containing a full size or density
      sweep.
- [ ] Every advertised axis tab renders content.
- [ ] Axis-tab presence matches prop eligibility for all 175 entries.
- [ ] Svelte, React, and GPUI agree on which axes each page teaches.
- [ ] **Operator review of the changed pages in the live Svelte and React
      previews before this card is called complete.** Unreviewed pages remain
      an explicit PR item.

## Stop Conditions

- Axis eligibility becomes a generated table any runtime imports. It is a
  property of the component's props, read at authoring time.
- The audio work turns into a rewrite of the audio examples themselves.

## Writable Scope

- audio specimens in the React gallery and `packages/render/src/audio_specimens.rs`
- the specimen files named in the audit's axis sections
- one batch log

## Validation

- focused preview tests, `effigy check:svelte`, `effigy react:build`,
  `effigy check:gpui`, `effigy docs:check`, `git diff --check`

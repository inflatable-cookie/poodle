# g15.016 — Specimen Idiom Convergence

Status: **planned** — orchestrator review required before dispatch
Consumes: `g15.011` partial screening baseline
Depends on: `g15.015` (caption integrity)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Outcome

One way to caption an example, and one way to lay out a specimen page, across
the Svelte and React catalogues.

`g15.011` found four caption idioms in use: `SpecimenGroup label=`, a bare
`<Eyebrow>` inside a hand-rolled `Surface`, `<section><h3>` in the audio
family, and React's separate `AudioSpecimenGroup title=`. They look different
on the page, they carry different spacing, and the divergence is what let the
broken fifth idiom hide.

Two pages also borrow another component's specimen outright, so the page title
names one component while the content teaches another.

## Scope

- pages listed under **hand-rolled captions** in the audit
- `ListCardCounter` and `MetaItem`, which currently render another
  component's page
- `AudioSpecimen*` helpers in the React gallery

## Goals

- [ ] Every web specimen page captions its examples through `SpecimenGroup`.
- [ ] The audio family's `<section><h3>` pages and React's `AudioSpecimenGroup`
      converge on it. Dense one-line composition may stay; the caption
      mechanism does not fork.
- [ ] `ListCardCounter` and `MetaItem` get their own pages that teach their own
      component.
- [ ] Svelte and React structure and copy agree page for page.

## Acceptance

- [ ] A live sweep reports one caption mechanism per page and zero blank
      captions across the catalogue.
- [ ] No page's caption count differs between Svelte and React.
- [ ] `ListCardCounter` and `MetaItem` pages name and teach themselves.
- [ ] **Operator review of the changed pages in the live Svelte and React
      previews before this card is called complete.** Unreviewed pages remain
      an explicit PR item.

## Stop Conditions

- Converging the idiom turns into rewriting example content — that is
  `g15.018`.
- `SpecimenGroup` grows options until it is a layout engine.

## Writable Scope

- the affected specimen files in both web previews
- `SpecimenGroup` / `SpecimenLayout` presentation, if convergence needs it
- the specimen registries, for the two new pages
- one batch log

## Validation

- focused preview tests, `effigy check:svelte`, `effigy react:build`,
  `effigy docs:check`, `git diff --check`

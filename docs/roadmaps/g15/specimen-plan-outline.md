# g15 — Renderer-Neutral Specimen Plan (outline)

Status: planning outline — written by `g15.011`
Date: 2026-08-17
Card: `docs/roadmaps/g15/011-specimen-catalogue-audit.md`
Governing refs: `docs/contracts/001-working-rules.md` (Catalogue Specimens),
`docs/roadmaps/g14/026-human-centred-specimen-catalogue-audit.md`
(shared-plan boundary), `docs/roadmaps/g15/specimen-catalogue-audit.md`

## What This Document Is

A shared description of **what a specimen page should contain and in what
order**, so Svelte, React, and GPUI can agree on structure without agreeing on
rendering.

It is a planning artifact. It defines vocabulary and page shape for future
curation cards to work against. It is deliberately not:

- a schema, IDL, or serialisation format
- a codegen source or generated adapter
- a render tree, node vocabulary, or scene description
- a callback, event, or handler format
- a runtime API that any preview imports

`g14.008` rejected an executable cross-runtime corpus and `g14.021` removed its
projection. Nothing here reintroduces that mechanism under a softer name. If a
future card cannot express a page in this outline without inventing a scene
language, the correct answer is to keep that page's renderer-owned adapter and
share only its outline.

## Page Shape

Every catalogue page is:

```text
Hero          component name, package, one-sentence purpose
Specimen      Examples [ · Sizes ] [ · Densities ]
Import        the import line
Usage docs    generated contract-derived props/notes
```

Only the specimen block is in scope here.

### Tab order

`examples` → `sizes` → `densities`. Fixed. `Examples` always exists. The two
axis tabs appear only when the axis applies (see Axis Eligibility).

No `Conformance` tab. Confirmed absent from all three previews by
`g15.011`; exhaustive fixtures, actions, and assertions stay in focused tests
beside the component.

### Section order inside `Examples`

Ordered, and each section is optional except the first:

| # | Section id | Answers | Rule |
| --- | --- | --- | --- |
| 1 | `default` | "What is this and how do I normally use it?" | required; one realistic use, not a prop showcase |
| 2 | `variants` | "What visually distinct forms exist?" | one axis only — never a prop cross-product |
| 3 | `tones` | "How does intent change it?" | all tones on **one** variant |
| 4 | `content` | "What can it hold?" | icons, counts, slots, truncation |
| 5 | `states` | "What does it look like when something is happening?" | only states this component really has |
| 6 | `interaction` | "What can the reader do here?" | the page's live behaviour, wired |
| 7 | `composition` | "How does it sit with other components?" | the realistic host arrangement |
| 8 | `layout` | "What happens when space is tight?" | narrow/overflow behaviour where it matters |

A page uses the sections it needs. A page that needs more than about six is
usually describing more than one component, or is repeating an axis that
belongs in a tab.

### Section budget

- target: **3–6** sections in `Examples`
- 7–9: allowed only with a stated reason
- 10+: treated as a curation defect

## Captions

Every section carries a caption. Captions describe user-facing meaning, not
fixture identifiers.

- good: "Danger and success tones", "What the reader sees while it loads"
- bad: `btn-tone-matrix-3`, "Case 12", "variant=ghost tone=danger"

A caption is part of the page, not decoration. A section whose caption does not
render is a defect regardless of how good the source copy is — see the
`SpecimenGroup` caption failure recorded in the audit.

Where a caption needs a sentence of explanation, the sentence belongs on the
page next to the caption, in every runtime that can show it.

## Example IDs

Stable, kebab-case, unique within a page, and derived from meaning:

```text
<section-id>/<example-id>          button/default/save-cancel
                                   button/variants/primary
                                   tabs/overflow/shed
```

IDs exist so the three runtimes can be compared and so a future card can say
which example changed. They are **not** test selectors and carry no assertions.

## Axis Eligibility

A page shows the `Sizes` tab when the component takes `size`, and the
`Densities` tab when it takes `density`. Both are decided from the component's
public props, not from habit.

| Condition | Result |
| --- | --- |
| component takes the prop | tab appears, filled with one representative example per step |
| component does not take the prop | tab is absent — not empty |
| component takes the prop, page shows no tab | missing teaching value (audit defect) |
| tab present with nothing in it | broken page (audit defect) |

Rules:

- The axis tabs own the axis. Size and density matrices do not appear in
  `Examples`.
- One representative example per step. A tab that renders three variants per
  size is an exhaustive matrix wearing a tab.
- `sizeRole` is not an axis. It is a semantic offset and belongs in `Examples`
  where it changes the story.

`g15.011` measured eligibility across the roster: 126 components take `size`
and 128 take `density`. The per-component result is in the audit.

## Cross-Runtime Agreement

| Layer | Shared | Runtime-owned |
| --- | --- | --- |
| tab set and order | yes | — |
| section order and ids | yes | — |
| captions and copy | yes (Svelte/React verbatim) | GPUI may shorten |
| example ids | yes | — |
| axis eligibility | yes | — |
| which fixture is shown | yes, by reference | — |
| layout, spacing, chrome | no | each renderer |
| interaction wiring | no | each renderer |
| element/node construction | no | each renderer |

Svelte is the reference implementation. React matches its structure and copy.
GPUI teaches the same component and the same representative states through
native composition; it is not required to reproduce web layout mechanics, but
the *evidence* it omits is a gap, not a style choice.

## Teaching Fixtures

Pages may reference a shared, named fixture instead of inventing data:

```text
fixtures/people/short-list          a handful of names and avatars
fixtures/files/changed-set          a small changed-file set
fixtures/audio/ppm-frame            one meter frame at a known level
```

A fixture reference names data only. It carries no behaviour, no callbacks, and
no rendering. Each runtime builds its own value for the name. Fixtures are a
convenience for consistency, not a transport.

## Bounded Renderer Adapters

A complex page may keep a renderer-owned adapter and publish only its outline —
tab set, section order, captions, example ids, axis eligibility. This is the
escape hatch that keeps the outline from growing into a scene language, and it
is expected for pages such as DockRegion, DataTable, and the audio family.

## What Would Make This Wrong

Stop and re-read `g14.026` if any of the following starts happening:

- the outline gains conditional expressions or computed values
- a runtime imports the outline at build or run time
- example ids become test selectors
- a section cannot be described without naming DOM, GPUI, or Jetstream objects
- the section list grows to cover every prop a component has

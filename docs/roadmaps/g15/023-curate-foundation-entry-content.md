# g15.023 — Overloaded Examples: foundation entry, content and status

Status: **awaiting operator live review** — PR #47 code gate accepted at
`dda190ed`; paired Svelte/React preview checkpoint remains
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Consumes: `g15.011` partial screening baseline
Depends on: `g15.016` and `g15.022` (complete)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`, and
the 11 component contracts named below

## Scope

Foundation pages where useful entry, content, and status stories have been
obscured by variant or state cross-products.

Catalogue families: `text-value-entry`, `actions-selection`,
`content-identity`, and `status-progress`.

### Pages this card owns (11)

- `Card`
- `DetailItem`
- `DragNumberField`
- `EmptyState`
- `Eyebrow`
- `Meter`
- `RefSelect`
- `Select`
- `Skeleton`
- `SplitButton`
- `TextInput`

This list is exact and exhaustive. It preserves the parent's original
partition even where caption restoration and axis placement have already
brought a page inside budget. No other card owns these pages, and this card
owns no others.

No component behavior or public API changes. The only contract edit permitted
is the narrow Meter specimen-definition correction described below.

## Remeasured Baseline

Counts are visible `Examples` captions on current `main` after `g15.022`.
Dedicated size and density panes are excluded.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| Card | 4 | 4 | 8 | keep paired web; trim native extras |
| DetailItem | 6 | 6 | 8 | keep paired web; trim native extras |
| DragNumberField | 8 | 8 | 8 | verified no-op; eight distinct contracted behaviors |
| EmptyState | 4 | 4 | 4 | verified no-op |
| Eyebrow | 4 | 4 | 3 | replace the stray status story and converge |
| Meter | 7 | 7 | 4 | remove the duplicated size matrix and add native ring teaching |
| RefSelect | 9 | 9 | 8 | combine related states and converge |
| Select | 8 | 8 | 11 | combine related modes and remove native matrices |
| Skeleton | 7 | 7 | 8 | keep paired web; trim one native extra |
| SplitButton | 7 | 7 | 10 | curate state/tone cross-products |
| TextInput | 7 | 7 | 16 | curate mode/state cross-products |

The earlier audit counted page shapes before caption and axis repair. This card
uses the current catalogue. DragNumberField and Skeleton may remain at eight
and seven sections: their contracts name distinct behaviors or presets rather
than a repeated size, density, or tone matrix.

## Target Teaching Outline

Use these sections in this order. Svelte and React captions and explanatory
copy stay verbatim. GPUI teaches the same ordered sections and meaning.
Several component instances may share a section when they answer one reader
question and remain visually distinct.

| Page | Target sections |
| --- | --- |
| Card | keep web: Default variant; Outlined variant; Elevated variant; Interactive |
| DetailItem | keep web: Inline layout (default); With description; With action slot; With value slot; Stacked layout; Surface presentation |
| DragNumberField | keep: Default; Integer step; Formatted dB; Coarse / fine drag (Shift); Direct entry (click); Keyboard bounds (Home / End); Negative range; Disabled |
| EmptyState | keep: Neutral; Search; First run; Compact custom visual |
| Eyebrow | Above a page title; Primitive category; Composite category; Semantic heading |
| Meter | Default usage; Threshold states; Custom range; Ring shape and readout; Ring tones |
| RefSelect | Branch and tag selection; Search and no matches; Loading and short-list search; Trigger presentation; Selection states |
| Select | Native select; Custom dropdown; Search and freeform entry; Rich and grouped options; Clearable selection; Disabled |
| Skeleton | keep web: Basic shapes; Preset: avatar-line; Preset: list-item (x3); Preset: table-row (x3); Preset: card; Preset: detail-section; Static (no animation) |
| SplitButton | Save split action; Secondary export; Intent tones; Loading and disabled states |
| TextInput | Default field; Validation and async availability; Search input; Prefix and suffix; Multiline; Disabled |

Use the multiplication sign already rendered by the Skeleton pages when
pinning their exact captions; the ASCII form above is only roadmap prose.

## Required Story Mapping

The reduction preserves these stories, not just the caption count.

- Card keeps the four contract specimens. GPUI-only selected, media,
  horizontal, and compact examples leave the catalogue only after their
  focused or contract evidence is named in the batch log.
- DetailItem keeps the six current web questions. GPUI's redundant
  simple-versus-surface and empty-value groups are removed only after the
  remaining evidence is named.
- DragNumberField is a true no-op. Its eight groups are the contract's eight
  distinct interaction and formatting stories, not an axis matrix.
- EmptyState is a true no-op. Preserve its three variants and compact custom
  visual across all active previews.
- Eyebrow teaches the contract's section label, primitive category, composite
  category, and semantic heading. The web-only status ribbon is not a public
  Eyebrow posture.
- Meter combines high and low threshold states in one section, keeps custom
  range, and teaches the ring's readout and tone domain. The `Ring sizes`
  matrix leaves `Examples`; the dedicated Sizes pane uses one ring
  representative per size so intrinsic ring scaling remains visible without a
  second matrix. GPUI must gain the supported ring stories rather than
  reducing web to its current linear-only page.
- RefSelect keeps grouped branches/tags, current-ref marking and descriptions;
  host search and no matches; loading and search-hidden short lists; outlined
  and subdued triggers; and no-selection plus disabled states. Retained search
  and selection examples remain host-driven and visibly live.
- Select keeps native and custom forms, search and freeform entry, rich snippet
  and grouped option rendering, clearing, and disabled state. Size and
  validation matrices stay outside `Examples`.
- Skeleton remains at seven because each public preset, the primitive shapes,
  and static animation posture are distinct. Remove GPUI's extra
  partial-width-lines group and preserve the paired-web order.
- SplitButton keeps a realistic save action first, secondary export,
  danger/success/warning in one tone row, and loading/disabled together.
  Primary and menu actions still update visible last-action feedback. Dropdown
  open, submit semantics, and constrained-scroll behavior move to named
  focused evidence rather than staying as catalogue-only claims.
- TextInput keeps ordinary entry; invalid, valid, and pending/async validation;
  search; affixes; multiline; and disabled. Icon, character-count, read-only,
  and repeated multiline permutations leave only after their focused evidence
  is named.

Focused component tests remain the exhaustive behavior authority. `Examples`
teaches representative use; it does not become a conformance corpus.

## Meter Specimen-Definition Correction

`docs/contracts/components/meter.md` section 13 currently requires a linear
size matrix and separately names `Ring sizes` in Examples. That duplicates the
same public axis and conflicts with the catalogue outline's one-representative-
per-step rule.

The worker may edit only Meter section 13 to make the dedicated Sizes pane use
one ring representative per size and to remove the standalone `Ring sizes`
Examples requirement. Keep all component semantics, public props, thresholds,
custom range, ring readout, and ring tone requirements unchanged.

## Goals

- [ ] Every page in the group meets the parent's method.
- [ ] Svelte and React stay identical; GPUI teaches the same ordered set.
- [ ] DragNumberField and EmptyState remain verified no-ops.
- [ ] Retained interactive examples have observable specimen effects.
- [ ] Removals are named, with contract coverage checked first.

## Evidence

- Add `test/parity/g15-023-foundation-entry-content-specimens.test.tsx` for
  this exact 11-page set.
- Assert final ordered captions, paired Svelte/React equality, the normal 3–6
  section budget, and the stated 7–9 exceptions for DragNumberField and
  Skeleton.
- Assert DragNumberField and EmptyState's verified no-op captions.
- Assert contract-critical stories beyond captions: Eyebrow's four semantic
  uses; Meter's threshold pairing, ring readout/tone teaching, and ring-based
  Sizes pane; RefSelect's host-driven search/selection states; Select's
  retained modes; SplitButton's visible action feedback; and TextInput's
  invalid, valid, pending, search, affix, multiline, and disabled evidence.
- Record final GPUI caption order for all 11 pages with deterministic
  structural evidence. Shared audio or scene-backed no-op pages may use their
  existing authored source rather than inventing a GPUI-only fixture.
  `g15.026` still owns the mounted native page probe.
- The August batch log maps every removed or combined caption to retained
  catalogue coverage, focused behavior evidence, or an explicit coverage gap.

## Acceptance

Per the parent, including its operator-review checkpoint: **every changed web
page is reviewed live in the Svelte and React previews before this card is
called complete.** Card, DetailItem, DragNumberField, EmptyState, and Skeleton
need verification, not web churn. Any unreviewed changed page remains an
explicit PR item.

## Writable Scope

Paired web curation is limited to:

- `packages/svelte/preview/src/specimens/EyebrowSpecimen.svelte`
- `packages/svelte/preview/src/specimens/MeterSpecimen.svelte`
- `packages/svelte/preview/src/specimens/RefSelectSpecimen.svelte`
- `packages/svelte/preview/src/specimens/SelectSpecimen.svelte`
- `packages/svelte/preview/src/specimens/SplitButtonSpecimen.svelte`
- `packages/svelte/preview/src/specimens/TextInputSpecimen.svelte`
- the six matching files under
  `packages/react/preview/src/gallery/specimens/`

Native convergence is limited to:

- `packages/gpui/preview/src/specimens/card_specimen.rs`
- `packages/gpui/preview/src/specimens/detail_item_specimen.rs`
- `packages/gpui/preview/src/specimens/eyebrow.rs`
- `packages/gpui/preview/src/specimens/meter.rs`
- `packages/gpui/preview/src/specimens/ref_select_specimen.rs`
- `packages/gpui/preview/src/specimens/select.rs`
- `packages/gpui/preview/src/specimens/skeleton.rs`
- `packages/gpui/preview/src/specimens/split_button.rs`
- `packages/gpui/preview/src/specimens/text_input.rs`

Evidence and documentation are limited to:

- `docs/contracts/components/meter.md`, section 13 only
- `test/parity/g15-023-foundation-entry-content-specimens.test.tsx`
- one August batch log

Do not edit component implementations, public types, shared specimen shells,
catalogue navigation, generated scene infrastructure, shared audio specimen
definitions, or pages owned by another child. Do not churn the five paired-web
pages already at their accepted target.

## Validation

- focused `g15.023` parity regression
- `effigy test:parity`
- `effigy catalogue:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run windowed, native-visual, conformance, Jetstream, or
release selectors.

## Stop Conditions

In addition to the parent stop conditions, stop if:

- curation needs a component behavior or public API change;
- any contract edit beyond the narrow Meter section 13 correction appears
  necessary;
- DragNumberField or EmptyState no longer matches its recorded no-op state;
- an eight-story DragNumberField or seven-story Skeleton page cannot remain
  clear without turning into a matrix;
- a retained RefSelect, Select, SplitButton, or TextInput control cannot be
  made visibly live with specimen-local state;
- Svelte and React need different section order, copy, or behavior;
- GPUI cannot teach the target outline without component work or the
  `g15.026` native page probe;
- work escapes the exact 11-page set or writable files.

## Continuation

Push one PR and stop for orchestrator review. Changed web pages require live
paired-preview operator review before merge. `g15.024` is the next curation
child; do not absorb it into this run.

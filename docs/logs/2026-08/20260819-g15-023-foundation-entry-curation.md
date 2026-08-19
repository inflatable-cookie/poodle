# g15.023 — Foundation Entry, Content and Status Curation (August batch log)

Date: 2026-08-19
Card: `docs/roadmaps/g15/023-curate-foundation-entry-content.md`
Parent: `docs/roadmaps/g15/018-overloaded-examples-curation.md`
Handoff: `docs/handoffs/20260819-201239-g15-023-foundation-entry-curation.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-eea8d8fa`
Branch: `t3code/curate-foundation-entry-pages`
Accepted head: pending orchestrator closeout
PR head: review-round 1 on this branch; first review saw `e87e6773`
Worker base: `87ee87da2ef6e1525e1b941de94ee3a83ccd6037` (`origin/main` at
dispatch; handoff planning base `aa451297961be3fd98e3c038774af7f5151d9eed`
confirmed as an ancestor)

## Summary

Eleven foundation entry, content, and status pages re-measured before
editing. The committed baseline matched the card. Six paired-web pages were
curated; Card, DetailItem, and Skeleton kept their accepted web outlines and
only had native extras trimmed; DragNumberField and EmptyState stayed verified
no-ops. Meter section 13 lost the duplicated ring-size Examples matrix and now
uses one ring representative per size in the Sizes pane.

## Change class

- **Change class:** specimen curation, plus the card-authorized Meter
  section 13 wording correction
- **Packages touched:** `poodle-svelte` preview, `poodle-react` preview,
  `poodle-gpui-preview`
- **Public entry points:** none
- **Downstream re-check:** none — no public surface changed
- **app_state.rs:** unused

## Baseline recount at the worker base

Matched the card's remeasured table.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| Card | 4 | 4 | 8 | keep paired web; trim native extras |
| DetailItem | 6 | 6 | 8 | keep paired web; trim native extras |
| DragNumberField | 8 | 8 | 8 | verified no-op |
| EmptyState | 4 | 4 | 4 | verified no-op |
| Eyebrow | 4 | 4 | 3 | replace the stray status story and converge |
| Meter | 7 | 7 | 4 | remove the duplicated size matrix and add native ring teaching |
| RefSelect | 9 | 9 | 8 | combine related states and converge |
| Select | 8 | 8 | 11 | combine related modes and remove native matrices |
| Skeleton | 7 | 7 | 8 | keep paired web; trim one native extra |
| SplitButton | 7 | 7 | 10 | curate state/tone cross-products |
| TextInput | 7 | 7 | 16 | curate mode/state cross-products |

Counts are captioned examples in the `Examples` pane, excluding size and
density axis panes. EmptyState web captions come from the generated scene.
DragNumberField GPUI captions come from the shared audio specimen source.

## After

| Page | Svelte | React | GPUI |
| --- | ---: | ---: | --- |
| Card | 4 | 4 | 4 |
| DetailItem | 6 | 6 | 6 |
| DragNumberField | 8 | 8 | 8 |
| EmptyState | 4 | 4 | 4 |
| Eyebrow | 4 | 4 | 4 |
| Meter | 5 | 5 | 5 |
| RefSelect | 5 | 5 | 5 |
| Select | 6 | 6 | 6 |
| Skeleton | 7 | 7 | 7 |
| SplitButton | 4 | 4 | 4 |
| TextInput | 6 | 6 | 6 |

Svelte and React captions are verbatim identical on every page. GPUI teaches
the same ordered intent. Two no-op pages keep their existing shortened native
captions: EmptyState `Compact` for web `Compact custom visual`, and
DragNumberField drops the parenthetical interaction hints.

## Final ordered captions

**Card** — Default variant; Outlined variant; Elevated variant; Interactive

**DetailItem** — Inline layout (default); With description; With action slot;
With value slot; Stacked layout; Surface presentation

**DragNumberField** (unchanged) — Default; Integer step; Formatted dB;
Coarse / fine drag (Shift); Direct entry (click); Keyboard bounds (Home /
End); Negative range; Disabled. GPUI shortens the four interaction captions.

**EmptyState** (unchanged) — Neutral; Search; First run; Compact custom
visual. GPUI keeps `Compact`.

**Eyebrow** — Above a page title; Primitive category; Composite category;
Semantic heading

**Meter** — Default usage; Threshold states; Custom range; Ring shape and
readout; Ring tones

**RefSelect** — Branch and tag selection; Search and no matches; Loading and
short-list search; Trigger presentation; Selection states

**Select** — Native select; Custom dropdown; Search and freeform entry; Rich
and grouped options; Clearable selection; Disabled

**Skeleton** — Basic shapes; Preset: avatar-line; Preset: list-item (×3);
Preset: table-row (×3); Preset: card; Preset: detail-section; Static (no
animation)

**SplitButton** — Save split action; Secondary export; Intent tones; Loading
and disabled states

**TextInput** — Default field; Validation and async availability; Search
input; Prefix and suffix; Multiline; Disabled

## Named removals and combinations

- **Eyebrow — "Status ribbon".** Not a public Eyebrow posture. Replaced with
  the contract's composite category (`Composite` / `DataTable`). Status-ribbon
  teaching is not a public prop; no coverage gap.
- **Card GPUI — Selected; Media slot; Horizontal layout; Compact layout.**
  Focused `Card.test.ts` still projects `selected`, `layout="horizontal"`,
  media regions, and interactive data attributes. Contract §13 never required
  those four as catalogue groups.
- **DetailItem GPUI — "Simple vs surface presentation"; "Empty value
  (em-dash)".** Surface presentation remains as its own section. Empty-value
  fallback stays in `DetailItem.test.ts` (`emptyText` / em-dash). The first
  native group was renamed to match the web inline default.
- **Skeleton GPUI — "Partial-width lines".** Not a public preset. Primitive
  line/circle/block shapes remain in Basic shapes.
- **Meter — "With thresholds" + "Low value (optimal range)" → "Threshold
  states".** Both meters stay visible. **"Ring sizes"** left Examples; the
  Sizes pane now shows one ring per size. **"Default (50%)"** became
  **Default usage**; **"Custom range (0–500)"** became **Custom range**;
  **"Ring"** became **Ring shape and readout**.
- **RefSelect — nine groups → five.** Host-driven search + no matches share
  one section; loading + short-list (`searchable=false`) share one; outlined +
  subdued share trigger presentation; empty + disabled share selection states.
  Live selected-ref feedback stays on the first group.
- **Select — Searchable + Freeform → "Search and freeform entry"; Snippet
  rendering + Grouped → "Rich and grouped options".** GPUI dropped the
  in-Examples validation matrix and size matrix (those axes stay in dedicated
  panes) plus the extra "Searchable with groups" group. Native select and
  custom dropdown remain first.
- **SplitButton — Danger/Success/Warning → "Intent tones"; Loading + Disabled
  → "Loading and disabled states".** GPUI dropped the variant × tone matrix,
  standalone open-menu group, submit-semantics group, and constrained-scroll
  group. Last-action feedback stays visible as uncaptioned host copy.
  Dropdown/outside-dismiss remains in `SplitButton.test.ts`.
- **TextInput — "With validation" + "Slug" → "Validation and async
  availability".** Invalid/valid email and async slug stay live. The slug
  example no longer carries `prefix="/projects/"` — that prefix made
  `prefix + value` fail built-in slug-format validation before the custom
  async check ran. Prefix/suffix teaching stays in its own section. Review
  round 1 pins the pending → unavailable path on
  `.poodle-text-input[data-validation-state]` in both web runtimes.
  GPUI dropped icon, character-count, read-only, suffix-only, and repeated
  multiline groups. Character count remains on the retained multiline
  example. Leading icon, trailing icon, and read-only leave the catalogue
  because `g15-023-foundation-entry-content-specimens.test.tsx` now asserts
  them in both web runtimes: `data-has-leading` / `data-icon="search"`,
  `data-has-trailing` / `data-icon="x-circle"`, and the `readonly`
  attribute.

## Contract coverage

Preserved. Meter section 13 is the only contract edit: Sizes now uses one
ring representative per size, Examples no longer require a standalone
`Ring sizes` matrix, and high/low threshold states share one section. All
other Meter semantics are unchanged. No component implementation, public
prop, or token changed. No behaviour lost its only visible evidence without
a named disposition above.

## Pages intentionally left unchanged

- **DragNumberField (8)** — contract-named interaction and formatting
  stories, not an axis matrix.
- **EmptyState (4)** — Neutral, Search, First run, and compact custom visual
  already match the target. Web is scene-authored; GPUI keeps its existing
  shortened `Compact` caption.
- **Card, DetailItem, Skeleton paired web** — already at the accepted
  outline. Only GPUI was trimmed.

## Changed files

- `packages/svelte/preview/src/specimens/{Eyebrow,Meter,RefSelect,Select,SplitButton,TextInput}Specimen.svelte`
- `packages/react/preview/src/gallery/specimens/{Eyebrow,Meter,RefSelect,Select,SplitButton,TextInput}Specimen.tsx`
- `packages/gpui/preview/src/specimens/{card_specimen,detail_item_specimen,eyebrow,meter,ref_select_specimen,select,skeleton,split_button,text_input}.rs`
- `docs/contracts/components/meter.md` (section 13 only)
- `test/parity/g15-023-foundation-entry-content-specimens.test.tsx`
- this log

## Validation

- focused `g15-023` parity regression: 75 passed after review-round 1
  (74 at first push)
- `effigy test:parity`: 7 files, 439 passed at first push; focused
  regression re-run for this round
- `effigy catalogue:check`: passed
- `effigy check:svelte`: 0 errors
- `effigy react:build`: passed
- `effigy check:gpui`: passed
- `effigy docs:check`: passed
- `git diff --check origin/main...HEAD`: passed

Headless only. No windowed, native-visual, conformance, Jetstream, or
release selectors.

## Operator review

Pending. The six changed web pages need live Svelte and React preview review
before the card can close: Eyebrow, Meter, RefSelect, Select, SplitButton,
and TextInput. Card, DetailItem, DragNumberField, EmptyState, and Skeleton
need verification only.

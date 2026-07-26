# g12.013 — Native Spec-Surface Parity

**Status: active.** Gate landed; mechanical burn-down finished (93 → 19).
The 19 that remain need a contract decision, not a field — see below.

## Problem

Asked whether GPUI and Jetstream had tracked the g12-era Svelte work, the honest
answer was that nothing could measure it.

Component *coverage* was never the issue — a module-level diff shows both native
targets implement essentially the whole set, and the apparent holes are naming
(`time_input` / `time_field`, `status_bar` / `shell_status_bar`, `box` / `bx`).
The gap was one level down: **props**.

Both native targets read their props from one place, the `poodle-specs` crate. A
prop that lands in the contract and in the Svelte component but never reaches the
Spec struct is invisible to both natives — and no gate could see it, because:

- `contract-prop-drift.ts` compares the contract to **Svelte** only
- the parity gate compares **Svelte to React** — two web targets
- the visual gate diffs **Svelte against React** — two web targets
- `test:contracts` runs the spec crates' own unit tests, which cannot know what
  a contract documents

So every existing gate could be green while a documented prop never reached the
native surface at all.

## The Gate

`packages/svelte/preview/scripts/contract-spec-drift.ts`, wired into `docs:lint`
and available standalone as `effigy docs:spec-drift`.

It compares each contract's "### Public Props" table against the fields of the
matching `<Name>Spec`, and has to be smarter than a name match to be useful:

- **follows composition** — `ContextMenuSpec` holds a `MenuSpec`, so `items` is
  carried one level down. A top-level-only check reported gaps that were not there
- **normalises spelling** — camelCase → snake_case, and the `is_` / `has_` /
  `show_` / `shows_` prefixes Rust booleans take
- **`WEB_ONLY_PROPS`** — props that never reach a Spec by design: styling escape
  hatches (`className`, `style`), raw HTML attributes (`type`, `form*`,
  `spellcheck`), ARIA-by-DOM-id (`describedBy`), the rendered element (`as`,
  `asRole`), JS callbacks and timings (`validate*`, `debounce`)
- **`ALIASES`** — per-component, where the two deliberately differ: the contract's
  `items` is `TabsSpec::tabs`, `ToastStackSpec::toasts`,
  `CardRadioGroupSpec::options`; `IconSpec` still stores `name` because 229 native
  call sites construct by it

## Findings

**146 documented props missing across 53 components.** Of those, 53 were web-only
plumbing. **93 were real gaps** — props a native target simply could not render.

The worst were systematic rather than exotic:

| Missing | Components | Why it matters |
|---------|-----------|----------------|
| `ariaLabel` | 14 | 87 specs already had it; these 14 had no accessible name at all |
| `size` / `sizeRole` / `density` | 5 | the orthogonal presentation axes CLAUDE.md mandates — a native cannot resolve control height without them |
| `items` | 4 | `EditableListSpec` carried `item_count: usize` — enough to size a list, not to draw one |

## Burn-Down

Closed so far (42 props):

- `aria_label` + `with_aria_label` on AudioPlayer, Code, ColorPicker,
  DurationInput, EditableLabel, HoverCard, ListContainer, PageLoading, Pill,
  Rating, SplitView, ToastStack, TriStateSwitch, VideoPlayer
- presentation axes on EmbedInput, Icon, ListCard, Pill, Spinner
- `EditableListItem` + `items` on EditableListSpec, with `add_placeholder`,
  `has_embedded_handle`, `shows_workflow_chrome`
- `lower_value_text` / `upper_value_text` on RangeSlider (`aria-valuetext` per
  thumb — without it a screen reader reads the raw number)
- `page_size` / `has_chrome` on Pagination, `total_pages` on PaginationSummary

Third slice — the straightforward data props: `selectionMode` (Accordion),
`workingLabel` (AlertDialog), `selectAllLabel` (BulkActionBar), `columns`
(CardRadioGroup), `showCopyButton` (Code), `defaultPressed` (IconButton),
`selectionCount` (PickerShell), `captionsSrc` / `showCaptions` (VideoPlayer),
`showValidationStatus` (TextInput), `emptyVariant` (ListContainer),
`triggerAriaLabel` (Menu), `showTooltips` / `collapseWhenOverflow` /
`collapseLabel` (Tabs), and `hours` / `minutes` / `seconds` (DurationInput).

Three more turned out to be spelling, not absence, and are recorded as aliases:
`dialog.kind` is the deprecated name for `role`, `code.source` is the Spec's
`content`, and `editable-list.addPlaceholder` is its `placeholder`.

**A false positive the gate had been hiding.** `duration-input.seconds` looked
covered because `show_seconds` exists — the `show_` normalisation was matching
any prop against a `show_`-prefixed field. Tightened so only a prop that already
reads as a "show" toggle may match that spelling, which immediately surfaced
`seconds` as the real gap it was. The spec stored only the formatted `value`;
the three numbers a host actually binds are now on it, with `total_seconds()`
mirroring Svelte's `durationTotalSeconds`.

Fourth slice — the last of the ordinary ones: `adaptiveWidth` / `dot` / `title`
(Pill), the context-menu trio (ListCard), `posture` /
`showSubtitleWithBreadcrumbs` (PageHeader), `size` (EmptyState). Four more were
spelling: `pill.accent` is `accent_color`, `time-ago.datetime` is `timestamp`,
`block-editor.blockTypeItems` is `block_types`, and `tri-state-switch.options`
is decomposed into one `*_label` field per state.

Neither native draws a ListCard context menu yet. The spec carrying the entries
is the precondition for that, not the feature — a renderer had nothing to read
before.

**Loose normalisation removed.** The checker also tried plural/singular variants
(`items` ↔ `item`). An audit showed they matched nothing once the real gaps
closed, so they are gone: a rule that covers no case but can still fire is only
a way to hide the next `seconds`.

Remaining: **19 props across 7 components**, held in the gate's `OPEN_GAPS`
baseline. That is debt, not an allowlist — adding an entry means a prop shipped
to the web without reaching the shared spec surface, which is the thing the gate
exists to stop.

## Shared Rule De-duplication

Found while wiring `sizeRole`: `resolve_semantic_size` — the role-to-size shift
that must match Svelte's `resolveSemanticControlSize` exactly — existed in **five
private copies**: `markdown_editor.rs`, `app_header.rs` and `sidebar_nav.rs` in
`poodle-specs`, plus both native `presentation/metrics_a.rs` modules. All five
happened to agree; nothing made them.

Promoted to `poodle_specs::resolve_semantic_control_size`; the other four now
delegate. The full 15-cell table is pinned by a test — a spot check would not
have caught one copy drifting, which is the failure mode five copies invite.

`sizeRole` is now wired for real, not just carried: `IconSpec::resolved_size()`
and `SpinnerSpec::resolved_size()` apply the shift, and `size_token()` routes
through it, so an icon in chrome no longer out-sizes the control it sits in.
Icon clamps sooner than the control scale because it has three stops, not five.

## Renderer Pass

Adding a field makes a prop reachable; it does not make either target draw it.
First slice, both targets:

- **Spinner** — all five size-derived accessors (`size_px`, `ring_size_rem`,
  `grid_*_rem`, `cell_radius_rem`) read `resolved_size()` instead of `size`.
  One change in the spec, so both renderers honour the role without either
  being touched.
- **Pill** — both renderers switched from `spec.size` to `spec.resolved_size()`.
  `PillSpec::resolved_size()` reuses the shared five-stop shift.
- **Icon** — already routed through `size_token()`, which now resolves the role,
  so both targets picked it up for free.
- **EditableList** — Jetstream drew `"Item 1"`, `"Item 2"`: placeholder text
  standing in for content the spec gave it no way to reach. It now renders the
  real labels, honours `embedded_handle` (no second grip when the host draws its
  own), and counts rows from the items. GPUI already kept labels on its builder;
  `items()` now mirrors them onto the spec, so a spec round-trip no longer drops
  them.

Second slice — the cases where a prop was carried and actively ignored:

- **Pagination chrome.** Both natives keyed the container treatment off
  `standalone`, the *deprecated* inverse alias, which defaulted to `false` and
  so drew chrome **on** by default — the opposite of the contract and of
  Svelte. `standalone` is now `Option<bool>` (the contract types it
  `boolean | undefined`, and it may only override when the host actually set
  it), and `resolved_chrome()` implements the documented precedence. Both
  renderers read it. **This changes default native rendering**, which is the
  point: it was wrong.
- **PaginationSummary copy.** Three targets had drifted to three strings —
  Svelte `Showing 26-50 of 67`, GPUI the same with an en dash plus a
  `"No items"` empty case Svelte does not have, Jetstream a spaced en dash.
  Built once in the spec as `summary_text()`, with `accessible_label()` adding
  the page count exactly as Svelte's `aria-label` does. Both renderers read it.
- **ListCard leading box.** `leading_size_rem()` derived from `leading_shape`
  alone, so a card drew the same box whatever `size` the host asked for — the
  prop was carried and ignored. It now walks the `data-leading-size` ladder from
  `list-card.css` (`resolved_leading_size()` = resolved size + offset, clamped),
  with the box, icon and font rows all transcribed and pinned by test. Both
  renderers already called the accessor, so both picked it up.
- **EmbedInput axes.** The nested `TextInput` was constructed without them, so
  it always rendered at the default size. It now inherits the composite's
  resolved size and density in both targets.

`add_placeholder` was reverted — the spec's existing `placeholder` already IS
the contract's `addPlaceholder`, and two fields for one thing is worse than the
alias now recorded in the gate.

## Not Done

- **`aria_label` reaches no accessibility API in either target.** Traced: 95
  reads in GPUI, 8 in Jetstream, and every one stores the field or forwards it
  to another component's field. It terminates in a struct. So the 14 specs that
  gained it are consistent with the 87 that had it, and equally inert at
  runtime. Native accessibility here is artifact-backed
  (`native-accessibility-proof.json`), not live. Fixing that needs a decision
  about GPUI/Jetstream accessibility APIs, not more spec fields.
- **The remaining 19, which are a question rather than a task.** They fall into
  three groups, and none should be closed by adding a field without deciding
  first:

  | Group | Props | The question |
  |-------|-------|--------------|
  | Layout | `stack` width/height/min/overflow (5), `split-view` collapse breakpoints (4) | These read as `poodle-layout`'s job, not a component spec's. `LayoutIntent` already exists; do these belong there? |
  | CSS length strings | `popover.surfaceMinWidth` / `surfaceMaxWidth`, `detail-section.itemMinColumnWidth`, `filter-toolbar.minItemWidth`, `button.maxWidth` | All are raw CSS strings (`"10rem"`, `min(24rem, 90vw)`). A native needs a resolved unit, so this needs a `Dimension`-style type rather than a `String` field |
  | Not data | `select.loadOptions` (async callback), `select.native` (renders a platform `<select>`), `button.fit` / `truncate` (both intrinsic-sizing CSS) | Arguably `WEB_ONLY_PROPS`, but each deserves a decision recorded rather than a quiet exclusion |

## Verification

- `effigy ci`, `ci:rust`, `ci:native` all green
- 182 spec-crate tests pass (up 20 across the card: the size-role table and
  clamps, Icon/Spinner role shifts, EditableList item labels, the pagination
  chrome precedence, the summary strings, the ListCard leading ladder and
  context-menu defaults, and the DurationInput segments)
- the gate is green against its own baseline, and fails on new drift

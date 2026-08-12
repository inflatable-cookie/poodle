# RefSelect

Status: detailed contract
Updated: 2026-07-25

## 1. Purpose

- Component name: `RefSelect`
- Layer: `composites`
- Summary: a compact anchored chooser for a version-control ref — a branch, tag
  or commit — with a search field, a marker on the checked-out ref, and an
  async-friendly loading footer
- In scope: a host-supplied ref list with optional kind/description/group, a
  search field that filters locally or defers to the host, a "current" marker,
  loading and empty states, controlled/uncontrolled selection, a trigger that
  matches `ModelPicker`'s composer treatment (`variant`, `emphasis`)
- Out of scope: git itself — no fetching, no ref parsing, no ahead/behind maths,
  no checkout, no pagination policy. The host owns the ref list and what
  selecting one means

Sits beside `ThemeSelect` and `TimeZoneSelect`: a domain-shaped select built from
generic parts. Its home is the `AgentChatInput` footer, where an agent's working
ref belongs, but it stands alone in any toolbar.

Not `RelationPicker` (a drill-down browse-and-confirm workflow) and not
`PickerShell` (a titled panel with header, meta and footer regions) — both are
far heavier than a toolbar dropdown.

## 2. Anatomy

```text
[Root .ref-select] <div>  (trigger wrapper; carries data-size/data-density/data-open/data-disabled/data-variant/data-emphasis)
  ├── [Trigger .ref-select__trigger] <button aria-haspopup="dialog" aria-expanded aria-controls>
  │   ├── [Icon .ref-select__icon] Icon  (the selected ref's kind glyph)
  │   ├── [Label .ref-select__label] <span>  (selected ref label, or `placeholder`)
  │   └── [Chevron .ref-select__chevron] <span aria-hidden="true">
  └── [Surface .ref-select__surface] <div role="dialog" tabindex="-1"> (rendered inline when open)
      ├── [Search .ref-select__search] <div>  (conditional: searchable)
      │   └── TextInput (type="search", ariaLabel from `searchLabel`)
      ├── [List .ref-select__list] <div role="listbox">
      │   ├── [Group .ref-select__group] <span> (repeated; conditional: option has `group`)
      │   └── [Option .ref-select__option] <button role="option" aria-selected> (repeated)
      │       ├── [Option Icon .ref-select__option-icon] Icon (conditional)
      │       ├── [Option Text .ref-select__option-text] <span>
      │       │   ├── [Option Label .ref-select__option-label] <span>
      │       │   └── [Option Description .ref-select__option-description] <span> (conditional)
      │       └── [Option Marker .ref-select__option-marker] <span> (conditional: option is `currentRef`)
      ├── [Empty .ref-select__empty] <p> (conditional: no matches and not loading)
      └── [Loading .ref-select__loading] <p role="status"> (conditional: loading)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | trigger wrapper carrying every presentation data attribute | — |
| Trigger | yes | opener showing the ref's kind glyph, its label and a chevron; `aria-haspopup="dialog"` | `--poodle-size-control-height`, `--poodle-radius-control` |
| Icon | no | glyph for the selected ref's kind (`git-branch`, `tag`, `git-commit`), or the ref's own `icon` | `--poodle-color-text-secondary` |
| Label | yes | selected ref label, or `placeholder` when nothing is selected (`data-placeholder="true"`) | `--poodle-color-text-primary` / `--poodle-color-text-muted` |
| Chevron | yes | popover indicator (`▾`) | `--poodle-color-text-secondary` |
| Surface | yes | anchored `role="dialog"` panel, portalled to the theme root (`002-anchored-overlays.md`); opens upward by default and flips down when there is no room above | `--poodle-overlay-z-menu`, `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-elevation-overlay` |
| Search | no | a `TextInput` in search mode; filters locally unless the host takes over | (TextInput contract) |
| List | yes | `role="listbox"` of matching refs, height-capped and scrolling | — |
| Group | no | heading emitted before the first option of each `group` run | `--poodle-color-text-secondary` |
| Option | yes | one ref row; `role="option"`, `aria-selected`, `data-selected`, `data-disabled`, `data-current` | `--poodle-radius-control` |
| Option Marker | no | the "current" marker on the checked-out ref | `--poodle-color-text-secondary` |
| Empty | no | placeholder text when the query matches nothing | `--poodle-color-text-secondary` |
| Loading | no | `role="status"` footer used while more refs are arriving | `--poodle-color-text-secondary` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `refs` | `RefOption[]` | `[]` | yes | host-supplied refs |
| `value` | `string` | `""` | no | selected ref; controlled when bound |
| `currentRef` | `string \| null` | `null` | no | the checked-out ref, marked in the list. Often equals `value`, but a host browsing another ref keeps the marker where it belongs |
| `currentLabel` | `string` | `"current"` | no | marker text |
| `placeholder` | `string` | `"Select ref"` | no | trigger label when nothing is selected |
| `searchable` | `boolean` | `true` | no | render the search field |
| `searchValue` | `string \| null` | `null` | no | controlled query. When supplied the component stops filtering — the host owns which refs to pass |
| `searchPlaceholder` | `string` | `"Search refs…"` | no | search field placeholder |
| `searchLabel` | `string` | `"Search refs"` | no | search field accessible name |
| `loading` | `boolean` | `false` | no | show the loading footer |
| `loadingLabel` | `string` | `"Loading more refs…"` | no | loading footer text |
| `emptyLabel` | `string` | `"No refs found"` | no | empty-state text |
| `ariaLabel` | `string` | `"Ref"` | no | accessible name for the trigger and dialog |
| `disabled` | `boolean` | `false` | no | disables the trigger and the panel |
| `variant` | `"bare" \| "outlined"` | `"bare"` | no | matches `ModelPicker`: borderless inline trigger, or the standard bordered control |
| `emphasis` | `"default" \| "subdued"` | `"default"` | no | `subdued` dims the trigger so it recedes in a composer footer; hover, focus and open restore it |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `dismissOnOutsideInteract` | `boolean` | `true` | no | outside dismissal: a mousedown outside the trigger and dialog closes it |
| `onChange` | `((value: string) => void) \| null` | `null` | no | fires when a ref is chosen |
| `onSearchChange` | `((query: string) => void) \| null` | `null` | no | fires on every keystroke in the search field, for host-driven querying |

### Shared Types

Defined in `@inflatable-cookie/poodle-svelte` `types.ts`, re-exported from the package root,
redefined identically in `@inflatable-cookie/poodle-react`, mirrored in `poodle-specs` (snake_case).

```typescript
type RefKind = "branch" | "tag" | "commit";

type RefOption = {
  value: string;
  label: string;
  /** Drives the default glyph. Unknown kinds fall back to the branch glyph. */
  kind?: RefKind;
  /** Secondary line — a short sha, an ahead/behind summary, a commit subject. */
  description?: string;
  /** Overrides the kind glyph. */
  icon?: string;
  group?: string;
  disabled?: boolean;
};
```

### Controlled And Uncontrolled

- Controlled: bind `value`; edits are mirrored through `onChange`
- Uncontrolled: omit `value` and the component owns the selection
- Search is uncontrolled unless `searchValue` is supplied; `onSearchChange` fires
  either way, so a host can start fetching without taking over the field

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no ref selected | trigger shows `placeholder` in muted text |
| selected | a ref is selected | trigger shows the kind glyph and the ref label |
| open | trigger activated | anchored dialog with the search field focused |
| current | option matches `currentRef` | row carries `data-current="true"` and the marker text |
| filtered | query non-empty, local filtering | only matching rows render; group headings follow the surviving rows |
| no matches | query matches nothing, not loading | the empty message replaces the list |
| loading | `loading=true` | the loading footer renders under the list, and the empty message is suppressed — results may still be arriving |
| disabled | `disabled=true` | trigger disabled, root at disabled opacity |
| subdued | `emphasis="subdued"` | trigger dimmed at rest, restored on hover, focus and open — colour and opacity only, never weight |

### Filtering

`filterRefs(refs, query)` is the cross-renderer semantic reference:

- an empty query returns every ref
- otherwise a case-insensitive substring match against `label`, then `value`, then
  `description` — a user typing a sha or a path fragment finds the row
- when `searchValue` is supplied the component does **not** filter: the host is
  driving, and the passed list is already the answer

### Behavior Machine

Behavior classification: `styled-only (no machine)` — adapter-owned interaction.
The popover uses the shared dismissable-layer stack (outside dismissal guarded
by `dismissOnOutsideInteract`, default `true`); selection and query are
plain component state. Pure helpers (`filterRefs`, `refKindIcon`,
`groupHeadingFor`) live in `ref-select-model.ts` and as methods on
`RefSelectSpec`.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onChange` | a ref is chosen | `string` (ref value) | choosing closes the popover — selecting a ref is the terminal action here, unlike `ModelPicker` where axes follow |
| `onSearchChange` | every keystroke in the search field | `string` | fires whether or not `searchValue` is supplied |

## 6. Accessibility

| Element | Attribute | Value |
|---------|-----------|-------|
| Trigger | `aria-label` | `"{ariaLabel}: {selected label}"` |
| Trigger | `aria-haspopup` / `aria-expanded` / `aria-controls` | dialog wiring |
| Surface | `role` / `aria-label` / `tabindex` | `"dialog"` / `ariaLabel` / `"-1"` |
| List | `role` | `"listbox"` |
| Option | `role` / `aria-selected` | `"option"` / `"true"` on the selected ref |
| Option Marker | (text) | `currentLabel`, read as part of the option's name |
| Loading | `role` | `"status"` — arriving refs are announced without stealing focus |

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter`/`Space` on trigger | toggles the popover |
| `ArrowDown`/`ArrowUp` | moves through the filtered options, wrapping; works from the search field so typing and choosing are one gesture |
| `Enter` on an option | selects it and closes |
| `Escape` | dismisses and returns focus to the trigger |

Focus enters the search field on open (or the selected option when the field is
hidden), so a keyboard user can type immediately.

## 7. Layout

### Sizing

- Root: `display: inline-flex`
- Surface: `min-width: 16rem`, `max-width: min(24rem, 90vw)`. Portalled and
  viewport-positioned per `002-anchored-overlays.md`, requesting `top-start`
  with an `8px` offset — the same rule as `ModelPicker`: upward by default,
  flipped down by the resolver when there is no room above. It publishes the
  coarse side as `data-placement="top" | "bottom"` and carries its own
  `data-size` / `data-density`
- List: `max-height: 14rem`, vertical scroll; rows never shrink (a height-capped
  scroller squashes shrinkable rows below their own content)

### Composition

- parent expectations: the `AgentChatInput` footer, repository toolbars, status bars
- child expectations: `TextInput` (search), `Icon`

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Trigger | `--poodle-size-control-height`, `--poodle-radius-control` | size-stepped opener |
| Trigger (`outlined`) | `--poodle-color-background-surface`, `--poodle-color-border-default` | fill + border |
| Trigger hover | `color-mix(surface 84%, elevated)` | hover fill |
| Trigger focus | `--poodle-color-accent-focusRing`, `--poodle-border-width-focus` | focus ring |
| Label | `--poodle-color-text-primary` | selected ref |
| Label (placeholder) | `--poodle-color-text-muted` | empty state |
| Icon / Chevron | `--poodle-color-text-secondary` | glyphs |
| Surface | `--poodle-overlay-z-menu`, `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-elevation-overlay` | anchored panel |
| Option hover | `color-mix(accent-base 14%, transparent)` | row hover |
| Option (selected) | `--poodle-color-text-primary` | selected row text |
| Option Marker | `--poodle-color-text-secondary` | "current" marker |
| Empty / Loading | `--poodle-color-text-secondary` | state messages |
| Disabled | `--poodle-state-opacity-disabled` | disabled opacity |

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` / `data-density` | Root | size and density ladders |
| `data-variant` | Root | `"bare"`, `"outlined"` |
| `data-emphasis` | Root | `"default"`, `"subdued"` |
| `data-open` / `data-disabled` | Root | `"true"` / `"false"` |
| `data-placement` | Surface | `"top"` (default) / `"bottom"` when flipped |
| `data-placeholder` | Label | `"true"` when nothing is selected |
| `data-selected` / `data-current` / `data-disabled` | Option | `"true"` / `"false"` |
| `data-kind` | Option | the ref's kind, when set |

## 9. Svelte Notes

- owns its open state and anchored surface directly (mirrors `ModelPicker`)
- `registerDismissLayer` from `@inflatable-cookie/poodle-core`
- arrow-key roving is a plain `keydown` handler shared by the search field and
  the option rows
- pure logic from `./ref-select-model`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::ref_select`
- the render is a faithful function of the full spec state: `RefSelectSpec`
  carries `is_open` and `search_value`, so the native targets render the filtered
  list, the current marker, and the empty or loading footer
- no ARIA API, and the preview does not drive clicks — the shared render-only
  posture

## 10a. Jetstream Notes

- `RefSelect::from_spec(spec, theme).on_change(...)`, carrying the chosen ref's
  value.
- No `on_search_change`: the filter is a text field and this runtime raises no
  key events.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `refs` accepted; group headings emitted per `group` run
- [ ] `filterRefs` matches: empty query passes everything; otherwise label, then
      value, then description, case-insensitively
- [ ] a supplied `searchValue` disables local filtering
- [ ] `currentRef` marks its row independently of `value`
- [ ] choosing a ref closes the popover and fires `onChange`
- [ ] loading suppresses the empty message
- [ ] disabled suppresses all interaction

### Tier 2: Visual Parity

- [ ] trigger anatomy (kind glyph, label, chevron) and the `bare`/`outlined` split
- [ ] `subdued` emphasis dims at rest and restores on approach, without changing
      any layout metric
- [ ] surface anchoring (upward, flipping down), radius, elevation
- [ ] option rows never shrink inside the capped, scrolling list
- [ ] all five sizes and three densities

### Tier 3: Implementation Freedom

- [ ] popover/animation/portal behavior is platform-owned
- [ ] search input internals are the `TextInput` primitive's business

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI/Jetstream render from spec state and don't drive typing or clicks | shared render-only posture across all native components | accepted | host wires interaction |
| Native `subdued` renders at its resting strength only | hover/focus are web states | accepted | matches `ModelPicker` |
| No pagination or infinite scroll | the host owns fetch policy; `loading` is the only affordance Poodle offers | accepted (by design) | revisit if a consumer needs sentinel-based paging |

## 13. Approval And Adoption Notes

- contract status: `implemented`
- approvers: pending review
- downstream adopters: `AgentChatInput` footer, Loophole, Underlay apps
- future follow-up: a create-ref affordance ("New branch from…") if a consumer
  needs it

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): default with branches
and tags grouped; the current ref marked while another is selected; a query that
filters; a query with no matches; loading footer; host-driven search
(`searchValue` supplied); search hidden (`searchable=false`); descriptions
(short shas); no selection (placeholder); disabled; `outlined` variant; `subdued`
emphasis; full size ladder; density variants.

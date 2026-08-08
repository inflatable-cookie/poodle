# ModelPicker

Status: detailed contract
Updated: 2026-07-24

## 1. Purpose

- Component name: `ModelPicker`
- Layer: `composites`
- Summary: one cohesive popover widget that selects an engine/model *and* the
  capability axes that belong to it (effort levels, speed modes, context sizes,
  boolean toggles), presented as a single trigger with a combined summary
- In scope: a host-supplied model list with optional descriptions/badges/icons,
  host-declared capability axes (single-select or toggle) declared once by key,
  per-model axis references with optional overrides, a combined trigger summary,
  a popover holding the model list plus one section per axis the selected model
  exposes, controlled/uncontrolled selection
- Out of scope: vendor vocabulary (no built-in "reasoning", "fast mode",
  "context window" semantics), model availability/pricing/quota lookups,
  network calls, persistence, request construction

The companion pattern to `FilterBuilder`: Poodle understands *models* and
*axes*; the host owns what those mean. An app that wants "model + reasoning +
fast mode" declares two axes; an app that wants "voice + accent" declares two
different ones. Nothing about the component changes.

The axis configuration is **keyed**, and every model says which keys it exposes.
That is what makes a cross-harness list work: one app can list several agent
providers side by side, each with its own effort scale, its own toggles, or none
at all. Because the *key* is what lands in `ModelSelection.axes`, the host still
reads `selection.axes.effort` no matter which provider is selected — only the
level set behind it changes.

## 2. Anatomy

```text
[Root .model-picker] <div>  (trigger wrapper, carries data-size/data-density/data-open/data-disabled)
  ├── [Trigger .model-picker__trigger] <button aria-haspopup="dialog" aria-expanded aria-controls>
  │   ├── [Icon .model-picker__icon] <span> (conditional: selected model has `image` or `icon`)
  │   │   └── Icon, or [Image .model-picker__image] <img> when the model supplies one
  │   ├── [Label .model-picker__label] <span>  (selected model label, or `placeholder`)
  │   ├── [Summary .model-picker__summary] <span aria-hidden> (conditional: showAxisSummary && axis summary non-empty)
  │   └── [Chevron .model-picker__chevron] <span aria-hidden="true">
  └── [Surface .model-picker__surface] <div role="dialog" tabindex="-1"> (rendered inline when open)
      └── [Panel .model-picker__panel] <div>
          ├── [Models .model-picker__models] <div role="radiogroup">
          │   ├── [Group Label .model-picker__group] <span> (repeated; conditional: option has `group`)
          │   └── [Option .model-picker__option] <button role="radio" aria-checked> (repeated)
          │       ├── [Option Icon .model-picker__option-icon] Icon (conditional)
          │       ├── [Option Text .model-picker__option-text] <span>
          │       │   ├── [Option Label .model-picker__option-label] <span>
          │       │   └── [Option Description .model-picker__option-description] <span> (conditional)
          │       ├── [Option Badge .model-picker__option-badge] <span> (conditional)
          │       └── [Option Check .model-picker__option-check] Icon (conditional: selected)
          └── [Axes .model-picker__axes] <div> (conditional: the selected model has applicable axes)
              └── [Axis .model-picker__axis] <div> (repeated; one per applicable axis)
                  ├── [Axis Label .model-picker__axis-label] <span>
                  ├── [Axis Description .model-picker__axis-description] <span> (conditional)
                  └── [Axis Control] one of:
                      ├── SegmentedControl                     (kind="select", control resolves to "segmented")
                      ├── Switch                               (kind="toggle")
                      └── [Axis List .model-picker__axis-list] <div role="radiogroup">  (kind="select", control resolves to "list")
                          └── [Axis Option .model-picker__axis-option] <button role="radio" aria-checked> (repeated)
                              ├── [Axis Option Label .model-picker__axis-option-label] <span>
                              └── [Axis Option Check .model-picker__axis-option-check] Icon (conditional: selected)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | trigger wrapper; carries `data-size`, `data-density`, `data-open`, `data-disabled` | — |
| Trigger | yes | borderless-by-default opener showing icon + model label + axis summary + chevron; `aria-haspopup="dialog"` | `--poodle-size-control-height`, `--poodle-radius-control`, `--poodle-color-text-primary` |
| Icon | no | leading mark for the selected model: a registry `Icon`, or an `<img>` when the model supplies `image` | `--poodle-color-text-secondary` |
| Image | no | `<img>` sized to the icon box it replaces (`object-fit: contain`, small radius) so rows align whichever a model supplies | — |
| Label | yes | selected model label, or `placeholder` when nothing is selected (`data-placeholder="true"`) | `--poodle-color-text-primary` / `--poodle-color-text-muted` |
| Summary | no | axis values joined by `·` (e.g. `High · Fast · 1M`); mirrors the panel state so the axes are readable without opening | `--poodle-color-text-secondary` |
| Chevron | yes | popover indicator (`▾`) | `--poodle-color-text-secondary` |
| Surface | yes | anchored `role="dialog"` panel, portalled to the theme root (`002-anchored-overlays.md`) | `--poodle-overlay-z-menu`, `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-elevation-overlay` |
| Models | yes | `role="radiogroup"` list of model options | — |
| Group Label | no | section heading emitted before the first option of each `group` | `--poodle-color-text-secondary` |
| Option | yes | one model row; `role="radio"`, `aria-checked`, `data-selected`, `data-disabled` | `--poodle-color-background-hover`, `--poodle-radius-control` |
| Option Image | no | `<img>` at the option-row icon size; same alignment rule as the trigger | — |
| Option Badge | no | short trailing tag (e.g. a tier or state word) supplied per option | `--poodle-color-border-subtle`, `--poodle-color-text-secondary` |
| Option Check | no | check glyph on the selected row | `--poodle-color-accent-base` |
| Axes | no | the right-hand column holding every applicable axis section; separated from the model list by a vertical rule. Absent when the selected model has no applicable axes | `--poodle-color-border-subtle` |
| Axis | no | one section per applicable axis; carries `data-kind` and `data-control` | `--poodle-color-border-subtle` |
| Axis Control | no | `SegmentedControl` or the Axis List for `kind="select"` (see Axis Control Resolution), `Switch` for `kind="toggle"` | (composed primitives) |
| Axis List | no | scrolling `role="radiogroup"` of option rows for a many-level select axis; capped at `max-height: 12rem` | `--poodle-radius-control` |
| Axis Option | no | one level row; `role="radio"`, `aria-checked`, `data-selected`, `data-disabled`; selected row is bolded with a check glyph | `--poodle-color-accent-base`, `--poodle-color-text-primary` / `--poodle-color-text-secondary` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `models` | `ModelOption[]` | `[]` | yes | host-supplied model list |
| `axes` | `ModelCapabilityAxis[]` | `[]` | no | host-declared capability axes |
| `value` | `ModelSelection \| undefined` | `undefined` | no | controlled selection; when supplied it is the source of truth |
| `placeholder` | `string` | `"Select model"` | no | trigger label when no model is selected |
| `ariaLabel` | `string` | `"Model"` | no | accessible name for the trigger and dialog |
| `disabled` | `boolean` | `false` | no | disables the trigger and every control in the panel |
| `showAxisSummary` | `boolean` | `true` | no | when false the trigger shows only the model label |
| `showModelDescriptions` | `boolean` | `true` | no | when false option descriptions are not rendered |
| `variant` | `"bare" \| "outlined"` | `"bare"` | no | `bare` is a borderless inline trigger (composer toolbars); `outlined` draws the standard control border/fill |
| `emphasis` | `"default" \| "subdued"` | `"default"` | no | `subdued` dims the trigger's label, icon, summary and chevron so the picker recedes beside a more important control; hover, focus and the open state restore full strength. Orthogonal to `variant` |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role for inherited sizing |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onChange` | `((value: ModelSelection) => void) \| null` | `null` | no | fires on every committed change (model or axis) |

### Naming Rules

Follows Poodle conventions: `camelCase` multi-word props, `on*` handler props,
`size`/`density`/`sizeRole` opt into the shared presentation axes. `disabled` is
documented without the `is` prefix; the Rust spec keeps `is_disabled`.

### Shared Types

Defined in `@inflatable-cookie/poodle-svelte` `types.ts`, re-exported from the package root,
redefined identically in `@inflatable-cookie/poodle-react`, mirrored in `poodle-specs` (snake_case).

```typescript
type ModelOption = {
  value: string;
  label: string;
  description?: string;
  badge?: string;
  icon?: string;
  group?: string;
  disabled?: boolean;
  /** A name from the icon registry (lucide via `IconProvider`), inheriting the
   * current text colour. */
  icon?: string;
  /** Any image — a provider logo, a data URI, an asset path. Takes precedence
   * over `icon` when both are set. */
  image?: ModelImage;
  /** Which capability axes this model exposes, by key, in display order. Omit to
   * inherit every declared axis — the single-provider case. `[]` means none. */
  axes?: ModelAxisRef[];
};

/** `alt` defaults to `""`: the model label sits beside the mark, so the image is
 * decorative unless the host says otherwise. */
type ModelImage = { src: string; alt?: string };

/** A model's reference to a declared axis: the key alone, or the key plus
 * overrides for that model. Every field but `key` falls back to the shared
 * definition, so a provider can swap the level set while inheriting the label,
 * kind and summary behaviour. */
type ModelAxisBinding = {
  key: string;
  label?: string;
  description?: string;
  options?: ModelAxisOption[];
  control?: "auto" | "segmented" | "list";
  defaultValue?: ModelAxisValue;
  onLabel?: string;
  offLabel?: string;
  showInSummary?: boolean;
  disabled?: boolean;
};

type ModelAxisRef = string | ModelAxisBinding;

type ModelAxisKind = "select" | "toggle";

type ModelAxisOption = { value: string; label: string; description?: string; disabled?: boolean };

type ModelCapabilityAxis = {
  key: string;
  label: string;
  kind: ModelAxisKind;
  description?: string;
  /** `select` axes only. */
  options?: ModelAxisOption[];
  /** Control shape for a `select` axis. `auto` (the default) uses a
   * SegmentedControl up to three options and a vertical list beyond that. */
  control?: "auto" | "segmented" | "list";
  /** Default applied when the axis has no value for the selected model. */
  defaultValue?: string | boolean;
  /** Labels used in the trigger summary for a `toggle` axis. */
  onLabel?: string;
  offLabel?: string;
  /** When false the axis is omitted from the trigger summary. Default true. */
  showInSummary?: boolean;
  disabled?: boolean;
};

type ModelAxisValue = string | boolean;

type ModelSelection = { model: string; axes: Record<string, ModelAxisValue> };
```

### Cross-Provider Example

```typescript
const axes = [
  { key: "effort", label: "Effort", kind: "select", options: [
    { value: "low", label: "Low" }, { value: "high", label: "High" } ] },
  { key: "fast", label: "Fast mode", kind: "toggle", onLabel: "Fast", offLabel: "Normal" },
  { key: "verbosity", label: "Verbosity", kind: "select", options: [ /* … */ ] },
];

const models = [
  // Inherits the shared effort scale plus the toggle.
  { value: "atlas", label: "Atlas", group: "Atlas", axes: ["effort", "fast"] },
  // Same `effort` key, this provider's own levels, plus a different second axis.
  { value: "corvid-1", label: "Corvid 1", group: "Corvid", axes: [
    { key: "effort", options: [
      { value: "minimal", label: "Minimal" },
      { value: "deep", label: "Deep" } ], defaultValue: "minimal" },
    "verbosity",
  ] },
  // No knobs at all.
  { value: "corvid-mini", label: "Corvid Mini", group: "Corvid", axes: [] },
];
```

### Controlled And Uncontrolled

- Controlled: supply `value`; every edit is mirrored through `onChange`
- Uncontrolled: omit `value` and the component owns the selection, seeded from
  the first non-disabled model plus each applicable axis' resolved default
- Default selection when unset: `{ model: "", axes: {} }` until a model exists

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no model selected | trigger shows `placeholder` in muted text; no summary |
| selected | a model is selected | trigger shows the model label plus the axis summary |
| subdued at rest | `emphasis="subdued"` | label drops to secondary; icon, summary and chevron sit at `--poodle-state-opacity-muted` |
| subdued on approach | `emphasis="subdued"` and hover / focus-within / open | label returns to primary and the dimmed parts to full opacity, so the control never reads as disabled once reached for |

Emphasis changes colour and opacity only — never font weight or any other metric.
A weight change would reflow the label under the pointer and shift everything
after it in the toolbar.
| open | trigger activated | anchored dialog surface by the trigger; `data-open="true"`; stays open through model and axis edits |
| split | the selected model has applicable axes | panel is two columns — models left, axes right, divided by a vertical rule; surface widens (`data-layout="split"`) |
| single | no applicable axes | panel is the model list alone at the narrow surface width (`data-layout="single"`) |
| option selected | option matches `value.model` | row carries `aria-checked="true"`, `data-selected="true"` and a check glyph |
| option disabled | `option.disabled` | row is non-interactive at disabled opacity |
| axis not exposed | selected model does not reference the axis key | that axis section is not rendered and its value is dropped from the emitted selection |
| icon mark | model sets `icon` | registry icon in the trigger and the option row, inheriting text colour |
| image mark | model sets `image` | `<img>` in place of the icon, at the same box size; wins when both are set |
| axis rebound | model references the key with a binding | the section renders the binding's label/levels/default, still under the shared key |
| axis as segments | `select` axis resolving to `segmented` | SegmentedControl; `data-control="segmented"` |
| axis as list | `select` axis resolving to `list` | vertical radiogroup of level rows, selected row bolded with a check; `data-control="list"` |
| axis disabled | `axis.disabled` or root `disabled` | axis control is disabled |
| disabled | `disabled=true` | trigger disabled, root at disabled opacity, popover cannot open |

### Component States

Open/closed plus (uncontrolled only) the local selection. Everything else is
derived from props.

### Selection Resolution

`resolveSelection(models, axes, selection)` is the cross-renderer semantic
reference and runs on every emission:

1. the model is kept as-is (the host may legitimately hold a value not in the
   current list — the trigger then falls back to that raw value as its label)
2. `axesForModel` resolves the model's references: each entry is looked up by
   key in `axes`, bindings are merged over the shared definition, and references
   to undeclared keys are dropped. A model with no `axes` declaration inherits
   every axis in declaration order
3. axes the model does not expose are **dropped** from the emitted `axes`
4. remaining axes with no value take `defaultValue`, else the first non-disabled
   option (`select`) or `false` (`toggle`)
5. `select` values not present in *this model's* option set fall back the same
   way — so a level that only exists on another provider never sticks across a
   model switch

### Axis Control Resolution

`axisControlKind(axis)` picks the control a `select` axis renders as:

| Condition | Control |
|-----------|---------|
| `axis.control === "segmented"` | SegmentedControl |
| `axis.control === "list"` | Axis List |
| `auto` (default) and `options.length <= 3` | SegmentedControl |
| `auto` (default) and `options.length > 3` | Axis List |
| `kind === "toggle"` | Switch (the hint does not apply) |

The threshold is `SEGMENTED_AXIS_MAX_OPTIONS = 3`: a six- or seven-level effort
scale cannot read as segments in the rail's width, and equal-width segments make
every label truncate at once. Hosts that know better override with `control`.

### Summary Text

`summaryText(models, axes, selection)`:

- `select` axis → the matching option's label
- `toggle` axis → `onLabel` when true / `offLabel` when false; an axis with no
  label for its current state contributes nothing
- axes with `showInSummary === false` contribute nothing
- the applicable axes' contributions are joined with `" · "` in declaration order

### Behavior Machine

Behavior classification: `styled-only (no machine)` — adapter-owned interaction.

Like `FilterBuilder` and `OrderBy`, the popover uses the shared dismissable-layer
stack and the selection is plain component state; there is no `@inflatable-cookie/poodle-headless`
machine and no conformance vectors. The pure helpers (`applicableAxes`,
`resolveSelection`, `axisSummary`, `summaryText`, `modelLabel`,
`axisControlKind`) live in `model-picker-model.ts` (TS) and as methods on
`ModelPickerSpec` / `ModelCapabilityAxis` (Rust).

#### Machinery Dependencies

Dismissable layer (`registerDismissLayer` from `@inflatable-cookie/poodle-headless`), presentation
context (size/density), id wiring for the dialog surface. No focus trap.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onChange` | model chosen or an axis value changed | `ModelSelection` | already passed through `resolveSelection`, so scoped-out axes never leak |

Neither a model choice nor an axis change closes the popover: the axes belong to
the model just chosen, so closing on selection would force a second trip to
adjust them. Escape or an outside interaction dismisses.

## 6. Accessibility

### Semantics

| Element | Attribute | Value |
|---------|-----------|-------|
| Trigger | `aria-label` | `"{ariaLabel}: {model label}{, axis summary}"` |
| Trigger | `aria-haspopup` | `"dialog"` |
| Trigger | `aria-expanded` | `"true"` when open |
| Trigger | `aria-controls` | dialog surface id when open |
| Trigger | `disabled` | native attribute when `disabled` |
| Summary | `aria-hidden` | `"true"` (carried in the trigger accessible name) |
| Surface | `role` | `"dialog"` |
| Surface | `aria-label` | `ariaLabel` |
| Surface | `tabindex` | `"-1"` |
| Models | `role` | `"radiogroup"` |
| Models | `aria-label` | `"Model"` |
| Image / Option Image | `alt` | from `image.alt`, defaulting to `""` — the label carries the name |
| Option | `role` | `"radio"` |
| Option | `aria-checked` | `"true"` on the selected row |
| Option | `disabled` | native attribute when the option is disabled |
| Axis (select, segmented) | `ariaLabel` on SegmentedControl | axis `label` |
| Axis List | `role` / `aria-label` | `"radiogroup"` / axis `label` |
| Axis Option | `role` / `aria-checked` | `"radio"` / `"true"` on the current level |
| Axis (toggle) | `ariaLabel` on Switch | axis `label` |

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter`/`Space` on trigger | toggles the popover |
| `Enter`/`Space` on an option | selects that model; the popover stays open |
| `ArrowDown`/`ArrowUp` in the model list | moves focus to the next/previous enabled option (wrapping) |
| `Tab` | moves through the option list and axis controls (child components own their focus rings) |
| `Escape` | dismisses the popover and returns focus to the trigger |

### Focus And Announcement

- focus entry: on open, focus moves to the selected option, else the first
  enabled option
- focus exit: Escape and outside dismiss return focus to the trigger. Selecting
  a model does not move focus — the row keeps it, so arrow keys can walk to
  another model or Tab can continue into the axes
- the trigger's accessible name always carries model + axis summary, so the
  current configuration is announced without opening the panel
- GPUI-native accessibility mapping: no ARIA API — accessible names map to native
  exposure where available; documented as an accepted delta

## 7. Layout

### Sizing

- Root: `position: relative`, `display: inline-flex`, `min-width: 0`
- Trigger: `inline-flex`, `min-height: var(--poodle-size-control-height)`,
  gap `0.375rem`, label ellipsis on overflow
- Surface: `min-width: 18rem`, `max-width: min(26rem, 90vw)`. It is portalled
  and viewport-positioned per `002-anchored-overlays.md`, requesting
  `top-start` with an `8px` offset — the picker's home is a composer toolbar at
  the bottom of a viewport, so it opens **upward** and the resolver flips it
  down when there is no room above. It publishes the coarse side it landed on
  as `data-placement="top" | "bottom"`, and carries its own `data-size` /
  `data-density` so the density rules still reach the panel
- Panel: a grid. With applicable axes it is two columns —
  `minmax(0, 1fr)` model list and a `minmax(11rem, 15rem)` axes rail divided by
  a vertical rule — and the surface widens to `min-width: 34rem`,
  `max-width: min(42rem, 92vw)`. Native targets have no grid, so they use a
  fixed `13rem` rail inside the same widened surface. With no applicable axes it collapses to the
  single model column at the narrow surface width
- Model list: `max-height: 18rem`, vertical scroll. Option rows and group
  headings never shrink (`flex: none`) — the list is height-capped, so a
  shrinkable row squashes below its own content and spills the description out of
  its hover/selected box. Rows are top-aligned and their descriptions are
  single-line with ellipsis, keeping row heights uniform in the narrow column
- Group headings take space above (except the first) so group runs read as
  sections rather than one continuous list
- Panel columns stretch (not `align-items: start`) so the rail's dividing rule
  runs the panel's full height even when the axes stack is shorter than the
  model list
- Axes rail: sections stack with a `0.75rem` gap; the first takes no top rule
  (the column rule already separates it), later ones do. `toggle` axes lay out
  label-above-control in the rail rather than the wide label/control row

### Composition

- parent expectations: composer toolbars (`AgentChatInput`), settings rows,
  status bars
- child expectations: `Icon`, `SegmentedControl`, `Switch`
- the popover surface is owned locally by `ModelPicker`, not by `Popover`

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Trigger | `--poodle-size-control-height` | size-stepped min height |
| Trigger | `--poodle-radius-control` | corner radius |
| Trigger (`outlined`) | `--poodle-color-background-surface`, `--poodle-color-border-default` | fill + border |
| Trigger hover | `color-mix(surface 84%, elevated)` | hover fill |
| Trigger focus | `--poodle-color-accent-focusRing`, `--poodle-border-width-focus` | focus ring |
| Label | `--poodle-color-text-primary` | selected model |
| Label (placeholder) | `--poodle-color-text-muted` | empty state |
| Summary / Chevron | `--poodle-color-text-secondary` | secondary text |
| Surface | `--poodle-overlay-z-menu`, `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-elevation-overlay` | anchored panel |
| Option hover | `--poodle-color-background-hover` | row hover fill |
| Option check | `--poodle-color-accent-base` | selected glyph |
| Option description | `--poodle-color-text-secondary` | supporting text |
| Option badge | `--poodle-color-border-subtle`, `--poodle-color-text-secondary` | tag outline + text |
| Axis divider | `--poodle-color-border-subtle` | section rule |
| Disabled | `--poodle-state-opacity-disabled` | disabled opacity |

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`–`"xl"` |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` |
| `data-variant` | Root | `"bare"`, `"outlined"` |
| `data-emphasis` | Root | `"default"`, `"subdued"` |
| `data-open` | Root | `"true"` / `"false"` |
| `data-disabled` | Root | `"true"` / `"false"` |
| `data-placeholder` | Label | `"true"` when no model is selected |
| `data-selected` | Option | `"true"` on the selected row |
| `data-disabled` | Option | `"true"` when the option is disabled |
| `data-kind` | Axis | `"select"` / `"toggle"` |
| `data-control` | Axis | `"segmented"` / `"list"` |
| `data-selected` | Axis Option | `"true"` on the current level |
| `data-disabled` | Axis Option | `"true"` when the option is disabled |
| `data-placement` | Surface | `"top"` (default) / `"bottom"` when flipped |
| `data-layout` | Surface | `"split"` (models + axes columns) / `"single"` |

## 9. Svelte Notes

- owns its open state and anchored panel surface directly (mirrors `FilterBuilder`)
- `registerDismissLayer` from `@inflatable-cookie/poodle-headless`; dismiss on outside interact /
  Escape via a `$effect` guarded by `open`
- size/density resolve via `getUiPresentation` + `resolveSemanticControlSize`
- controlled/uncontrolled via `$bindable(value)` plus a `sync()` that writes
  `value` (controlled) or local state (uncontrolled) then calls `onChange`
- pure logic imported from `./model-picker-model` (`axesForModel` is the entry
  point; `applicableAxes`, `resolveSelection` and `summaryText` all take the
  model list so bindings resolve)
- arrow-key roving over the option list is a plain `keydown` handler on the
  radiogroup — no roving-tabindex machine
- placement flip uses `resolveSurfacePlacement` from `./model-picker-model`
  (estimate on open) plus a measured correction once the panel has rendered

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::model_picker`
- theme access via `GpuiThemeProvider`; all dimensions/colors resolved from tokens
- the render is a faithful function of the full spec state: `ModelPickerSpec`
  carries `is_open`, so GPUI and Jetstream render the complete panel (model rows
  with selection marks, plus one section per applicable axis). The shared native
  limitation applies — the preview does not drive clicks, and anchored
  positioning is platform-owned, so the surface renders inline below the trigger
- no ARIA API — accessible-name intent documented as an accepted delta

## 10a. Jetstream Notes

- `ModelPicker::from_spec(spec, theme).on_change(...)`, carrying the chosen
  model's value. Axis options report through the same event: both are choices in
  the same panel, and a host wiring only one would silently drop the other.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `models` accepted; group headings emitted per `group`
- [ ] a model's `image` renders in place of its `icon` in both the trigger and
      the option row, at the same box size; `alt` defaults to empty
- [ ] `axes` accepted; `select` and `toggle` kinds both representable
- [ ] axis control resolution matches: explicit `control` wins, else segmented at
      three options or fewer and a list beyond
- [ ] per-model axis references resolve by key, in the model's declared order;
      unknown keys drop; no declaration inherits every axis; `[]` exposes none
- [ ] bindings override only the fields they set, and keep the shared key
- [ ] switching models re-resolves values against the new model's option set
- [ ] `resolveSelection` defaults match (declared default → first enabled option → false)
- [ ] trigger summary text matches, joined with `" · "` in declaration order
- [ ] neither selecting a model nor changing an axis closes the popover
- [ ] `value` controlled; external replacement re-renders
- [ ] disabled suppresses all interaction
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] trigger anatomy (icon, label, summary, chevron) and spacing match
- [ ] `bare` vs `outlined` trigger treatment matches
- [ ] `subdued` dims label/icon/summary/chevron at rest and restores them on
      hover, focus and open; accessible names, font weight and layout metrics are
      unchanged either way
- [ ] surface anchoring (upward), radius, elevation match
- [ ] option row anatomy (icon, label, description, badge, check) matches
- [ ] two-column split (models left, axes right, divided) whenever axes apply;
      single column and the narrow surface width when they do not
- [ ] axis sections separated by a subtle rule
- [ ] all five sizes and three densities match the ladders

### Tier 3: Implementation Freedom

- [ ] popover/animation/portal behavior is platform-owned
- [ ] surface placement flipping is web-only (native renders the panel inline)
- [ ] scroll behavior of the model list is platform-owned
- [ ] axis control internals are the composed primitives' business

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Native targets render `subdued` at its resting strength only | hover and focus-within are web states; the native previews resolve a spec, not a pointer position | accepted | host restores emphasis on interaction if it wires one |
| GPUI/Jetstream preview doesn't drive live clicks | shared render-only posture across all native components; the panel is rendered from full spec state | accepted | host wires interaction |
| Only `select` and `toggle` axis kinds | v1 scope; covers effort levels, speed modes, context sizes and booleans | accepted (by design) | multi-select / numeric axes if a consumer needs them |
| The segmented/list threshold is a constant, not a prop | one number, and `control` already gives per-axis escape | accepted | promote to a prop if a consumer needs a different global cut |
| Jetstream loads images from file paths, not URLs or data URIs | the runtime's image widget takes an asset path; remote fetching is host/app work | accepted | hosts pass a resolved asset path on that target |
| Axis values are `string \| boolean` | keeps the selection JSON-serializable and cross-renderer trivial | accepted | numeric axes would extend the union |

## 13. Approval And Adoption Notes

- contract status: `implemented`
- approvers: pending review
- downstream adopters: `AgentChatInput`, Loophole, Underlay apps
- future follow-up: axis groups/sections in the rail if a provider exposes more
  knobs than fit one column

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): default picker with
three models and two axes; a model marked with a registry `icon` beside one marked with an `image`
(provider logo); a many-level axis rendering as a list (7 levels); an explicit
`control="list"` override on a short axis; model-scoped axis (an axis that only
applies to one model); toggle axis on/off; grouped model list; options with badges and
descriptions; `outlined` variant; `default` beside `subdued` emphasis; summary suppressed
(`showAxisSummary=false`);
no model selected (placeholder); disabled; disabled option; controlled value with
live serialized output; full size ladder; density variants.

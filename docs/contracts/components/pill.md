# Pill

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Pill`
- Layer: `foundation`
- Summary: a rounded inline label for compact categorization or metadata
- In scope: tone, appearance (tint/solid/subtle/badge), semantic
  sizing, optional monospace styling
- Out of scope: removable chips, multi-select tag inputs

## 2. Anatomy

```text
[Root .pill]
  ├── [Dot (optional, .poodle-pill__dot)]
  ├── [Icon (optional, inline svg / .poodle-icon)]
  └── [Content (default slot)]
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `<span>` | yes | rounded metadata shell with inline-flex layout; children separated by `--poodle-pill-gap` |
| Dot | `.poodle-pill__dot` | no | optional leading status dot (`dot` prop), `0.5em` square, `aria-hidden`, fill follows the semantic tone |
| Icon | `<svg>` / `.poodle-icon` | no | optional inline icon, sized `1em` square and `flex-shrink: 0` |
| Content | slot | yes | short label text |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `"neutral" \| "info" \| "success" \| "warning" \| "danger"` | `"neutral"` | no | semantic tone controlling fill/border/text color |
| `appearance` | `"tint" \| "solid" \| "subtle" \| "badge"` | `"tint"` | no | visual treatment variant; mutually exclusive |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit pill size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `font` | `"normal" \| "mono"` | `"normal"` | no | content font variant |
| `typography` | `"label" \| "inherit"` | `"label"` | no | label typography by default; use `"inherit"` when parent inline text should own font size and line-height |
| `accent` | `string \| null` | `null` | no | optional custom accent color overriding the semantic tone colors |
| `muted` | `boolean` | `false` | no | visual de-emphasis via reduced opacity |
| `adaptiveWidth` | `boolean` | `false` | no | when `true`, emits `data-adaptive-width` and sets `min-width: 0` so the pill collapses to its content instead of honoring the per-size `min-width` floor |
| `dot` | `boolean` | `false` | no | renders a leading `0.5em` status dot filled with the tone's status color (accent color when `accent` is set, `--poodle-color-text-secondary` for neutral) |
| `title` | `string \| null` | `null` | no | optional native tooltip forwarded to the root element's `title` attribute |
| `ariaLabel` | `string \| null` | `null` | no | optional explicit accessible name |

### Controlled And Uncontrolled

- Display primitive only; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | default | neutral fill, subtle border, secondary text |
| info | `tone="info"` | blue-tinted fill and border, primary text |
| success | `tone="success"` | green-tinted fill and border, primary text |
| warning | `tone="warning"` | amber-tinted fill and border, primary text |
| danger | `tone="danger"` | red-tinted fill and border, primary text |
| custom accent | `accent` provided | accent-tinted fill, border, and text using the provided color |
| tint | default (`appearance="tint"`) | ordinary tone-tinted shell; the base recipes below apply with no appearance override |
| solid | `appearance="solid"` | opaque tone-and-theme fill with primary foreground; uses a custom accent as the tone base |
| subtle | `appearance="subtle"` | 50% transparent fill overlay |
| muted | `muted=true` | reduced opacity (0.72) |
| xs | `size="xs"` | smallest metadata chip sizing |
| sm | `size="sm"` | compact metadata chip sizing |
| md | inherited default or `size="md"` | default metadata chip sizing |
| lg | `size="lg"` | emphasized metadata chip sizing |
| xl | `size="xl"` | largest metadata chip sizing |

### Component States

No internal state.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive display primitive |

## 6. Accessibility

### Semantics

- Role: inline text by default (no ARIA role)
- Required attributes: none
- Optional attributes: `aria-label` when visible text is abbreviated or symbolic
- Labeling rules: pills stay non-interactive unless a higher-order contract
  wraps them

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive by default |

### Focus And Announcement

- focus entry: not focusable by default
- live-region behavior: none
- GPUI-native accessibility mapping notes: expose pill content as text-like
  metadata, not as a control

## 7. Layout

### Sizing

- Pill sizes to content with compact padding
- `typography="inherit"` keeps the selected size preset, but expresses pill
  geometry in `em` so shell height and padding scale with the inherited text
- Content may truncate according to parent layout rules
- Uses `white-space: nowrap` to prevent wrapping

### Composition

- parent expectations: metadata rows, headers, cards, filter summaries
- child expectations: short text content

## 8. Token Usage

### Root `.pill` (default / neutral / md)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `gap` | `var(--poodle-pill-gap)` (md `0.25rem`) |
| `min-height` | `1.25rem` |
| `min-width` | `calc(var(--poodle-pill-min-width-base) + var(--poodle-pill-min-width-adjust))` (md base `2.875rem`) |
| `padding` | `0.1875rem 0.625rem` |
| `border` | `0.0625rem solid var(--poodle-pill-border)` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-pill-fill)` |
| `color` | `var(--poodle-pill-text)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `white-space` | `nowrap` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font-size` | size-specific `em` value derived from the selected preset |
| `min-height` | size-specific `em` value derived from the selected preset |
| `padding` | size-specific `em` value derived from the selected preset |

### Component custom properties (neutral default)

The ordinary tone-tinted recipes assign `--poodle-pill-tint-fill`;
`--poodle-pill-fill` defaults to `var(--poodle-pill-tint-fill)` so the
`subtle` appearance can derive from the tint base without a custom-property
cycle. `solid` and `badge` assign `--poodle-pill-fill` directly.

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-tint-fill` | `color-mix(in srgb, var(--poodle-color-background-surface) 90%, transparent)` |
| `--poodle-pill-fill` | `var(--poodle-pill-tint-fill)` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)` |
| `--poodle-pill-text` | `var(--poodle-color-text-secondary)` |

### Tone: info `.pill[data-tone="info"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-tint-fill` | `color-mix(in srgb, var(--poodle-color-status-info) 14%, var(--poodle-color-background-surface))` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-status-info) 38%, var(--poodle-color-border-subtle))` |
| `--poodle-pill-text` | `var(--poodle-color-text-primary)` |

### Tone: success `.pill[data-tone="success"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-tint-fill` | `color-mix(in srgb, var(--poodle-color-status-success) 14%, var(--poodle-color-background-surface))` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-status-success) 38%, var(--poodle-color-border-subtle))` |
| `--poodle-pill-text` | `var(--poodle-color-text-primary)` |

### Tone: danger `.pill[data-tone="danger"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-tint-fill` | `color-mix(in srgb, var(--poodle-color-status-danger) 14%, var(--poodle-color-background-surface))` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-status-danger) 38%, var(--poodle-color-border-subtle))` |
| `--poodle-pill-text` | `var(--poodle-color-text-primary)` |

### Custom accent `.pill[data-accent="custom"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-tint-fill` | `color-mix(in srgb, var(--poodle-pill-accent) 18%, rgba(148, 163, 184, 0.08))` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-pill-accent) 30%, rgba(148, 163, 184, 0.12))` |
| `--poodle-pill-text` | `color-mix(in srgb, var(--poodle-pill-accent) 88%, white)` |

### Appearance: subtle `.pill[data-appearance="subtle"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-fill` | `color-mix(in srgb, var(--poodle-pill-tint-fill) 50%, transparent)` |

### Appearance: solid `.pill[data-appearance="solid"]`

Solid resolves the background as an opaque sRGB mix of the tone base at 40%
and `color.background.surface` at 60%, uses the raw tone base as the border,
and uses `color.text.primary` as the text color. The neutral solid background
mixes `color.text.secondary` and `color.background.surface` equally and its
border is `color.border.strong`. A custom `accent` replaces the semantic tone
as the mix base. Appearance is a single mutually exclusive axis: solid never
combines with subtle's opacity reduction or badge's uppercase typography. The
optional dot uses the primary foreground in the solid appearance.

### Appearance: badge `.pill[data-appearance="badge"]`

Base badge typography (all tones):

| Custom Property / Property | Value |
|----------------------------|-------|
| `font-weight` | `700` |
| `letter-spacing` | `0.04em` |
| `text-transform` | `uppercase` |

Neutral badge `.pill[data-appearance="badge"][data-tone="neutral"]`:

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-fill` | `color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary))` |
| `--poodle-pill-text` | `var(--poodle-color-text-secondary)` |

Tone badges (`success` / `info` / `warning` / `danger`):

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-fill` | `color-mix(in srgb, var(--poodle-color-status-{tone}) 18%, transparent)` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-status-{tone}) 42%, transparent)` |

### Size: sm `.pill[data-size="sm"]`

| Property | Value |
|----------|-------|
| `min-height` | `1rem` |
| `min-width` | `2.5rem` |
| `padding` | `0.125rem 0.5rem` |
| `font-size` | `0.625rem` |
| `gap` | `0.1875rem` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font-size` | `0.6429em` |
| `min-height` | `1.6em` |
| `min-width` | `2.8571em` |
| `padding` | `0.2em 0.8em` |

### Size: xs `.pill[data-size="xs"]`

| Property | Value |
|----------|-------|
| `min-height` | `0.875rem` |
| `min-width` | `2.125rem` |
| `padding` | `0.0625rem 0.4375rem` |
| `font-size` | `0.5625rem` |
| `gap` | `0.15625rem` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font-size` | `0.5786em` |
| `min-height` | `1.5556em` |
| `min-width` | `2.4444em` |
| `padding` | `0.1111em 0.7778em` |

### Size: md `.pill[data-size="md"]`

| Property | Value |
|----------|-------|
| `min-height` | `1.25rem` |
| `min-width` | `2.875rem` |
| `padding` | `0.1875rem 0.625rem` |
| `font-size` | `0.6875rem` |
| `gap` | `0.25rem` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font-size` | `0.7071em` |
| `min-height` | `1.8182em` |
| `min-width` | `3.2727em` |
| `padding` | `0.2727em 0.9091em` |

### Size: lg `.pill[data-size="lg"]`

| Property | Value |
|----------|-------|
| `min-height` | `1.375rem` |
| `min-width` | `3.25rem` |
| `padding` | `0.25rem 0.75rem` |
| `font-size` | `0.75rem` |
| `gap` | `0.25rem` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font-size` | `0.7714em` |
| `min-height` | `1.8333em` |
| `min-width` | `3.5833em` |
| `padding` | `0.3333em 1em` |

### Size: xl `.pill[data-size="xl"]`

| Property | Value |
|----------|-------|
| `min-height` | `1.5rem` |
| `min-width` | `3.625rem` |
| `padding` | `0.3125rem 0.9375rem` |
| `font-size` | `0.8125rem` |
| `gap` | `0.25rem` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font-size` | `0.8357em` |
| `min-height` | `1.8462em` |
| `min-width` | `3.9231em` |
| `padding` | `0.3846em 1.1538em` |

### Font: mono `.pill[data-font="mono"]`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-code-family)` |
| `letter-spacing` | `0.02em` |

### Muted `.pill[data-muted="true"]`

| Property | Value |
|----------|-------|
| `opacity` | `0.72` |

### Adaptive width `.pill[data-adaptive-width="true"]`

| Property | Value |
|----------|-------|
| `min-width` | `0` (collapses the pill to its content, overriding the per-size `min-width` floor) |

### Dot `.poodle-pill__dot`

| Property | Value |
|----------|-------|
| `width` / `height` | `0.5em` |
| `border-radius` | `999px` |
| `flex-shrink` | `0` |
| `background` | `var(--poodle-pill-dot-fill, var(--poodle-color-text-secondary))` |

Tone dot fills (`--poodle-pill-dot-fill` on the root):

| Selector | Value |
|----------|-------|
| `[data-tone="info"]` | `var(--poodle-color-status-info)` |
| `[data-tone="success"]` | `var(--poodle-color-status-success)` |
| `[data-tone="warning"]` | `var(--poodle-color-status-warning)` |
| `[data-tone="danger"]` | `var(--poodle-color-status-danger)` |
| `[data-accent="custom"]` | `var(--poodle-pill-accent)` |

### Density

Density adjusts pill spacing via the `--poodle-pill-*-adjust` custom properties.
Unlike most components — where density must never touch vertical padding — the
pill is a sub-text-line metadata chip, so its density variants deliberately
nudge `padding-y` (along with `min-width`, `padding-x`, and `gap`) to keep the
chip optically balanced at its tiny size. This vertical-padding override is the
explicitly-justified exception permitted by the repo Size/Density contract.

| Custom Property | `compact` | `comfortable` |
|-----------------|-----------|---------------|
| `--poodle-pill-min-width-adjust` | `-0.09375rem` | `0.1875rem` |
| `--poodle-pill-padding-y-adjust` | `-0.0625rem` | `0.0625rem` |
| `--poodle-pill-padding-x-adjust` | `-0.125rem` | `0.125rem` |
| `--poodle-pill-gap` | `0.125rem` | `0.25rem` |

(`typography="inherit"` expresses the same adjustments in `em`: compact
`min-width -0.1364em` / `padding-y -0.0909em` / `padding-x -0.1818em`;
comfortable `min-width 0.2727em` / `padding-y 0.0909em` / `padding-x 0.1818em`.)

## 9. Svelte Notes

- Renders as a styled inline `<span>` with a default slot
- Tone, appearance, size, font, and muted state are driven via `data-*`
  attributes for CSS selector targeting
- `typography="inherit"` uses the proportional-inherit rule from
  `docs/contracts/001-working-rules.md`: the selected size preset is converted
  from token `rem` values into equivalent `em` values
- Component custom properties (`--poodle-pill-tint-fill`,
  `--poodle-pill-fill`, `--poodle-pill-border`, `--poodle-pill-text`) are set
  on the root element and consumed by the same element's CSS, enabling tone
  overrides without class proliferation
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- **Pill context composition surface** (`pill-context.ts`): a parent may call
  `setPillContext({ size?, typography? })` to force the `size` and `typography`
  of descendant pills. When present, the context's `size`/`typography` win over
  the pill's own `size`/`typography` props (`size` still falls back to the
  semantic `sizeRole` resolution when the context omits it). This lets a host
  (e.g. an inline run of text) drive a consistent pill scale without threading
  props through every pill.

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::pill`
- keep semantics non-interactive unless wrapped by a control-specific contract
- `color-mix` blending should be replicated using equivalent alpha-blended color
  calculations in GPUI's color system
- for `typography="inherit"`, non-CSS runtimes may approximate parent-owned
  `em` behavior with ratio-preserving metrics from a 1rem baseline until
  parent-relative inline layout exists

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive metadata semantics match
- [ ] tone custom property overrides produce equivalent colors
- [ ] `appearance="solid"` uses the shared opaque surface recipe for every tone and custom accent

### Tier 2: Visual Parity

- [ ] `xs | sm | md | lg | xl` sizes produce correct min-height, padding, and font-size
- [ ] mono font variant uses code family with correct letter-spacing
- [ ] tint appearance preserves the ordinary tone-tinted shell
- [ ] subtle appearance halves fill opacity
- [ ] badge appearance keeps its uppercase typography and weight
- [ ] muted state applies 0.72 opacity

### Tier 3: Implementation Freedom

- [ ] truncation and rendering internals stay internal
- [ ] `color-mix` may be replaced by pre-computed equivalents

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| `color-mix` implementation | GPUI may pre-compute blended colors rather than using CSS `color-mix` | allowed | ensure visual equivalence across themes |
| Jetstream mono font styling | current Jetstream `JsEl` text surface does not expose font-family or letter-spacing controls | allowed | implement text-family and tracking support in Jetstream, then apply `font="mono"` literally |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Tones

Five pills in a horizontal row with 8px gap:

| Content | Tone |
|---------|------|
| Neutral | neutral |
| Success | success |
| Danger | danger |

### Appearances

Preview apps include one compact appearances group showing the four mutually
exclusive treatments: `tint` (the default ordinary shell), `solid` (opaque
tone-and-theme fill with primary foreground), `subtle`, and `badge`. Tests own the
tone-by-appearance matrix; the specimen stays representative.

### Sizes

Five pills in a horizontal row with 8px gap:

| Content | Size |
|---------|------|
| Extra small | xs |
| Small | sm |
| Medium | md |
| Large | lg |
| Extra large | xl |

### Code font

Five pills with `font: "mono"` in a horizontal row:

| Content | Tone | Font |
|---------|------|------|
| v2.4.1 | neutral | mono |
| stable | success | mono |
| beta | danger | mono |

### Muted

Three muted pills in a horizontal row:

| Content | Tone | Muted |
|---------|------|-------|
| Muted neutral | neutral | true |
| Muted success | success | true |
| Muted danger | danger | true |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: metadata displays, labels, status tags, card headers
- future follow-up: add dismissible-chip semantics separately if needed

## Rust Spec Note

`poodle-specs` currently exposes BOTH `PillSpec` (the full primitive) AND a
legacy `BadgeSpec` struct. `BadgeSpec` is the pre-consolidation Rust shape
that predates `PillAppearance::Badge` — new code should use `PillSpec` with
`appearance = PillAppearance::Badge` instead. The legacy `BadgeSpec` is
retained for backward compatibility with existing consumers and will be
removed in a future cleanup pass.
### Tone: warning `.pill[data-tone="warning"]`

| Custom Property | Value |
|-----------------|-------|
| `--poodle-pill-tint-fill` | `color-mix(in srgb, var(--poodle-color-status-warning) 14%, var(--poodle-color-background-surface))` |
| `--poodle-pill-border` | `color-mix(in srgb, var(--poodle-color-status-warning) 38%, var(--poodle-color-border-subtle))` |
| `--poodle-pill-text` | `var(--poodle-color-text-primary)` |

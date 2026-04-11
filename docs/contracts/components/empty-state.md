# EmptyState

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `EmptyState`
- Layer: `composites`
- Summary: a standardized empty-result or no-content message with optional
  supporting actions, decorative visual, and variant-driven background
  treatment — supports neutral, search, and first-run postures
- In scope: title, descriptive copy, optional illustration/icon via visual
  slot, action group via actions slot, variant-driven background styling,
  compact size variant, density-aware spacing
- Out of scope: async loading progress, blocking errors, app-specific
  onboarding flows

## 2. Anatomy

```text
[Root .empty-state]  <section aria-label>
  ├── [Visual .empty-state__visual]  <div aria-hidden="true">
  │     └── (slot: visual) or default Icon per variant
  ├── [Copy .empty-state__copy]  <div>
  │     ├── [Title]  <h3>
  │     └── [Message]  <p> (optional)
  └── [Actions .empty-state__actions]  <div> (optional, slot-driven)
        └── (slot: actions)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<section>` container with dashed border, variant-driven background | spacing, border, radius, background |
| Visual | yes | decorative icon circle; hidden from accessibility tree | background-panel, text-secondary, size |
| Copy | yes | title heading and optional message paragraph | text-primary, text-secondary, typography |
| Title | yes | `<h3>` primary empty-state message | font-size, line-height |
| Message | no | `<p>` supporting explanation | text-secondary, font-size |
| Actions | no | flex container for remediation/creation action buttons (slot-driven) | gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | — | yes | primary message |
| `message` | `string \| null` | `null` | no | supporting explanation |
| `variant` | `"neutral" \| "search" \| "firstRun"` | `"neutral"` | no | semantic posture; controls background color and default icon |
| `size` | `"default" \| "compact"` | `"default"` | no | controls visual and copy sizing |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `ariaLabel` | `string \| null` | `null` | no | accessible label; falls back to `title` when null |

### Slots

| Slot | Description |
|------|-------------|
| `visual` | custom visual content replacing the default variant icon |
| `actions` | remediation or creation action buttons |

### Controlled And Uncontrolled

- declarative state composite
- action behavior remains host-owned via the actions slot
- no internal state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | `variant="neutral"` (default) | generic no-content posture; dashed border, surface background at 76% alpha, inbox icon |
| search | `variant="search"` | search posture; accent-base background at 7% alpha, search icon |
| firstRun | `variant="firstRun"` | invitational posture; success background at 7% alpha, plus icon |
| compact | `size="compact"` | smaller visual circle, smaller heading and message text |
| actionable | actions slot populated | remediation/creation controls visible below copy |

### Component States

No internal state.

## 5. Events

No component-owned events. Action behavior is delegated to slotted children.

## 6. Accessibility

### Semantics

- Root: `<section>` with `aria-label` (resolves to `title` when `ariaLabel` is null)
- Visual container: `aria-hidden="true"` — decorative content excluded from
  accessibility tree
- Title: `<h3>` heading element

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches any visible action buttons in logical order |

### Focus And Announcement

- focus entry: the empty-state container is not focusable by default
- focus exit: actions and surrounding shell controls remain in sensible order
- live-region behavior: none; host may announce empty-state appearance when
  it results from a user-triggered search or filter change
- GPUI-native accessibility mapping notes: GPUI must preserve the text message
  and action labeling explicitly and avoid relying on visual illustration alone

## 7. Layout

### Sizing

- root: centered grid layout, full parent width
- copy area: max-width 24rem
- actions: flex-wrap for narrow widths

### Composition

- composes: `Icon` primitive (for default variant icons)
- parent expectations: `DetailShell`, settings scopes, browse surfaces
- child expectations: text, icon/illustration (via visual slot), action
  primitives (via actions slot)
- resizing rules: actions may wrap below the message on narrow widths

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-variant` | root `<section>` | `"neutral"`, `"search"`, `"firstRun"` |
| `data-size` | root `<section>` | `"default"`, `"compact"` |
| `data-density` | root `<section>` | `"compact"`, `"default"`, `"comfortable"` |

### Root (`.empty-state`)

| Property | Value |
|----------|-------|
| display | `grid` |
| justify-items | `center` |
| text-align | `center` |
| gap | `var(--poodle-space-stack-md)` |
| padding | `calc(var(--poodle-space-panel-y) * 1.5) var(--poodle-space-panel-x)` |
| border | `0.0625rem dashed var(--poodle-color-border-default)` |
| border-radius | `calc(var(--poodle-radius-surface) - 0.125rem)` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent)` |

#### Root Variant: `[data-variant="search"]`

| Property | Value |
|----------|-------|
| background | `color-mix(in srgb, var(--poodle-color-accent-base) 7%, transparent)` |

#### Root Variant: `[data-variant="firstRun"]`

| Property | Value |
|----------|-------|
| background | `color-mix(in srgb, var(--poodle-color-status-success) 7%, transparent)` |

### Visual (`.empty-state__visual`)

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| justify-content | `center` |
| width | `2.25rem` |
| height | `2.25rem` |
| border-radius | `999rem` |
| background | `color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent)` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `1.125rem` |
| font-weight | `600` |

### Copy (`.empty-state__copy`)

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-inline-sm)` |
| max-width | `24rem` |

### Copy Heading (`.empty-state__copy h3`)

| Property | Value |
|----------|-------|
| margin | `0` |
| font-size | `1.125rem` |
| line-height | `1.2` |

### Copy Message (`.empty-state__copy p`)

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.8125rem` |
| line-height | `1.5` |

### Actions (`.empty-state__actions`)

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-wrap | `wrap` |
| gap | `var(--poodle-space-inline-sm)` |

### Variant Icons

| Variant | Icon |
|---------|------|
| `neutral` | `inbox` |
| `search` | `search` |
| `firstRun` | `plus` |

### Size Adjustments

#### `data-size="compact"`

| Part | Property | Value |
|------|----------|-------|
| `.empty-state__visual` | width, height | `1.75rem` |
| `.empty-state__visual` | font-size | `0.9375rem` |
| `.empty-state__copy h3` | font-size | `0.9375rem` |
| `.empty-state__copy p` | font-size | `0.75rem` |

### Density Adjustments

#### `data-density="compact"`

| Part | Property | Value |
|------|----------|-------|
| `.empty-state` | gap | `var(--poodle-space-stack-sm)` |
| `.empty-state` | padding | `var(--poodle-space-stack-lg) var(--poodle-space-panel-x)` |

#### `data-density="comfortable"`

| Part | Property | Value |
|------|----------|-------|
| `.empty-state` | gap | `var(--poodle-space-stack-lg)` |
| `.empty-state` | padding | `calc(var(--poodle-space-panel-y) * 2) var(--poodle-space-panel-x)` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- `data-variant` attribute on root `<section>` controls variant-specific background
- `data-size` attribute on root reflects the size prop
- `data-density` attribute reflects resolved density from prop or inherited presentation
- visual slot uses `$$slots.visual` check to detect custom content
- actions slot rendered conditionally via `$$slots.actions` check
- `ariaLabel` on `<section>` falls back to `title` when null
- `Icon` imported from `@poodle/svelte-primitives`
- `EmptyStateVariant` and `EmptyStateSize` types imported from shared `types.ts`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::empty_state`
- spec struct: `EmptyStateSpec` with title, message, variant, size, density
- GPUI may use native layout and icon rendering, but the textual message and
  accessible action names remain the semantic core

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] title/message semantics match
- [ ] variant-driven icon and background match
- [ ] decorative-vs-meaningful visual behavior matches (aria-hidden)
- [ ] action ordering and naming match when present
- [ ] ariaLabel fallback to title matches

### Tier 2: Visual Parity

- [ ] spacing, hierarchy, and optional visual emphasis use comparable token roles
- [ ] dashed border and radius match
- [ ] variant background alpha mixing matches
- [ ] compact size visual and text reductions match
- [ ] density spacing adjustments match

### Tier 3: Implementation Freedom

- [ ] illustration style and alignment details stay internal
- [ ] icon rendering approach may differ

## 12. Specimen Definitions

### Neutral

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Neutral | `title="No projects yet"`, `message="Create your first project to get started."`, actions slot with primary "Create project" button | centered title, inbox icon, supporting message, dashed border, and primary action button |

### Search

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Search | `variant="search"`, `title="No results found"`, `message="Try adjusting your search terms or clearing filters."`, actions slot with secondary "Clear filters" button | search-posture empty state with accent-tinted background, search icon, title, message, and secondary action |

### First Run

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| First run | `variant="firstRun"`, `title="Welcome to your workspace"`, `message="This is where your team's components will appear once you start building."` | invitational posture with success-tinted background, plus icon, title and message only, no action controls |

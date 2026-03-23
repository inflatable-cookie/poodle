# EmptyState

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `EmptyState`
- Layer: `composites`
- Summary: a standardized empty-result or no-content message with optional
  supporting actions
- In scope: title, descriptive copy, optional illustration/icon, action group,
  browse vs first-run posture
- Out of scope: async loading progress, blocking errors, app-specific onboarding
  flows

## 2. Anatomy

```text
[Root State]
  ├── [Visual] (optional)
  ├── [Title]
  ├── [Message] (optional)
  └── [Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root State | yes | empty-state container | spacing, alignment |
| Visual | no | decorative or meaningful icon/illustration | icon, size |
| Title | yes | primary empty-state message | typography, text color |
| Message | no | supporting explanation | typography, text-muted |
| Actions | no | remediation or creation actions | action spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | none | yes | primary message |
| `message` | `string \| null` | `null` | no | supporting explanation |
| `variant` | `"neutral" \| "search" \| "firstRun"` | `"neutral"` | no | semantic posture |
| `ariaLabel` | `string \| null` | `null` | no | optional label when the state is independently addressable |

### Controlled And Uncontrolled

- declarative state composite
- action behavior remains host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | base variant | generic no-content posture |
| search | `variant="search"` | empty-result posture for filtered/browsed content |
| firstRun | `variant="firstRun"` | more invitational creation posture |
| actionable | actions present | remediation/creation controls visible |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child action behavior.

## 6. Accessibility

### Semantics

- Role: usually grouped informational content, status region, or neutral section
  depending on context
- Required attributes: meaningful textual message
- Optional attributes: region label when independently navigable
- Labeling rules: decorative visual elements must stay out of the accessibility
  tree unless they communicate unique meaning not present in text

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches any visible actions in logical order |

### Focus And Announcement

- focus entry: the empty-state container is not focusable by default
- focus exit: actions and surrounding shell controls should remain in a
  sensible order after empty content appears
- live-region behavior: empty-state appearance may be announced by the host when
  it results from a user-triggered search or filter change
- GPUI-native accessibility mapping notes: GPUI must preserve the text message
  and action labeling explicitly and avoid relying on visual illustration alone

## 7. Layout

### Sizing

- empty states center or align within available parent space according to host
  context
- content remains readable in narrow and wide containers

### Composition

- parent expectations: `DetailShell`, settings scopes
- child expectations: text, icon/illustration, and action primitives
- resizing rules: actions may stack below the message on narrow widths

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root State | spacing and alignment roles | state layout |
| Visual | icon/color roles | optional cue |
| Title | heading/text roles | primary message |
| Message | subdued text roles | explanation |
| Actions | action spacing roles | remediation controls |

### Token Usage — Exact CSS Values

#### `.empty-state` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `justify-items` | `center` |
| `text-align` | `center` |
| `gap` | `var(--poodle-space-stack-md)` |
| `padding` | `calc(var(--poodle-space-panel-y) * 1.5) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem dashed var(--poodle-color-border-default)` |
| `border-radius` | `calc(var(--poodle-radius-surface) - 0.125rem)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent)` |

#### `.empty-state[data-variant="search"]`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 7%, transparent)` |

#### `.empty-state[data-variant="firstRun"]`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-status-success) 7%, transparent)` |

#### `.empty-state__visual`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `2.25rem` |
| `height` | `2.25rem` |
| `border-radius` | `999rem` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `1.125rem` |
| `font-weight` | `600` |

#### `.empty-state__copy`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |
| `max-width` | `24rem` |

#### `.empty-state__copy h3`, `.empty-state__copy p`

| Property | Value |
|----------|-------|
| `margin` | `0` |

#### `.empty-state__copy h3`

| Property | Value |
|----------|-------|
| `font-size` | `1.125rem` |
| `line-height` | `1.2` |

#### `.empty-state__copy p`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |

#### `.empty-state__actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |

### Variant Icons

| Variant | Icon |
|---------|------|
| `neutral` | `inbox` |
| `search` | `search` |
| `firstRun` | `plus` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-variant` | `.empty-state` root `<section>` | controls variant-specific background color |

## 9. Svelte Notes

- expected substrate: `Stack`, `Inline`, icon primitives, and button family
- wrapper strategy: visuals should be optional and removable without changing
  the textual core of the state

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::empty_state`
- implementation-only details: GPUI may use native layout and icon rendering,
  but the textual message and accessible action names remain the semantic core

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] title/message semantics match
- [ ] decorative-vs-meaningful visual behavior matches
- [ ] action ordering and naming match when present

### Tier 2: Visual Parity

- [ ] spacing, hierarchy, and optional visual emphasis use comparable token roles

### Tier 3: Implementation Freedom

- [ ] illustration style and alignment details stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| illustration style may differ | runtime rendering tools differ | allowed | keep textual semantics strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Neutral

A neutral empty state with action:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Neutral | `title="No projects yet"`, `message="Create your first project to get started."`, actions slot with primary Create project button | centered title, supporting message, and primary action button |

### Search

A search-variant empty state with action:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Search | `variant="search"`, `title="No results found"`, `message="Try adjusting your search terms or clearing filters."`, actions slot with secondary Clear filters button | search-posture empty state with title, message, and secondary action |

### First run

A first-run variant empty state without actions:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| First run | `variant="firstRun"`, `title="Welcome to your workspace"`, `message="This is where your team's components will appear once you start building."` | invitational posture with title and message only, no action controls |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: browsers, settings scopes, detail shells, onboarding-lite
  surfaces
- future follow-up: connect deeper empty/error/loading workflow suites in `g02`

## Next Task

Use `EmptyState` for no-content messaging and keep onboarding flows, tutorials,
and app-specific remediation logic outside the generic composite contract.

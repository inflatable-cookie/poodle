# InlineListSection

Status: active
Updated: 2026-07-10

## 1. Purpose

- Component name: `InlineListSection`
- Layer: `composites`

`InlineListSection` provides the compact card-contained related-list shell used
inside detail tabs and metadata pages for versions, usages, aliases, and other
small related collections.

It owns:

- the outer `Card`
- the compact uppercase section header
- optional header actions
- the stacked list container
- the compact muted row chrome
- the default empty-state copy posture when the host wants the empty case inside
  the card

It does not own:

- data loading
- row-specific actions
- domain-specific pill/status meaning
- row navigation or mutation handlers
- parent route shells like `PageHeader`, `MetaBar`, or top-level `Tabs`

## Anatomy

```text
[Card]  (only when framed=true)
  └── [Root .poodle-inline-list-section]  <section aria-label={title}>
        ├── [Header .poodle-inline-list-section__header]  <div>
        │     ├── [Heading .poodle-inline-list-section__heading]  <div>
        │     │     ├── [Title .poodle-inline-list-section__title]  <h4>
        │     │     └── [Count .poodle-inline-list-section__count]  <span>  (when count !== null)
        │     └── [HeaderActions .poodle-inline-list-section__header-actions]  <div>  (when actions)
        └── (items.length === 0)
              ├── [Empty .poodle-inline-list-section__empty]  <p>  (when emptyMessage)
              └── [Items .poodle-inline-list-section__items]  <ul>  (otherwise)
                    └── [Item .poodle-inline-list-section__item]  <li>  per entry
```

When `framed=false` the outer `Card` is dropped and the bare `<section>` is
rendered directly.

## Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | — | yes | Section heading text (rendered uppercase) |
| `items` | `T[]` | — | yes | Row data; each rendered via the `item` snippet |
| `item` | `Snippet<[T]>` | — | yes | Renders one row from an entry |
| `actions` | `Snippet` | — | no | Optional header actions, right-aligned |
| `emptyMessage` | `string \| null` | `"No items yet."` | no | Empty-state copy; suppressed when `null` |
| `count` | `number \| string \| null` | `null` | no | Optional pill-style count badge beside the title; hidden when `null` |
| `framed` | `boolean` | `true` | no | When `true`, wraps the section in a `Card`; when `false`, renders the bare section with no card |

## Rules

- use this for compact related-item sections that live under a stable parent
  detail shell
- keep item content host-owned via the `item` snippet
- keep row actions and status pills host-owned
- prefer this over route-local duplicated `Card` + header + compact list chrome
  when multiple apps share the same shell posture
- do not use this for full browse lists, selection-mode lists, or card-grid tab
  content; those should stay on `ListGrid`, `ListCard`, `DataTable`, or host
  list composition

## Example

```svelte
<InlineListSection
  title="Versions"
  items={versions}
  emptyMessage="No versions uploaded yet."
>
  {#snippet actions()}
    <IconButton icon="upload" variant="primary" size="sm" ariaLabel="Upload new version" />
  {/snippet}

  {#snippet item(version)}
    <div class="version-row__content">
      <span class="version-row__dot"></span>
      <span class="version-row__label-group">
        <span class="version-row__label">{version.sha256}</span>
        <span class="version-row__sublabel">{version.mimeType}</span>
      </span>
    </div>

    <div class="version-row__trailing">
      <Pill tone="success" appearance="badge" size="lg">Ready</Pill>
    </div>
  {/snippet}
</InlineListSection>
```

## Token Usage — Exact Values

### Root `.poodle-inline-list-section`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |

### Header `.poodle-inline-list-section__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `0.75rem` |

### Heading `.poodle-inline-list-section__heading`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `min-width` | `0` |

### Title `.poodle-inline-list-section__title`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |
| `color` | `var(--poodle-color-text-secondary)` |

### Count `.poodle-inline-list-section__count`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-width` | `1.875rem` |
| `height` | `1.375rem` |
| `padding` | `0 0.5rem` |
| `border-radius` | `999rem` |
| `border` | `1px solid var(--poodle-color-border)` |
| `background` | `var(--poodle-surface-elevated)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `1` |

### Header Actions `.poodle-inline-list-section__header-actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` (`0.25rem` at `max-width: 45rem`) |

### Items `.poodle-inline-list-section__items`

| Property | Value |
|----------|-------|
| `list-style` | `none` |
| `margin` | `0` |
| `padding` | `0` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `var(--poodle-space-stack-sm)` |

### Item `.poodle-inline-list-section__item`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.75rem` |
| `min-width` | `0` |
| `padding` | `0.5rem 0.625rem` |
| `border-radius` | `calc(var(--poodle-radius-surface) - 0.1875rem)` |
| `background` | `color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))` |

### Empty `.poodle-inline-list-section__empty`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `font-style` | `italic` |
| `color` | `var(--poodle-color-text-secondary)` |

### Composed Primitives

| Part | Delegates To |
|------|-------------|
| Card | Card contract (foundation), present only when `framed=true` |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 2. Accessibility

- root `Card` wrapper: present only when `framed=true`; uses default card
  semantics with no additional ARIA role
- root `<section>`: carries `aria-label={title}`
- section header: rendered as a visible heading; heading level is determined by
  the host context
- empty state: empty-message text is rendered inline inside the card so screen
  readers announce it naturally
- item rows: host-owned content is responsible for its own row-level
  accessibility attributes
- action buttons in the header: host must supply `ariaLabel` on each action

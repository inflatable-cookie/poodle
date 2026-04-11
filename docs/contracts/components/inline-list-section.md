# InlineListSection

Status: active
Updated: 2026-04-09

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

## Props

- `title: string`
- `items: T[]`
- `item: Snippet<[T]>`
- `actions?: Snippet`
- `emptyMessage?: string | null`

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

## 2. Accessibility

- root `Card` wrapper: uses default card semantics with no additional ARIA role
- section header: rendered as a visible heading; heading level is determined by
  the host context
- empty state: empty-message text is rendered inline inside the card so screen
  readers announce it naturally
- item rows: host-owned content is responsible for its own row-level
  accessibility attributes
- action buttons in the header: host must supply `ariaLabel` on each action

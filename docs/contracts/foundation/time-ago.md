# TimeAgo

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `TimeAgo`
- Layer: `foundation`
- Summary: a non-interactive time display primitive that renders a human-readable
  relative timestamp (e.g., "5m ago") with optional live updating
- In scope: relative time formatting, live interval updates, absolute time in
  title/tooltip, past and future time support
- Out of scope: date pickers, countdown timers, duration formatting, interactive
  time selection

## 2. Anatomy

```text
[Root .time-ago]  <time>
  └── [Relative text]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | semantic `<time>` element | color, typography |
| Relative text | yes | human-readable relative string | text content |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `datetime` | `Date \| string \| number` | — | yes | timestamp to display relative to now |
| `live` | `boolean` | `true` | no | enable periodic re-computation of relative text |
| `interval` | `number` | `30000` | no | live update interval in milliseconds |
| `ariaLabel` | `string \| null` | `null` | no | override accessible label |

### Controlled And Uncontrolled

- display primitive; internal state limited to `now` timestamp updated by
  interval timer

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| live | `live=true` (default) | relative text updates every `interval` ms |
| static | `live=false` | relative text computed once and not updated |
| past | datetime is before now | shows "{value} ago" format |
| future | datetime is after now | shows "in {value}" format |
| just-now | difference less than 5 seconds | shows "just now" |

### Component States

- `now`: internal timestamp, initialized to `Date.now()`, updated by
  `setInterval` when `live=true`

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | display primitive only |

## 6. Accessibility

### Semantics

- Element: `<time>` with `datetime` attribute set to ISO 8601 string
- `title` attribute: absolute formatted date via `toLocaleString` with
  `{ year: "numeric", month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" }`
- `aria-label`: `ariaLabel` prop if provided, otherwise
  `"{relativeText} ({absoluteText})"`

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive, not focusable |

### Focus And Announcement

- Not focusable by default
- No live-region behavior (updates are visual-only; parent should manage
  announcements if needed)
- Tooltip via native `title` attribute provides absolute time on hover

## 7. Layout

### Sizing

- Inline text element, sizes to content
- `cursor: default` prevents text cursor on hover
- `font-variant-numeric: tabular-nums` ensures stable width as numbers change

### Composition

- parent expectations: table cells, metadata rows, card footers, list items,
  activity feeds
- child expectations: none (text-only content)
- resizing rules: flows inline with surrounding text

## 8. Token Usage — Exact Values

### Root `.time-ago`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `font-variant-numeric` | `tabular-nums` |
| `cursor` | `default` |

### HTML attributes

| Attribute | Value |
|-----------|-------|
| `datetime` | ISO 8601 string of the timestamp |
| `title` | absolute date via `toLocaleString({ year: "numeric", month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" })` |
| `aria-label` | `ariaLabel ?? "{relativeText} ({absoluteText})"` |

### Relative text formatting rules

| Condition | Past format | Future format |
|-----------|-------------|---------------|
| diff < 5s | `"just now"` | `"just now"` |
| diff < 60s | `"{seconds}s ago"` | `"in {seconds}s"` |
| diff < 1h | `"{minutes}m ago"` | `"in {minutes}m"` |
| diff < 24h | `"{hours}h ago"` | `"in {hours}h"` |
| diff < 30d | `"{days}d ago"` | `"in {days}d"` |
| diff < 365d | `"{months}mo ago"` | `"in {months}mo"` |
| diff >= 365d | `"{years}y ago"` | `"in {years}y"` |

Values are computed using integer division (floor). Thresholds use seconds:
60s, 3600s, 86400s, 2592000s (30d), 31536000s (365d).

## 9. Svelte Notes

- Renders a `<time>` element with class `time-ago`
- Live updates via `setInterval` with `interval` prop as delay; cleared on
  component destroy via `onDestroy`
- `datetime` prop accepts `Date`, ISO string, or Unix timestamp (number);
  internally normalized to a `Date` object
- No slots, no child components, no dispatched events

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::time_ago`
- GPUI must implement periodic re-render when `live=true` using a timer or
  frame-based update mechanism
- `toLocaleString` for title: GPUI should use platform locale formatting or
  `chrono` crate equivalent
- `font-variant-numeric: tabular-nums`: GPUI must use a font feature setting
  for tabular figures to prevent layout jitter
- Relative text formatting: GPUI must replicate the exact threshold table and
  abbreviation format (e.g., "5m ago", not "5 minutes ago")

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] renders `<time>` element (or semantic equivalent)
- [ ] `datetime` attribute contains ISO 8601 string
- [ ] `title` attribute shows absolute formatted date
- [ ] `aria-label` combines relative and absolute text
- [ ] relative text thresholds and abbreviations match exactly
- [ ] live update interval defaults to 30000ms
- [ ] timer cleanup on destroy

### Tier 2: Visual Parity

- [ ] color uses `--pug-color-text-secondary`
- [ ] font-family uses `--pug-typography-body-family`
- [ ] font-size uses `--pug-typography-body-size`
- [ ] font-variant-numeric tabular-nums matches
- [ ] cursor default matches

### Tier 3: Implementation Freedom

- [ ] timer/interval mechanism is platform-owned
- [ ] date parsing internals stay implementation-specific
- [ ] locale formatting for title attribute is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| locale formatting may differ slightly | `toLocaleString` output varies by platform/locale | allowed | keep format options consistent |
| timer precision may differ | GPUI frame-based updates vs JS setInterval | allowed | keep default interval at 30000ms |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: activity feeds, table cells, card metadata, list item
  timestamps, comment threads
- future follow-up: none expected

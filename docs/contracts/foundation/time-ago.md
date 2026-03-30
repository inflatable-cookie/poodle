# TimeAgo

Status: detailed contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `TimeAgo`
- Layer: `foundation`
- Summary: a non-interactive time display primitive that renders a human-readable
  relative timestamp in short or long form with optional live updating
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
| `short` | `boolean` | `true` | no | use compact output like `"5m ago"` instead of long phrases |
| `tooltipFormat` | `"full" \| "date" \| "datetime"` | `"datetime"` | no | absolute-time format used for the native title tooltip |
| `timezone` | `string \| null` | `null` | no | optional IANA timezone for tooltip formatting |

### Controlled And Uncontrolled

- display primitive; internal state limited to `now` timestamp updated by
  interval timer

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| live | `live=true` (default) | relative text updates every `interval` ms |
| static | `live=false` | relative text computed once and not updated |
| short | `short=true` | compact relative labels like `"5m ago"` |
| long | `short=false` | long-form labels like `"5 minutes ago"` and `"yesterday"` |
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
- `title` attribute: absolute formatted date via `tooltipFormat` and optional `timezone`
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
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
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
| diff < 5s | `"now"` or `"just now"` | `"now"` or `"just now"` |
| diff < 60s | `"{seconds}s ago"` / `"{seconds} seconds ago"` | `"in {seconds}s"` / `"in {seconds} seconds"` |
| diff < 1h | `"{minutes}m ago"` / `"{minutes} minutes ago"` | `"in {minutes}m"` / `"in {minutes} minutes"` |
| diff < 24h | `"{hours}h ago"` / `"{hours} hours ago"` | `"in {hours}h"` / `"in {hours} hours"` |
| diff < 30d | `"{days}d ago"` / `"{days} days ago"` | `"in {days}d"` / `"in {days} days"` |
| diff < 365d | `"{months}mo ago"` / `"{months} months ago"` | `"in {months}mo"` / `"in {months} months"` |
| diff >= 365d | `"{years}y ago"` / `"{years} years ago"` | `"in {years}y"` / `"in {years} years"` |

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

- expected crate/module surface: `poodle_gpui::primitives::time_ago`
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

- [ ] color uses `--poodle-color-text-secondary`
- [ ] font-family uses `--poodle-typography-body-family`
- [ ] font-size uses `--poodle-typography-body-size`
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

## 13. Specimen Definitions

### Group: Recent timestamps

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| 2 minutes ago | `datetime=(now - 2min)` | Relative text showing "2m ago"; title tooltip shows absolute date |
| 3 hours ago | `datetime=(now - 3h)` | Relative text showing "3h ago" |
| 2 days ago | `datetime=(now - 2d)` | Relative text showing "2d ago" |

### Group: Future timestamp

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| In 5 minutes | `datetime=(now + 5min)` | Relative text showing "in 5m" |

### Group: Static (live updates off)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Static | `datetime=(now - 2min)`, `live=false` | Relative text computed once; does not update over time |

### Group: From ISO string

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| ISO string | `datetime="2026-03-14T00:00:00Z"` | Relative text computed from fixed ISO date; live-updates by default |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: activity feeds, table cells, card metadata, list item
  timestamps, comment threads
- future follow-up: none expected

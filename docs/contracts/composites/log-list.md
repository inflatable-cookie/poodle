# LogList

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `LogList`
- Layer: `composites`
- Summary: a scrollable log output viewer with level-based filtering, text search, auto-scroll, and color-coded entries
- In scope: log entry display (timestamp, level, message), level filter buttons with counts, text search, auto-scroll to latest, scroll-to-bottom button, level-based color coding, max entry cap
- Out of scope: log ingestion/streaming, log persistence, log export, regex search, column sorting, log grouping/folding, virtualization

## 2. Anatomy

```text
[Root]  role="log"
  ├── [Toolbar]
  │     ├── [Filters]
  │     │     ├── [Filter Button: All]
  │     │     ├── [Filter Button: Info]
  │     │     ├── [Filter Button: Warn]
  │     │     └── [Filter Button: Error]
  │     └── [Search Input]
  ├── [Scroll Container]
  │     ├── [Empty Message]  (when no entries match)
  │     └── [Entry...]
  │           ├── [Timestamp]
  │           ├── [Level Badge]
  │           └── [Message]
  └── [Scroll-to-Bottom Button]  (floating, when user has scrolled up)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | `role="log"`, `aria-label`, flex column with border |
| toolbar | `<div>` | Flex row with filter buttons and search input |
| filter-button | `<button>` | Toggles level filter; shows level name and count |
| search-input | `<input>` | Text filter for log messages, `aria-label="Filter log messages"` |
| scroll-container | `<div>` | Scrollable area, max-height `20rem`, monospace font |
| entry | `<div>` | Flex row with timestamp, level, message; `data-level` attribute for styling |
| timestamp | `<time>` | Formatted as `HH:MM:SS.mmm` (24-hour with milliseconds) |
| level-badge | `<span>` | Uppercase level text, fixed width, color-coded by level |
| message | `<span>` | Log message text, word-break for long content |
| empty-message | `<div>` | Centered message when no entries match filters |
| scroll-button | `<button>` | Floating pill button to scroll to latest entries |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `entries` | `LogEntry[]` | `[]` | no | Array of log entries to display |
| `maxEntries` | `number` | `500` | no | Maximum number of entries to render (shows last N) |
| `autoScroll` | `boolean` | `true` | no | Auto-scroll to bottom on new entries |
| `filterLevel` | `LogLevel \| null` | `null` | no | Active level filter; `null` shows all |
| `filterText` | `string` | `""` | no | Text search filter for messages |
| `ariaLabel` | `string` | `"Log output"` | no | Accessible label for the log region |

### Types

```ts
type LogLevel = "info" | "warn" | "error";

type LogEntry = {
  id?: string;
  timestamp: Date | string | number;
  level: LogLevel;
  message: string;
};
```

### Slots

None.

### Controlled / Uncontrolled

`entries` is controlled externally (append new entries to the array). `filterLevel` and `filterText` are managed internally but can be set from outside.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| empty | No entries or no matches | Centered empty message |
| filter-active | A level filter button is active | Button shows accent/warning/danger tinted background |
| entry-info | `level="info"` | Level badge in accent color |
| entry-warn | `level="warn"` | Level badge in warning color, row has warning-tinted background |
| entry-error | `level="error"` | Level badge in danger color, row has danger-tinted background |
| entry-hover | Mouse over entry row | Slightly elevated background |
| user-scrolled | User scrolls up from bottom | Floating scroll-to-bottom button appears |
| search-focus | Search input focused | Border changes to `accent-focusRing` |

### Component States

| State | Description |
|-------|-------------|
| auto-scrolling | New entries automatically scroll into view |
| user-scrolled | User has scrolled away from bottom; auto-scroll paused; scroll button shown |
| filtering | Level or text filter active, reducing visible entries |

## 5. Events

None. The component is display-only; entries are provided externally.

## 6. Accessibility

### Semantics

- Root has `role="log"` and `aria-label` -- the `log` role indicates a live region where new content is appended
- Search input has `aria-label="Filter log messages"`
- Scroll-to-bottom button has `aria-label="Scroll to latest"`

### Keyboard

- Tab navigates between filter buttons, search input, and scroll-to-bottom button
- Filter buttons toggle on click/Enter/Space
- Search input accepts standard text input

### Focus

- Search input: border changes to `accent-focusRing` on focus
- Filter buttons inherit standard button focus behavior

## 7. Layout

### Sizing

- Root: flex column, border `0.0625rem solid border-subtle`, `radius-surface`, overflow hidden, position relative
- Toolbar: flex row, wrapping, gap `0.5rem`, padding `0.375rem 0.5rem`, border-bottom
- Filter button: padding `0.1875rem 0.5rem`, font-size `0.6875rem`, monospace, `radius-control`
- Search input: flex 1, min-width `8rem`, padding `0.1875rem 0.5rem`, font-size `0.6875rem`, monospace
- Scroll container: max-height `20rem`, overflow-y auto, font-size `0.75rem`, line-height `1.6`
- Entry: flex row, gap `0.625rem`, padding `0.125rem 0.5rem`, border-bottom subtle
- Timestamp: flex-shrink 0, nowrap
- Level badge: flex-shrink 0, width `3rem`, right-aligned, font-weight 600
- Message: flex 1, min-width 0, word-break
- Scroll button: absolute positioned, bottom `0.5rem`, centered, pill shape, shadow

### Composition

Uses `Icon` primitive for the scroll-to-bottom button arrow icon.

## 8. Token Usage

| Property | Token |
|----------|-------|
| Root border | `color-border-subtle` |
| Root radius | `radius-surface` |
| Root background | `color-background-panel` |
| Toolbar background | `color-background-elevated` at 92% mix |
| Toolbar border | `color-border-subtle` |
| Filter button border | `color-border-default` |
| Filter button radius | `radius-control` |
| Filter button text | `color-text-secondary` |
| Filter button font | `typography-code-family` |
| Filter active (all/info) bg | `color-accent-base` at 16% |
| Filter active (all/info) border | `color-accent-base` at 42% |
| Filter active text | `color-text-primary` |
| Filter active (warn) bg | `color-status-warning` at 16% |
| Filter active (warn) border | `color-status-warning` at 42% |
| Filter active (error) bg | `color-status-danger` at 16% |
| Filter active (error) border | `color-status-danger` at 42% |
| Search border | `color-border-default` |
| Search bg | `color-background-surface` |
| Search text | `color-text-primary` |
| Search focus border | `color-accent-focusRing` |
| Search font | `typography-code-family` |
| Scroll container font | `typography-code-family` |
| Entry border | `color-border-subtle` at 42% |
| Entry hover bg | `color-background-elevated` at 42% |
| Entry warn bg | `color-status-warning` at 6% |
| Entry error bg | `color-status-danger` at 8% |
| Timestamp color | `color-text-tertiary` |
| Level info color | `color-accent-base` |
| Level warn color | `color-status-warning` |
| Level error color | `color-status-danger` |
| Message color | `color-text-primary` |
| Empty text color | `color-text-tertiary` |
| Scroll button border | `color-border-default` |
| Scroll button bg | `color-background-elevated` |
| Scroll button text | `color-accent-base` |
| Scroll button shadow | `elevation-overlay` |
| Scroll button hover bg | `color-accent-base` at 12% |
| Motion duration | `motion-duration-interaction` |
| Motion easing | `motion-easing-standard` |

## 9. Svelte Notes

- `displayEntries` reactive: filters by level and text, then slices to `maxEntries` from the end
- `levelCounts` reactive: counts entries per level for filter button badges
- `afterUpdate` auto-scrolls when `autoScroll` is true and user has not scrolled away
- `handleScroll` detects user scroll: sets `isUserScrolled` when more than 32px from bottom
- `scrollToBottom()` resets `isUserScrolled` and uses `tick()` to scroll after DOM update
- `formatTimestamp` converts to `HH:MM:SS.mmm` using `toLocaleTimeString` with `fractionalSecondDigits: 3`
- Entry key uses `entry.id` or falls back to `${entry.timestamp}-${entry.message}`

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

| Feature | Svelte | GPUI | Jetstream |
|---------|--------|------|-----------|
| Level filter buttons with counts | Yes | -- | -- |
| Text search filter | Yes | -- | -- |
| Auto-scroll to bottom | Yes | -- | -- |
| Scroll-to-bottom button | Yes | -- | -- |
| Level-coded entry colors | Yes | -- | -- |
| Max entries cap | Yes | -- | -- |
| Timestamp formatting | Yes | -- | -- |
| Empty state | Yes | -- | -- |
| Entry hover highlight | Yes | -- | -- |

## 12. Known Deltas

None yet (single implementation).

## 13. Specimen Definitions

### Log Output With Filtering

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Log output with filtering | 10 pre-populated entries (mixed info/warn/error), `ariaLabel="Application logs"`, button to add random entries | Toolbar with level filters and search; scrollable log entries color-coded by level; new entries auto-scroll |

## 14. Approval And Adoption Notes

Use `LogList` for displaying application logs, build output, or event streams in developer tools and admin panels. The component caps visible entries at `maxEntries` (default 500) for performance; for larger log volumes, the consuming application should handle windowing or pagination before passing entries. The `role="log"` semantics indicate to assistive technology that this is a live region where content is appended.

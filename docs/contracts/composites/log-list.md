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

## 8. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-level` | entry `<div>` | `"info"`, `"warn"`, `"error"` |

### Root

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| border | `0.0625rem solid var(--pug-color-border-subtle)` |
| border-radius | `var(--pug-radius-surface)` |
| background | `var(--pug-color-background-panel)` |
| overflow | `hidden` |
| position | `relative` |

### Toolbar

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.5rem` |
| padding | `0.375rem 0.5rem` |
| border-bottom | `0.0625rem solid var(--pug-color-border-subtle)` |
| background | `color-mix(in srgb, var(--pug-color-background-elevated) 92%, transparent)` |
| flex-wrap | `wrap` |

### Filters Container

| Property | Value |
|----------|-------|
| display | `flex` |
| gap | `0.25rem` |

### Filter Button

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| gap | `0.25rem` |
| padding | `0.1875rem 0.5rem` |
| border | `0.0625rem solid var(--pug-color-border-default)` |
| border-radius | `var(--pug-radius-control)` |
| background | `transparent` |
| color | `var(--pug-color-text-secondary)` |
| font-size | `0.6875rem` |
| font-family | `var(--pug-typography-code-family)` |
| line-height | `1` |
| transition | `background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard)` |

#### Filter Button States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | background | `color-mix(in srgb, var(--pug-color-background-elevated) 72%, transparent)` |
| `.active` (all/info) | background | `color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent)` |
| `.active` (all/info) | border-color | `color-mix(in srgb, var(--pug-color-accent-base) 42%, transparent)` |
| `.active` (all/info) | color | `var(--pug-color-text-primary)` |
| `.active` (warn) | background | `color-mix(in srgb, var(--pug-color-status-warning, #eab308) 16%, transparent)` |
| `.active` (warn) | border-color | `color-mix(in srgb, var(--pug-color-status-warning, #eab308) 42%, transparent)` |
| `.active` (error) | background | `color-mix(in srgb, var(--pug-color-status-danger, #ef4444) 16%, transparent)` |
| `.active` (error) | border-color | `color-mix(in srgb, var(--pug-color-status-danger, #ef4444) 42%, transparent)` |

### Count Badge

| Property | Value |
|----------|-------|
| opacity | `0.7` |
| font-size | `0.625rem` |

### Search Input

| Property | Value |
|----------|-------|
| flex | `1` |
| min-width | `8rem` |
| padding | `0.1875rem 0.5rem` |
| border | `0.0625rem solid var(--pug-color-border-default)` |
| border-radius | `var(--pug-radius-control)` |
| background | `var(--pug-color-background-surface)` |
| color | `var(--pug-color-text-primary)` |
| font-size | `0.6875rem` |
| font-family | `var(--pug-typography-code-family)` |
| outline | `none` |
| `:focus` border-color | `var(--pug-color-accent-focusRing)` |

### Scroll Container

| Property | Value |
|----------|-------|
| max-height | `20rem` |
| overflow-y | `auto` |
| font-family | `var(--pug-typography-code-family)` |
| font-size | `0.75rem` |
| line-height | `1.6` |

### Entry

| Property | Value |
|----------|-------|
| display | `flex` |
| gap | `0.625rem` |
| padding | `0.125rem 0.5rem` |
| border-bottom | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 42%, transparent)` |

#### Entry States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | background | `color-mix(in srgb, var(--pug-color-background-elevated) 42%, transparent)` |
| `[data-level="warn"]` | background | `color-mix(in srgb, var(--pug-color-status-warning, #eab308) 6%, transparent)` |
| `[data-level="error"]` | background | `color-mix(in srgb, var(--pug-color-status-danger, #ef4444) 8%, transparent)` |

### Timestamp

| Property | Value |
|----------|-------|
| color | `var(--pug-color-text-tertiary)` |
| flex-shrink | `0` |
| white-space | `nowrap` |

### Level Badge

| Property | Value |
|----------|-------|
| flex-shrink | `0` |
| width | `3rem` |
| text-align | `right` |
| font-weight | `600` |

#### Level Badge Colors By Data-Level

| data-level | color |
|------------|-------|
| `info` | `var(--pug-color-accent-base, #6366f1)` |
| `warn` | `var(--pug-color-status-warning, #eab308)` |
| `error` | `var(--pug-color-status-danger, #ef4444)` |

### Message

| Property | Value |
|----------|-------|
| flex | `1` |
| min-width | `0` |
| word-break | `break-word` |
| color | `var(--pug-color-text-primary)` |

### Empty Message

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| min-height | `4rem` |
| color | `var(--pug-color-text-tertiary)` |
| font-size | `0.8125rem` |

### Scroll-To-Bottom Button

| Property | Value |
|----------|-------|
| position | `absolute` |
| bottom | `0.5rem` |
| left | `50%` |
| transform | `translateX(-50%)` |
| padding | `0.25rem 0.75rem` |
| border | `0.0625rem solid var(--pug-color-border-default)` |
| border-radius | `999rem` |
| background | `var(--pug-color-background-elevated)` |
| color | `var(--pug-color-accent-base)` |
| font-size | `0.6875rem` |
| box-shadow | `var(--pug-elevation-overlay)` |
| transition | `background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard)` |
| `:hover` background | `color-mix(in srgb, var(--pug-color-accent-base) 12%, transparent)` |

### Light Theme Overrides

None.

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

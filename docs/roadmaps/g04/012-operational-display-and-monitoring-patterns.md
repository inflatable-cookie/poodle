# g04.012 Operational Display And Monitoring Patterns

Status: planned
Owner: Pug Core
Updated: 2026-03-14
Depends on: g04.002 through g04.010
Primary repos: `pug`

## Goals

- [ ] implement LogList as a composite for timestamped event/log display
- [ ] extend StateTile with trend indicators and optional sparkline/micro-chart

## Execution Checklist

- [ ] write contract for LogList: entries with timestamp/level/message, level
  filtering, auto-scroll, search, max entries with virtualization
- [ ] implement LogList composite in `@pug/svelte-composites`
- [ ] amend StateTile contract: add `trend` prop (up/down/flat), `trendLabel`,
  optional sparkline data array
- [ ] implement StateTile trend indicator (directional arrow + percentage)
- [ ] implement StateTile sparkline as a minimal inline SVG chart
- [ ] create LogList specimen
- [ ] update StateTile specimen with trend examples
- [ ] register LogList in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] LogList renders timestamped entries with level-based color coding
- [ ] LogList supports level filtering (info, warning, error)
- [ ] LogList auto-scrolls to newest entries with scroll-lock toggle
- [ ] StateTile trend indicator shows directional arrow with delta value
- [ ] StateTile sparkline renders an inline line chart from data points
- [ ] both components pass build and render in the preview catalogue

## Next Task

Open `g04.013` and implement rich text and markdown editing.

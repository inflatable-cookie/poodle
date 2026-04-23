# g04.006 Temporal Display And Duration Input

Status: planned
Owner: Poodle Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `poodle`

## Goals

- [ ] implement TimeAgo as a primitive for displaying relative timestamps that
  auto-update
- [ ] implement DurationInput as a primitive for hours/minutes/seconds entry

## Execution Checklist

- [ ] write contract for TimeAgo: datetime prop, live-update interval, tooltip
  with absolute time, locale support, thresholds for switching from "seconds
  ago" to "minutes ago" etc.
- [ ] implement TimeAgo primitive in `@poodle/svelte`
- [ ] write contract for DurationInput: hours/minutes/seconds segments,
  min/max constraints, step increments, format display
- [ ] implement DurationInput primitive in `@poodle/svelte`
- [ ] create specimens for TimeAgo and DurationInput
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] TimeAgo renders relative time strings ("2 minutes ago", "3 hours ago")
- [ ] TimeAgo updates live on a configurable interval
- [ ] TimeAgo shows absolute datetime in a tooltip on hover
- [ ] DurationInput renders segmented hour/minute/second fields
- [ ] DurationInput supports keyboard increment/decrement per segment
- [ ] DurationInput validates against min/max duration constraints
- [ ] both components pass build and render in the preview catalogue

## Next Task

Open `g04.007` and implement list interaction, reordering, and sort controls.

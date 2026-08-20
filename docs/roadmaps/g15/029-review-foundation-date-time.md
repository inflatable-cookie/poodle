# g15.029 — Review Screen-Clear Foundation Date and Time

Status: **complete** — PR #53 merged as `b2e8114b`; operator approved the
changed paired-web DurationInput specimens
Parent: `027-screen-clear-human-review.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.026` (live native evidence)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Pages This Card Owns (7)

- `Calendar`
- `DatePicker`
- `DateRangePicker`
- `DateTimePicker`
- `DateTimeRangePicker`
- `DateTimeZonePicker`
- `DurationInput`

This list is exact and exhaustive. It contains every `keep` page in the date
and time audit family, no other child owns them, and this card owns no others.

## Goal

Apply the parent's human teaching review to all seven pages; keep good pages
unchanged and repair only bounded specimen defects found.

## Acceptance

Per the parent, including a recorded verdict for every page and live operator
review of every changed Svelte and React page before completion.

## Writable Scope

- the seven named specimen pages across Svelte, React, and GPUI
- their audit rows and one August batch log

## Closeout

Four pages stayed unchanged. Calendar and DurationInput received bounded
specimen repairs. DateTimeZonePicker exposed a real paired-web nested-layer
defect: pointer selection in its portalled TimeZoneSelect dismisses the outer
picker without committing. That blocker is routed to `g15.039` before
`g15.030` resumes the serial review.

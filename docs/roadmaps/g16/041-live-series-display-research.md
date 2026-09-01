# g16.041 — Live Series Display Research

Status: research-complete — PR #132; reject a Poodle live-series primitive,
keep future product charts consumer-owned
Opened: 2026-09-01
Depends on: merged `g16.034` at `369a24f8c`; architecture 012; current
MetricTile and WaveformDisplay ownership
Governing refs: `../../architecture/012-semantic-motion-policy.md`,
`../../contracts/components/metric-tile.md`,
`../../contracts/components/waveform-display.md`
Intake: DesEngs candidate 5, merged in PR #126
Source lead: [Liveline](https://github.com/benjitaylor/liveline)

## Goal

Research whether a bounded, live windowed series belongs as a MetricTile mode,
a reusable display primitive, or consumer-owned visualization. Separate it
from static sparklines, audio waveforms, full charts, and trading chrome.

This card authorizes research only. `LiveSeries`, canvas/path exceptions, and
scrubbing are hypotheses rather than accepted APIs.

## Questions

- Which real Poodle consumers need streaming points rather than a static
  sparkline or meter?
- Who owns point identity, time window, downsampling, gaps, current value,
  pause, and optional scrub?
- Is interpolation semantic continuity or decorative motion?
- What bounded renderer-neutral shape can shared Rust and GPUI represent?
- When are canvas or path drawing justified, and what do reduced, frozen,
  capture, and accessibility modes show?

## Required Evidence

- Inspect Liveline from a pinned primary revision; record licensing, data
  model, interpolation, lifecycle, rendering mechanism, and cleanup.
- Audit MetricTile, WaveformDisplay, meter families, and at least two real
  workstation or agent-dashboard consumers.
- Compare SVG, canvas, renderer-neutral polyline, and static-host rendering
  with explicit CPU, memory, point-count, update-rate, and frame budgets.
- Cover bursty updates, gaps, resize, hidden surfaces, unmount, narrow layouts,
  reduced/frozen policy, and readable non-visual summaries.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/live-series-display.md` with an
extend/add/consumer-owned/reject recommendation and benchmark plan. Promotion
requires a named semantic consumer, accepted data ownership, numeric budgets,
and an active-cohort rendering posture.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Do not edit
architecture, contracts, source, packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.

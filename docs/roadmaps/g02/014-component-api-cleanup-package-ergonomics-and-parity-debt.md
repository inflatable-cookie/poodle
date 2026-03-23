# g02.014 Component API Cleanup, Package Ergonomics, And Parity Debt

Status: completed
Owner: Poodle Core
Updated: 2026-03-11
Depends on: g02.008, g02.009, g02.010, g02.011, g02.012
Primary repos: `poodle`

## Goals

- [x] tighten the public package surface before downstream repos depend on it
- [x] identify rough component APIs, naming, and ownership seams across the current catalogue
- [x] record GPUI and downstream parity debt explicitly before adoption work starts

## Execution Checklist

- [x] review exported package surfaces for unnecessary noise or weak naming
- [x] identify implementation shortcuts that should not become public precedent
- [x] capture current GPUI parity gaps and downstream-facing debt explicitly
- [x] define what must be cleaned up before first Underlay or GPUI adoption tranches
- [x] confirm which APIs are stable enough for packaging and release baselines

## Acceptance Criteria

- [x] package ergonomics and API cleanup targets are explicit
- [x] parity debt is explicit enough to gate downstream adoption deliberately

## Deliverables

- [x] package/API cleanup tranche plan
- [x] parity debt register

## Next Task

Open `g02.015` and define packaging, release, and versioning baseline.

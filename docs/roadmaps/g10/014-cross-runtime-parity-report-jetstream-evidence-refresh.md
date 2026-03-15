# g10.014 — Cross-Runtime Parity Report: Jetstream Evidence Refresh

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.013
Primary repos: `pug`

## Goals

- [ ] regenerate the cross-runtime parity report with full Jetstream evidence
  from g10
- [ ] update parity artifacts in all preview packages

## Execution Checklist

- [ ] update component coverage inventory with Jetstream specimen counts
- [ ] add parity tier classification for each Jetstream component
- [ ] incorporate delta register summary from g10.013
- [ ] verify token resolution coverage across all themes and densities
- [ ] include Jetstream test results (test count, pass/fail)
- [ ] regenerate `cross-runtime-parity-report.json` artifact
- [ ] update Svelte `parity-report.json` with Jetstream cross-references
- [ ] update GPUI parity artifacts with Jetstream cross-references
- [ ] verify three-runtime parity report shows consistent data

## Acceptance Criteria

- [ ] parity report covers all three runtimes (Svelte, GPUI, Jetstream)
- [ ] every component has a parity tier in each runtime
- [ ] delta register is reflected in the report
- [ ] automated parity checks pass
- [ ] all parity artifacts are regenerated and committed

## Next Task

Open `g10.015` and verify accessibility and input model.

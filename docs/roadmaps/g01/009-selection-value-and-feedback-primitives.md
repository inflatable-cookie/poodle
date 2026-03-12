# g01.009 Selection, Value, And Feedback Primitives

Status: completed
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.003, g01.004, g01.005, g01.006, g01.007
Primary repos: `pug`

## Context

Choice controls, value controls, and lightweight feedback widgets form the
second structural primitive tranche after text and action controls.

## Goals

- [ ] define checkbox, radio, switch, tri-state switch, segmented control,
  select, slider, and range-slider contracts
- [ ] define progress, skeleton, badge, pill, callout, banner, and status
  indicators
- [ ] define value-step, validation, and assistive-state semantics

## Execution Checklist

- [ ] bound the selection and value-control family
- [ ] bound the feedback and status primitive family
- [ ] define step, range, and validation semantics
- [ ] define disabled, selected, mixed, and pending states
- [ ] document accessibility expectations for each family

## Acceptance Criteria

- [ ] selection and value controls are bounded
- [ ] feedback primitives are bounded
- [ ] state semantics are explicit enough for parity review

## Deliverables

- [ ] selection/value contract set
- [ ] feedback/status contract set

## Next Task

Open `g01.010` and define overlays, tabs, menus, dialogs, and interaction
primitives.

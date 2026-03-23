# g01.008 Action And Text-Entry Primitives

Status: completed
Owner: Poodle Core
Updated: 2026-03-11
Depends on: g01.003, g01.004, g01.005, g01.006, g01.007
Primary repos: `poodle`

## Context

Buttons and text-entry controls are the first user-facing primitives most
downstream systems will touch.

## Goals

- [ ] define button family contracts
- [ ] define text input, text area, search field, editable label, and
  number-entry contracts
- [ ] define disabled, focus, validation, loading, and icon-adorned states

## Execution Checklist

- [ ] define the action primitive family and its state model
- [ ] define the text-entry primitive family and its validation model
- [ ] document icon-leading and icon-trailing affordance rules
- [ ] document editable-label and search-field variants
- [ ] define parity-sensitive focus and submission semantics

## Acceptance Criteria

- [ ] action primitives are documented
- [ ] text-entry primitives are documented
- [ ] parity-sensitive interaction states are explicit

## Deliverables

- [ ] button-family contract set
- [ ] text-entry contract set

## Next Task

Open `g01.009` and define the selection, value, and feedback primitive family.

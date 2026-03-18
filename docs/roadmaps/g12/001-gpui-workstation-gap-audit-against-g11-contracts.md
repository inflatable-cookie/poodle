# g12.001 — GPUI Workstation Gap Audit Against g11 Contracts

Status: planned
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.013
Primary repos: `pug`

## Goals

- [ ] identify what maps directly from g11 contracts to existing GPUI patterns
- [ ] identify what requires new GPUI components or spec structs
- [ ] classify any GPUI-specific adaptations as intentional deltas

## Execution Checklist

- [ ] review each workstation contract from `docs/contracts/workstation/`
- [ ] map to existing GPUI spec structs where applicable
- [ ] list new spec structs or component implementations needed
- [ ] identify any GPUI-specific rendering constraints that require adaptation
- [ ] produce a prioritized implementation plan for 002–003

## Acceptance Criteria

- [ ] every g11 workstation contract has an explicit GPUI implementation status
- [ ] new work is scoped and sequenced for 002 and 003
- [ ] any intentional deltas are pre-documented

## Next Task

Open `g12.002` and begin GPUI workstation implementation batch 1.

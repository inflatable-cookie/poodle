# g01.011 Product Composites And Information Architecture Baseline

Status: completed
Owner: Poodle Core
Updated: 2026-03-11
Depends on: g01.007, g01.008, g01.009, g01.010
Primary repos: `poodle`

## Context

Poodle needs to serve Underlay-style product applications as well as workstation
apps. The first composite tranche should prove that broader utility.

## Goals

- [x] define cards, headers, breadcrumbs, detail rows, detail sections, and
  information-display shells
- [x] define filter-toolbar and empty-state composites
- [x] define how primitives compose into app-facing patterns without leaking
  app-specific logic

## Execution Checklist

- [x] list the first product-style composites to include
- [x] define how each composite composes from the primitive layer
- [x] define what remains generic versus app-owned
- [x] document layout, token, and state expectations for each composite family

## Acceptance Criteria

- [x] first product composite catalogue is explicit
- [x] information architecture patterns are bounded
- [x] composition rules are documented

## Deliverables

- [x] product composite catalogue
- [x] information architecture baseline notes

## Next Task

Open `g01.012` and define the workstation-shell composite layer above the now
live primitive and product-composite surfaces.

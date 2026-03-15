# g09.017 — Cross-Runtime Parity Report Update and Evidence Refresh

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.016
Primary repos: `pug`

## Goals

- [ ] update the cross-runtime parity report to reflect the full g09 GPUI
  surface
- [ ] regenerate parity evidence artifacts

## Execution Checklist

- [ ] update component coverage inventory: list every Svelte component and
  its GPUI counterpart with parity tier classification
- [ ] classify each component's parity tier:
  - **Strict parity** — visual and behavioral match
  - **Visual parity** — visual match with minor behavioral differences
  - **Native adaptation** — intentional deviation for GPUI platform
- [ ] update the intentional delta register with all documented deltas from
  g09.016
- [ ] verify token resolution coverage — every token referenced by a
  component resolves correctly in the GPUI theme provider
- [ ] verify behavioral parity for interactive components: click, focus,
  keyboard navigation, state changes
- [ ] regenerate `cross-runtime-parity-report.json` artifact
- [ ] update `parity-report.json` in Svelte preview artifacts with GPUI
  evidence cross-references
- [ ] verify parity report passes all automated checks

## Acceptance Criteria

- [ ] parity report covers all 127 Svelte components (or documents explicit
  exclusions)
- [ ] every component has a parity tier classification
- [ ] delta register is complete with justifications
- [ ] automated parity checks pass
- [ ] report artifacts are regenerated and committed

## Next Task

Open `g09.018` and close the generation.

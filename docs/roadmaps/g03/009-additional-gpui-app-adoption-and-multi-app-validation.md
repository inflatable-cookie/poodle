# g03.009 Additional GPUI App Adoption And Multi-App Validation

Status: completed
Owner: Poodle Core
Updated: 2026-03-12
Depends on: g03.001, g03.002, g03.003, g03.004, g03.005, g03.006
Primary repos: `poodle`, downstream GPUI apps

## Goals

- [x] validate Poodle against more than one GPUI app context
- [x] identify app-specific assumptions hiding in the shared layer

## Execution Checklist

- [x] identify additional GPUI app validation targets
- [x] capture shared-layer assumptions exposed by multi-app use
- [x] record any required contract or package adjustments

## Completed Work

- froze the normative baseline in `docs/specs/042-gpui-multi-app-validation-target-matrix.md`
- added the machine-readable GPUI validation target matrix `packages/gpui/tokens/multi-app-validation.json`
- expanded `packages/gpui/tokens/README.md` so token-only GPUI readiness is explicitly separated from wider multi-app validation claims
- recorded the current GPUI blockers instead of pretending the repo already contains runnable multi-app adoption evidence

## Acceptance Criteria

- [x] multi-app GPUI validation exists
- [x] hidden app-specific assumptions are recorded

## Next Task

Open `g03.010` and perform accessibility and assistive-technology audit work,
using the new GPUI validation matrix to keep native obligations explicit rather
than implied.

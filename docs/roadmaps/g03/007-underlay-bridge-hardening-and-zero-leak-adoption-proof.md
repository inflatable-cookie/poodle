# g03.007 Underlay Adoption Tranche, Bridge Hardening, And Zero-Leak Proof

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g03.001, g03.002, g03.003, g03.004, g03.005, g03.006
Primary repos: `pug`, `underlay`

## Goals

- [x] validate the first real Underlay adoption tranche without exposing Pug
  publicly
- [x] harden bridge boundaries and migration posture
- [x] record remaining adoption friction clearly

## Execution Checklist

- [x] validate the first Underlay-owned adoption surfaces with no public Pug leakage
- [x] harden bridge boundaries and migration posture
- [x] record remaining adoption friction clearly

## Completed Work

- froze the normative baseline in `docs/specs/040-underlay-bridge-zero-leak-adoption-proof-baseline.md`
- added a bridge-owned zero-leak proof artifact in `packages/bridges/underlay/ts/zero-leak-proof.ts`
- widened the bridge README so package purpose, proof surfaces, and remaining adoption friction are explicit
- kept the bridge package narrow and bridge-owned rather than expanding it into a second app-facing component layer

## Acceptance Criteria

- [x] zero-leak Underlay adoption proof exists
- [x] bridge hardening posture is explicit

## Next Task

Open `g03.008` and validate the Loophole-facing foundation adoption and
DAW-extension boundary using the now-explicit extension-SDK and zero-leak
bridge baselines.

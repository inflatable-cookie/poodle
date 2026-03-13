# g04.010 GPUI Native Accessibility, Focus, Keyboard, And Assistive-Technology Proof

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g04.003, g04.004, g04.005, g04.006, g04.007, g04.008, g04.009
Primary repos: `pug`

## Goals

- [x] replace browser-inferred accessibility optimism with real GPUI evidence
- [x] define the native focus, keyboard, and assistive-technology posture for
  the implemented GPUI surface

## Execution Checklist

- [x] audit the implemented GPUI surface for focus entry, focus recovery,
  keyboard traversal, and state exposure
- [x] define how assistive-technology evidence is captured honestly for native surfaces
- [x] update the delta register wherever native behavior diverges from browser assumptions
- [x] avoid claiming complete native accessibility without real runtime proof

## Acceptance Criteria

- [x] GPUI native accessibility posture is explicit
- [x] GPUI focus and keyboard proof posture is explicit

## Completed Work

- added the normative baseline `docs/specs/057-gpui-native-accessibility-focus-keyboard-and-assistive-technology-proof-baseline.md`
- added the machine-readable artifact `packages/gpui/native-accessibility-proof.json`
- updated `packages/svelte/preview/src/accessibility.ts` so the shared GPUI
  review sections now carry explicit `manual` GPUI proof posture instead of
  defaulting to `blocked`
- updated `packages/svelte/preview/scripts/build-accessibility-report.ts` and
  regenerated `packages/svelte/preview/artifacts/accessibility-report.json`
  so the browser audit surface now reflects the explicit GPUI proof posture
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI
  accessibility proof artifact is checked against the widened primitive,
  composite, and workstation baselines plus the section-level accessibility
  audit source
- updated the current GPUI package READMEs so the repo now points at explicit
  accessibility proof posture rather than implying it is still entirely future
  work
- rolled roadmap and index surfaces forward so the repo now points at `g04.011`

## Next Task

Open `g04.011` and harden the cross-runtime parity report, intentional delta
register, and acceptance harness expansion.

# g15.016 specimen idiom convergence — batch log

Date: 2026-08-18
Branch: `t3code/specimen-idiom-convergence`
Card: `docs/roadmaps/g15/016-specimen-idiom-convergence.md`

## Scope delivered

Twenty-nine paired web catalogue routes converged on preview-local `SpecimenGroup`:

- 13 bare-Eyebrow Svelte/React caption routes
- 1 SettingsShell route (captioned in both runtimes)
- 13 audio-helper routes (Svelte `<section><h3>` and React `AudioSpecimenGroup` removed)
- 2 dedicated pages: `ListCardCounter`, `MetaItem`

## Idiom census (before → after)

| Idiom | Svelte before | Svelte after | React before | React after |
|-------|---------------|--------------|--------------|-------------|
| `SpecimenGroup label=` | partial | 29/29 scoped | partial | 29/29 scoped |
| bare caption `Eyebrow` | 13 routes | 0 (Eyebrow page demos only) | 13 routes | 0 (Eyebrow page demos only) |
| `<section><h3>` audio captions | 13 routes | 0 | n/a | n/a |
| `AudioSpecimenGroup title=` | n/a | n/a | 13 routes | 0 |
| borrowed registry mappings | 2 | 0 | 2 | 0 |

## Files touched (high level)

- Svelte/React specimens for all scoped routes
- `packages/react/preview/src/gallery/specimens/AudioSpecimen.tsx` — removed forked caption helper; `AudioAxes` uses `SpecimenGroup`
- new: `ListCardCounterSpecimen`, `MetaItemSpecimen` (both runtimes)
- registries: `registry.ts`, `specimen-map.ts`
- contracts: `list-card-counter.md`, `meta-item.md` (specimen sections only)
- evidence: `packages/svelte/preview/test/specimen-idiom-convergence.test.ts`

## Validation

- `effigy check:svelte` — pass
- `effigy react:build` — pass
- `effigy catalogue:check` — pass
- `effigy ci:web` — pass
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass
- focused: `vitest run test/specimen-idiom-convergence.test.ts` — pass

## Operator review (open)

Live paired caption review requested for these 29 routes:

1. `#components/split-button`
2. `#components/tri-state-switch`
3. `#components/select`
4. `#components/text-input`
5. `#components/token-input`
6. `#components/time-input`
7. `#components/time-zone-select`
8. `#components/eyebrow`
9. `#components/alert-dialog`
10. `#components/dialog`
11. `#components/drawer`
12. `#components/menu`
13. `#components/markdown-editor`
14. `#components/settings-shell`
15. `#components/drag-number-field`
16. `#components/audio-meter`
17. `#components/audio-switch`
18. `#components/envelope-editor`
19. `#components/fader`
20. `#components/gain-reduction-meter`
21. `#components/keyboard`
22. `#components/knob`
23. `#components/mod-matrix-grid`
24. `#components/value-readout`
25. `#components/waveform-display`
26. `#components/xy-pad`
27. `#components/meter-surface`
28. `#components/list-card-counter`
29. `#components/meta-item`

## Deviations

None beyond formatting cleanup on auto-converted caption blocks. No axis placement, example curation, or component API changes.

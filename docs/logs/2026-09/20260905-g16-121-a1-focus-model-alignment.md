# g16.121 — A1 focus-model alignment

Status: implementation complete — pending independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/121-a1-focus-model-alignment.md`
Base: `origin/main` at `cab7faf1054d61bc3251d53ddcd5cbf224992c61`
Branch: `worker/g16-121-a1-focus-model-alignment`
Runtime/evidence source pin:
`06316f34fffe607a78405cbd2bef0d518fe32080`
Lock digest:
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`

## Outcome

The final four focus-model differences were not GPUI defects.

| Row | Result |
| --- | --- |
| RadioGroup | Svelte extraction now applies the named native-radio group law: one sequential entry stop at the checked or first enabled radio |
| SegmentedControl | The same extractor law covers its hidden native-radio group |
| AgentTranscript | The shared action is now a programmatic append, so both runtimes prove that append does not move focus |
| Menu | Svelte and React MenuSurface keep the first enabled item at `tabIndex` 0 and later items at `-1`, matching GPUI's roving stop |

The new unit and component tests cover the extractor law and Menu keyboard
behaviour. The complete cohort was re-emitted through the real native selector:
29 M1 and 29 A1 receipts, all pinned to the source and lock identities above.
The four divergence stores were deleted. The ledger now reports GPUI
accessibility 29/29 across 176 component evidence rows.

## Validation

- `effigy regressions:native` — 233 passed; complete cohort emitted
- `effigy test:nucleus-a11y` — 31 passed
- `effigy test:components` — 387 files / 3,743 tests passed
- standalone `bunx svelte-check --workspace packages/svelte/components --tsconfig ./tsconfig.json --threshold error` — 0 errors, 4 existing warnings
- `effigy test:nucleus-parity-receipts` — 11 passed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy ci:web` — passed
- `effigy ci:rust` — passed at the revised implementation source pin
- `git diff --check` — clean

The prior `effigy ci:web` run at `da369c40f` passed the full board before
the numeric-only correction; the revised run also passed the full board,
including 387 files and 3,743 component tests.

No windowed selector was run. No merge was performed. The worker stops after
one pushed PR for independent exact-head review.

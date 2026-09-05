# g16.116 — Nucleus A1 NP-5 Command And Attention

Status: complete — all three rows executed; each is a recorded GPUI/Svelte divergence
Base: `3dbabac39` (origin/main, dispatch manifest revision 17)

## Rows

| Row | Result |
| --- | --- |
| CommandPalette | Divergence recorded: native close button is first in the projection; Svelte heading is first, with native textbox/status/results roles shifted. |
| MessageCenter | Divergence recorded: native trigger controls target `-1`, native surface/list projection differs from the Svelte banner/heading/list shape. |
| ToastHost | Divergence recorded: native alert name is absent and the action/dismiss ordering differs from Svelte. |

The executed GPUI snapshots and diffs are stored under
`docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/`. No contract,
Svelte, backend, or render projection fix was made: these differences require
semantic decisions beyond this receipt tranche.

## Validation

- `POODLE_NUCLEUS_A11Y_WRITE=1 effigy test:nucleus-a11y` — pass, 7 tests.
- Executed native NP-5 proofs with `POODLE_NUCLEUS_RECEIPT_DIR=target/nucleus-receipts`; all three produced snapshots and diffs.
- Normal native proofs are marked ignored with the exact divergence reason; `effigy regressions:native` is the final headless board check.
- No windowed, native-visual, or capture selector was run.

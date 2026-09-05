# g16.117 — Select A1 alignment

Status: implementation complete — pending independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/117-select-a1-alignment.md`
Dispatch handoff: `docs/handoffs/20260905-g16-117-select-a1-alignment.md`
Closeout handoff: `docs/handoffs/20260905-g16-117-select-a1-alignment-closeout.md`
Base: `origin/main` at `4b92c9c5e` (manifest revision 18)
Branch: `worker/g16-117-select-a1-alignment`

## Outcome

Select A1 now has one semantic contract across Svelte, React, and GPUI:

- non-searchable GPUI triggers expose `button` semantics and no trigger value
  text; searchable triggers remain `combobox`;
- the Select label is projected onto the listbox;
- Svelte and React options are not sequential tab stops;
- the Svelte and React chevrons remain pointer toggles but are decorative
  (`tabindex="-1"`, `aria-hidden="true"`), so Select has one tab stop;
- the Select A1 receipt is empty-diff and the former divergence store is
  deleted.

## Validation

- `cargo test --manifest-path packages/render/Cargo.toml select --quiet` — 70 passed
- focused Svelte and React Select suites — 30 passed
- `POODLE_NUCLEUS_A11Y_WRITE=1 effigy test:nucleus-a11y` — regenerated the
  reference snapshot
- `POODLE_NUCLEUS_RECEIPT_DIR=$PWD/target/nucleus-receipts cargo test
  --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions
  select_a1 -- --ignored` — passed with empty diff

The receipt cohort was repinned to reachable implementation commit
`f1e7032c0a3eb7d22fdb3686dade75d0cea0b796` and the current preview lock
digest `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`;
the live evidence ledger was regenerated. No windowed selector was run.

Additional gates:

- `effigy test:nucleus-a11y` — 4 passed
- `effigy test:a11y` — 179 passed
- `effigy test:nucleus-parity-receipts` — 11 passed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy docs:check` — passed
- `effigy docs:react-prop-drift` — 176 checked
- `effigy ci:web` — passed (386 files, 3714 tests)
- `git diff --check` — clean

## Review state

The worker will push one focused PR and stop. Merge and independent exact-head
review remain orchestrator-owned.

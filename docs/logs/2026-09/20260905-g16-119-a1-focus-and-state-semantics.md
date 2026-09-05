# g16.119 — A1 focus and state semantics

Status: implementation complete — pending fresh exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/119-a1-focus-and-state-semantics.md`
Base: `origin/main` at `e4407101e5836843e124c75b4884c118f74bb1e3`
Branch: `worker/g16-119-a1-focus-state-semantics`
Runtime/evidence repin:
`1636f70bd373323128e3bf4dd5e923fed0066e45`
Lock digest:
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`

## Outcome

The original five-row scope is complete within its contract boundary.

| Row | Result |
| --- | --- |
| AgentQuestion | prompt-to-option-group `labelled_by` relation and stable post-action focus repaired; empty diff and A1 receipt |
| Menu | opening focus repaired; one unresolved tab-stop divergence retained |
| AgentTranscript | contract requires no focus on append; extractor click-focus divergence retained |
| RadioGroup | `orientation` projection removed; extractor radio tab-order divergence retained |
| SegmentedControl | `selected` projection removed; extractor radio tab-order divergence retained |

The five overlay rows activated after g16.118 are now proved at the same final
head. `poodle-node::NodeA11y` carries one `initial_focus: bool` record field.
Dialog, Popover, ConfirmAction, MessageCenter, and ModelPicker each mark
exactly one node in the open overlay. The backend claims that marker once per
runtime identity and routes the existing mount focus request, without
re-queueing focus after user navigation. ConfirmAction inherits Dialog's
panel marker; ModelPicker marks the selected enabled model row, falling back to
the first enabled row.

All five overlay A1 diffs are empty and their g16.118 focus-only stores were
consumed. The five receipts are:

- `dialog--nucleus-navigation-dialog--a1.json`
- `popover--nucleus-navigation-popover--a1.json`
- `confirmaction--nucleus-settings-confirm-action--a1.json`
- `messagecenter--nucleus-attention-message-center--a1.json`
- `modelpicker--nucleus-agent-model-picker--a1.json`

The complete final cohort is 29 M1 receipts and 21 A1 receipts. All receipts
carry the runtime/evidence source pin above. GPUI snapshots for all A1 rows are
committed beside the shared Svelte snapshots. The active divergence index
retains the four owned honest stores plus older NP-3/NP-1 records; it no longer
contains overlay focus stores.

## Validation

- `effigy regressions:native` — 232 passed, 0 failed, 0 ignored; receipts
  emitted from the clean final target
- `effigy test:nucleus-a11y` — 30 passed
- `effigy test:nucleus-parity-receipts` — 11 passed
- `effigy test:parity-evidence-ledger` — 6 passed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy ci:web` — 386 files and 3,740 tests passed
- `effigy docs:check` — passed; existing Svelte diagnostics and ratcheted
  value-domain findings remain informational
- `effigy docs:react-prop-drift` — 176 checked, 0 skipped, passed
- `git diff --check` — clean

`effigy qa` reached the broad repository board but stopped at the two known
`origin/main` `poodle-render` unit failures already recorded in `PAPERCUTS.md`:
`context::tests::the_provider_adds_no_wrapper_node_layout_or_accessibility_entry`
and `segmented_control::tests::icon_only_without_an_icon_keeps_the_visible_label`.
Neither test or source line is owned by this card; no unrelated repair was
attempted.

No windowed selector was run. No merge was performed. PR #223 remains for
coordinator review at the pushed exact head.

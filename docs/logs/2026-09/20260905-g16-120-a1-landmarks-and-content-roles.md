# g16.120 — A1 landmarks and content roles

Status: implementation complete — pending fresh exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/120-a1-landmarks-and-content-roles.md`
Base: `origin/main` at `cceb6646a2bf7776b670fb63f586bce037d0ee6e`
Branch: `worker/g16-120-nucleus-a1-shell-agent-plan`
Runtime/evidence source pin:
`54646ba2369959150a1b4953e06de5871b3ffe8f`
Lock digest:
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`

## Outcome

The four-row landmark/content-role scope is complete within the A1 contract
boundary.

| Row | Result |
| --- | --- |
| AppHeader | Root now projects `Banner`; the existing label fallback is preserved |
| SplitView | Separator projects current value `0`; collapse toggles use the full primary/secondary names |
| AgentChatInput | Editor and action project `TextInput`/`Button`; action keeps stable backend identity while the editor owns the visible focus ring |
| AgentPlan | First markdown heading projects `Heading`, its level, label, and stable title identity |

All four paired A1 snapshots agree. Their receipts are empty-diff receipts at
the source pin above, and the superseded g16.114/NP-1 divergence stores were
deleted. The older active stores for AgentTranscript, Menu, RadioGroup, and
SegmentedControl remain honest and unchanged.

The complete final cohort contains 29 M1 and 25 A1 receipts. All receipts and
the four committed GPUI snapshots carry the exact source pin above. The ledger
was regenerated and validates 176 component evidence rows.

## Validation

- `effigy regressions:native` — 233 passed, 0 failed, 0 ignored; complete
  cohort receipts emitted from the final implementation head
- `effigy test:nucleus-a11y` — 30 passed
- `effigy test:nucleus-parity-receipts` — 11 passed
- `effigy test:parity-evidence-ledger` — 6 passed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy docs:check` — passed; existing Svelte diagnostics and ratcheted
  value-domain findings remain informational
- `effigy ci:web` — passed; 386 test files and 3,740 tests, plus the
  package-install consumer's 11 files and 22 tests
- `effigy ci:rust` — passed
- `cargo test --manifest-path packages/render/Cargo.toml` — 642 passed; two
  known origin-main failures remain in `context` and `segmented_control`, as
  recorded in `PAPERCUTS.md`
- `git diff --check` — clean

Hosted `effigy ci:web` and `effigy ci:rust` results are added before push.

No windowed selector was run. No merge was performed. The worker stops after
one pushed PR for independent exact-head review.

# g16.067 — Nucleus Icon and IconButton M1 Receipts

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/067-nucleus-icon-icon-button-m1.md`
Handoff: `docs/handoffs/20260903-025000-g16-067-nucleus-icon-receipts.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/icon.md`,
`docs/contracts/components/icon-button.md`
Branch: `feature/g16-067-nucleus-icon-receipts`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-067-nucleus-icon-receipts`
Planning base: `f4a4c80d2` (`origin/main`)
Worker PR: https://github.com/inflatable-cookie/poodle/pull/173

## Outcome

`Icon` and `IconButton` now have validated `M1` execution receipts emitted
from the production GPUI render, node backend, and test-platform path in
`effigy regressions:native`. `IconProvider` remains a non-rendered setup
prerequisite, keeping the Nucleus rendered denominator at 29. The manifest,
existing Button receipt, new Icon and IconButton receipts, and generated
ledger pin the exact runtime source commit `8578cc03facedc43384b76d515dee20b720a152b`.

## What landed

- Headless regressions: `packages/gpui/preview/tests/headless_regressions.rs`.
  - Added `icon_resolves_named_glyph_token_size_tint_and_label_through_mounted_backend`:
    proves registered asset resolution, token sizing, primary tint, explicit
    accessible label, probe capture (`content.text-icon.icon`), and mounted
    bounds under `IconProvider` wrapping.
  - Strengthened `icon_button_activation_toggle_and_tooltip_through_mounted_pointer_and_keyboard`:
    proves hit-tested pointer and keyboard activation, controlled/seeded toggle
    rebuilds, disabled inertness, and the full 300ms tooltip lifecycle
    (hover delay, visible text, departure dismissal, fallback text, and
    Escape dismissal).
  - Wired deterministic `nucleus_receipts::emit_if_configured` calls for
    `Icon` (`nucleus.shell.icon`) and `IconButton` (`nucleus.shell.icon-button`).
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/icon--nucleus-shell-icon.json`
  - `docs/roadmaps/g16/nucleus-parity-receipts/iconbutton--nucleus-shell-icon-button.json`
  - Refreshed `docs/roadmaps/g16/nucleus-parity-receipts/button--nucleus-shell-button.json`
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated Icon `expected_test` and `source_commit`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts --write`; reports 3 mounted rows (Button, Icon, IconButton).

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Icon uses the production path | bypass render or skip probe channel | `assert!(probe_channels.contains(&"content.text-icon.icon"))` failed |
| Provider is setup only | promote IconProvider to rendered row 30 or emit receipt | `validateNucleusManifest` threw `manifest has 30 rendered rows, expected 29`; `validateNucleusReceipt` threw `receipt component is not a rendered manifest entry: IconProvider` |
| Registry is real | omit the named icon from the installed registry (`"search_nonexistent"`) | `assert!(icon_asset_exists(icon_name))` panicked: `Icon fixture requires a real registered icon asset` |
| IconButton input is dispatched | call handler directly without `HeadlessDriver` input | `observation.is_valid()` panicked: `receipt requires observed mounted paint and GPUI input dispatch` |
| Receipt means execution | unmanifested receipt or synthetic JSON | `validateNucleusReceipt` / `loadValidatedNucleusReceipts` rejected unmanifested and unobserved receipts |
| Evidence identity is exact | keep stale source SHA after test change | `loadValidatedNucleusReceipts` threw `receipt source commit ... no longer matches the mounted runtime source` |
| Levels stay separate | label receipt `A1` or `V1` | `validateNucleusReceipt` threw `receipt proof level must be M1; A1 and V1 require separate evidence` |

## Validation

Focused:
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions icon` — 3 pass
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 pass
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 pass
- `bun scripts/parity-evidence-ledger.ts` — validated 176 component evidence rows

Required boards:
- `effigy regressions:native` — 182 pass (receipts emitted)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves mounted production-path render, node backend, and test-platform
  input dispatch only; it does not claim `A1` (accessibility semantics) or
  `V1` (visual comparison).
- `IconProvider` remains non-rendered fixture setup.
- Merge remains orchestrator-owned.

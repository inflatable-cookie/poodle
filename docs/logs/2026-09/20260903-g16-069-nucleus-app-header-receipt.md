# g16.069 — Nucleus AppHeader M1 Receipt

Status: in-review
Date: 2026-09-03
Card: `docs/roadmaps/g16/069-nucleus-app-header-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-034145-g16-069-nucleus-app-header-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/app-header.md`,
`docs/contracts/components/icon.md`,
`docs/contracts/components/text.md`
Branch: `feature/g16-069-nucleus-app-header-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-069-nucleus-app-header-receipt`
Planning base: `a6b3a8e41` (`origin/main`)

## Outcome

`AppHeader` now has a validated `M1` execution receipt emitted from the
production GPUI render, node backend, and test-platform path in
`effigy regressions:native`. A centered custom-identity fixture exercises
`AppHeader` with composed `Icon` and `Text` slot children, verifying
background/border styling, size ladders (min-height, typography), density ladders
(padding, gaps), center slot presence layout modes (flat vs symmetric trailing
grow column), accessible label resolution (title fallback and aria-label override),
mounted child containment, stable region ordering, inert non-focusable root, and
backend probe channels. The manifest, existing receipts (Button, Icon, IconButton,
Surface, Text), and new AppHeader receipt pin the exact runtime source commit
`3256d11da8799429eaba55e22dabe4c526f77695`. The ledger records 6 mounted Nucleus rows.

## What landed

- Contracts & Render Unit Tests:
  - `packages/contracts/components/src/app_header.rs`: added unit tests validating default and builder properties, semantic token mapping (`background_token`, `border_token`, `title_color_token`, `subtitle_color_token`), size ladder resolution across all `ControlSize` variants (`min_height_rem`, `title_size_rem`, `subtitle_size_rem`), density ladder resolution across `ControlDensity` variants (`gap_rem`, `region_gap_rem`, `pad_y_rem`, `pad_x_rem`), and `effective_size_resolution` under `SemanticControlSizeRole`.
  - `packages/render/src/app_header.rs`: added unit tests covering token styling (background panel, bottom border width 1.0, fill-width, row layout), size ladder rendering, density ladder rendering, custom identity slot replacing default title group, accessible label fallback to title, explicit `aria_label` override, and scoped `SlotBuilder` context passing.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: added `app_header_resolves_structure_token_styling_and_layout_through_mounted_backend` which exercises AppHeader spec/render token and layout resolution, center presence structural differences, mounts a centered custom-identity fixture with Icon and Text children, captures backend probe channels (`structure.identity.container`, `content.text-icon.text`, `content.text-icon.icon`, `surface.channels.background`, `surface.channels.border`), checks element bounds (`bounds_for`) for positive dimensions, child containment, and region ordering, confirms the styled-only root remains outside the focus chain, and terminally emits the `nucleus.shell.app-header` M1 receipt.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/appheader--nucleus-shell-app-header.json`
  - Refreshed existing receipts for `Button`, `Icon`, `IconButton`, `Surface`, `Text` with source commit `3256d11da8799429eaba55e22dabe4c526f77695`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated AppHeader `expected_test` to `app_header_resolves_structure_token_styling_and_layout_through_mounted_backend` and updated `source_commit` to `3256d11da8799429eaba55e22dabe4c526f77695`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts`; reports 6 mounted rows (AppHeader, Button, Icon, IconButton, Surface, Text).

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production renderer owns the shell | substitute a raw container for AppHeader | exact shell metadata and structure assertions fail |
| Proven dependencies remain real | replace the identity Icon or Text with raw nodes | component-specific Node metadata/probe assertions fail |
| Center presence owns grouping | group the no-center regions or flatten the centered trailing column | child order, count, sizing, or containment assertion fails |
| Size and density are exact | collapse two size or density steps | exact min-height, typography, padding, or gap assertion fails |
| Labeling is contract-owned | remove title fallback or explicit override | exact Node label assertion fails |
| Styled-only stays inert | make the header root focusable or activatable | focus-chain or interaction assertion fails |
| Receipt is terminal | fail the final mounted assertion | no AppHeader receipt is emitted for the current source |
| Evidence identity is exact | retain the g16.068 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib app_header::` — pass
- `cargo test --manifest-path packages/render/Cargo.toml --lib app_header::` — pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions app_header` — pass
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 pass
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 pass
- `bun scripts/parity-evidence-ledger.ts` — validated 176 component evidence rows

Required boards:
- `effigy regressions:native` — 184 pass (all receipts emitted)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves mounted production-path render, node backend, slot containment,
  and test-platform input dispatch; it does not claim `A1` (accessibility tree verification)
  or `V1` (pixel-level visual comparison).
- `AppHeader` styled root remains inert and non-focusable.
- Merge remains orchestrator-owned.

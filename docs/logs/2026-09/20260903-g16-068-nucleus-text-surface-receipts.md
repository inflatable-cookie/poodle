# g16.068 — Nucleus Text and Surface M1 Receipts

Status: in-review
Date: 2026-09-03
Card: `docs/roadmaps/g16/068-nucleus-text-surface-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-031500-g16-068-nucleus-text-surface-receipts.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/text.md`,
`docs/contracts/components/surface.md`
Branch: `feature/g16-068-nucleus-text-surface-receipts`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-068-nucleus-text-surface-receipts`
Planning base: `91f039cd9` (`origin/main`)

## Outcome

`Text` and `Surface` now have validated `M1` execution receipts emitted
from the production GPUI render, node backend, and test-platform path in
`effigy regressions:native`. A single bounded mounted composite exercises
`Text` within `Surface`, verifying typography resolution, container styling,
child containment, accessibility roles/labels, inert focus chains, and probe
channels. The manifest, existing receipts (Button, Icon, IconButton), and new
receipts (Text, Surface) pin the exact runtime source commit `dd202ae5899e8c181b7b40bcc34b8c8b1e1c97b5`.
The ledger records 5 mounted Nucleus rows.

## What landed

- Render fixes and unit tests:
  - `packages/render/src/surface.rs`: mapped `SurfaceRole` (Group, Region) and `spec.label` to `Node.a11y.role` and `Node.a11y.label`; resolved border width via `resolve_border_width`; added unit tests covering tones, borders, elevation shadow, radius, padding scales, and accessibility semantics.
  - `packages/render/src/text.rs`: added unit tests covering all contract tones (Default, Secondary, Muted, Success, Danger, Warning), sizes (Xs, Sm, Md), weights (Normal, Medium, Semibold, Bold), leading (Normal, Relaxed), clamp (overflow hidden), and compact spacing.
- Headless regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: added `text_and_surface_resolve_typography_container_styling_and_layout_through_mounted_backend` which proves Text variant styling/metadata, Surface container styling/metadata, mounted composite layout with child containment (`text_bounds` within `surface_bounds`), inertness of styled-only primitives outside the focus chain, probe channel capture (`structure.identity.container`, `content.text-icon.text`, `surface.channels.background`, `surface.channels.border`, `content.typography.size`, `content.typography.weight`, `accessibility.projection.received`), and terminal receipt emission for both components.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/text--nucleus-shell-text.json`
  - `docs/roadmaps/g16/nucleus-parity-receipts/surface--nucleus-shell-surface.json`
  - Refreshed existing receipts for `Button`, `Icon`, `IconButton` with source commit `dd202ae5899e8c181b7b40bcc34b8c8b1e1c97b5`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated Text and Surface `expected_test` and `source_commit` (`dd202ae5899e8c181b7b40bcc34b8c8b1e1c97b5`).
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts`; reports 5 mounted rows (Button, Icon, IconButton, Surface, Text).

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production recipes own metadata | replace Text or Surface with a raw Node | resolved token/style assertion fails; missing style descriptor and probe channels |
| Composition is real | omit the Text child from Surface | mounted containment assertion fails (`text_bounds` inside `surface_bounds`); `content.text-icon.text` probe fails |
| Styled-only stays inert | make either root focusable or activatable | `focus_handle_for` / `focus_state_for` returns `Some(_)`, failing focus chain assertion |
| Text variants remain distinct | collapse tone, size, weight, line-height, spacing, or clamp | exact metadata assertion fails |
| Surface variants remain distinct | collapse fill, border, padding, radius, or elevation | exact metadata assertion fails |
| Receipt is terminal | fail the final mounted assertion | test panics before terminal `nucleus_receipts::emit_if_configured`; neither receipt emitted |
| Evidence identity is exact | retain g16.067 receipt SHA | `validateNucleusReceipt` throws: `receipt source commit ... no longer matches the mounted runtime source` |
| Levels stay separate | label either receipt A1 or V1 | `validateNucleusReceipt` throws: `receipt proof level must be M1; A1 and V1 require separate evidence` |

## Validation

Focused:
- `cargo test --manifest-path packages/render/Cargo.toml --lib text::` — pass
- `cargo test --manifest-path packages/render/Cargo.toml --lib surface::` — pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions text_and_surface` — pass
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 pass
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 pass
- `bun scripts/parity-evidence-ledger.ts` — validated 176 component evidence rows

Required boards:
- `effigy regressions:native` — 183 pass (all receipts emitted)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves mounted production-path render, node backend, typography/surface metadata,
  layout child containment, and test-platform input dispatch; it does not claim `A1`
  (accessibility tree verification) or `V1` (pixel-level visual comparison).
- `Text` and `Surface` remain styled-only and inert.
- Merge remains orchestrator-owned.

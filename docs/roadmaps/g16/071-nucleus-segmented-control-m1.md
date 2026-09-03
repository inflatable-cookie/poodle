# g16.071 — Nucleus SegmentedControl M1 Receipt

Status: ready
Type: Nucleus NP-2 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.070`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/segmented-control.md`
Log: `../../logs/2026-09/20260903-g16-071-nucleus-segmented-control-receipt.md`
PR: https://github.com/inflatable-cookie/poodle/pull/177
Handoff: `../../handoffs/20260903-050013-g16-071-nucleus-segmented-control-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `SegmentedControl` row
through the production Rust render, Node, GPUI backend, and test-platform
paths. Strengthen the manifest's existing mounted exclusive-selection and
focus-identity regression. Prove the plain labelled three-option mode-switch
shape used by the cohort without importing Nucleus data.

## Fixed Boundary

- Strengthen
  `segmented_control_exclusive_focus_identity_and_disabled_paths`; retain that
  exact manifest test name and its current pointer, keyboard, disabled, rebuild,
  and two-instance identity coverage.
- Build controls only through `poodle_render::segmented_control`. Use a bounded
  three-option labelled fixture with one disabled option and a controlled host
  rebuild after accepted changes.
- Pin exact root radiogroup semantics, label, direction, height, track
  background/border/radius/padding/gap, and equal-width posture against
  contract-owned spec/token values.
- Pin each segment's role, selected/toggled state, roving tab stop, accessible
  name, height, typography, truncation, cursor, disabled opacity, focus patch,
  selected fill/text/shadow, and stable runtime identity.
- Dispatch pointer selection and directional keyboard input through the
  mounted test platform. Prove same-value inertia, disabled-option skip,
  wraparound, controlled rebuild, focus transfer, and isolation between two
  composed instances.
- Emit the SegmentedControl receipt only after every claimed assertion passes.
  Refresh the manifest resolution, every existing receipt, and generated
  ledger from the exact committed runtime source. No other row advances.

## Acceptance

- SegmentedControl names the existing executed mounted test in the manifest
  and has one valid `nucleus.navigation.segmented-control` M1 receipt.
- Replacing the production renderer with raw buttons, collapsing exclusive
  semantics into free buttons, losing caller-scoped option identity, bypassing
  mounted input, accepting disabled/same-value selection, or changing an exact
  track/segment mapping fails before receipt emission.
- Two composed controls with the same values keep distinct focus identities
  and handler streams. The denominator stays 29.
- Existing seven receipts remain valid. M1 does not infer A1, V1, Nucleus
  adoption, browser hidden-radio implementation, or pixel-level parity.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production renderer owns structure | substitute raw buttons for the control | exact root/segment metadata fails |
| Exclusive semantics are exact | omit radiogroup/radio selected+toggled state | semantic assertions fail before receipt |
| Identity is caller-scoped | derive option IDs from value alone | the two composed controls alias focus |
| Input is mounted | invoke `on_change` directly | mounted observation or callback trace is absent |
| Controlled rebuild is real | record payload without rebuilding the supplied value | selected/toggled state remains stale |
| Disabled and same-value paths are inert | arm every option identically | callback trace gains a forbidden value |
| Roving focus skips and wraps | focus the disabled option or stop at an edge | exact focus and callback trace fails |
| Visual metadata is exact | change track mix/border/radius or selected segment paint | exact token/style assertion fails |
| Receipt is terminal | fail the final mounted assertion | no SegmentedControl receipt is emitted |
| Evidence identity is exact | retain the g16.070 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

The existing SegmentedControl mounted regression; focused SegmentedControl
spec/render tests; a focused shared Rust or GPUI repair only if a mounted
counterexample proves it necessary; receipt emission and exact
manifest/receipt/ledger refresh; this card; one execution log; and new
papercuts. Do not edit Nucleus, web behavior, public APIs, accessibility
authority, visual-lab code, Jetstream, workflows, versions, releases, or other
component rows.

## Validation

Run focused SegmentedControl spec/render/backend tests, the named mounted
fixture, the real `effigy regressions:native` receipt run,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof requires a new public API, Nucleus
application data, browser hidden-radio mechanics, pixel inspection, or
accessibility claims unavailable from the headless backend.

## Continuation

After merge, compile the Tabs M1 receipt child from the refreshed receipt
identity. Later Nucleus receipt cards remain serial.

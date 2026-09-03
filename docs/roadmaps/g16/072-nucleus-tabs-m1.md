# g16.072 — Nucleus Tabs M1 Receipt

Status: ready
Type: Nucleus NP-2 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.065`, completed `g16.071`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/tabs.md`
Log: `../../logs/2026-09/20260903-g16-072-nucleus-tabs-receipt.md`
PR: pending
Handoff: `../../handoffs/20260903-054548-g16-072-nucleus-tabs-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `Tabs` row through the
production Rust render, Node, GPUI backend, and test-platform paths. Strengthen
the manifest's existing mounted Tabs regression rather than creating a second
fixture. Prove the controlled navigation, close, reorder, focus-identity, and
panel relationship already exercised by that production path without reopening
the public Tabs contract.

## Fixed Boundary

- Strengthen `tabs_drag_keyboard_and_identity_rebuild_the_host_spec`; retain
  that exact manifest test name and its automatic/manual, horizontal/vertical,
  close, keyboard reorder, pointer reorder, cancellation, and controlled host
  rebuild coverage.
- Build Tabs only through `poodle_render::tabs_with_panel` or
  `poodle_render::tabs_with_handlers`. Pin exact contract-owned root/list/tab,
  selected, disabled, close, panel, orientation, activation, focus, and
  drag/drop metadata before mounted behavior claims.
- Prove visible tab names at the Node boundary, stable caller-scoped runtime
  identities, and no aliasing between two composed instances using the same
  item values. Keep callback and focus streams isolated.
- Dispatch selection, close, directional navigation, manual activation,
  keyboard reorder, and pointer reorder through the mounted test platform.
  Assert controlled rebuilds and terminal drag cleanup. Direct callback calls
  are not M1 evidence.
- Preserve the merged g16.065 300 ms tooltip lifecycle as existing behavior;
  do not turn this receipt into another tooltip-policy card. Preserve g16.060
  web-only controlled-panel focus policy without adding a native analogue.
- Emit the Tabs receipt only after every claimed assertion passes. Refresh the
  manifest resolution, every existing receipt, and generated ledger from the
  exact committed runtime source. No other row advances.

## Acceptance

- Tabs names the existing executed mounted test in the manifest and has one
  valid `nucleus.navigation.tabs` M1 receipt.
- Replacing production Tabs with raw buttons, losing tablist/tab/tabpanel
  relationships, deriving IDs from item values alone, bypassing mounted input,
  accepting disabled selection, crossing instance streams, or failing to
  rebuild controlled state fails before receipt emission.
- Automatic and manual activation remain distinct. Horizontal and vertical
  key axes, close behavior, keyboard/pointer reorder, self-drop refusal, focus
  retention, and drag cancellation are independently legible in the proof.
- Existing eight receipts remain valid. The denominator stays 29. M1 does not
  infer A1, V1, Nucleus adoption, browser DOM parity, or pixel-level parity.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production renderer owns structure | substitute raw buttons and a text panel | exact tablist/tab/panel metadata fails |
| Names and relationships are exact | drop a visible label, controls, or labelled-by relation | Node-boundary assertions fail before receipt |
| Identity is caller-scoped | derive IDs from item values alone | two composed instances alias focus or input |
| Input is mounted | invoke handlers directly | mounted observation or callback trace is absent |
| Controlled rebuild is real | record a value/order without rebuilding supplied state | selected state or order remains stale |
| Activation modes stay distinct | automatic focus does not select or manual focus selects early | exact focus/change traces fail |
| Axis and disabled behavior are exact | use the wrong directional key or land on disabled Skip | exact value/focus trace fails |
| Close behavior is exact | close the wrong tab or retain a removed tab | close trace and rebuilt tree fail |
| Reorder lifecycle is exact | accept self-drop, lose focus, or omit terminal cleanup | order/start/end and live drag state fail |
| Receipt is terminal | fail any final mounted assertion | no Tabs receipt is emitted |
| Evidence identity is exact | retain the g16.071 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

The existing Tabs mounted regression; focused Tabs spec/render tests; a focused
shared Rust or GPUI repair only if a biting mounted counterexample proves it
necessary; receipt emission and exact manifest/receipt/ledger refresh; this
card; one execution log; and new papercuts. Do not edit Nucleus, web behavior,
public APIs, tooltip or motion policy, accessibility authority, visual-lab
code, Jetstream, workflows, versions, releases, or other component rows.

## Validation

Run focused Tabs spec/render/backend tests, the named mounted fixture, the real
`effigy regressions:native` receipt run,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof requires a new public API, Nucleus
application data, browser-only controlled-panel focus behavior, another
tooltip runtime, pixel inspection, or accessibility claims unavailable from
the headless backend.

## Continuation

After merge, compile the next Nucleus receipt child from the refreshed receipt
identity. Later Nucleus receipt cards remain serial.

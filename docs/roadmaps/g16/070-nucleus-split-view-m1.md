# g16.070 — Nucleus SplitView M1 Receipt

Status: complete
Type: Nucleus NP-1 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.067`, completed `g16.069`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/split-view.md`,
`../../contracts/components/resize-handle.md`,
`../../contracts/components/collapse-toggle.md`,
`../../contracts/components/icon.md`
Log: `../../logs/2026-09/20260903-g16-070-nucleus-split-view-receipt.md`
PR: https://github.com/inflatable-cookie/poodle/pull/176
Handoff: `../../handoffs/20260903-041500-g16-070-nucleus-split-view-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `SplitView` row through the
production Rust render, Node, GPUI backend, and test-platform paths. Strengthen
the manifest's existing two-instance regression instead of creating a parallel
fixture. Prove mounted structure, independent divider identity, real keyboard
resize dispatch, and the production collapse-toggle/Icon seam. Keep app-owned
ratio conversion and higher evidence levels separate.

## Fixed Boundary

- Strengthen
  `two_composed_split_views_do_not_share_a_divider_focus_handle`; keep that
  exact manifest test name.
- Build both splits only through `poodle_render::split_view`, with stable,
  distinct caller-owned instance IDs. Use the same visible inputs where that
  makes an identity collision adversarial.
- Pin the production root orientation, pane ordering, ratio-derived bases,
  minimum sizes, overflow posture, divider role/orientation/value metadata,
  and mounted child containment against contract-owned values.
- Focus each real divider through the mounted backend. Dispatch axis keys plus
  Home/End through the test platform. Assert the expected `ResizePhase` and
  pixel deltas reach only the owning split; direct callback invocation is not
  M1 evidence.
- Enable one bounded collapse toggle on the subject split. Exercise it through
  mounted keyboard or pointer input, assert the next externally owned collapse
  value, and prove its chevron is built through the production CollapseToggle
  and Icon renderers. The collision witness need not expose a toggle.
- Emit the SplitView receipt only after every claimed assertion passes. Refresh
  the manifest resolution, every existing receipt, and the generated ledger
  from the exact committed runtime source. No other row advances.

## Acceptance

- SplitView names the existing executed mounted test in the manifest and has
  one valid `nucleus.shell.split-view` M1 receipt.
- Replacing either SplitView with raw layout, deriving divider identity from
  orientation/label, bypassing mounted key dispatch, crossing handler streams,
  or replacing the CollapseToggle/Icon seam fails before receipt emission.
- Mounted bounds prove both panes and dividers remain inside their owning
  split. Focusing or resizing one divider does not focus or mutate the other.
- Existing Button, Icon, IconButton, Text, Surface, and AppHeader receipts
  remain valid. The denominator stays 29.
- M1 does not infer A1, V1, Nucleus adoption, web breakpoint parity, native
  ratio ownership, drag-to-collapse parity, or visual seam parity.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production renderer owns structure | substitute raw row/panes for one SplitView | exact structure, metadata, or containment assertion fails |
| Instance identity is caller-owned | derive both divider IDs from shared orientation or label | focusing one divider aliases or replaces the other |
| Input is mounted | call `on_resize` directly | input observation or callback trace is absent |
| Resize ownership is isolated | route both handlers to one sink | the witness receives the subject's key delta or phase |
| Axis and endpoints are exact | accept a cross-axis key or change Home/End delta | the exact callback trace fails |
| Collapse composition is real | replace CollapseToggle or its Icon with a raw button/node | exact action/icon metadata or mounted activation proof fails |
| Layout metadata is exact | collapse ratio, min-size, or orientation mappings | exact Node metadata or mounted bounds fail |
| Receipt is terminal | fail the final mounted assertion | no SplitView receipt is emitted for the current source |
| Evidence identity is exact | retain the g16.069 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

The existing SplitView mounted regression; focused SplitView/ResizeHandle/
CollapseToggle renderer or spec tests; a focused shared Rust or GPUI repair only
if the mounted counterexample proves it necessary; receipt emission and exact
manifest/receipt/ledger refresh; this card; one execution log; and new
papercuts. Do not edit Nucleus, web behavior, public APIs, accessibility
authority, visual-lab code, Jetstream, workflows, versions, releases, or other
component rows.

## Validation

Run focused SplitView, ResizeHandle, CollapseToggle, Icon, renderer, and backend
tests; the named mounted fixture; the real `effigy regressions:native` receipt
run; `effigy check:parity-evidence-ledger`; `effigy ci:rust`;
`effigy ci:native`; `effigy docs:check`; and
`git diff --check origin/main...HEAD`. Do not run windowed or native-visual
selectors.

## Stop Conditions

Stop for orchestrator review if the proof requires a new public API, Nucleus
application data, guessed axis extent, pixel inspection, repair of documented
rail/threshold differences, or accessibility claims unavailable from the
headless backend.

## Continuation

After merge, compile the SegmentedControl M1 child from the refreshed receipt
identity. Later Nucleus receipt cards remain serial.

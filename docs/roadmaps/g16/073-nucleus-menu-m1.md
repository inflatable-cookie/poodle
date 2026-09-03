# g16.073 — Nucleus Menu M1 Receipt

Status: complete
Type: Nucleus NP-2 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.067`, completed `g16.072`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/menu.md`, `../../contracts/components/icon.md`
Log: `../../logs/2026-09/20260903-g16-073-nucleus-menu-receipt.md`
PR: pending
Handoff: `../../handoffs/20260903-064013-g16-073-nucleus-menu-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `Menu` row through the
production Rust render, Node, GPUI backend, and test-platform paths. Establish
the manifest's first named mounted Menu fixture. Prove the flat action-menu
shape Nucleus uses while retaining generic checked and shortcut rows needed by
the contract. Keep trigger/open ownership and nested-pointer policy outside
this panel-only native boundary.

## Fixed Boundary

- Add one mounted regression named
  `menu_items_semantics_activation_and_identity_rebuild_the_host_spec` and set
  that exact name on the manifest row. Do not reuse a Select, ContextMenu, or
  raw Node fixture as Menu evidence.
- Build the panel only through `poodle_render::menu`. Use bounded generic rows:
  enabled action, disabled action, separator, destructive action, shortcut
  action, checked checkbox, unchecked checkbox, and one radio row. Do not copy
  Nucleus labels or application data.
- Pin exact contract-owned panel fill, border, radius, shadow, overlay posture,
  minimum width, padding, column layout, item height/padding/gap/radius,
  typography, hover, danger, disabled, shortcut, separator, role, toggled, and
  production check-Icon metadata.
- Mount through `HeadlessDriver`. Prove positive ordered row bounds and panel
  containment. Dispatch pointer plus Enter/Space activation through the test
  platform. Disabled and separator rows remain inert. An accepted checkbox or
  radio action reaches one host stream and rebuilds the externally supplied
  checked state.
- Prove the keyboard command-list floor required by the Menu contract:
  next/previous movement with disabled/separator skip, Home/End, and
  activation after the host focuses an enabled item. Escape must not activate
  an item; dismissal and trigger-focus restoration remain host-owned. Make a focused renderer/backend
  repair only if a biting mounted counterexample proves the existing panel
  path lacks that behavior. Stop if the repair needs a new public API or a
  second menu machine.
- Treat native Menu as the documented panel-only surface: it does not own a
  trigger or raise `onOpenChange`. Do not claim or implement trigger placement,
  outside dismissal, focus restoration to a consumer trigger, recursive
  submenu behavior, or web `onSurfaceGeometryChange` in this card.
- Emit the Menu receipt only after every claimed assertion passes. Refresh the
  manifest resolution, every existing receipt, and generated ledger from the
  exact committed runtime source. No other row advances.

## Acceptance

- Menu names the executed mounted test in the manifest and has one valid
  `nucleus.navigation.menu` M1 receipt.
- Replacing the production renderer with raw rows, losing roles/toggled state,
  bypassing mounted input, accepting disabled/separator rows, dropping the
  production check Icon, crossing host streams, or failing controlled checked
  state rebuild fails before receipt emission.
- Keyboard navigation skips inert rows in both directions, Home/End land on
  enabled boundaries, Enter/Space activate once, and Escape never activates a
  row. Host-owned dismissal is not inferred. If the panel cannot provide the
  command-list floor without API widening, return the exact gap to planning.
- Existing nine receipts remain valid. The denominator stays 29. M1 does not
  infer A1, V1, Nucleus adoption, browser trigger/overlay behavior, recursive
  submenus, or pixel-level parity.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production renderer owns structure | substitute a raw column and buttons | exact panel/item metadata fails |
| Flat Nucleus shape is honest | omit action, disabled, separator, or danger posture | exact row assertions fail before receipt |
| Selectable semantics are exact | collapse checkbox/radio into action rows | roles and toggled states fail |
| Icon dependency is real | replace the checked mark with a raw/text glyph | production Icon metadata fails |
| Input is mounted | invoke `on_action` directly | mounted observation or callback trace is absent |
| Controlled rebuild is real | record checkbox/radio action without rebuilding items | toggled state remains stale |
| Disabled and separator paths are inert | arm every row identically | callback trace gains a forbidden value |
| Keyboard navigation is exact | focus an inert row or stop at an enabled boundary | focus and action traces fail |
| Host ownership stays bounded | infer trigger/open/placement behavior from the panel | receipt validation or scope check fails |
| Receipt is terminal | fail any final mounted assertion | no Menu receipt is emitted |
| Evidence identity is exact | retain the g16.072 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

One new Menu mounted regression; focused Menu spec/render/backend tests; a
focused shared Rust or GPUI repair only if a biting mounted counterexample
requires it; receipt emission and exact manifest/receipt/ledger refresh; this
card; one execution log; and new papercuts. Do not edit Nucleus, web behavior,
public APIs, nested-menu pointer policy, accessibility authority, visual-lab
code, Jetstream, workflows, versions, releases, or other component rows.

## Validation

Run focused Menu spec/render/backend tests, the named mounted fixture, the real
`effigy regressions:native` receipt run,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof requires a public API, a second menu
machine, Nucleus application data, nested submenu policy, browser geometry,
pixel inspection, or accessibility claims unavailable from the headless
backend. Record an exact native keyboard/dismissal gap rather than weakening
the M1 receipt.

## Continuation

After merge, compile the Dialog M1 receipt child from the refreshed receipt
identity. Later Nucleus receipt cards remain serial.

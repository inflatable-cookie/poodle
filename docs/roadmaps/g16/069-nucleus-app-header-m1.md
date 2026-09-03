# g16.069 — Nucleus AppHeader M1 Receipt

Status: in-review
Type: Nucleus NP-1 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.067`, completed `g16.068`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/app-header.md`,
`../../contracts/components/icon.md`, `../../contracts/components/text.md`
Log: `../../logs/2026-09/20260903-g16-069-nucleus-app-header-receipt.md`
PR: https://github.com/inflatable-cookie/poodle/pull/175

## Goal

Produce one validated `M1` receipt for the Nucleus `AppHeader` row through the
production Rust render, Node, GPUI backend, and test-platform paths. Prove the
shell-header composition that becomes ready now that Icon and Text have exact
receipts. Keep native accessibility and visual acceptance separate.

## Fixed Boundary

- Build AppHeader only through `poodle_render::app_header`. Build its custom
  identity and other bounded slot content through the production Icon and Text
  renderers; a raw container may group slot children but may not replace a
  component renderer.
- Cover the default title/subtitle path and the custom-identity path. Cover
  center absence and presence: without center, actions and utility remain flat
  root regions; with center, they share the trailing grow column.
- Pin exact resolved background, bottom border, min-height, title/subtitle
  typography, density padding/gaps, fill-width posture, label fallback and
  explicit-label override against contract-owned spec/token values.
- Mount one centered custom-identity fixture containing the proven Icon and
  Text dependencies. Prove positive bounds, child containment, stable region
  ordering, and that the styled-only header root remains outside the focus
  chain.
- Dispatch a harmless pointer move through the mounted tree solely for the M1
  production-input observation. Do not invent AppHeader interaction or native
  window-drag behavior.
- Emit the AppHeader receipt only after every claimed assertion passes. Refresh
  the manifest resolution, every existing receipt, and the generated ledger
  from the exact committed runtime source. No other row advances.

## Acceptance

- AppHeader names one executed mounted test in the manifest and has one valid
  `nucleus.shell.app-header` M1 receipt.
- Replacing the production AppHeader renderer, dropping its Icon or Text slot
  dependency, flattening the centered trailing group, wrapping the uncentered
  regions, or changing an exact token/size/density mapping fails before receipt
  emission.
- Mounted bounds prove the identity, center, and trailing regions remain inside
  the header. The root has no focus handle and no activation.
- Existing Button, Icon, IconButton, Text, and Surface receipts remain valid.
  The denominator stays 29; M1 does not infer A1, V1, window dragging, responsive
  web breakpoints, or Nucleus adoption.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
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

## Writable Scope

One AppHeader production-path mounted fixture; exact AppHeader renderer/spec
tests; focused shared Rust or GPUI repair only if a mounted counterexample proves
it necessary; receipt emission and exact manifest/receipt/ledger refresh; this
card; one execution log; and new papercuts. Do not edit Nucleus, web behavior,
public APIs, accessibility authority, visual-lab code, Jetstream, workflows,
versions, releases, or other component rows.

## Validation

Run focused AppHeader spec/render/backend tests, the named mounted fixture, the
real `effigy regressions:native` receipt run,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof requires a new public API, Nucleus
application data, pixel inspection, platform window-drag hooks, or accessibility
claims unavailable from the headless backend.

## Continuation

After merge, compile the SplitView M1 child from its existing mounted regression
and the refreshed receipt identity. Later Nucleus receipt cards remain serial.

# g16.068 — Nucleus Text And Surface M1 Receipts

Status: in-review
Type: Nucleus NP-1 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.067`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/text.md`,
`../../contracts/components/surface.md`
Log: `../../logs/2026-09/20260903-g16-068-nucleus-text-surface-receipts.md`
PR: https://github.com/inflatable-cookie/poodle/pull/174

## Goal

Produce validated `M1` receipts for the Nucleus `Text` and `Surface` rows from
one bounded mounted composite through the production Rust render, Node, GPUI
backend, and test-platform paths. Keep style metadata and mounted layout proof
separate from accessibility and visual acceptance.

## Fixed Boundary

- Mount production `Text` children inside production `Surface` variants. Do
  not construct equivalent raw Nodes or GPUI div/text elements in the fixture.
- Text proof covers exact content plus resolved tone, size, weight,
  line-height, compact spacing, and clamped overflow metadata. It proves
  backend handoff and mounted text layout, not font rasterization or pixels.
- Surface proof covers panel/elevated resolution, border, radius, padding,
  shadow, child containment, and explicit region/group semantics on the Node.
  It proves backend handoff and mounted container layout, not a screenshot.
- Styled-only primitives remain non-focusable and emit no activation. Dispatch
  a harmless pointer move through the mounted tree solely to satisfy the M1
  production-input observation; do not invent component interaction.
- Emit separate deterministic `M1` receipts only at the terminal boundary
  after every claimed Text and Surface assertion passes.
- Refresh the manifest resolution, all existing receipts, and generated
  ledger from the exact committed runtime source. No other row advances.

## Acceptance

- `Text` and `Surface` each name the executed mounted test in the manifest and
  each have one validated receipt for their fixed Nucleus scenario.
- Removing either production renderer, replacing it with a raw Node, losing a
  required style/semantic field, or dropping the Surface child fails before
  receipt emission.
- Mounted bounds prove the composed Text remains inside the Surface and the
  styled-only roots do not enter the focus chain.
- A late assertion failure cannot leave a newly emitted Text or Surface
  receipt for the current source commit.
- Existing Button, Icon, and IconButton receipts remain valid. The Nucleus
  denominator stays 29; `M1` does not infer `A1`, `V1`, or adoption.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production recipes own metadata | replace Text or Surface with a raw Node | resolved token/style assertions fail; missing style descriptor and probe channels |
| Composition is real | omit the Text child from Surface | mounted containment assertion fails (`text_bounds` inside `surface_bounds`); `content.text-icon.text` probe fails |
| Styled-only stays inert | make either root focusable or activatable | `focus_handle_for` / `focus_state_for` returns `Some(_)`, failing focus chain assertion |
| Border width uses width resolver | query `resolve_space("border.width.default")` under `GpuiThemeProvider` | yields `0.0`, failing `assert_eq!(border.width, 1.0)` and suppressing `surface.channels.border` probe channel |
| Exact Surface metadata | swap panel/canvas tokens, alter mix ratios (96%/98%/74%), alter insets, or omit elevation shadow | exact `assert_eq!` on style descriptor fields (`background`, `border`, `shadow`, `corner_radii`, `padding`) fails |
| Exact Text metadata | collapse tone, size, weight, line-height, spacing, or clamp | exact metadata assertion fails |
| Receipt is terminal | fail the final mounted assertion | test panics before terminal `nucleus_receipts::emit_if_configured`; neither receipt emitted |
| Evidence identity is exact | retain stale receipt SHA | `validateNucleusReceipt` throws: `receipt source commit ... no longer matches the mounted runtime source` |
| Levels stay separate | label either receipt A1 or V1 | `validateNucleusReceipt` throws: `receipt proof level must be M1; A1 and V1 require separate evidence` |

## Writable Scope

Focused Text/Surface shared Rust or GPUI repair only if a mounted
counterexample proves it necessary; one mounted native fixture; receipt
emission and exact manifest/receipt/ledger refresh; this card; one execution
log; and new papercuts. Do not edit Nucleus, web behavior, public APIs,
accessibility authority, visual-lab code, Jetstream, workflows, versions,
release surfaces, or other component rows.

## Validation

Run focused Text/Surface Rust and backend tests, the named mounted composite,
the real `effigy regressions:native` receipt run,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof requires pixel inspection, a new
public API, synthetic Nucleus data, or accessibility claims unavailable from
the headless backend.

## Continuation

After merge, compile the next small NP-1 child from the validated manifest.
All later native receipt cards remain serial on the exact shared source and
receipt identity.

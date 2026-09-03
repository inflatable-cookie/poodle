# g16.074 — Nucleus Dialog M1 Receipt

Status: complete
Type: Nucleus NP-2 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.068`, completed `g16.073`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/dialog.md`, `../../contracts/components/surface.md`,
`../../contracts/components/button.md`
Log: `../../logs/2026-09/20260903-g16-074-nucleus-dialog-receipt.md`
PR: pending
Handoff: `../../handoffs/20260903-074500-g16-074-nucleus-dialog-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `Dialog` row through the
production Rust render, Node, GPUI backend, and test-platform paths. Prove the
small titled modal shape Nucleus uses: description, close affordance, bounded
body, and explicit cancel/confirm actions. Keep host-owned open state and the
`M1`/`A1`/`V1` evidence boundaries honest.

## Fixed Boundary

- Add one mounted regression named
  `dialog_dismissal_axes_and_controlled_rebuild_reach_the_mounted_backend` and
  set that exact name on the manifest row. Do not reuse SettingsShell,
  AlertDialog, FormDialog, or a raw Node fixture as Dialog evidence.
- Build the modal only through `poodle_render::dialog`. Use generic content,
  not Nucleus labels or application data. Compose the body Surface and the
  cancel/confirm Buttons through their production renderers so the manifest's
  already-proven dependencies remain real.
- Pin exact contract-owned backdrop fill and overlay posture; surface fill,
  border, radius, dialog elevation, width preset, max height, overflow,
  padding, column layout, and section spacing; title/description typography;
  close affordance metadata; body/action placement; and production dependency
  metadata.
- Mount through `HeadlessDriver`. Prove positive bounds, surface/content/action
  containment, and stable header-body-actions order. Dispatch close-button,
  backdrop, Escape, and action activation through the test platform. Do not
  call handlers directly.
- Keep the dismissal axes independent: close always requests close; backdrop
  requests close only when `dismiss_on_backdrop` is true; Escape requests close
  only when `dismiss_on_escape` is true. A click inside the surface must not
  become a backdrop dismissal. One accepted request reaches one host stream;
  the host then rebuilds without the dialog. A host refusal leaves the dialog
  mounted without emitting a second request.
- A focused renderer/backend/GPUI compatibility repair is allowed only when a
  committed mounted counterexample proves the current production path lacks a
  required route. Use the existing modal and dismiss-layer machinery. Stop if
  correctness needs a new public API, a second modal machine, or app-owned
  focus policy.
- `M1` does not certify the accessibility tree, modal background suppression,
  focus trap, initial-focus resolution, or focus restoration. Those remain
  `A1` work. It also does not certify pixels, animations, browser body-scroll
  locking, nested overlays, or Nucleus adoption.
- Emit the Dialog receipt only after every claimed assertion passes. Refresh
  the manifest resolution, every existing receipt, and generated ledger from
  the exact committed runtime source. No other row advances.

## Acceptance

- Dialog names the executed mounted test in the manifest and has one valid
  `nucleus.navigation.dialog` M1 receipt.
- Replacing the production Dialog, Surface, or Button path with raw nodes;
  bypassing mounted input; coupling Escape to backdrop policy; allowing an
  inside click to dismiss; accepting disabled action input; omitting the
  controlled host rebuild; or emitting the receipt before the final assertion
  fails the proof.
- Close, backdrop, Escape, cancel, and confirm traces are exact and single.
  Backdrop/Escape false postures remain inert. The accepted close path removes
  the modal only through host rebuild; refusal keeps it mounted.
- Existing ten receipts remain valid. The denominator stays 29. M1 does not
  infer A1, V1, Nucleus M2, browser modality, alert-dialog behavior, or
  pixel-level parity.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production renderer owns structure | substitute a raw backdrop/panel | exact modal metadata or mounted containment fails |
| Dependencies are real | replace Surface or Button with a raw container | renderer-owned metadata or mounted activation fails |
| Input is mounted | invoke a callback directly | mounted observation or request/action trace is absent |
| Dismissal axes stay separate | reuse backdrop policy for Escape | false/true policy matrix fails |
| Surface clicks stay inside | let panel activation bubble to backdrop | close trace gains an extra request |
| Controlled ownership is real | record close without rebuilding the supplied tree | dialog remains mounted after accepted close |
| Refusal is stable | remove the dialog on a refused request | mounted witness disappears |
| Disabled action is inert | arm every action identically | action trace gains a forbidden value |
| Receipt is terminal | fail the final mounted assertion | no Dialog receipt is emitted |
| Evidence identity is exact | retain the g16.073 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

One new Dialog mounted regression; focused Dialog spec/render/backend/GPUI
compatibility tests; a focused native repair only if a biting mounted
counterexample requires it; receipt emission and exact
manifest/receipt/ledger refresh; this card; one execution log; and new
papercuts. Do not edit Nucleus, web behavior, public APIs, accessibility
authority, visual-lab code, Jetstream, workflows, versions, releases, or other
component rows.

## Validation

Run focused Dialog spec/render/backend tests, the named mounted fixture, the
real `effigy regressions:native` receipt run,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof requires a public API, a second
modal machine, Nucleus application data, browser-only focus selectors, broad
accessibility claims, pixel inspection, or an app-owned focus controller.
Record the exact native gap rather than weakening the M1 receipt.

## Continuation

After merge, compile the Popover M1 receipt child from the refreshed receipt
identity. Later Nucleus receipt cards remain serial.

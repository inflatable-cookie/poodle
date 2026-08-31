# GPUI Drag Input Capability Gate

Status: open — operator decision required before `g16.025` promotion
Captured: 2026-08-31
Source: `g16.025` readiness gate against crates.io `gpui` 0.2.2

## Finding

The exact crates.io dependency exposes a dependable desktop mouse and keyboard
surface, but not a general pointer surface.

| Capability | GPUI 0.2.2 evidence | Readiness result |
| --- | --- | --- |
| Mouse | `MouseDownEvent`, `MouseMoveEvent`, `MouseUpEvent`, and `MouseExitEvent`; move reports `pressed_button` | supported |
| In-window drag capture | `on_drag` arms after the stock threshold; typed `on_drag_move` receives moves inside or outside the source hitbox; `on_mouse_up_out` receives release outside the source | supported observable result, not a public per-pointer capture handle |
| Keyboard | key down/up, actions, focus handles, and headless keystroke simulation | supported |
| Explicit cancellation | Escape can call public `App::stop_active_drag`; Poodle already cancels on host rebuild. There is no platform `PointerCancel` equivalent. | partial: authored/host cancellation supported; device cancellation event absent |
| Pen | public events carry no input-device or pointer-type identity | unavailable as a distinct sensor; OS mouse synthesis cannot prove pen semantics |
| Touch contact | no touch down/move/up/contact event exists. `TouchPhase` belongs only to `ScrollWheelEvent` / trackpad scrolling. | unavailable |
| Headless proof | test support simulates mouse events, arbitrary exposed `InputEvent`s, and keystrokes | mouse/keyboard proof available; no touch/pen proof can be authored |

Exact upstream evidence lives in `gpui` 0.2.2 `src/interactive.rs`,
`src/elements/div.rs`, `src/app.rs`, and `src/app/test_context.rs`. Poodle's
existing GPUI backend already uses `on_drag` / `on_drag_move`,
`on_mouse_up_out`, Escape cancellation, and real headless mouse dispatch in
`packages/gpui/node-backend/src/interaction.rs` and
`packages/gpui/preview/tests/headless_regressions.rs`.

## Consequence

Architecture 011 currently says touch, mouse, pen, and keyboard drive the
internal transport. The working rules treat a missing active-runtime
capability as a gap, not parity. `g16.025` therefore cannot honestly check off
touch, distinct pen behavior, or device-originated pointer cancellation on
stock GPUI 0.2.2.

A GPUI fork is forbidden. Reaching beneath GPUI for platform-specific AppKit,
Win32, or Linux pointer plumbing would create a second native input backend and
is outside the card.

## Recommended Decision

Proceed with `g16.025` as the stock-GPUI desktop baseline:

- advertise and certify mouse plus keyboard;
- use stock `on_drag_move` as the in-window capture-equivalent mechanism;
- certify Escape, release, source/target loss, host rebuild, and explicit host
  cancellation;
- advertise pen and touch as unsupported capabilities on GPUI 0.2.2 rather
  than treating synthesized mouse events as proof;
- keep touch fully required on the web runtime used by browsers, Electron, and
  Tauri webviews;
- retain GPUI pen/touch/device-cancel as named active-runtime debt, reopened
  when crates.io GPUI exposes those events.

This lets the shared Rust semantic substrate and real GPUI desktop behavior
land without a fork or a false parity claim. It narrows only the GPUI sensor
capability statement; it does not weaken the semantic lifecycle or web touch
contract.

## Alternatives

1. Keep `g16.025` blocked until upstream crates.io GPUI exposes touch, pen
   identity, and cancellation. This preserves literal sensor parity but stops
   the native programme indefinitely.
2. Build platform input beneath GPUI. Reject: it duplicates GPUI ownership,
   expands across three OS backends, and violates the card boundary.

## Promotion Route

If the operator accepts the recommendation, amend architecture 011, spec 069,
and `g16.025` with the explicit GPUI capability delta; add a review oracle that
forbids claiming pen/touch from mouse synthesis; mark `g16.025` ready and write
its worker handoff. Otherwise keep the card blocked at this gate.

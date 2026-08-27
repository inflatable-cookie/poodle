# g16.011 — IconButton Activation, Toggle, And Mounted Parity

Date: 2026-08-27
Status: complete — PR #85
Branch: `t3code/icon-button-worker-handoff`
Card: `docs/roadmaps/g16/011-icon-button-activation-toggle-and-mounted-parity.md`
Source triage: `docs/triage/20260827-125702-post-g16-010-native-lane-decision.md`

## Outcome

Shared Rust IconButton ignored `default_pressed`, could not report
`onPressedChange`, dropped tooltip text before `Node.tooltip`, and had no
named mounted GPUI proof. The renderer now matches the web command/toggle
order through a handler-bundle entry point while keeping the simple
`icon_button(..., on_click)` helper for composites.

The generated ledger moves only IconButton's GPUI mounted-behaviour cell:
`missing` → `mounted` (39 → 40 mounted, 135 → 134 missing). Known-delta
totals stay 115 present / 60 not-applicable. GPUI accessibility stays
`manual`. GPUI visual stays `missing`. Jetstream stays deferred.

## Repair

- Toggle mode is `is_pressed` or `default_pressed` present. Effective state is
  `is_pressed.or(default_pressed).unwrap_or(false)`.
- Available toggle activation reports the inverse boolean once, then invokes
  the command callback once. Command-only activation never manufactures a
  pressed change.
- The renderer stays stateless. The host owns the current pressed value and
  rebuilds after `on_pressed_change`. `default_pressed` is the seed.
- Disabled and loading targets emit nothing and are not sequential focus
  stops.
- The same square target owns button role, accessible name, tab position,
  structured focus ring, optional toggled state, and disclosure state.
- Explicit tooltip text wins; otherwise a non-empty `aria_label` reaches
  `Node.tooltip`. Empty text is omitted. GPUI native tooltip chrome owns
  timing, placement, and paint.
- `tooltip_placement` now defaults to `top`, matching the contract. Native
  does not consume placement yet.
- The GPUI compatibility wrapper exposes command and pressed-change handlers.
  Interactive specimen examples use that production path and keep the compact
  last-action / toggle feedback.

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#icon_button_activation_toggle_and_tooltip_through_mounted_pointer_and_keyboard`
proves, through production hit testing, focus, and key dispatch:

- pointer command activation emits once and does not invent a pressed change
- Enter then Space on a controlled toggle report the inverse value and rebuild
  the host's toggled state
- `default_pressed=true` starts toggled and first activation reports false
- explicit and fallback tooltip text reach `Node.tooltip`
- role, accessible name, tab position, focus ring, and disclosure state ride
  the same target
- disabled and loading targets never emit and are skipped by sequential focus

Fixture-local ids are stamped after render. No public instance-id prop was
added. Direct handler calls are used only in focused renderer tests.

## Explicit non-claims

- no Tooltip overlay, web timer/Escape lifecycle, placement paint, or
  `aria-describedby` parity
- no `aria-busy` and no broad native assistive-technology coverage
- no Svelte/React public behavior or prop change
- no variant, tone, size, density, spinner, or specimen-page redesign
- no EditableLabel, NumberInput, TimeInput, Pill, or IconButton-consuming
  composite work beyond compilation of the additive renderer entry point
- no Jetstream admission, visual comparison, or other ledger row

## Validation

Focused `poodle-specs` IconButton tests (1), `poodle-render` IconButton tests
(23), Svelte and React IconButton tests (12, unchanged), named mounted
IconButton regression. `effigy regressions:native` (86),
`effigy probe:gpui-specimens` (8), `effigy drift:handlers`,
`effigy drift:events`, `effigy docs:spec-drift`,
`effigy test:parity-evidence-ledger` (5), `effigy check:parity-evidence-ledger`
(175 rows), `effigy ci:native`, `effigy ci:web`, `effigy docs:check`,
`effigy qa`, and `git diff --check origin/main...HEAD`.

`effigy doctor` is already red on the planning base (generated-in-src,
god-files, stale-suppressions). That baseline is unchanged and was not
absorbed. Northstar rust-quality activation is not installed in this
repository; installing it would have mutated `AGENTS.md` outside this card.

## Remaining gaps

- native tooltip overlay, timer, Escape, and `aria-describedby` stay web
- native accessibility, visual comparison, and Jetstream admission are
  unchanged and unclaimed
- the next evidence lane is an orchestrator checkpoint against 40 mounted /
  134 missing

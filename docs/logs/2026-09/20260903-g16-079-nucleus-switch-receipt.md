# g16.079 — Nucleus Switch M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/079-nucleus-switch-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-115100-g16-079-nucleus-switch-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/switch.md`
Branch: `feature/g16-079-nucleus-switch-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-079-nucleus-switch-receipt`
Planning base: `6a3311924` (`origin/main`)
Worker PR: `https://github.com/inflatable-cookie/poodle/pull/183`

## Outcome

`Switch` now has a validated `M1` execution receipt emitted from the
production GPUI render, GPUI Switch compat adapter
(`node_compat::Switch::from_spec(...).into_element()`), node backend, and
test-platform path in `effigy regressions:native`. The retained regression
`switch_toggle_readonly_and_disabled_rebuild_the_host_spec` was strengthened —
not replaced — into the executed controlled composition.

The fixture proves, through `HeadlessDriver` mounted pointer/keyboard input
and host-owned rebuilds: exact switch role, accessible name projection from
explicit `aria_label`, visible `label`, and dual-label (`left_label` / `right_label`)
fallback compositions; checked and toggled state; track and thumb geometry
(md `2.25rem × 1.375rem` track, `1.125rem` thumb, `0.125rem` padding,
`0.875rem` travel, pill radii, drop shadows, inset highlight); custom
`on_color` / `off_color` hex override precedence over semantic tones
(`right_tone` / `left_tone`) for track fill, thumb color, and borders across
both checked and unchecked states; dual-label inactive tinting; mounted positive
bounds, non-overlapping vertical stack ordering (subject → witness → readonly → disabled → dual),
and production mount-box containment `(32, 32) + (400, 300)`; pointer activation and controlled host
rebuilds; keyboard toggle parity via both Space and Enter keys; disabled switch
focus handle suppression, opacity, interaction disablement, and pointer inertia
proven against a live `on_change` sink and stable host state; read-only focusability
with reverted mutations; and duplicate-content instances with caller-scoped runtime
IDs, isolated focus handles, and independent callback streams. The receipt is
emitted only after the terminal assertion.

The manifest, all 15 existing receipts (`AppHeader`, `Button`, `Dialog`,
`Icon`, `IconButton`, `Menu`, `Popover`, `RadioGroup`, `SegmentedControl`,
`Select`, `SplitView`, `Surface`, `Tabs`, `Text`, `TextInput`), and the new
Switch receipt pin the exact runtime source commit
`f12cf2a84b0dab16e871255dd5b949c3786a840d`. The ledger records 16 mounted
Nucleus rows out of 29.

## What landed

- Preparation (parallel, before the g16.078 merge):
  - `packages/contracts/components/src/switch.rs`: added unit tests for contract
    defaults, public builders, `is_dual_label`, `current_checked` fallback,
    and `SwitchTone` token mappings.
  - `packages/contracts/headless/src/switch.rs`: added unit tests for `switch_transition`
    (`Toggle` interactive/disabled/read-only and `SetChecked`).
  - `packages/gpui/preview/tests/headless_regressions.rs`: strengthened the retained
    regression to mount `node_compat::Switch::from_spec(...).into_element()` through
    `HeadlessDriver::new_element_in_box` across production structure proof,
    controlled multi-instance mount with live disabled sink, mounted geometry bounds,
    non-overlapping vertical stack ordering, `(32, 32) + (400, 300)` production mount-box
    containment, focus handle isolation, pointer activation, Space/Enter keyboard toggle parity,
    disabled/read-only paths, dual-label toggle, custom color precedence on thumb and borders,
    and terminal M1 receipt emission.
  - `packages/render/src/switch.rs`: repaired custom color vs semantic tone precedence
    and added unit tests covering thumb, track fill, and track border across checked
    and unchecked states.
- Finalization (after the g16.078 merge, on the rebased identity):
  - All 16 receipts re-emitted at runtime commit
    `f12cf2a84b0dab16e871255dd5b949c3786a840d` (Switch new; the rest
    re-pinned), the manifest resolution pinned to the same SHA, and the
    generated ledger moved exactly Switch's GPUI mounted-behaviour cell
    from `missing` to `mounted` (16 mounted, 159 missing).

## Focused repair

The committed counterexample proof (`0b5983370`, red at that commit) failed at
custom color precedence on the thumb background: when explicit `on_color` /
`off_color` hex values were supplied alongside non-default semantic tones,
the tone color token was resolved first for the thumb background and track border.

Repair (`f12cf2a84`): `poodle_render::switch` resolves explicit `on_color` and
`off_color` hex overrides first before falling back to `right_tone` and `left_tone`
semantic color tokens. Renderer unit tests pin custom color precedence for thumb,
track fill, and track border on both checked and unchecked states; the strengthened
mounted fixture passes completely. No public API, machine, web, or backend change
was needed.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::switch` directly | the fixture mounts `node_compat::Switch::from_spec(...).into_element()` through `HeadlessDriver::new_element_in_box`; adapter-stamped IDs (`poodle-switch-*`) drive bounds, focus handles, and dispatches |
| Identity is caller-scoped | reuse one runtime id | `poodle-switch-subject` and `poodle-switch-witness` alias focus handles and callback sinks |
| Switch semantics are exact | omit checked/readonly/disabled/name | Node assertion fails before receipt |
| Input is mounted | invoke `on_change` directly | mounted observation or callback trace is absent |
| Controlled ownership is real | record callback without rebuilding host state | every claim is driven through host-owned rebuilds; painted state stays stale without rebuild |
| Disabled/read-only differ | let disabled focus or read-only commit | disabled focus handle check fails; read-only mutation changes state; live disabled sink confirms no events |
| Pointer and keys agree | Space/Enter diverge from pointer | keyboard dispatches verify identical state mutation |
| Visual metadata is exact | alter tone/color precedence or track/thumb geometry | exact token/style assertion fails; committed counterexample failed at thumb color precedence |
| Mounted bounds & containment | reverse vertical stack order or exceed mount bounds | non-overlapping stack ordering `sub.y + sub.h <= wit.y <= ro.y <= dis.y <= dual.y` and `(32, 32) + (400, 300)` containment assertions fail |
| Receipt is terminal | fail the final isolation assertion | no Switch receipt emitted |
| Evidence identity is exact | emit before the g16.078 predecessor merge | cohort rebase re-pins every receipt and the manifest to `f12cf2a84`; receipts re-emitted at that exact source |
| Levels stay separate | label the receipt A1 or V1 | `scripts/nucleus-parity-receipts.test.ts` rejects `proof_level: "A1"` |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib switch::` — 5 passed
- `cargo test --manifest-path packages/contracts/headless/Cargo.toml --lib switch::` — 4 passed
- `cargo test --manifest-path packages/render/Cargo.toml --lib switch::` — 6 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions switch_toggle_readonly_and_disabled_rebuild_the_host_spec` — passed

Required boards:
- `effigy regressions:native` — 187 passed (all 16 receipts emitted at runtime commit `f12cf2a84`)
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed
- `effigy check:parity-evidence-ledger` — passed (176 component evidence rows)
- `effigy ci:rust` — clean
- `effigy ci:native` — clean
- `effigy docs:check` — clean
- `git diff --check origin/main...HEAD` — clean

## Limits

- Focuses only on Nucleus `Switch` M1 mounted parity evidence.
- Does not edit `docs/roadmaps/g16/001-nucleus-gpui-parity-programme.md` or any other g16 front door.
- Does not touch Nucleus, web components, Svelte, or React.
- Does not claim A1 (interaction accessibility audit) or V1 (visual regression).
- Never runs windowed or native-visual selectors.

# g16.078 — Nucleus RadioGroup M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/078-nucleus-radio-group-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-115000-g16-078-nucleus-radio-group-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/003-radio-group-native-identity-and-mounted-parity.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/radio-group.md`
Branch: `feature/g16-078-nucleus-radio-group-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-078-nucleus-radio-group-receipt`
Planning base: `a766972c` (`origin/main`)
Worker PR: `https://github.com/inflatable-cookie/poodle/pull/185`

## Outcome

`RadioGroup` now has a validated `M1` execution receipt emitted from the
production GPUI render, GPUI RadioGroup compat adapter
(`node_compat::RadioGroup::from_spec(...).into_element()`), node backend, and
test-platform path in `effigy regressions:native`. The retained regression
`radio_group_exclusive_focus_identity_and_disabled_paths` was strengthened —
not replaced — into the executed controlled composition.

The fixture proves, through `HeadlessDriver` mounted pointer/keyboard input
and host-owned rebuilds: exact radiogroup role, aria label, orientation, and
per-option radiobutton names (including the option `aria_label` override);
selected/toggled state and the single roving tab stop with unknown-value and
disabled-selected fallbacks; indicator/dot production structure (1.125rem
indicator, 0.5rem dot, radii, surface fill, 0.0625rem border), selected accent
and caller `selectedColor` precedence, label typography and text color from the
size ladder; disabled-option and disabled-group structure (single root opacity
dim, per-row cursor, no rings or wiring); orientation layout, gap, and density
tokens; mounted positive bounds, stacking/ordering, and containment in the
adapter-carried group root; pointer selection; Space activation whose painted
wiring follows controlled rebuilds (selected rows inert, unselected rows
activate); Arrow navigation with disabled-option skip, boundary wrap, and
cross-axis inertness; Home/End jumps to the first/last enabled option with
inert extremes; focus retention across rebuilds; group-disabled mounts with no
focus handles or callback; and two duplicate-content instances with
caller-scoped runtime IDs, focus handles, and callback streams. The receipt is
emitted only after the terminal assertion.

The manifest, all 14 existing receipts (`AppHeader`, `Button`, `Dialog`,
`Icon`, `IconButton`, `Menu`, `Popover`, `SegmentedControl`, `Select`,
`SplitView`, `Surface`, `Tabs`, `Text`, `TextInput`), and the new RadioGroup
receipt pin the exact runtime source commit
`7343e25a887fba68978f9cc763fca845a54cdd7a`. The ledger records 15 mounted
Nucleus rows out of 29.

## What landed

- Preparation (parallel, before the g16.077 merge):
  - `packages/gpui/preview/tests/headless_regressions.rs`: the retained
    two-instance test became the terminal M1 fixture — `Phase 0` production
    spec/token structure proof, then `Phase 1` vertical controlled mount,
    `Phase 2` horizontal mount, `Phase 3` disabled-group mount, and `Phase 4`
    duplicate-content instances with the terminal receipt emission. Mounted
    phases drive `node_compat::RadioGroup::from_spec(...).into_element()`
    through `HeadlessDriver::new_element_in_box` with host-owned spec/value
    state.
  - `packages/render/src/radio_group.rs`: roving Home/End support (below) and
    its unit test.
- Finalization (after the g16.077 merge, on the rebased identity):
  - All 15 receipts re-emitted at runtime commit
    `7343e25a887fba68978f9cc763fca845a54cdd7a` (RadioGroup new; the rest
    re-pinned), the manifest resolution pinned to the same SHA, and the
    generated ledger moved exactly RadioGroup's GPUI mounted-behaviour cell
    from `missing` to `mounted` (15 mounted, 160 missing).

## Focused repair

The committed counterexample proof (`4f74edbd7`, red at that commit) passed
every mounted claim on committed source except Home/End: after the wrap
sequence an `End` dispatch left the selection unchanged (5 value changes
instead of 6). RadioGroup's roving `axis_step` mapped only Arrow keys, so the
contract's Home/End jumps were inert.

Repair (`7343e25a8`): Home/End now map axis-independently to the first/last
enabled roving option; extremes stay inert through the existing same-value
guard. A renderer unit test pins the disabled-option skip and the inert
extremes; the mounted fixture's Home/End phases pass. No public API, machine,
web, or backend change was needed.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::radio_group` directly | the fixture mounts `node_compat::RadioGroup::from_spec(...).into_element()` through `HeadlessDriver::new_element_in_box`; dropping the adapter root id fails `expect("group root bounds")` |
| Identity is caller-scoped | compose both instances with one scope | `focus handle for radio:right:option:free never appeared` |
| Exclusive semantics are exact | bind activation on the selected row too | Phase 0 "re-picking the selected option is inert" fails |
| Roving input is exact | focus a disabled row or stop at the edge | disabled rows register no focus handle; End after wrap fails (5 vs 6 values) on the committed counterexample |
| Controlled ownership is real | record the callback without rebuilding host state | every claim is driven through host-owned rebuilds; painted Space wiring follows the rebuilt spec (selected inert, unselected fires) |
| Disabled/same-value are inert | include disabled options in the roving list, or let group disablement leave rows focusable | disabled-selected tab-stop fallback flips to `Some(-1)`; disabled-group `interaction.disabled` assertion fails |
| Axis is exact | accept cross-axis keys | "unrelated-axis arrows are inert on a vertical group": values grow |
| Visual metadata is exact | degrade the selected color or the label size ladder | selected-color and `text_size` (12.0 vs 13.0) assertions fail |
| Receipt is terminal | fail the final duplicate-instance isolation assertion | no RadioGroup receipt emitted |
| Evidence identity is exact | emit before the g16.077 predecessor merge | cohort rebase re-pins every receipt and the manifest to `7343e25a8`; receipts re-emitted at that exact source |
| Levels stay separate | label the receipt A1 or V1 | `scripts/nucleus-parity-receipts.test.ts` rejects `proof_level: "A1"` |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml` — 322 + 5 passed (incl. `radio_group_selects_default_option`)
- `cargo test --manifest-path packages/render/Cargo.toml radio_group` — 11 passed (incl. new Home/End unit test)
- `cargo test --manifest-path packages/gpui/node-backend/Cargo.toml` — 51 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions radio_group_exclusive_focus_identity_and_disabled_paths` — passed
- shared `single_select` machine untouched (no RadioGroup machine change)

Required boards:
- `effigy regressions:native` — 187 passed (all 15 receipts emitted at runtime commit `7343e25a8`)
- `bun test scripts/nucleus-parity-receipts.test.ts` — pending closeout
- `bun test scripts/parity-evidence-ledger.test.ts` — pending closeout
- `effigy check:parity-evidence-ledger` — pending closeout
- `effigy ci:rust` — pending closeout
- `effigy ci:native` — pending closeout
- `effigy docs:check` — pending closeout
- `git diff --check origin/main...HEAD` — pending closeout

No windowed or native-visual selectors were run.

## Limits

- `M1` proves the mounted production render/adapter/backend path: renderer
  metadata and tokens, controlled rebuilds, pointer/Space selection,
  Arrow/Home/End roving with disabled skip and wrap, disablement,
  orientation-exact axes, duplicate-instance isolation, and bounded layout. It
  does not claim `A1` (accessibility tree), `V1` (pixel comparison), browser
  form-name or native radio-group behavior parity, or Nucleus adoption.
- Merge remains orchestrator-owned.

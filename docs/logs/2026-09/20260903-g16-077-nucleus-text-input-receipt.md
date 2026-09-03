# g16.077 — Nucleus TextInput M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/077-nucleus-text-input-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-104800-g16-077-nucleus-text-input-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/007-text-input-controlled-editing-and-mounted-evidence.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/text-input.md`
Branch: `feature/g16-077-nucleus-text-input-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-077-nucleus-text-input-receipt`
Planning base: `0df1acaaf` (`origin/main`)

## Outcome

`TextInput` now has a validated `M1` execution receipt emitted from the
production GPUI render, GPUI TextInput compat adapter
(`node_compat::TextInput::from_spec(...).into_element()`), node backend, and
test-platform path in `effigy regressions:native`. The retained regression
`text_input_controlled_editing_and_identity_rebuild_the_host_spec`
was strengthened — not replaced — into the executed controlled composition.

The fixture proves, through `HeadlessDriver` mounted pointer/keyboard input
and host-owned rebuilds:
- Production renderer composition (Section 0): exact control height, density padding,
  surface background, border tokens, focus rings, hover borders, corner radii,
  affixes (prefix/suffix), icons (leading/trailing), character count (`6/10`),
  validation indicators, and accessibility semantics (`NodeRole::TextInput`, label,
  invalid state, described_by).
- Search clear button (Section 0 & 2): button role, label, focusability, and ordered
  value-before-clear mutation callbacks (`query/change:` before `query/clear`).
- Disabled & ReadOnly handling (Section 0, 1 & 2): disabled field lacks focus handle,
  has `CursorHint::NotAllowed` and disabled opacity, and remains inert to pointer/keyboard;
  read-only field accepts focus and selection but suppresses value mutation.
- Controlled editing, Unicode scalar `maxLength`, placeholder, and blur (Section 1):
  caret placement, printable text insertion, multi-byte astral emoji scalar length
  saturation (`earth 🌍` rejected when max is 7 scalars), selection overwriting,
  submit (Enter) and cancel (Escape) dispatch without value mutation, and single
  blur focus loss callback.
- Paired field identity (Section 3): two fields with equal initial values (`"same"`)
  maintain separate focus handles, caret positions, selection ranges, and undo histories.
  The receipt is emitted only after the terminal assertion.

The manifest, all 13 existing receipts (`AppHeader`, `Button`, `Dialog`,
`Icon`, `IconButton`, `Menu`, `Popover`, `SegmentedControl`, `Select`,
`SplitView`, `Surface`, `Tabs`, `Text`), and the new `TextInput` receipt pin the
exact runtime source commit `b654b46eaedd8f41988b80719fd3c489f2c5428c`. The ledger records
14 mounted Nucleus rows out of 29.

## What landed

- GPUI adapter:
  - `packages/gpui/preview/src/node_compat.rs`: added `on_submit`, `on_cancel`,
    and `on_clear` builder handlers to `node_compat::TextInput` and forwarded them
    into `poodle_render::TextInputHandlers`.
- Headless regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: strengthened
    `text_input_controlled_editing_and_identity_rebuild_the_host_spec` to mount
    via `node_compat::TextInput::from_spec(...).into_element()` through
    `HeadlessDriver::new_element_in_box` with host-owned rebuilds, covering
    unmounted composition, controlled editing, astral scalar maxLength, search
    clear button, disabled/read-only isolation, paired field identity, and terminal
    receipt emission via `nucleus_receipts::emit_if_configured`.

## Focused repair

No renderer or GPUI backend repairs were required. The existing editing and
rendering logic in `poodle_render` and `poodle_gpui_node_backend` already satisfied
all contract invariants.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::text_input` directly | the fixture mounts `node_compat::TextInput::from_spec(...).into_element()` through `HeadlessDriver::new_element_in_box` |
| Identity is caller-scoped | give equal-valued fields one id | focus, selection, history, or callbacks cross streams |
| Input is mounted | call edit transition or handlers directly | mounted observation token verification fails in `emit_if_configured` |
| Controlled ownership is real | record callback without rebuilding supplied state | painted value/selection disagrees with host state |
| Disabled/read-only differ | let read-only mutate or disabled focus | exact focus/edit trace fails |
| Scalar length is exact | enforce by UTF-16/code units | astral-character max-length vector fails |
| Placeholder is not value | copy or submit placeholder text | value/selection/clipboard witness fails |
| Commands are exact | mutate on Enter/Escape or double-fire | value and callback counts fail |
| Clear ordering is exact | emit clear before empty value | ordered effect trace fails |
| Blur/teardown are exact | double blur or emit on unmount | terminal callback trace fails |
| Structure and tokens are exact | drop semantic projection or field metadata | Node assertion fails |
| Receipt is terminal | fail final independent-field assertion | no TextInput receipt is emitted |
| Evidence identity is exact | retain the g16.076 source SHA | receipts and manifest pin `b654b46ea`; `currentSourceMatchesReceipt` diffs `SOURCE_PATHS` |
| Levels stay separate | label the receipt A1 or V1 | `scripts/nucleus-parity-receipts.test.ts` rejects `proof_level: "A1"` |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml text_input` — 3 passed
- `cargo test --manifest-path packages/render/Cargo.toml text_input::` — 9 passed
- `cargo test --manifest-path packages/gpui/node-backend/Cargo.toml` — 51 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions text_input_controlled_editing_and_identity_rebuild_the_host_spec` — passed
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed
- `bun test --timeout 120000 scripts/parity-evidence-ledger.test.ts` — 6 passed
- `bun scripts/parity-evidence-ledger.ts` — 176 component evidence rows validated

Required boards:
- `effigy regressions:native` — 187 passed (all 14 receipts emitted at runtime commit `b654b46ea`)
- `effigy check:parity-evidence-ledger` — passed (176 component evidence rows)
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

`TextInput` M1 proves mounted controlled editing, identity rebuilds, scalar maxLength,
affixes, adornments, clear commands, and focus/selection isolation in GPUI headless
regression. It does not claim browser DOM parity, OS IME composition parity, multiline,
rich text, A1 accessibility tree inspection, or V1 pixel comparisons.

# g16.065 — Tabs Native Tooltip Parity

Status: blocked — returned to planning; PR #169
Date: 2026-09-02
Card: `docs/roadmaps/g16/065-tabs-native-tooltip-parity.md`
Handoff: `docs/handoffs/20260902-225241-g16-065-tabs-tooltips.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `fix/g16-065-tabs-native-tooltips`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-065-tabs-native-tooltips`
PR: https://github.com/inflatable-cookie/poodle/pull/169

## Outcome

Stopped. Exact-head review of `ab225efb6` correctly rejected treating GPUI
0.2.2 `.tooltip()` limits as accepted parity deltas.

`shows_tooltips` now reaches `Node.tooltip`. Web overlay timing (300ms, blur,
Escape) is unchanged. Native does not meet the card: delay is GPUI's private
500ms, hide is hover-owned, and a disabled tab still projects a label. Those
are blocking gaps, not this-card deltas.

## Blocking Boundary

Existing production mapping: `Node.tooltip: Option<String>` → GPUI
`.tooltip()`. No new Node field was added.

| Card requirement | On this boundary |
| --- | --- |
| False inert | Yes — omit the field |
| Label projection | Yes — landed |
| 300ms delay | No — `TOOLTIP_SHOW_DELAY` is a private 500ms const; `.tooltip()` has no delay argument |
| Hide on leave | Yes — non-hoverable `.tooltip()` |
| Hide on focus departure | No — hitbox/hover only; `clear_active_tooltip` is `pub(crate)` |
| Hide on disable | Renderer could omit `Node.tooltip`; not a GPUI ceiling. Current code still projects, matching web. Card names hide. Unresolved |
| Removal / teardown | Yes — unpaint drops `WaitingForShow` |

Meeting 300ms plus focus-departure on the house path needs a planned public
contract (delay and dismiss policy on Node, or a backend tooltip runtime that
is not `.tooltip()`). This lane must not invent either.

## What this PR keeps

Projection work is coherent and stays. It is not completion.

- Spec: `TabsSpec::with_shows_tooltips`.
- Renderer: `tab_tooltip_text` projects onto `Node.tooltip` from
  `wire_collection_semantics` (card, pill, and block). Vertical strips project
  even when the flag is false. Empty labels are omitted. Disabled tabs still
  get the label (web wrap); that is not card hide.
- GPUI: frame-scoped `painted_tooltip_text()` recorded from the tooltip view
  and cleared in `overlay_frame_begin`.
- Mounted evidence: compact/`sm` chrome fixture (`Explorer`/`Search`/`Git`/
  `Terminal` icons), no Nucleus data. Named as GPUI-boundary evidence, not the
  card oracle. False inert at 500ms; silent at contract 300ms; paints at GPUI
  500ms; leave / removal-while-pending / teardown cancel; blur-while-hovered
  keeps the tooltip; disabled Git still paints.
- Paired Svelte/React `TabsTooltips` suites with fake timers (not in
  `Tabs.test.*`, which stub `requestAnimationFrame` as sync). Those prove the
  web contract, not native parity.

## Falsification

Green proofs first. Plants restored after each row.

| Row | Plant | Result |
| --- | --- | --- |
| Dropped projection | `node.tooltip = None` | `show_tooltips_true_projects_each_tab_label` expected `Some("Explorer")`, got `None` |
| Label-only projection | ignore `shows_tooltips` / vertical | `show_tooltips_false_does_not_project_hover_text` saw a tooltip |
| Skip delay | `wait_ms(0)` after the 300ms None row | mounted test expected `Some("Search")`, got `None` |
| Skip hide | omit leave `pointer_hover` | mounted test expected `None`, got `Some("Search")` |

Restored sources reran green for the renderer plants. Hide/delay plants were
restored in the mounted test before the native board.

## Validation

Focused (projection land):

- `bun run --cwd packages/core test test/tabs.test.ts` — 23 pass
- `cargo test --manifest-path packages/contracts/components/Cargo.toml shows_tooltips` — 1 pass
- renderer tooltip projection tests — 5 pass
- Svelte `TabsTooltips` + existing Tabs/controlled-focus files — 34 pass
- React `TabsTooltips` + existing Tabs/controlled-focus files — 34 pass
- `tabs_show_tooltips_delay_and_hide_through_mounted_gpui` — pass after restoring delay/hide plants

Boards (projection land):

- `effigy ci:web` — pass after warming `react:package` dist (first cold run failed three `react-preview` suites; papercut recorded)
- `effigy ci:rust` — pass
- `effigy ci:native` — pass, including 175 headless regressions
- `effigy docs:check` — pass
- `git diff --check` on the lane files — pass

Stop-condition rewrite:

- `tabs_node_tooltip_gpui_boundary_evidence` — pass, including blur-while-hovered still showing `Search`
- `effigy docs:check` — pass
- `git diff --check` — pass

## Limits

- Card acceptance is unmet. Do not merge this as native tooltip parity.
- No Nucleus edit, windowed/native-visual run, Jetstream, workflow, or shared
  g16 front-door change.
- No ad hoc Tabs overlay, `tooltip_delay` field, or second tooltip mechanism.

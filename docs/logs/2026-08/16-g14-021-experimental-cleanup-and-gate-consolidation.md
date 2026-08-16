# g14.021 — Rejected pilot cleanup and evidence retention

Status: complete
Date: 2026-08-16
Card: `docs/roadmaps/g14/021-experimental-cleanup-and-gate-consolidation.md`
Governing verdict: `docs/roadmaps/g14/008-pilot-verdict.md`
Recovery commit: `71160b8a` (`docs(g14): reject conformance pilot`)

## What landed

The rejected executable-conformance authority is gone. Every product and
backend defect it caught has a named owner beside the code it belongs to, the
five generated Rust declarations are hand-written again, the four web shells
that inferred their props from the pilot interface declare them, and
`effigy ci:conformance` is now a small honest native regression board.

No shipped API changed. No component behaviour changed. The 1,205 HistoryCenter
comparison failures are not "fixed" — the comparator that produced them is
deleted along with the plane it compared, and its evidence stays in `g14.008`.

## Defect ledger

Every defect credited to the pilot, and who owns the claim now. "Retired"
means the defect was in the pilot's own machinery, not in shipped code.

### Button (g14.001)

| Defect | Retained owner |
| --- | --- |
| GPUI double activation — one Enter fired the handler twice | `headless_regressions.rs::one_enter_activates_a_focused_control_exactly_once` |
| Native projected no `aria-pressed` / `aria-expanded` / focus-visible | `poodle-render` `button::tests::toggle_disclosure_and_focus_state_reach_the_accessibility_channel` |
| Native stamped no semantic token roles | `poodle-render` `button::tests::semantic_token_roles_are_stamped_with_resolved_values` |

### RangeSlider (g14.003)

| Defect | Retained owner |
| --- | --- |
| React missing `role="group"` around the two thumbs | React `RangeSlider.test.tsx` — "groups the two thumbs under one container" (+ the embedded pair) |
| React commit fired from continuous `onChange` | React `RangeSlider.test.tsx` — "commits a standard thumb only after its value changed" (pre-existing) |
| GPUI scrub never fired under `on_drag` + synthetic drag events | `headless_regressions.rs::a_scrub_reports_change_while_dragging_and_commits_once_at_release` |
| Stale `FOCUS_STATES` after remount onto disabled thumbs | **not retained** — harness-only. The fix was the pilot adapter's own blur-before-remount discipline; `focus_state_for` has no shipped consumer that reads it across a remount. |
| Empty-string root name on web vs `null` on native | **not retained** — an artefact of the normalized observation shape, not a component defect. |

### Tabs (g14.004)

| Defect | Retained owner |
| --- | --- |
| React manual activation left the focused tab outside the roving tab stop | React `TabsRovingFocus.test.tsx` (both directions: focus moves the stop, selection reseeds it) |
| Native Tabs declared focusability without joining backend tracked focus | `poodle-render` `tabs::tests::instance_scope_isolates_ids_relationships_and_focus_requests` (pre-existing) |
| Global native trigger ids collided across two tabsets | same test (pre-existing) |
| Missing native orientation on embedded RangeSlider controls | `poodle-render` `range_slider::tests::embedded_thumbs_announce_their_orientation` |
| Native token observation reported the unresolved base size | **retired** — the bug was in the pilot observer, not the renderer. `button::tests::semantic_token_roles_…` pins the resolved-value rule generally. |
| Driver completed arrow navigation itself; observer queried focus by semantic id | **retired** — pilot driver/observer defects. |

### Popover (g14.005)

| Defect | Retained owner |
| --- | --- |
| Nested overlay registration order — Escape dismissed the outer first | `packages/core/test/dismiss.test.ts` (16 cases incl. registered peers, portalled ancestry, three-level nesting) — pre-existing |
| Controlled `open` while `disabled` rendered the surface | `PopoverRetained.svelte.test.ts` / `PopoverRetained.test.tsx` — "keeps a controlled open request inert while disabled" |
| Node-backend panicked on a popover inside a popover (`defer_draw` during deferred drawing) | `headless_regressions.rs::overlay_layers_survive_independent_conversions_within_one_frame` |
| Nested open state rebuilt fresh each frame | **retired** — the pilot's GPUI adapter held the state, not shipped code. |

### TextInput (g14.006)

| Defect | Retained owner |
| --- | --- |
| Non-BMP selection boundary | `poodle-gpui-node-backend` `input_text` + `ime` tests (UTF-16 round trip, surrogate pairs, emoji caret) — pre-existing |
| Search clear emitted the wrong order | `TextInput.test.ts` / `.tsx` — "emits valueChange with the empty value before clear" |
| React multiline dropped `autocorrect` | React `TextInput.test.tsx` (pre-existing) |
| IME start/update must not commit; commit is one insert | `poodle-gpui-node-backend` `ime` tests (pre-existing) |
| Editing order (type then submit) | **retired** — asserted only through the corpus's event trace; the underlying paths are covered by the node-backend edit tests. |

### HistoryCenter (g14.007)

| Defect | Retained owner |
| --- | --- |
| `icon_button` carries no focus style, so the backend created no focus handle | `poodle-render` `history_center::tests::every_focusable_control_carries_the_ring_the_backend_keys_handles_on`; the `icon_button` gap itself is in `PAPERCUTS.md` |
| Roving focus moved the tab stop without moving backend focus | `poodle-render` `history_center::tests::the_tab_stop_follows_the_machines_roving_focus` |
| Shift dropped in the rename path; a key *name* appended as content | `poodle-render` `history_center::tests::the_rename_input_reports_keys_as_content_with_shift_and_space_intact` |
| Failed/loading status row only rendered when the list was empty | `HistoryCenter.test.ts` / `.tsx` — "still reports loading and failed status when rows are already listed" |
| `Popover` restored focus to the trigger wrapper, not the interactive control | `PopoverRetained.svelte.test.ts` / `.tsx` — "restores focus to the interactive trigger, not its wrapper" |
| `Select` options and `Menu` items carried no value attribute | `Select.test.ts` / `.tsx` and `Menu.test.ts` / `.tsx` — "addresses every option/item by its value" |
| Bounded list scrolls rather than growing | `poodle-render` `history_center::tests::the_list_is_bounded_and_scrolls` (pre-existing) |
| Hierarchy level reaches assistive tech | `history_center::tests::rows_carry_stable_identity_and_an_announced_level` + the web `aria-level` tests (pre-existing) |
| Escape ownership inverted between the rename input and the dismiss route | **host rule, not component behaviour.** The component emits both signals; which one claims the keystroke is the host's call. Owned on web by `HistoryCenter.test.ts` — "cancelling a picker rename restores the Select and returns focus to the pencil". No native host in this repo implements the guard; the pilot adapter did. |
| Web runner dropped relative-bounds geometry; `assert_part` could not assert absence; `contains: "text"` read only a Text node | **retired** — all three were defects in the pilot's own observers. |

### Infrastructure (g14.002, g14.023)

| Surface | Disposition |
| --- | --- |
| Headless GPUI in-memory test platform (no window, no focus theft, ~0.05s) | **kept** — `packages/gpui/preview/src/headless_driver.rs`, exercised by `tests/headless_regressions.rs` |
| Primitive capability roster, probes, and `primitive-capability-report.v1` | **retired** — 21 passing rows against a roster that still deferred `semantic.selected`, modified activation, key, scrub, drag/drop, context, and every input row. It certified less than the profile logs claimed. |
| `poodle-render::primitive_probes` | **retired** — it carried `probe_focus_gpui` / `probe_activate_gpui` and `gpui.*` evidence fields, the backend-neutrality failure `g14.008` named. |
| Native visual compare/refresh + `--control-size` (g14.002) | **kept** — `effigy test:native-visual` is a standing local workflow and owns it. |
| Planted-failure tests | **retired** — they tested the harness's ability to catch defects, not the product. |

## Restored authorities

Five generated Rust declarations are hand-written again, inlined into the
module that already owned their token recipes and derived queries:

- `poodle_specs::ButtonSpec` + `ButtonFit`
- `poodle_specs::PopoverSpec`
- `poodle_specs::TextInputSpec` (native caret and compatibility fields kept)
- `poodle_specs::HistoryCenterSpec` + `HistoryCenterStatus`, `HistoryCenterRejection`
- `TabsPortableSpec` had no shipped consumer — only the pilot adapter — so it
  and its `From<…> for TabsSpec` are deleted. `TabsSpec` is unchanged.

Four web shells declare their own props instead of inferring them from the
pilot interface, with identical public shapes: Svelte and React `Button`,
`Tabs`, `Popover`, `TextInput`. `RangeSlider` and `HistoryCenter` already had
this posture.

## Deleted

| Surface | Path |
| --- | --- |
| Portable interfaces, case corpora, primitive roster, corpus projection | `packages/core/src/conformance/**` |
| Serializer, cost script, primitive report | `packages/core/scripts/conformance-*.ts`, `primitive-capability-report.ts` |
| Generated interface/case/roster JSON | `packages/codegen/fixtures/conformance/**` |
| Conformance parser and Rust target | `packages/codegen/src/conformance.rs`, `src/targets/conformance_rust.rs`, CLI `--conformance`/`--cases` mode |
| Generated Rust declarations | `packages/contracts/components/src/generated/**`, `generated.rs` |
| Web corpus hosts, adapters, runner, observer, comparator | `test/conformance/**` |
| Native observer and primitive probes | `packages/render/src/conformance.rs`, `primitive_probes.rs` |
| GPUI corpus adapters, fixture support, probe driver, completion board | `packages/gpui/preview/src/conformance_*.rs`, `primitive_probes_gpui.rs`, `tests/conformance_headless.rs` |
| Case-authority test | `packages/core/test/component-case-authority.test.ts` |

`packages/codegen/src/targets/conformance.rs` and
`packages/codegen/generated/conformance/vectors.json` are the **g13 machine
vector** target. Untouched — different mechanism, same word.

## Retained selector scope

- `regressions:native` — the focused headless GPUI board (5 tests).
- `ci:conformance` — an alias for it, kept only because
  `.github/workflows/ci-conformance.yml` calls that name. Its output and its
  task comment state the scope: native regressions, nothing about portable
  interfaces, shared corpora, normalized observation, primitive certification,
  or six-profile completion.
- `docs:check` and `ci:web` no longer run `conformance:serialize-check`,
  `conformance:codegen-check`, `conformance:typecheck`, or
  `conformance:test-web`. Those selectors are gone.
- `ci:native` runs `regressions:native` in place of `conformance:test-gpui`.
- Deleted selectors: `conformance:serialize`, `:serialize-check`, `:codegen`,
  `:codegen-check`, `:build`, `:check`, `:typecheck`, `:test-web`,
  `:test-gpui`, `:test-primitives-rust`, `:primitives-report`, `:test`,
  `:compare`, `:complete`, `:cost`.

## For the orchestrator

`.github/workflows/ci-conformance.yml` is now misdescribed: its path filters
name deleted directories (`packages/core/src/conformance/**`,
`test/conformance/**`, `packages/core/scripts/conformance-*`), so it will stop
triggering on anything that matters, and its comment still calls it "the
executed active-cohort conformance gate". Its `effigy ci:conformance`
entrypoint stays valid, so nothing breaks. Workflow edits need explicit
operator approval; `g14.022` decides whether to rename or delete it.

## Validation

- `effigy ci:conformance` — 5 native regressions pass
- `effigy check:gpui` — pass
- `effigy check:svelte` — 0 errors
- `effigy react:build` — pass
- `effigy docs:check` — pass
- `bunx vitest run --project svelte-components --project react-components`
- `cargo test -p poodle-render`, `-p poodle-specs`, codegen tests
- `effigy doctor`, `effigy scan orphans`, `git diff --check`
- No `*-windowed` selector run. No Jetstream selector run.
